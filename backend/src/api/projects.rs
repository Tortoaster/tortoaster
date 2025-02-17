use axum::{
    extract::{Query, State},
    Form, Json, Router,
};
use axum_extra::{
    extract::WithRejection,
    routing::{RouterExt, TypedPath},
};
use axum_valid::Valid;
use serde::Deserialize;

use crate::{
    dto::projects::{NewProject, Project, ProjectId, ProjectIndex, ProjectPreview},
    error::{AppError, AppResult, WithAppRejection},
    repository::projects::ProjectRepository,
    state::AppState,
    utils::{
        claims::Admin,
        pagination::{Page, Pager},
    },
};

pub fn public_router() -> Router<AppState> {
    Router::new()
        .typed_get(list_projects)
        .typed_get(get_project)
}

pub fn protected_router() -> Router<AppState> {
    Router::new()
        .typed_post(post_project)
        .typed_put(put_project)
        .typed_delete(delete_project)
}

#[derive(Copy, Clone, Debug, TypedPath)]
#[typed_path("/projects")]
pub struct ProjectsUrl;

#[derive(Clone, Debug, Deserialize, TypedPath)]
#[typed_path("/projects/:id")]
pub struct ProjectUrl {
    pub id: String,
}

async fn list_projects(
    _: ProjectsUrl,
    State(repo): State<ProjectRepository>,
    WithRejection(Valid(Query(pager)), _): WithAppRejection<Valid<Query<Pager<ProjectIndex>>>>,
) -> AppResult<Json<Page<ProjectPreview>>> {
    let page = repo.list(&pager).await?;
    Ok(Json(page))
}

async fn get_project(
    ProjectUrl { id }: ProjectUrl,
    State(project_repo): State<ProjectRepository>,
) -> AppResult<Json<Project>> {
    let project = project_repo.read(&id).await?.ok_or(AppError::NotFound)?;
    Ok(Json(project))
}

async fn post_project(
    _: ProjectsUrl,
    _: Admin,
    State(repo): State<ProjectRepository>,
    WithRejection(Valid(Form(new_project)), _): WithAppRejection<Valid<Form<NewProject>>>,
) -> AppResult<Json<ProjectId>> {
    let project = repo.create(new_project).await?;
    Ok(Json(project))
}

async fn put_project(
    ProjectUrl { id }: ProjectUrl,
    _: Admin,
    State(repo): State<ProjectRepository>,
    WithRejection(Valid(Form(new_project)), _): WithAppRejection<Valid<Form<NewProject>>>,
) -> AppResult<()> {
    repo.update(&id, new_project).await?;
    Ok(())
}

async fn delete_project(
    ProjectUrl { id }: ProjectUrl,
    _: Admin,
    State(repo): State<ProjectRepository>,
) -> AppResult<()> {
    repo.delete(&id).await?;
    Ok(())
}
