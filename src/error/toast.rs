use axum::response::{IntoResponse, Response};
use axum_extra::extract::WithRejection;
use tracing::error;

use crate::{error::AppError, render::Render, template::error::ErrorToast};

/// Helper type that displays a toast message on error, for dynamically loaded
/// content.
pub type ToastResult<T> = Result<T, ToastError>;

/// Helper type that turns a rejection into a toast message on error, for
/// dynamically loaded content.
pub type WithToastRejection<E> = WithRejection<E, ToastError>;

/// Global error type that represents common errors that can occur anywhere in
/// the project. This type implements [`IntoResponse`] to make it easy to
/// generate error messages in toasts for dynamically generated content.
#[derive(Debug)]
pub struct ToastError(pub AppError);

impl<T: Into<AppError>> From<T> for ToastError {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl IntoResponse for ToastError {
    fn into_response(self) -> Response {
        let status_code = self.0.status_code();
        let message = self.0.to_string();

        match self.0 {
            AppError::NotFound => (),
            _ => error!("user encountered {status_code}:\n{:?}", self.0),
        }

        let template = ErrorToast { message };

        (status_code, Render(template)).into_response()
    }
}
