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

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("I couldn't access the database! :(")]
    Database(#[from] sqlx::Error),
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
    #[error("Something weird went wrong :(")]
    MultipartError(#[from] axum::extract::multipart::MultipartError),
    #[error("Something seems to be wrong with your session, please try logging in again!")]
    Session(#[from] axum_oidc::error::MiddlewareError),
    #[error("Something went wrong while retrieving your file :(")]
    ObjectEncoding,
    #[error("I don't understand that type of file :(")]
    FileType,
    #[error("Please add an image for the project! :3")]
    FileMissing,
    #[error("I couldn't find the page you're looking for! :(")]
    NotFound,
    #[error("You're not allowed to do that!")]
    Unauthorized,
    #[error("Please fill out all the fields!")]
    Form(#[from] FormRejection),
    #[error("Something weird went wrong :(")]
    MultipartRejection(#[from] MultipartRejection),
    #[error("Something went wrong logging you out :(")]
    Logout(#[from] axum_oidc::error::ExtractorError),
    #[error("Something seems to be wrong with your session, please try logging in again!")]
    User(#[from] crate::utils::claims::UserRejection),
    #[error("Please change the following fields :3\n{0}")]
    Validate(#[from] axum_valid::ValidRejection<QueryRejection>),
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Database(_) => StatusCode::SERVICE_UNAVAILABLE,
            AppError::Template(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::PutObject(_) => StatusCode::INSUFFICIENT_STORAGE,
            AppError::GetObject(_) | AppError::ObjectEncoding | AppError::NotFound => {
                StatusCode::NOT_FOUND
            }
            AppError::Form(_)
            | AppError::MultipartError(_)
            | AppError::MultipartRejection(_)
            | AppError::Validate(_)
            | AppError::FileType
            | AppError::FileMissing => StatusCode::BAD_REQUEST,
            AppError::Session(_)
            | AppError::Logout(_)
            | AppError::User(_)
            | AppError::Unauthorized => StatusCode::UNAUTHORIZED,
        }
    }
}
