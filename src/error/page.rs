use axum::response::{IntoResponse, Response};
use axum_extra::extract::WithRejection;
use tracing::error;

use crate::{
    error::AppError,
    template::{error::ErrorPage, Render},
};

/// Helper type that displays a full page with status code and message on error.
pub type PageResult<T> = Result<T, PageError>;

/// Helper type that turns a rejection into a full page error.
pub type WithPageRejection<E> = WithRejection<E, PageError>;

/// Global error type that represents common errors that can occur anywhere in
/// the project. This type implements [`IntoResponse`] to make it easy to
/// generate full-page error responses.
#[derive(Debug)]
pub struct PageError(pub AppError);

impl<T: Into<AppError>> From<T> for PageError {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl IntoResponse for PageError {
    fn into_response(self) -> Response {
        let status_code = self.0.status_code();
        let message = self.0.to_string();

        match self.0 {
            AppError::NotFound => (),
            _ => error!("user encountered {status_code}:\n{:?}", self.0),
        }

        // TODO: Don't supply user at all
        let template = ErrorPage::new(None, status_code, message);

        (status_code, Render(template)).into_response()
    }
}
