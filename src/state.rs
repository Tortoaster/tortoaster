use std::time::Duration;

use axum::extract::FromRef;
use axum_oidc::OidcAuthLayer;
use backoff::ExponentialBackoff;
use sqlx::PgPool;
use strum::IntoEnumIterator;
use tokio::task::AbortHandle;
use tower_sessions_redis_store::fred::{clients::RedisPool, interfaces::ClientLike};
use tracing::{info, warn};

use crate::{
    config::{AppBucket, AppConfig},
    repository::{files::FileRepository, projects::ProjectsRepository},
    user::AppClaims,
};

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: PgPool,
    pub s3_client: aws_sdk_s3::Client,
    pub redis_pool: RedisPool,
}

impl AppState {
    pub async fn new() -> (Self, OidcAuthLayer<AppClaims>, AbortHandle) {
        let config = AppConfig::get();

        let pool = backoff::future::retry_notify(
            ExponentialBackoff::default(),
            || async {
                let pool = PgPool::connect(config.database_url()).await?;
                Ok(pool)
            },
            |error, duration: Duration| {
                warn!("failed to connect to database: {error}");
                warn!("retrying in {} seconds", duration.as_secs());
            },
        )
        .await
        .expect("failed to connect to database");

        info!("connected to database");

        let s3_client = aws_sdk_s3::Client::from_conf(
            aws_sdk_s3::config::Builder::from(&config.s3_config().await)
                .force_path_style(true)
                .build(),
        );

        let output = backoff::future::retry_notify(
            ExponentialBackoff::default(),
            || async {
                let exists = s3_client.list_buckets().send().await?;
                Ok(exists)
            },
            |error, duration: Duration| {
                warn!("failed to find s3 instance: {error}");
                warn!("retrying in {} seconds", duration.as_secs());
            },
        )
        .await
        .expect("failed to find s3 instance");
        let buckets = output.buckets();

        info!(
            "found s3 instance with {} bucket(s): {}",
            buckets.len(),
            buckets
                .iter()
                .map(|bucket| bucket.name.as_deref().unwrap_or("<unnamed>"))
                .collect::<Vec<_>>()
                .join(", ")
        );

        for app_bucket in AppBucket::iter() {
            assert!(
                buckets
                    .iter()
                    .any(|bucket| bucket.name() == Some(&app_bucket)),
                "{} bucket not found",
                &*app_bucket
            );
        }

        let oidc_auth_layer = backoff::future::retry_notify(
            ExponentialBackoff::default(),
            || async {
                let oidc_auth_layer = OidcAuthLayer::<AppClaims>::discover_client(
                    config
                        .oidc
                        .redirect_url
                        .parse()
                        .expect("invalid application base url"),
                    config.oidc.issuer_url.clone(),
                    config.oidc.client_id.clone(),
                    config.oidc.client_secret.clone(),
                    vec![],
                )
                .await?;
                Ok(oidc_auth_layer)
            },
            |error, duration: Duration| {
                warn!("failed to discover oidc client: {error}");
                warn!("retrying in {} seconds", duration.as_secs());
            },
        )
        .await
        .expect("failed to discover oidc client");

        let (redis_pool, redis_handle) = backoff::future::retry_notify(
            ExponentialBackoff::default(),
            || async {
                let redis_pool = RedisPool::new(config.cache_config(), None, None, None, 6)?;
                let redis_handle = redis_pool.init().await?;
                Ok((redis_pool, redis_handle))
            },
            |error, duration: Duration| {
                warn!("failed to connect to redis: {error}");
                warn!("retrying in {} seconds", duration.as_secs());
            },
        )
        .await
        .expect("failed to connect to redis");

        let state = AppState {
            pool,
            s3_client,
            redis_pool,
        };

        (state, oidc_auth_layer, redis_handle.abort_handle())
    }
}

impl FromRef<AppState> for ProjectsRepository {
    fn from_ref(input: &AppState) -> Self {
        Self::new(input.pool.clone(), FileRepository::from_ref(input))
    }
}

impl FromRef<AppState> for FileRepository {
    fn from_ref(input: &AppState) -> Self {
        Self::new(input.s3_client.clone(), input.redis_pool.clone())
    }
}
