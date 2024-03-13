use std::time::Duration;

use axum::extract::FromRef;
use backoff::ExponentialBackoff;
use sqlx::PgPool;
use tracing::warn;

use crate::{config::AppConfig, error::AppResult, repository::auth::AuthRepository};

#[derive(Clone, Debug)]
pub struct AppState {
    pub pool: PgPool,
}

impl AppState {
    pub async fn new() -> AppResult<Self> {
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
        .await?;

        Ok(AppState { pool })
    }
}

impl FromRef<AppState> for PgPool {
    fn from_ref(input: &AppState) -> Self {
        input.pool.clone()
    }
}

impl FromRef<AppState> for AuthRepository {
    fn from_ref(input: &AppState) -> Self {
        AuthRepository::new(input.pool.clone())
    }
}
