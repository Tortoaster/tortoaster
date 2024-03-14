use axum::{routing::get, Router};
use axum_extra::routing::RouterExt;
use sqlx::migrate;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;

use crate::{config::AppConfig, error::FullPageError, state::AppState};

mod api;
mod config;
mod error;
mod model;
mod pagination;
mod render;
mod repository;
mod state;

#[tokio::main]
async fn main() {
    let config = AppConfig::get();

    tracing_subscriber::fmt()
        .with_env_filter(config.env_filter())
        .init();

    let state = AppState::new().await.expect("failed to initialize state");

    migrate!()
        .run(&state.pool)
        .await
        .expect("failed to run database migrations");

    let app = Router::new()
        .route("/", get(api::index))
        .route("/projects/:id", get(api::projects::project))
        .route("/auth/login", get(api::auth::login))
        .route("/auth/register", get(api::auth::register))
        .typed_get(api::projects::list_projects)
        .nest_service("/static", ServeDir::new("static"))
        .fallback(|| async { FullPageError::NotFound })
        .with_state(state);
    let addr = config.socket_addr();
    let listener = TcpListener::bind(addr).await.unwrap();

    info!("listening on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
