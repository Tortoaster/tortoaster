use askama::Template;
use axum::response::{Html, IntoResponse, Response};

use crate::error::PageError;

#[derive(Debug)]
pub struct Render<T>(pub T);

impl<T: Template> IntoResponse for Render<T> {
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(error) => PageError::from(error).into_response(),
        }
    }
}

impl<T> From<T> for Render<T> {
    fn from(value: T) -> Self {
        Render(value)
    }
}
