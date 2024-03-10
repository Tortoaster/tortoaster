use axum::extract::{Path, Query, State};
use axum_extra::routing::TypedPath;
use sqlx::{query_as, PgPool};

use crate::{
    error::AppResult,
    model::Project,
    pagination::{Pager, PaginatedResponse},
    template::{IndexTemplate, ProjectTemplate, TemplateResponse},
};

pub async fn index() -> TemplateResponse<IndexTemplate> {
    TemplateResponse(IndexTemplate::default())
}

#[derive(Copy, Clone, Default, TypedPath)]
#[typed_path("/projects")]
pub struct ListProjects;

pub async fn list_projects(
    _: ListProjects,
    State(pool): State<PgPool>,
    pager: Option<Query<Pager>>,
) -> AppResult<PaginatedResponse<Project, ListProjects>> {
    let after_id = pager.map(|pager| pager.after_id).unwrap_or_default();
    let projects = query_as!(
        Project,
        "SELECT * FROM projects WHERE id > $1 ORDER BY id LIMIT 10;",
        after_id
    )
    .fetch_all(&pool)
    .await?;

    Ok(PaginatedResponse(projects, ListProjects))
}

pub async fn project(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
) -> AppResult<TemplateResponse<ProjectTemplate>> {
    let project = query_as!(Project, "SELECT * FROM projects WHERE id = $1;", id)
        .fetch_one(&pool)
        .await?;

    Ok(TemplateResponse(ProjectTemplate { project }))
}

// pub async fn get_project(
//     State(pool): State<PgPool>,
//     Path(id): Path<u32>,
// ) -> AppResult<Json<Project>> {
//     todo!()
// }
//
// pub async fn post_project(
//     State(pool): State<PgPool>,
//     Json(project): Json<NewProject>,
// ) -> AppResult<()> {
//     todo!()
// }
//
// pub async fn patch_project(State(pool): State<PgPool>) {
//     todo!()
// }
//
// pub async fn delete_project(State(pool): State<PgPool>) {
//     todo!()
// }
