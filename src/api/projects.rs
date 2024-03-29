use axum::extract::{Path, Query, State};
use axum_extra::{extract::WithRejection, routing::TypedPath};
use axum_valid::Valid;

use crate::{
    dto::projects::Project,
    error::{PageError, PageResult, ToastResult, WithPageRejection, WithToastRejection},
    pagination::{Pager, PaginatedResponse},
    render::Render,
    repository::projects::ProjectsRepository,
    template::projects::{ProjectListPage, ProjectPage},
};

#[derive(Copy, Clone, Default, TypedPath)]
#[typed_path("/")]
pub struct ListProjectsPageUrl;

pub async fn list_projects_page(
    _: ListProjectsPageUrl,
    State(repo): State<ProjectsRepository>,
    WithRejection(Valid(Query(pager)), _): WithPageRejection<Valid<Query<Pager<i32>>>>,
) -> PageResult<Render<ProjectListPage>> {
    let projects = repo.list(&pager).await?;
    Ok(Render(ProjectListPage { projects }))
}

#[derive(Copy, Clone, Default, TypedPath)]
#[typed_path("/projects")]
pub struct ListProjectsPartialUrl;

pub async fn list_projects_partial(
    url: ListProjectsPartialUrl,
    State(repo): State<ProjectsRepository>,
    WithRejection(Valid(Query(pager)), _): WithToastRejection<Valid<Query<Pager<i32>>>>,
) -> ToastResult<PaginatedResponse<Project, ListProjectsPartialUrl, i32>> {
    let items = repo.list(&pager).await?;
    Ok(PaginatedResponse { items, url, pager })
}

pub async fn get_project(
    Path(id): Path<i32>,
    State(repo): State<ProjectsRepository>,
    WithRejection(Valid(Query(pager)), _): WithPageRejection<Valid<Query<Pager<i32>>>>,
) -> PageResult<Render<ProjectPage>> {
    let (project, comments) = repo
        .get_with_comments(id, &pager)
        .await?
        .ok_or(PageError::NotFound)?;

    Ok(Render(ProjectPage { project, comments }))
}
