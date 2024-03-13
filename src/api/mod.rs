use askama::Template;

use crate::{api::projects::ListProjectsUrl, render::Render};

pub mod auth;
mod comments;
pub mod projects;

#[derive(Default, Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    list_projects_url: ListProjectsUrl,
}

pub async fn index() -> Render<IndexTemplate> {
    Render(IndexTemplate::default())
}
