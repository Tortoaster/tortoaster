use sea_orm::ActiveValue::Set;
use serde::Deserialize;
use sqlx::types::time::OffsetDateTime;
use validator::{Validate, ValidationError};

use crate::{
    error::{AppError, AppResult},
    model::projects,
    pagination::Paginatable,
    template::projects::ProjectComponent,
};

#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "kebab-case")]
pub struct NewProject {
    #[validate(length(min = 1, max = 128))]
    pub name: String,
    #[validate(length(min = 1))]
    pub description: String,
    #[validate(
        length(min = 1, max = 128),
        custom(function = "supported_image_extension")
    )]
    pub thumbnail_key: String,
    #[serde(default)]
    #[validate(length(min = 1, max = 2000), url)]
    pub project_url: Option<String>,
}

impl NewProject {
    pub fn new(
        name: String,
        description: String,
        thumbnail_filename: &str,
        project_url: Option<String>,
    ) -> AppResult<Self> {
        let id = Self::generate_id(&name);

        Ok(NewProject {
            name,
            description,
            thumbnail_key: format!("{id}/{}", Self::generate_key(thumbnail_filename)?),
            project_url,
        })
    }

    pub fn id(&self) -> String {
        Self::generate_id(&self.name)
    }

    fn invalid_id_char(c: char) -> bool {
        const ID_CHARS: [char; 2] = ['-', '_'];
        !c.is_alphanumeric() && !ID_CHARS.contains(&c)
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

    fn invalid_key_char(c: char) -> bool {
        const KEY_CHARS: [char; 3] = ['-', '_', '.'];
        !c.is_alphanumeric() && !KEY_CHARS.contains(&c)
    }

    fn generate_key(filename: &str) -> AppResult<String> {
        const MAX_KEY_LENGTH: usize = 128;

        let (name, extension) = filename.rsplit_once('.').ok_or(AppError::MissingFile)?;

        let mut id = name
            .replace(' ', "_")
            .replace(Self::invalid_key_char, "")
            .to_lowercase();

        id.truncate(MAX_KEY_LENGTH - extension.len());
        id += ".";
        id += extension;

        Ok(id)
    }
}

impl From<NewProject> for projects::ActiveModel {
    fn from(value: NewProject) -> Self {
        Self {
            name: Set(value.name),
            description: Set(value.description),
            thumbnail_key: Set(value.thumbnail_key),
            project_url: Set(value.project_url),
            ..Default::default()
        }
    }
}

fn supported_image_extension(value: &str) -> Result<(), ValidationError> {
    const THUMBNAIL_EXTENSIONS: [&str; 6] = [".png", ".jpg", ".jpeg", ".gif", ".webp", ".svg"];

    if value
        .chars()
        .any(|c| NewProject::invalid_key_char(c) && c != '/')
    {
        return Err(ValidationError::new("Invalid characters in filename"));
    }

    if !THUMBNAIL_EXTENSIONS
        .iter()
        .any(|extension| value.ends_with(extension))
    {
        return Err(ValidationError::new(
            "Unsupported file type, only .png, .jpg, .jpeg, .gif, .webp, and .svg are supported",
        ));
    }

    Ok(())
}

#[derive(Debug)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub thumbnail_key: String,
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
            description: value.description,
            thumbnail_key: value.thumbnail_key,
            project_url: value.project_url,
            date_posted: value.date_posted,
        }
    }
}
