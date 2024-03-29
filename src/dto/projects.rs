use sea_orm::ActiveValue::Set;
use sqlx::types::time::OffsetDateTime;
use validator::Validate;

use crate::{model::projects, pagination::Paginatable, template::projects::ProjectComponent};

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

impl From<NewProject> for projects::ActiveModel {
    fn from(value: NewProject) -> Self {
        Self {
            name: Set(value.name),
            description: Set(value.description),
            thumbnail_url: Set(value.thumbnail_url),
            project_url: Set(value.project_url),
            ..Default::default()
        }
    }
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
    type Template = ProjectComponent;

    fn into_template(self) -> Self::Template {
        ProjectComponent { project: self }
    }

    fn id(&self) -> Self::Id {
        self.id
    }
}

impl From<projects::Model> for Project {
    fn from(value: projects::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            thumbnail_url: value.thumbnail_url,
            project_url: value.project_url,
            date_posted: value.date_posted,
            date_updated: value.date_updated,
        }
    }
}
