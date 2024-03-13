use askama::Template;
use axum::extract::{Path, Query, State};
use axum_extra::routing::TypedPath;
use sqlx::{query_as, PgPool};

use crate::{
    error::AppResult,
    model::Project,
    pagination::{Pager, PaginatedResponse},
    render::Render,
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
    State(pool): State<PgPool>,
    pager: Option<Query<Pager>>,
) -> AppResult<PaginatedResponse<Project, ListProjectsUrl>> {
    let after_id = pager.map(|pager| pager.after_id).unwrap_or_default();
    let projects = query_as!(
        Project,
        "SELECT * FROM projects WHERE id > $1 ORDER BY id LIMIT 10;",
        after_id
    )
    .fetch_all(&pool)
    .await?;

    Ok(PaginatedResponse(projects, ListProjectsUrl))
}

#[derive(Template)]
#[template(path = "projects/page.html")]
pub struct ProjectPage {
    pub project: Project,
}

pub async fn project(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> AppResult<Render<ProjectPage>> {
    let project = query_as!(Project, "SELECT * FROM projects WHERE id = $1;", id)
        .fetch_one(&pool)
        .await?;

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
