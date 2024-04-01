use axum::{
    extract::{Query, State},
    response::Redirect,
    Form, Router,
};
use axum_extra::{
    extract::WithRejection,
    routing::{RouterExt, TypedPath},
};
use axum_valid::Valid;
use serde::Deserialize;
use validator::{Validate, ValidationErrors};

use crate::{
    dto::projects::{NewProject, Project},
    error::{AppError, PageResult, ToastResult, WithPageRejection, WithToastRejection},
    pagination::{Pager, PaginatedResponse},
    render::Render,
    repository::projects::ProjectsRepository,
    state::AppState,
    template::projects::{GetProjectPage, ListProjectsPage, ProjectForm, ProjectFormPage},
};

pub fn router() -> Router<AppState> {
    Router::new()
        .typed_get(list_projects)
        .typed_get(list_projects_partial)
        .typed_get(get_project)
        .typed_get(get_project_form)
        .typed_post(post_project_form)
        .typed_get(project_form_partial)
}

// Pages

#[derive(Copy, Clone, Default, TypedPath)]
#[typed_path("/projects")]
pub struct ListProjectsUrl;

async fn list_projects(
    _: ListProjectsUrl,
    State(repo): State<ProjectsRepository>,
    WithRejection(Valid(Query(pager)), _): WithPageRejection<Valid<Query<Pager<i32>>>>,
) -> PageResult<Render<ListProjectsPage>> {
    let projects = repo.list(&pager).await?;
    Ok(Render(ListProjectsPage::new(projects)))
}

#[derive(Copy, Clone, Deserialize, TypedPath)]
#[typed_path("/projects/:id")]
pub struct GetProjectUrl {
    id: i32,
}

async fn get_project(
    GetProjectUrl { id }: GetProjectUrl,
    State(repo): State<ProjectsRepository>,
    WithRejection(Valid(Query(pager)), _): WithPageRejection<Valid<Query<Pager<i32>>>>,
) -> PageResult<Render<GetProjectPage>> {
    let (project, comments) = repo
        .get_with_comments(id, &pager)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Render(GetProjectPage { project, comments }))
}

#[derive(Copy, Clone, TypedPath)]
#[typed_path("/projects/form")]
pub struct GetProjectFormUrl;

async fn get_project_form(_: GetProjectFormUrl) -> Render<ProjectFormPage> {
    Render(ProjectFormPage {
        errors: ValidationErrors::new(),
        project: None,
    })
}

#[derive(Copy, Clone, TypedPath)]
#[typed_path("/projects/form")]
pub struct PostProjectFormUrl;

async fn post_project_form(
    _: PostProjectFormUrl,
    State(repo): State<ProjectsRepository>,
    WithRejection(Form(project), _): WithPageRejection<Form<NewProject>>,
) -> PageResult<Result<Redirect, Render<ProjectFormPage>>> {
    match project.validate() {
        Ok(_) => {
            let project = repo.create(&project).await?;
            Ok(Ok(Redirect::to(
                &GetProjectUrl { id: project.id }.to_string(),
            )))
        }
        Err(errors) => Ok(Err(Render(ProjectFormPage {
            errors,
            project: None,
        }))),
    }
}

// Partials

#[derive(Copy, Clone, Default, TypedPath)]
#[typed_path("/partial/projects")]
pub struct ListProjectsPartialUrl;

async fn list_projects_partial(
    url: ListProjectsPartialUrl,
    State(repo): State<ProjectsRepository>,
    WithRejection(Valid(Query(pager)), _): WithToastRejection<Valid<Query<Pager<i32>>>>,
) -> ToastResult<PaginatedResponse<Project, ListProjectsPartialUrl, i32>> {
    let items = repo.list(&pager).await?;
    Ok(PaginatedResponse { items, url, pager })
}

#[derive(Copy, Clone, TypedPath)]
#[typed_path("/partial/projects/form")]
pub struct ProjectFormPartialUrl;

async fn project_form_partial(_: ProjectFormPartialUrl) -> Render<ProjectForm> {
    Render(ProjectForm { project: None })
}
