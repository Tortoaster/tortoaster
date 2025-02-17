use std::sync::OnceLock;

use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, NoneAsEmptyString};
use sqlx::types::time::OffsetDateTime;
use uuid::Uuid;
use validator::Validate;

// Requests

#[serde_as]
#[derive(Debug, Deserialize, Validate)]
#[serde(rename_all = "kebab-case")]
pub struct NewProject {
    #[validate(length(min = 1, max = 128))]
    pub name: String,
    #[validate(length(min = 1))]
    pub content: String,
    pub thumbnail_id: Uuid,
    #[serde(default)]
    #[serde_as(as = "NoneAsEmptyString")]
    #[validate(length(min = 1, max = 2000), url)]
    pub project_url: Option<String>,
}

impl NewProject {
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

    pub fn preview(&self) -> String {
        const PREVIEW_LENGTH: usize = 300;
        static RE: OnceLock<Regex> = OnceLock::new();

        let parser = pulldown_cmark::Parser::new(&self.content);
        let mut html = String::new();
        pulldown_cmark::html::push_html(&mut html, parser);
        let stripped = html.strip_suffix('\n').unwrap_or(&html);
        let re = RE.get_or_init(|| Regex::new(r"<[^>]*>").unwrap());
        let mut preview = re.replace_all(stripped, "").to_string();

        if preview.len() >= PREVIEW_LENGTH {
            preview.truncate(PREVIEW_LENGTH - 3);
            preview += "...";
        }

        preview
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ProjectIndex {
    #[serde(with = "time::serde::rfc3339")]
    pub date_posted: OffsetDateTime,
    pub id: String,
}

// Responses

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectPreview {
    pub id: String,
    pub name: String,
    pub preview: String,
    pub thumbnail_id: Uuid,
    #[serde(with = "time::serde::rfc3339")]
    pub date_posted: OffsetDateTime,
}

#[derive(Debug, Serialize)]
pub struct ProjectId {
    pub id: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectThumbnailId {
    pub thumbnail_id: Uuid,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub thumbnail_id: Uuid,
    pub project_url: Option<String>,
    #[serde(with = "time::serde::rfc3339")]
    pub date_posted: OffsetDateTime,
}
