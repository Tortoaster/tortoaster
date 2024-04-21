use std::sync::Arc;

use aws_sdk_s3::primitives::{ByteStream, SdkBody};
use axum::{
    extract::{Multipart, Query, State},
    response::Redirect,
    Router,
};
use axum_extra::{
    extract::WithRejection,
    routing::{RouterExt, TypedPath},
};
use axum_valid::Valid;
use serde::Deserialize;
use time::OffsetDateTime;
use validator::{Validate, ValidationErrors};

use crate::{
    config::AppConfig,
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
    WithRejection(Valid(Query(pager)), _): WithPageRejection<
        Valid<Query<Pager<(OffsetDateTime, String)>>>,
    >,
) -> PageResult<Render<ListProjectsPage>> {
    let projects = repo.list(&pager).await?;
    Ok(Render(ListProjectsPage::new(projects)))
}

#[derive(Clone, Deserialize, TypedPath)]
#[typed_path("/projects/:id")]
pub struct GetProjectUrl {
    id: String,
}

async fn get_project(
    GetProjectUrl { id }: GetProjectUrl,
    State(repo): State<ProjectsRepository>,
    WithRejection(Valid(Query(pager)), _): WithPageRejection<Valid<Query<Pager<i32>>>>,
) -> PageResult<Render<GetProjectPage>> {
    let (project, comments) = repo
        .get_with_comments(&id, &pager)
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
    State(client): State<Arc<aws_sdk_s3::Client>>,
    WithRejection(mut parts, _): WithPageRejection<Multipart>,
) -> PageResult<Result<Redirect, Render<ProjectFormPage>>> {
    const SUPPORTED_FILE_TYPES: [&str; 6] = [".png", ".jpg", ".jpeg", ".gif", ".webp", ".svg"];

    #[derive(Default)]
    struct IncompleteNewProject {
        name: Option<String>,
        description: Option<String>,
        project_url: Option<Option<String>>,
    }
    let mut thumbnail = None;

    let mut data = IncompleteNewProject::default();

    while let Some(field) = parts.next_field().await? {
        match field.name() {
            Some("name") => data.name = Some(field.text().await?),
            Some("description") => data.description = Some(field.text().await?),
            Some("project-url") => {
                let text = field.text().await?;
                data.project_url = Some((!text.is_empty()).then_some(text))
            }
            Some("thumbnail") => {
                let file_name = field.file_name().ok_or(AppError::MissingFile)?;
                if !SUPPORTED_FILE_TYPES
                    .iter()
                    .any(|extension| file_name.ends_with(extension))
                {
                    return Err(AppError::UnsupportedImageType.into());
                }
                thumbnail = Some(field.bytes().await?)
            }
            _ => continue,
        }
    }

    let thumbnail = thumbnail.ok_or(AppError::MissingFields)?;
    let project = NewProject::new(
        data.name.ok_or(AppError::MissingFields)?,
        data.description.ok_or(AppError::MissingFields)?,
        data.project_url.ok_or(AppError::MissingFields)?,
    );

    match project.validate() {
        Ok(_) => {
            let response = client
                .put_object()
                .bucket(AppConfig::get().thumbnail_bucket())
                .key(project.thumbnail_id.to_string())
                .body(ByteStream::new(SdkBody::from(thumbnail)))
                .send()
                .await?;
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
    WithRejection(Valid(Query(pager)), _): WithToastRejection<
        Valid<Query<Pager<(OffsetDateTime, String)>>>,
    >,
) -> ToastResult<PaginatedResponse<Project, ListProjectsPartialUrl, (OffsetDateTime, String)>> {
    let items = repo.list(&pager).await?;
    Ok(PaginatedResponse { items, url, pager })
}

#[derive(Copy, Clone, TypedPath)]
#[typed_path("/partial/projects/form")]
pub struct ProjectFormPartialUrl;

async fn project_form_partial(_: ProjectFormPartialUrl) -> Render<ProjectForm> {
    Render(ProjectForm {
        errors: ValidationErrors::new(),
        project: None,
    })
}
