use std::{
    fmt::{Display, Formatter},
    str::FromStr,
    sync::OnceLock,
};

use regex::Regex;
use serde::Deserialize;
use serde_with::{serde_as, DeserializeFromStr, NoneAsEmptyString};
use sqlx::types::time::OffsetDateTime;
use time::format_description::well_known;
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

// Responses

#[derive(Debug)]
pub struct ProjectData {
    pub name: String,
    pub content: String,
    pub thumbnail_id: Uuid,
    pub project_url: Option<String>,
}

#[derive(Debug)]
pub struct ProjectPreview {
    pub id: String,
    pub name: String,
    pub preview: String,
    pub thumbnail_id: Uuid,
    pub date_posted: ProjectTime,
}

#[derive(Debug)]
pub struct ProjectId {
    pub id: String,
}

#[derive(Debug)]
pub struct ProjectName {
    pub name: String,
}

#[derive(Debug)]
pub struct ProjectThumbnailId {
    pub thumbnail_id: Uuid,
}

#[derive(Debug)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub thumbnail_id: Uuid,
    pub project_url: Option<String>,
    pub date_posted: ProjectTime,
}

#[derive(Debug, DeserializeFromStr)]
pub struct ProjectIndex {
    pub date_posted: ProjectTime,
    pub id: String,
}

impl FromStr for ProjectIndex {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (date_posted, id) = s.split_once(',').ok_or("invalid project index")?;

        let index = ProjectIndex {
            date_posted: date_posted.parse().map_err(|_| "invalid date syntax")?,
            id: id.to_owned(),
        };

        Ok(index)
    }
}

#[derive(Debug, DeserializeFromStr)]
pub struct ProjectTime(OffsetDateTime);

impl ProjectTime {
    pub fn as_offset(&self) -> &OffsetDateTime {
        &self.0
    }
}

impl From<OffsetDateTime> for ProjectTime {
    fn from(value: OffsetDateTime) -> Self {
        Self(value)
    }
}

impl Display for ProjectTime {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format(&well_known::Rfc3339).unwrap())
    }
}

impl FromStr for ProjectTime {
    type Err = time::error::Parse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ProjectTime(OffsetDateTime::parse(s, &well_known::Rfc3339)?))
    }
}
