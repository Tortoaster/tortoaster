use axum::{
    extract::{
        multipart::MultipartRejection,
        rejection::{FormRejection, QueryRejection},
    },
    http::StatusCode,
};
use thiserror::Error;

mod page;
mod toast;

pub use page::{PageError, PageResult, WithPageRejection};
pub use toast::{ToastResult, WithToastRejection};

#[derive(Debug, Error)]
pub enum AppError {
    #[error("I couldn't access the database! :(")]
    Database(#[from] sqlx::Error),
    #[error("I couldn't access the database! :(")]
    Orm(#[from] sea_orm::DbErr),
    #[error("I couldn't display this page :(")]
    Template(#[from] askama::Error),
    #[error("Something went wrong while uploading your file :(")]
    PutObject(
        #[from] aws_sdk_s3::error::SdkError<aws_sdk_s3::operation::put_object::PutObjectError>,
    ),
    #[error("Something went wrong while retrieving your file :(")]
    GetObject(
        #[from] aws_sdk_s3::error::SdkError<aws_sdk_s3::operation::get_object::GetObjectError>,
    ),
    #[error("Something went wrong while retrieving your file :(")]
    ObjectEncoding,
    #[error("I couldn't find the page you're looking for! :(")]
    NotFound,
    #[error("Please fill out all the fields!")]
    Form(#[from] FormRejection),
    #[error("Something weird went wrong :(")]
    MultipartError(#[from] axum::extract::multipart::MultipartError),
    #[error("Something weird went wrong :(")]
    MultipartRejection(#[from] MultipartRejection),
    #[error("Please change the following fields :3\n{0}")]
    Validate(#[from] axum_valid::ValidRejection<QueryRejection>),
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Database(_) | AppError::Orm(_) => StatusCode::SERVICE_UNAVAILABLE,
            AppError::Template(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::PutObject(_) => StatusCode::INSUFFICIENT_STORAGE,
            AppError::GetObject(_) | AppError::ObjectEncoding | AppError::NotFound => {
                StatusCode::NOT_FOUND
            }
            AppError::Form(_)
            | AppError::MultipartError(_)
            | AppError::MultipartRejection(_)
            | AppError::Validate(_) => StatusCode::BAD_REQUEST,
        }
    }
}
