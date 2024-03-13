use askama::Template;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

use crate::render::Render;

// TODO: Returning AppError is not often useful when working with partials, only
//  for unrecoverable errors, so a type alias may not be necessary
pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("template error: {0}")]
    Template(#[from] askama::Error),
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Database(_) => StatusCode::SERVICE_UNAVAILABLE,
            AppError::Template(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorPartial {
    pub status_code: StatusCode,
    pub message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        Render(ErrorPartial {
            status_code: self.status_code(),
            message: self.to_string(),
        })
        .into_response()
    }
}
