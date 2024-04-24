use sea_orm::ActiveValue::Set;
use serde::Deserialize;
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;
use validator::Validate;

use crate::{model::projects, pagination::Paginatable, template::projects::ProjectComponent};

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "kebab-case")]
pub struct NewProject {
    #[validate(length(min = 1, max = 128))]
    pub name: String,
    pub content_id: Uuid,
    pub thumbnail_id: Uuid,
    #[serde(default)]
    #[validate(length(min = 1, max = 2000), url)]
    pub project_url: Option<String>,
}

impl NewProject {
    pub fn new(name: String, project_url: Option<String>) -> Self {
        NewProject {
            name,
            content_id: Uuid::new_v4(),
            thumbnail_id: Uuid::new_v4(),
            project_url,
        }
    }

    pub fn id(&self) -> String {
        Self::generate_id(&self.name)
    }

    fn generate_id(name: &str) -> String {
        const MAX_ID_LENGTH: usize = 128;

        let mut id = name
            .replace(' ', "-")
            .replace(Self::invalid_id_char, "")
            .to_lowercase();
        id.truncate(MAX_ID_LENGTH);
        id
    }

    fn invalid_id_char(c: char) -> bool {
        const ID_CHARS: [char; 2] = ['-', '_'];
        !c.is_alphanumeric() && !ID_CHARS.contains(&c)
    }
}

impl From<NewProject> for projects::ActiveModel {
    fn from(value: NewProject) -> Self {
        Self {
            name: Set(value.name),
            content_id: Set(value.content_id),
            thumbnail_id: Set(value.thumbnail_id),
            project_url: Set(value.project_url),
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub preview: String,
    pub content_id: Uuid,
    pub thumbnail_id: Uuid,
    pub project_url: Option<String>,
    pub date_posted: OffsetDateTime,
}

impl Paginatable for Project {
    type Id = (OffsetDateTime, String);
    type Template = ProjectComponent;

    fn into_template(self) -> Self::Template {
        ProjectComponent { project: self }
    }

    fn id(&self) -> Self::Id {
        (self.date_posted, self.id.clone())
    }
}

impl From<projects::Model> for Project {
    fn from(value: projects::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            preview: value.preview,
            content_id: value.content_id,
            thumbnail_id: value.thumbnail_id,
            project_url: value.project_url,
            date_posted: value.date_posted,
        }
    }
}
