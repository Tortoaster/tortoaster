use serde_with::serde_derive::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::dto::projects::ProjectTime;

// Requests

#[derive(Debug, Deserialize, Validate)]
pub struct NewComment {
    #[validate(length(min = 1, max = 4096))]
    pub message: String,
}

// Helpers

#[derive(Debug)]
pub struct CommentUserId {
    pub id: i32,
    pub user_id: Uuid,
    pub message: String,
    pub date_posted: ProjectTime,
}

// Responses

#[derive(Debug)]
pub struct CommentWithUser {
    pub id: i32,
    pub user_id: Uuid,
    pub name: Option<String>,
    pub is_admin: bool,
    pub message: String,
    pub date_posted: ProjectTime,
}

#[derive(Debug)]
pub struct CommentMessage {
    pub message: String,
}

impl From<NewComment> for CommentMessage {
    fn from(value: NewComment) -> Self {
        Self {
            message: value.message,
        }
    }
}
