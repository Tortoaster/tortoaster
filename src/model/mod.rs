use sqlx::types::time::OffsetDateTime;
use validator::Validate;

use crate::{api::projects::ProjectCard, pagination::Paginatable};

pub mod auth;

#[derive(Debug, Validate)]
pub struct NewProject {
    #[validate(length(min = 1, max = 32))]
    pub name: String,
    #[validate(length(min = 1))]
    pub description: String,
    #[validate(url)]
    pub thumbnail_url: String,
    #[validate(url)]
    pub project_url: Option<String>,
}

#[derive(Debug)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub thumbnail_url: String,
    pub project_url: Option<String>,
    pub date_posted: OffsetDateTime,
    pub date_updated: Option<OffsetDateTime>,
}

impl Paginatable for Project {
    type Template = ProjectCard;

    fn into_template(self) -> Self::Template {
        ProjectCard { project: self }
    }

    fn id(&self) -> i32 {
        self.id
    }
}

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
