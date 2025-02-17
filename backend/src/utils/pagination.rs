use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Default, Deserialize, Validate)]
pub struct Pager<Id> {
    pub after: Option<Id>,
    pub before: Option<Id>,
    #[validate(range(min = 1, max = 50))]
    pub items: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    pub has_previous: bool,
    pub has_next: bool,
}
