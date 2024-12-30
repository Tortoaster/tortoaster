use serde::Serialize;
use serde_with::serde_derive::Deserialize;
use time::OffsetDateTime;
use uuid::Uuid;
use validator::Validate;

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
    pub date_posted: OffsetDateTime,
}

// Responses

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentWithUser {
    pub id: i32,
    pub user_id: Uuid,
    pub name: Option<String>,
    pub is_admin: bool,
    pub message: String,
    #[serde(with = "time::serde::rfc3339")]
    pub date_posted: OffsetDateTime,
}
