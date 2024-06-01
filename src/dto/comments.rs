use serde_with::serde_derive::Deserialize;
use validator::Validate;

use crate::dto::{projects::ProjectTime, users::UserId};

// Requests

#[derive(Debug, Deserialize, Validate)]
pub struct NewComment {
    #[validate(length(min = 1, max = 4096))]
    pub message: String,
}

// Responses

#[derive(Debug)]
pub struct CommentWithUser {
    pub id: i32,
    pub user_id: UserId,
    pub name: Option<String>,
    pub message: String,
    pub date_posted: ProjectTime,
}
