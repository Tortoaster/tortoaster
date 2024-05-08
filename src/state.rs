use std::{sync::Arc, time::Duration};

use axum::extract::FromRef;
use backoff::ExponentialBackoff;
use sqlx::PgPool;
use tracing::{info, warn};

use crate::{config::AppConfig, repository::projects::ProjectsRepository};

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: PgPool,
    pub s3_client: Arc<aws_sdk_s3::Client>,
}

impl AppState {
    pub async fn new() -> Self {
        let config = AppConfig::get();

        let pool = backoff::future::retry_notify(
            ExponentialBackoff::default(),
            || async {
                let pool = PgPool::connect(&config.database_url).await?;
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

        let s3_client = Arc::new(aws_sdk_s3::Client::from_conf(
            aws_sdk_s3::config::Builder::from(&config.s3_config().await)
                .force_path_style(true)
                .build(),
        ));

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

        assert!(
            buckets
                .iter()
                .any(|bucket| bucket.name() == Some(&config.buckets().thumbnails)),
            "{} bucket not found",
            config.buckets().thumbnails
        );

        assert!(
            buckets
                .iter()
                .any(|bucket| bucket.name() == Some(&config.buckets().content)),
            "{} bucket not found",
            config.buckets().content
        );

        AppState { pool, s3_client }
    }
}

impl FromRef<AppState> for PgPool {
    fn from_ref(input: &AppState) -> Self {
        input.pool.clone()
    }
}

impl FromRef<AppState> for ProjectsRepository {
    fn from_ref(input: &AppState) -> Self {
        ProjectsRepository::new(input.pool.clone())
    }
}

impl FromRef<AppState> for Arc<aws_sdk_s3::Client> {
    fn from_ref(input: &AppState) -> Self {
        input.s3_client.clone()
    }
}
