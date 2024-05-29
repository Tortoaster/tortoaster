use askama::Template;
use axum::response::{Html, IntoResponse, Response};

use crate::error::PageError;

pub mod comments;
pub mod error;
pub mod files;
mod filters;
pub mod projects;

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

#[derive(Debug)]
pub struct RenderBoth<T, U>(pub T, pub U);

impl<T: Template, U: Template> IntoResponse for RenderBoth<T, U> {
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html0) => match self.1.render() {
                Ok(html1) => Html(html0 + &html1).into_response(),
                Err(error) => PageError::from(error).into_response(),
            },
            Err(error) => PageError::from(error).into_response(),
        }
    }
}
