use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use crate::{
    api::ListProjects,
    error::AppError,
    model::{Comment, Project},
};

#[derive(Default, Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    list_projects_url: ListProjects,
}

#[derive(Template)]
#[template(path = "project.html")]
pub struct ProjectTemplate {
    pub project: Project,
}

#[derive(Template)]
#[template(path = "partial/project.html")]
pub struct ProjectPartial {
    pub project: Project,
}

#[derive(Template)]
#[template(path = "partial/comment.html")]
pub struct CommentPartial {
    pub comment: Comment,
}

#[derive(Template)]
#[template(path = "partial/new_comment.html")]
pub struct NewCommentPartial {
    pub project_id: u32,
}

#[derive(Template)]
#[template(path = "partial/error.html")]
pub struct ErrorPartial {
    pub status_code: StatusCode,
    pub message: String,
}

#[derive(Debug)]
pub struct Render<T>(pub T);

impl<T: Template> IntoResponse for Render<T> {
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(error) => AppError::from(error).into_response(),
        }
    }
}

impl<T> From<T> for Render<T> {
    fn from(value: T) -> Self {
        Render(value)
    }
}

mod filters {
    use askama::Result;
    use sqlx::types::time::OffsetDateTime;
    use time_humanize::HumanTime;

    pub fn humantime(time: &OffsetDateTime) -> Result<String> {
        let human_time = HumanTime::from_duration_since_timestamp(time.unix_timestamp() as u64)
            - HumanTime::now();

        Ok(human_time.to_string())
    }
}
