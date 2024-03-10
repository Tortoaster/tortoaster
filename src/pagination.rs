use std::fmt::{Display, Formatter};

use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use axum_extra::routing::TypedPath;
use serde::{Deserialize, Serialize};

use crate::error::AppError;

#[derive(Serialize, Deserialize)]
pub struct Pager {
    pub after_id: i32,
}

#[derive(Debug)]
pub struct PaginatedResponse<T, U>(pub Vec<T>, pub U);

impl<T: Paginatable, U: TypedPath> IntoResponse for PaginatedResponse<T, U> {
    fn into_response(self) -> Response {
        let last_id = match self.0.last() {
            None => return Html("").into_response(),
            Some(t) => t.id(),
        };

        let lazy_list = LazyList {
            url: self.1.with_query_params(Pager { after_id: last_id }),
            trigger: Trigger::Revealed,
        };

        let templates: Result<String, askama::Error> = self
            .0
            .into_iter()
            .map(T::into_template)
            .map(|template| template.render())
            .chain(Some(lazy_list.render()))
            .collect();

        match templates {
            Ok(html) => Html(html).into_response(),
            Err(error) => AppError::from(error).into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "pagination/lazy_list.html")]
pub struct LazyList<U: Display> {
    pub url: U,
    pub trigger: Trigger,
}

#[non_exhaustive]
pub enum Trigger {
    Revealed,
}

impl Display for Trigger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Trigger::Revealed => write!(f, "revealed"),
        }
    }
}

pub trait Paginatable {
    type Template: Template;

    fn into_template(self) -> Self::Template;

    fn id(&self) -> i32;
}
