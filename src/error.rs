use askama::Template;
use axum::{
    extract::rejection::QueryRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum_extra::extract::WithRejection;
use thiserror::Error;

use crate::render::Render;

pub type FullPageResult<T> = Result<T, FullPageError>;

pub type WithFullPageRejection<E> = WithRejection<E, FullPageError>;

// TODO: Maybe this shouldn't exist at all
/// Global error type that represents common errors that can occur anywhere in
/// the project. This type implements [`IntoResponse`] to make it easy to
/// generate full-page error responses.
#[derive(Debug, Error)]
pub enum FullPageError {
    #[error("I couldn't access the database! :(")]
    Database(#[from] sqlx::Error),
    // TODO: Probably shouldn't be here
    #[error("template error: {0}")]
    Template(#[from] askama::Error),
    #[error("I couldn't do what you asked! :(\n{0}")]
    Validate(#[from] axum_valid::ValidRejection<QueryRejection>),
    #[error("I couldn't find the page you're looking for! :(")]
    NotFound,
}

impl FullPageError {
    fn status_code(&self) -> StatusCode {
        match self {
            FullPageError::Database(_) => StatusCode::SERVICE_UNAVAILABLE,
            FullPageError::Template(_) => StatusCode::INTERNAL_SERVER_ERROR,
            FullPageError::Validate(_) => StatusCode::BAD_REQUEST,
            FullPageError::NotFound => StatusCode::NOT_FOUND,
        }
    }
}

#[derive(Template)]
#[template(path = "error.html")]
struct Error {
    status_code: StatusCode,
    message: String,
}

impl IntoResponse for FullPageError {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let message = self.to_string();

        let template = Error {
            status_code,
            message,
        };

        (status_code, Render(template)).into_response()
    }
}

mod filters {
    use askama::Result;
    use axum::http::StatusCode;

    pub fn with_reason(status_code: &StatusCode) -> Result<String> {
        Ok(format!(
            "{} {}",
            status_code.as_u16(),
            status_code
                .canonical_reason()
                .unwrap_or("No idea what went wrong")
        ))
    }
}
