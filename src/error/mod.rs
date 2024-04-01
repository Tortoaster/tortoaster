mod page;
mod toast;

use axum::{
    extract::rejection::{FormRejection, QueryRejection},
    http::StatusCode,
};
pub use page::{PageError, PageResult, WithPageRejection};
use thiserror::Error;
pub use toast::{ToastError, ToastResult, WithToastRejection};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("I couldn't access the database! :(")]
    Database(#[from] sqlx::Error),
    #[error("I couldn't access the database! :(")]
    Orm(#[from] sea_orm::DbErr),
    #[error("I couldn't display this page :(")]
    Template(#[from] askama::Error),
    #[error("I couldn't find the page you're looking for! :(")]
    NotFound,
    // Rejections
    #[error("Please fill out all the fields!")]
    Form(#[from] FormRejection),
    #[error("Please change the following fields :3\n{0}")]
    Validate(#[from] axum_valid::ValidRejection<QueryRejection>),
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Database(_) | AppError::Orm(_) => StatusCode::SERVICE_UNAVAILABLE,
            AppError::Template(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Form(_) | AppError::Validate(_) => StatusCode::BAD_REQUEST,
        }
    }
}
