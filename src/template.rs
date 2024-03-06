use askama::Template;
use axum::{
    http::StatusCode,
    response::{Html, IntoResponse, Response},
};

use crate::{
    error::AppError,
    model::{Comment, Project},
};

#[derive(Template)]
#[template(path = "index.html")]
pub struct Index;

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
pub struct TemplateResponse<T>(pub T);

impl<T: Template> IntoResponse for TemplateResponse<T> {
    fn into_response(self) -> Response {
        match self.0.render() {
            Ok(html) => Html(html).into_response(),
            Err(error) => AppError::from(error).into_response(),
        }
    }
}

#[derive(Debug)]
pub struct Templates<T>(pub Vec<T>);

impl<T> IntoResponse for Templates<T>
where
    T: Template,
{
    fn into_response(self) -> Response {
        let templates: Result<String, askama::Error> = self
            .0
            .into_iter()
            .map(|template| template.render())
            .collect();

        match templates {
            Ok(html) => Html(html).into_response(),
            Err(error) => AppError::from(error).into_response(),
        }
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
