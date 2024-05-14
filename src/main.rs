use std::time::Duration;

use axum::{
    error_handling::HandleErrorLayer,
    extract::DefaultBodyLimit,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_oidc::{error::MiddlewareError, EmptyAdditionalClaims, OidcAuthLayer, OidcLoginLayer};
use axum_s3::ServeBucket;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::services::ServeDir;
use tower_sessions::{cookie::SameSite, ExpiredDeletion, Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::PostgresStore;
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
mod session;
mod state;
mod template;

#[tokio::main]
async fn main() {
    let config = AppConfig::get();

    tracing_subscriber::fmt()
        .with_env_filter(config.env_filter())
        .init();

    let state = AppState::new().await;

    let session_store = PostgresStore::new(state.pool.clone())
        .with_schema_name("public")
        .unwrap()
        .with_table_name("sessions")
        .unwrap();

    let deletion_task = tokio::task::spawn(
        session_store
            .clone()
            .continuously_delete_expired(Duration::from_secs(3600)),
    );

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(time::Duration::hours(2)));

    let tortoaster_handle_error_layer = HandleErrorLayer::new(|e: MiddlewareError| async {
        PageError(AppError::Session(e)).into_response()
    });

    let oidc_login_service = ServiceBuilder::new()
        .layer(tortoaster_handle_error_layer.clone())
        .layer(OidcLoginLayer::<EmptyAdditionalClaims>::new());

    let oidc_auth_service = ServiceBuilder::new()
        .layer(tortoaster_handle_error_layer)
        .layer(
            OidcAuthLayer::<EmptyAdditionalClaims>::discover_client(
                config
                    .oidc
                    .application_base_url
                    .parse()
                    .expect("invalid application base url"),
                config.oidc.issuer.clone(),
                config.oidc.client_id.clone(),
                config.oidc.client_secret.clone(),
                vec![],
            )
            .await
            .expect("failed to discover oidc client"),
        );

    let app = Router::new()
        // Login required
        .merge(api::projects::protected_router())
        .layer(oidc_login_service)
        // Login optional
        .merge(api::projects::public_router())
        .route("/logout", get(session::logout))
        .layer(oidc_auth_service)
        // Publicly available
        .route(
            "/",
            get(|| async { Redirect::permanent(&ListProjectsUrl.to_string()) }),
        )
        .nest_service("/static", ServeDir::new("static"))
        .nest_service(
            "/thumbnails",
            ServeBucket::new(state.s3_client.clone(), &config.buckets().thumbnails),
        )
        .fallback(|| async { PageError(AppError::NotFound) })
        .layer(DefaultBodyLimit::max(1024 * 1024 * 5))
        .layer(session_layer)
        .with_state(state);

    let addr = config.socket_addr();
    let listener = TcpListener::bind(addr).await.unwrap();

    info!("listening on http://{addr}");
    axum::serve(listener, app)
        .with_graceful_shutdown(session::graceful_shutdown(deletion_task.abort_handle()))
        .await
        .unwrap();
}
