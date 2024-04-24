use axum::{extract::DefaultBodyLimit, response::Redirect, routing::get, Router};
use axum_s3::ServeBucket;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;

use crate::{
    api::projects::ListProjectsUrl,
    config::AppConfig,
    error::{AppError, PageError},
    state::AppState,
};

mod api;
mod config;
mod dto;
mod error;
mod model;
mod pagination;
mod render;
mod repository;
mod state;
mod template;

#[tokio::main]
async fn main() {
    let config = AppConfig::get();

    tracing_subscriber::fmt()
        .with_env_filter(config.env_filter())
        .init();

    let state = AppState::new().await;

    let app = Router::new()
        .route(
            "/",
            get(|| async { Redirect::permanent(&ListProjectsUrl.to_string()) }),
        )
        .merge(api::projects::router())
        .nest_service("/static", ServeDir::new("static"))
        .nest_service(
            "/thumbnails",
            ServeBucket::new(state.s3_client.clone(), &config.buckets().thumbnails),
        )
        .fallback(|| async { PageError(AppError::NotFound) })
        .layer(DefaultBodyLimit::max(1024 * 1024 * 5))
        .with_state(state);
    let addr = config.socket_addr();
    let listener = TcpListener::bind(addr).await.unwrap();

    info!("listening on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
