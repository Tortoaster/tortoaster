use std::fmt::{Display, Formatter};

use askama::Template;
use axum::response::{Html, IntoResponse, Response};
use axum_extra::routing::TypedPath;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::error::PageError;

#[derive(Debug, Default, Serialize, Deserialize, Validate)]
pub struct Pager<Id> {
    pub after: Option<Id>,
    #[validate(range(min = 1, max = 50))]
    pub items: Option<i64>,
}

#[derive(Debug)]
pub struct PaginatedResponse<T, U, Id> {
    pub items: Vec<T>,
    pub url: U,
    pub pager: Pager<Id>,
}

pub trait Paginatable {
    /// The type of the table column(s) that is/are unique and can be ordered in
    /// SQL.
    type Id;
    type Template: Template;

    fn into_template(self) -> Self::Template;

    fn id(&self) -> Self::Id;
}

impl<'a, T, U> IntoResponse for PaginatedResponse<T, U, T::Id>
where
    T: Paginatable,
    T::Id: Serialize,
    U: TypedPath,
{
    fn into_response(self) -> Response {
        let last_id = match self.items.last() {
            None => return Html(String::new()).into_response(),
            Some(t) => t.id(),
        };

        let mut pager = self.pager;
        pager.after = Some(last_id);

        let lazy_list = LazyList {
            url: self.url.with_query_params(pager),
            trigger: Trigger::Revealed,
        };

        let templates: Result<String, askama::Error> = self
            .items
            .into_iter()
            .map(T::into_template)
            .map(|template| template.render())
            .chain(Some(lazy_list.render()))
            .collect();

        match templates {
            Ok(html) => Html(html).into_response(),
            Err(error) => PageError::from(error).into_response(),
        }
    }
}

#[derive(Template)]
#[template(path = "pagination/lazy_list.html")]
struct LazyList<U: Display> {
    url: U,
    trigger: Trigger,
}

#[non_exhaustive]
enum Trigger {
    Revealed,
}

impl Display for Trigger {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Trigger::Revealed => write!(f, "revealed"),
        }
    }
}
