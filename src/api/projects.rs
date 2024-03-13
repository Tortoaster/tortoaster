use askama::Template;
use axum::extract::{Path, Query, State};
use axum_extra::routing::TypedPath;

use crate::{
    error::{FullPageError, FullPageResult},
    model::Project,
    pagination::{Pager, PaginatedResponse},
    render::Render,
    repository::projects::ProjectsRepository,
};

#[derive(Template)]
#[template(path = "projects/card.html")]
pub struct ProjectCard {
    pub project: Project,
}

#[derive(Copy, Clone, Default, TypedPath)]
#[typed_path("/projects")]
pub struct ListProjectsUrl;

pub async fn list_projects(
    _: ListProjectsUrl,
    Query(pager): Query<Pager>,
    State(repo): State<ProjectsRepository>,
) -> FullPageResult<PaginatedResponse<Project, ListProjectsUrl>> {
    let projects = repo.list(&pager).await?;
    Ok(PaginatedResponse(projects, ListProjectsUrl))
}

#[derive(Template)]
#[template(path = "projects/page.html")]
pub struct ProjectPage {
    pub project: Project,
}

pub async fn project(
    Path(id): Path<i32>,
    State(repo): State<ProjectsRepository>,
) -> FullPageResult<Render<ProjectPage>> {
    let project = repo.get(id).await?.ok_or(FullPageError::NotFound)?;
    Ok(Render(ProjectPage { project }))
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
