use axum::{
    extract::rejection::QueryRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::extract::WithRejection;
use thiserror::Error;

use crate::{render::Render, template::error::ErrorPage};

/// Helper type that displays a full page with status code and message on error.
pub type PageResult<T> = Result<T, PageError>;

/// Helper type that turns a rejection into a full page error.
pub type WithPageRejection<E> = WithRejection<E, PageError>;

/// Global error type that represents common errors that can occur anywhere in
/// the project. This type implements [`IntoResponse`] to make it easy to
/// generate full-page error responses.
#[derive(Debug, Error)]
pub enum PageError {
    #[error("I couldn't access the database! :(")]
    Database(#[from] sqlx::Error),
    #[error("I couldn't access the database! :(")]
    Database2(#[from] sea_orm::DbErr),
    #[error("template error: {0}")]
    Template(#[from] askama::Error),
    #[error("I couldn't do what you asked! :(\n{0}")]
    Validate(#[from] axum_valid::ValidRejection<QueryRejection>),
    #[error("I couldn't find the page you're looking for! :(")]
    NotFound,
}

impl PageError {
    fn status_code(&self) -> StatusCode {
        match self {
            PageError::Database(_) | PageError::Database2(_) => StatusCode::SERVICE_UNAVAILABLE,
            PageError::Template(_) => StatusCode::INTERNAL_SERVER_ERROR,
            PageError::Validate(_) => StatusCode::BAD_REQUEST,
            PageError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

impl IntoResponse for PageError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let message = self.to_string();

        let template = ErrorPage {
            status_code,
            message,
        };

        (status_code, Render(template)).into_response()
    }
}
