use sea_orm::ActiveValue::Set;
use sqlx::types::time::OffsetDateTime;
use validator::Validate;

use crate::{model::comments, pagination::Paginatable, template::comments::CommentComponent};

#[derive(Debug, Validate)]
pub struct NewComment {
    pub project_id: String,
    #[validate(length(min = 1, max = 32))]
    pub name: String,
    #[validate(email, length(max = 64))]
    pub email: String,
    #[validate(length(min = 1, max = 256))]
    pub message: String,
}

impl From<NewComment> for comments::ActiveModel {
    fn from(value: NewComment) -> Self {
        Self {
            project_id: Set(value.project_id),
            name: Set(value.name),
            email: Set(value.email),
            message: Set(value.message),
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct Comment {
    pub id: i32,
    pub name: String,
    pub message: String,
    pub date_posted: OffsetDateTime,
}

impl Paginatable for Comment {
    type Id = i32;
    type Template = CommentComponent;

    fn into_template(self) -> Self::Template {
        CommentComponent { comment: self }
    }

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl From<comments::Model> for Comment {
    fn from(value: comments::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            message: value.message,
            date_posted: value.date_posted,
        }
    }
}
