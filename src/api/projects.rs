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
    config::AppBucket,
    dto::projects::{NewProject, ProjectId, ProjectIndex, ProjectView},
    error::{AppError, PageResult, WithPageRejection},
    repository::{files::FileRepository, projects::ProjectsRepository},
    state::AppState,
    template::{
        projects::{
            CreateProjectFormPage, GetProjectPage, ListProjectsPage, UpdateProjectFormPage,
        },
        Render,
    },
    user::User,
    utils::pagination::Pager,
};

pub fn public_router() -> Router<AppState> {
    Router::new()
        .typed_get(list_projects)
        .typed_get(get_project)
}

pub fn protected_router() -> Router<AppState> {
    Router::new()
        .typed_get(get_project_post_form)
        .typed_get(get_project_put_form)
        .typed_post(post_project)
        .typed_post(post_put_project)
}

// Forms

#[derive(Copy, Clone, Debug, TypedPath)]
#[typed_path("/projects/form")]
pub struct GetProjectPostFormUrl;

#[derive(Clone, Debug, Deserialize, TypedPath)]
#[typed_path("/projects/:id/form")]
pub struct GetProjectPutFormUrl {
    id: String,
}

async fn get_project_post_form(
    _: GetProjectPostFormUrl,
    user: Option<User>,
) -> Render<CreateProjectFormPage> {
    Render(CreateProjectFormPage::new(user, ValidationErrors::new()))
}

async fn get_project_put_form(
    GetProjectPutFormUrl { id }: GetProjectPutFormUrl,
    State(repo): State<ProjectsRepository>,
    user: Option<User>,
) -> PageResult<Render<UpdateProjectFormPage>> {
    let project = repo.get(&id).await?;

    Ok(Render(UpdateProjectFormPage::new(
        user,
        PostPutProjectUrl { id },
        ValidationErrors::new(),
        project,
    )))
}

// API Pages

#[derive(Copy, Clone, Debug, TypedPath)]
#[typed_path("/projects")]
pub struct GetProjectsUrl;

#[derive(Clone, Debug, Deserialize, TypedPath)]
#[typed_path("/projects/:id")]
pub struct GetProjectUrl {
    pub id: String,
}

#[derive(Copy, Clone, Debug, TypedPath)]
#[typed_path("/projects")]
pub struct PostProjectUrl;

#[derive(Clone, Debug, Deserialize, TypedPath)]
#[typed_path("/projects/:id/put")]
pub struct PostPutProjectUrl {
    pub id: String,
}

async fn list_projects(
    _: GetProjectsUrl,
    State(repo): State<ProjectsRepository>,
    State(file_repo): State<FileRepository>,
    user: Option<User>,
    WithRejection(Valid(Query(pager)), _): WithPageRejection<Valid<Query<Pager<ProjectIndex>>>>,
) -> PageResult<Render<ListProjectsPage>> {
    const ABOUT_KEY: &str = "projects";

    let about = file_repo
        .retrieve_markdown(ABOUT_KEY, AppBucket::System)
        .await?;

    let page = repo.list(&pager).await?;

    Ok(Render(ListProjectsPage::new(user, about, page)))
}

async fn get_project(
    GetProjectUrl { id }: GetProjectUrl,
    State(repo): State<ProjectsRepository>,
    user: Option<User>,
    WithRejection(Valid(Query(pager)), _): WithPageRejection<Valid<Query<Pager<i32>>>>,
) -> PageResult<Render<GetProjectPage>> {
    let project = repo
        .get_with_comments(&id, &pager)
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Render(GetProjectPage::new(user, project)))
}

async fn post_project(
    _: PostProjectUrl,
    State(repo): State<ProjectsRepository>,
    user: Option<User>,
    WithRejection(new_project, _): WithPageRejection<Form<NewProject>>,
) -> PageResult<Result<Redirect, Render<CreateProjectFormPage>>> {
    if let Err(errors) = new_project.validate() {
        return Ok(Err(Render(CreateProjectFormPage::new(user, errors))));
    }

    let ProjectId { id } = repo.create(new_project.0).await?;

    Ok(Ok(Redirect::to(&GetProjectUrl { id }.to_string())))
}

async fn post_put_project(
    PostPutProjectUrl { id }: PostPutProjectUrl,
    State(repo): State<ProjectsRepository>,
    user: Option<User>,
    WithRejection(new_project, _): WithPageRejection<Form<NewProject>>,
) -> PageResult<Result<Redirect, Render<UpdateProjectFormPage>>> {
    if let Err(errors) = new_project.validate() {
        let project = ProjectView {
            name: new_project.0.name,
            content: new_project.0.content,
            thumbnail_id: new_project.0.thumbnail_id,
            project_url: new_project.0.project_url,
        };
        return Ok(Err(Render(UpdateProjectFormPage::new(
            user,
            PostPutProjectUrl { id },
            errors,
            project,
        ))));
    }

    repo.update(&id, new_project.0).await?;

    Ok(Ok(Redirect::to(&GetProjectUrl { id }.to_string())))
}
