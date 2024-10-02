use std::time::Duration;

use axum::extract::FromRef;
use axum_oidc::OidcAuthLayer;
use backoff::{ExponentialBackoff, ExponentialBackoffBuilder};
use sqlx::PgPool;
use strum::IntoEnumIterator;
use tokio::{join, task::AbortHandle};
use tower_sessions_redis_store::fred::{
    clients::RedisPool, interfaces::ClientLike, types::ConnectHandle,
};
use tracing::{info, warn};

use crate::{
    config::{AppBucket, AppConfig},
    repository::{
        comments::CommentRepository, files::FileRepository, projects::ProjectRepository,
        users::UserRepository,
    },
    utils::claims::AppClaims,
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

        let (pool, s3_client, oidc_auth_layer, redis_pool) = join!(
            init_pool(config),
            init_s3_client(config),
            init_oidc_auth_layer(config),
            init_redis_pool(config)
        );

        let (pool_ok, pool_err) = split(pool);
        let (s3_client_ok, s3_client_err) = split(s3_client);
        let (oidc_auth_layer_ok, oidc_auth_layer_err) = split(oidc_auth_layer);
        let (redis_pool_ok, redis_pool_err) = split(redis_pool);

        let errors = [pool_err, s3_client_err, oidc_auth_layer_err, redis_pool_err]
            .iter()
            .flatten()
            .copied()
            .collect::<Vec<_>>()
            .join(", ");

        if !errors.is_empty() {
            panic!("fatal: {}", errors);
        }

        let pool = pool_ok.unwrap();
        let s3_client = s3_client_ok.unwrap();
        let oidc_auth_layer = oidc_auth_layer_ok.unwrap();
        let (redis_pool, redis_handle) = redis_pool_ok.unwrap();

        let state = AppState {
            pool,
            s3_client,
            redis_pool,
        };

        (state, oidc_auth_layer, redis_handle.abort_handle())
    }
}

fn split<T, E>(result: Result<T, E>) -> (Option<T>, Option<E>) {
    match result {
        Ok(value) => (Some(value), None),
        Err(error) => (None, Some(error)),
    }
}

async fn init_pool(config: &AppConfig) -> Result<PgPool, &'static str> {
    let pool = backoff::future::retry_notify(
        backoff_config(),
        || async {
            let pool = PgPool::connect_with(config.pg_connect_options()).await?;
            Ok(pool)
        },
        |error, duration: Duration| {
            warn!("failed to connect to database: {error}");
            warn!("retrying in {} seconds", duration.as_secs());
        },
    )
    .await
    .map_err(|_| "failed to connect to database")?;

    info!("connected to database");

    Ok(pool)
}

async fn init_s3_client(config: &AppConfig) -> Result<aws_sdk_s3::Client, &'static str> {
    let s3_client = aws_sdk_s3::Client::from_conf(
        aws_sdk_s3::config::Builder::from(&config.s3_config().await)
            .force_path_style(true)
            .build(),
    );

    let output = backoff::future::retry_notify(
        backoff_config(),
        || async {
            let exists = s3_client.list_buckets().send().await?;
            Ok(exists)
        },
        |error, duration: Duration| {
            warn!("failed to find s3 instance: {error:?}");
            warn!("retrying in {} seconds", duration.as_secs());
        },
    )
    .await
    .map_err(|_| "failed to find s3 instance")?;
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
        if !buckets
            .iter()
            .any(|bucket| bucket.name() == Some(&app_bucket))
        {
            return Err("not all necessary buckets found");
        }
    }

    Ok(s3_client)
}

async fn init_oidc_auth_layer(
    config: &AppConfig,
) -> Result<OidcAuthLayer<AppClaims>, &'static str> {
    let oidc_auth_layer = backoff::future::retry_notify(
        backoff_config(),
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
    .map_err(|_| "failed to discover oidc client")?;

    info!("discovered oidc client");

    Ok(oidc_auth_layer)
}

async fn init_redis_pool(config: &AppConfig) -> Result<(RedisPool, ConnectHandle), &'static str> {
    let (redis_pool, redis_handle) = backoff::future::retry_notify(
        backoff_config(),
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
    .map_err(|_| "failed to connect to redis")?;

    info!("connected to redis");

    Ok((redis_pool, redis_handle))
}

fn backoff_config() -> ExponentialBackoff {
    ExponentialBackoffBuilder::new()
        .with_initial_interval(Duration::from_secs(1))
        .with_multiplier(2.0)
        .with_max_interval(Duration::from_secs(5))
        .with_randomization_factor(0.0)
        .with_max_elapsed_time(Some(Duration::from_secs(32)))
        .build()
}

impl FromRef<AppState> for PgPool {
    fn from_ref(input: &AppState) -> Self {
        input.pool.clone()
    }
}

impl FromRef<AppState> for ProjectRepository {
    fn from_ref(input: &AppState) -> Self {
        Self::new(input.pool.clone(), FileRepository::from_ref(input))
    }
}

impl FromRef<AppState> for CommentRepository {
    fn from_ref(input: &AppState) -> Self {
        Self::new(input.pool.clone(), UserRepository::from_ref(input))
    }
}

impl FromRef<AppState> for FileRepository {
    fn from_ref(input: &AppState) -> Self {
        Self::new(input.s3_client.clone(), input.redis_pool.clone())
    }
}

impl FromRef<AppState> for UserRepository {
    fn from_ref(input: &AppState) -> Self {
        Self::new(input.pool.clone())
    }
}
