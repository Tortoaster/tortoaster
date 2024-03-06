use serde::Serialize;
use sqlx::types::time::OffsetDateTime;
use validator::Validate;

use crate::{pagination::Paginatable, template::ProjectPartial};

#[derive(Debug, Validate, Serialize)]
pub struct NewProject {
    #[validate(length(min = 1, max = 32))]
    pub name: String,
    #[validate(length(min = 1))]
    pub description: String,
    #[validate(url)]
    pub url: Option<String>,
}

#[derive(Debug)]
pub struct Project {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub url: Option<String>,
    pub date_posted: OffsetDateTime,
    pub date_updated: Option<OffsetDateTime>,
}

impl Paginatable for Project {
    type Template = ProjectPartial;

    fn into_template(self) -> Self::Template {
        ProjectPartial { project: self }
    }

    fn id(&self) -> i32 {
        self.id
    }
}

#[derive(Debug, Serialize, Validate)]
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
