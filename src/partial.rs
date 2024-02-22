use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use sqlx::{query_as, PgPool};

use crate::{
    error::AppResult,
    model::{NewProject, Project},
    template::{ProjectPartial, Templates},
};

#[derive(Deserialize)]
pub struct Pager {
    after_id: i32,
}

pub async fn projects(
    State(pool): State<PgPool>,
    pager: Option<Query<Pager>>,
) -> AppResult<Templates<ProjectPartial>> {
    let after_id = pager.map(|pager| pager.after_id).unwrap_or_default();
    let projects = query_as!(
        Project,
        "SELECT * FROM projects WHERE id > $1 ORDER BY id LIMIT 10;",
        after_id
    )
    .fetch_all(&pool)
    .await?;
    Ok(Templates(
        projects
            .into_iter()
            .map(|project| ProjectPartial { project })
            .collect(),
    ))
}

pub async fn get_project(
    State(pool): State<PgPool>,
    Path(id): Path<u32>,
) -> AppResult<Json<Project>> {
    todo!()
}

pub async fn post_project(
    State(pool): State<PgPool>,
    Json(project): Json<NewProject>,
) -> AppResult<()> {
    todo!()
}

pub async fn patch_project(State(pool): State<PgPool>) {
    todo!()
}

pub async fn delete_project(State(pool): State<PgPool>) {
    todo!()
}
