use axum_extra::extract::WithRejection;
use axum_oidc::OidcRpInitiatedLogout;
use tokio::{signal, task::AbortHandle};

use crate::{config::AppConfig, error::WithPageRejection};

pub async fn logout(
    WithRejection(logout, _): WithPageRejection<OidcRpInitiatedLogout>,
) -> OidcRpInitiatedLogout {
    logout.with_post_logout_redirect(AppConfig::get().oidc.application_base_url.parse().unwrap())
}

pub async fn graceful_shutdown(deletion_task_abort_handle: AbortHandle) {
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
        _ = ctrl_c => { deletion_task_abort_handle.abort() },
        _ = terminate => { deletion_task_abort_handle.abort() },
    }
}
