use axum::{
    extract::rejection::QueryRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::extract::WithRejection;
use thiserror::Error;

use crate::{render::Render, template::error::ErrorToast};

/// Helper type that displays a toast message on error, for dynamically loaded
/// content.
pub type ToastResult<T> = Result<T, ToastError>;

/// Helper type that turns a rejection into a toast message on error, for
/// dynamically loaded content.
pub type WithToastRejection<E> = WithRejection<E, ToastError>;

/// Global error type that represents common errors that can occur anywhere in
/// the project. This type implements [`IntoResponse`] to make it easy to
/// generate error messages in toasts for dynamically generated content.
#[derive(Debug, Error)]
pub enum ToastError {
    #[error("I couldn't access the database! :(")]
    Database(#[from] sqlx::Error),
    #[error("I couldn't access the database! :(")]
    Database2(#[from] sea_orm::DbErr),
    #[error("template error: {0}")]
    Template(#[from] askama::Error),
    #[error("I couldn't do what you asked! :(\n{0}")]
    Validate(#[from] axum_valid::ValidRejection<QueryRejection>),
}

impl ToastError {
    fn status_code(&self) -> StatusCode {
        match self {
            ToastError::Database(_) | ToastError::Database2(_) => StatusCode::SERVICE_UNAVAILABLE,
            ToastError::Template(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ToastError::Validate(_) => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for ToastError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let message = self.to_string();

        let template = ErrorToast { message };

        (status_code, Render(template)).into_response()
    }
}
