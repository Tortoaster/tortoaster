use sqlx::types::time::OffsetDateTime;
use validator::Validate;

#[derive(Debug, Validate)]
pub struct NewComment {
    pub project_id: i32,
    #[validate(length(min = 1, max = 32))]
    pub name: String,
    #[validate(email, length(max = 64))]
    pub email: String,
    #[validate(length(min = 1, max = 256))]
    pub message: String,
}

#[derive(Debug)]
pub struct Comment {
    pub id: i32,
    pub project_id: i32,
    pub name: String,
    pub email: String,
    pub message: String,
    pub date_posted: OffsetDateTime,
}
