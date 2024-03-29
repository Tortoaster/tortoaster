use axum::{routing::get, Router};
use axum_extra::routing::RouterExt;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing::info;

use crate::{config::AppConfig, error::PageError, state::AppState};

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

    let state = AppState::new().await.expect("failed to initialize state");

    let app = Router::new()
        .route("/projects/:id", get(api::projects::get_project))
        .typed_get(api::projects::list_projects_page)
        .typed_get(api::projects::list_projects_partial)
        .nest_service("/static", ServeDir::new("static"))
        .fallback(|| async { PageError::NotFound })
        .with_state(state);
    let addr = config.socket_addr();
    let listener = TcpListener::bind(addr).await.unwrap();

    info!("listening on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}
