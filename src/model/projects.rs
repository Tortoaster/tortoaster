use sqlx::types::time::OffsetDateTime;
use validator::Validate;

use crate::{api::projects::ProjectCard, pagination::Paginatable};

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
    type Id = i32;
    type Template = ProjectCard;

    fn into_template(self) -> Self::Template {
        ProjectCard { project: self }
    }

    fn id(&self) -> Self::Id {
        self.id
    }
}