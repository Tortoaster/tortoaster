use std::{
    mem::MaybeUninit,
    sync::{Arc, OnceLock},
};

use aws_sdk_s3::primitives::{ByteStream, SdkBody};
use axum::{
    extract::{multipart::MultipartError, Multipart, Query, State},
    response::Redirect,
    Router,
};
use axum_extra::{
    extract::WithRejection,
    routing::{RouterExt, TypedPath},
};
use axum_valid::Valid;
use bytes::Bytes;
use comrak::Options;
use regex::Regex;
use serde::Deserialize;
use time::OffsetDateTime;
use validator::{Validate, ValidationError, ValidationErrors};

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
    State(client): State<Arc<aws_sdk_s3::Client>>,
    WithRejection(Valid(Query(pager)), _): WithPageRejection<Valid<Query<Pager<i32>>>>,
) -> PageResult<Render<GetProjectPage>> {
    let (project, comments) = repo
        .get_with_comments(&id, &pager)
        .await?
        .ok_or(AppError::NotFound)?;

    let content = String::from_utf8(
        client
            .get_object()
            .bucket(&AppConfig::get().buckets().content)
            .key(project.content_id.to_string())
            .send()
            .await?
            .body
            .collect()
            .await
            .map_err(|_| AppError::ObjectEncoding)?
            .to_vec(),
    )
    .map_err(|_| AppError::ObjectEncoding)?;

    Ok(Render(GetProjectPage {
        project,
        content,
        comments,
    }))
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
    WithRejection(parts, _): WithPageRejection<Multipart>,
) -> PageResult<Result<Redirect, Render<ProjectFormPage>>> {
    #[derive(Validate)]
    struct FormData {
        name: String,
        #[validate(length(min = 1))]
        content: String,
        thumbnail: Bytes,
        project_url: Option<String>,
    }

    impl TryFrom<IncompleteFormData> for FormData {
        type Error = ValidationErrors;

        fn try_from(value: IncompleteFormData) -> Result<Self, Self::Error> {
            let mut errors = ValidationErrors::new();

            let name = match value.name {
                None => {
                    errors.add("name", ValidationError::new("Please fill out this field"));
                    MaybeUninit::uninit()
                }
                Some(name) => MaybeUninit::new(name),
            };
            let content = match value.content {
                None => {
                    errors.add(
                        "content",
                        ValidationError::new("Please fill out this field"),
                    );
                    MaybeUninit::uninit()
                }
                Some(content) => MaybeUninit::new(content),
            };
            let thumbnail = match value.thumbnail {
                None => {
                    errors.add(
                        "thumbnail",
                        ValidationError::new("Please fill out this field"),
                    );
                    MaybeUninit::uninit()
                }
                Some(thumbnail) => MaybeUninit::new(thumbnail),
            };
            let project_url = match value.project_url {
                None => {
                    errors.add(
                        "project_url",
                        ValidationError::new("Please fill out this field"),
                    );
                    MaybeUninit::uninit()
                }
                Some(project_url) => MaybeUninit::new(project_url),
            };

            if !errors.is_empty() {
                return Err(errors);
            }

            let data = unsafe {
                FormData {
                    name: name.assume_init(),
                    content: content.assume_init(),
                    thumbnail: thumbnail.assume_init(),
                    project_url: project_url.assume_init(),
                }
            };

            Ok(data)
        }
    }

    #[derive(Default)]
    struct IncompleteFormData {
        name: Option<String>,
        content: Option<String>,
        thumbnail: Option<Bytes>,
        project_url: Option<Option<String>>,
    }

    impl IncompleteFormData {
        async fn from_multipart(mut parts: Multipart) -> Result<Self, MultipartError> {
            let mut data = IncompleteFormData::default();

            while let Some(field) = parts.next_field().await? {
                match field.name() {
                    Some("name") => data.name = Some(field.text().await?),
                    Some("description") => data.content = Some(field.text().await?),
                    Some("thumbnail") => data.thumbnail = Some(field.bytes().await?),
                    Some("project-url") => {
                        let text = field.text().await?;
                        data.project_url = Some((!text.is_empty()).then_some(text))
                    }
                    _ => continue,
                }
            }

            Ok(data)
        }
    }

    fn generate_preview(content: &str) -> String {
        const PREVIEW_LENGTH: usize = 300;
        static RE: OnceLock<Regex> = OnceLock::new();

        let html = comrak::markdown_to_html(content, &Options::default());
        let re = RE.get_or_init(|| Regex::new(r"<[^>]*>").unwrap());
        let mut preview = re.replace_all(&html, "").to_string();

        if preview.len() >= PREVIEW_LENGTH {
            preview.truncate(PREVIEW_LENGTH - 3);
            preview += "...";
        }

        preview
    }

    let result: Result<FormData, _> = IncompleteFormData::from_multipart(parts).await?.try_into();
    let data = match result {
        Ok(data) => data,
        Err(errors) => {
            return Ok(Err(Render(ProjectFormPage {
                errors,
                project: None,
            })))
        }
    };
    if let Err(errors) = data.validate() {
        return Ok(Err(Render(ProjectFormPage {
            errors,
            project: None,
        })));
    }
    let project = NewProject::new(data.name, data.project_url);

    match project.validate() {
        Ok(_) => {
            let (project, transaction) = repo
                .create(&project, &generate_preview(&data.content))
                .await?;

            client
                .put_object()
                .bucket(&AppConfig::get().buckets().thumbnails)
                .key(project.thumbnail_id.to_string())
                .body(ByteStream::new(SdkBody::from(data.thumbnail)))
                .send()
                .await?;

            client
                .put_object()
                .bucket(&AppConfig::get().buckets().content)
                .key(project.content_id.to_string())
                .body(ByteStream::new(SdkBody::from(data.content)))
                .send()
                .await?;

            transaction.commit().await?;

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
