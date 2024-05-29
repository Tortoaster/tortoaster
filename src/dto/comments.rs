use serde_with::serde_derive::Deserialize;
use validator::Validate;

use crate::{
    dto::{projects::ProjectTime, users::User},
    model::{comments, user_entity},
};

// Requests

#[derive(Debug, Deserialize, Validate)]
pub struct NewComment {
    #[validate(length(min = 1, max = 4096))]
    pub message: String,
}

// Responses

#[derive(Debug)]
pub struct Comment {
    pub id: i32,
    pub user: User,
    pub message: String,
    pub date_posted: ProjectTime,
}

impl Comment {
    pub fn from_model(comment: comments::Model, user: Option<user_entity::Model>) -> Self {
        let user = user
            .and_then(|user| user.try_into().ok())
            .unwrap_or_else(User::deleted);

        Comment {
            id: comment.id,
            user,
            message: comment.message,
            date_posted: ProjectTime::from(comment.date_posted),
        }
    }
}
