use std::time::Duration;

use axum::{
    error_handling::HandleErrorLayer,
    extract::DefaultBodyLimit,
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use axum_extra::routing::RouterExt;
use axum_oidc::{error::MiddlewareError, OidcAuthLayer, OidcLoginLayer};
use axum_s3::ServeBucket;
use tokio::{net::TcpListener, signal, task::AbortHandle};
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
    user::AppClaims,
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
mod user;

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
            .continuously_delete_expired(Duration::from_secs(1800)),
    );

    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(time::Duration::minutes(30)));

    let tortoaster_handle_error_layer = HandleErrorLayer::new(|e: MiddlewareError| async {
        PageError(AppError::Session(e)).into_response()
    });

    let oidc_login_service = ServiceBuilder::new()
        .layer(tortoaster_handle_error_layer.clone())
        .layer(OidcLoginLayer::<AppClaims>::new());

    let oidc_auth_service = ServiceBuilder::new()
        .layer(tortoaster_handle_error_layer)
        .layer(
            OidcAuthLayer::<AppClaims>::discover_client(
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
            .await
            .expect("failed to discover oidc client"),
        );

    let app = Router::new()
        // Login required
        .merge(api::projects::protected_router())
        .typed_get(api::auth::login)
        .layer(oidc_login_service)
        // Login optional
        .merge(api::projects::public_router())
        .typed_get(api::auth::logout)
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
        .with_graceful_shutdown(graceful_shutdown(deletion_task.abort_handle()))
        .await
        .unwrap();
}

pub async fn graceful_shutdown(abort_handle: AbortHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => { abort_handle.abort() },
        _ = terminate => { abort_handle.abort() },
    }
}
