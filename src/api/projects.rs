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
    user::User,
};

pub fn public_router() -> Router<AppState> {
    Router::new()
        .typed_get(list_projects)
        .typed_get(get_project)
}

pub fn protected_router() -> Router<AppState> {
    Router::new()
        .typed_get(get_project_form)
        .typed_post(post_project_form)
}

// Pages

#[derive(Copy, Clone, Default, TypedPath)]
#[typed_path("/projects")]
pub struct ListProjectsUrl;

async fn list_projects(
    _: ListProjectsUrl,
    State(repo): State<ProjectsRepository>,
    State(client): State<Arc<aws_sdk_s3::Client>>,
    user: Option<User>,
    WithRejection(Valid(Query(pager)), _): WithPageRejection<
        Valid<Query<Pager<(OffsetDateTime, String)>>>,
    >,
) -> PageResult<Render<ListProjectsPage>> {
    const ABOUT_KEY: &str = "projects";

    let projects = repo.list(&pager).await?;

    let about = String::from_utf8(
        client
            .get_object()
            .bucket(&AppConfig::get().buckets().system)
            .key(ABOUT_KEY)
            .send()
            .await?
            .body
            .collect()
            .await
            .map_err(|_| AppError::ObjectEncoding)?
            .to_vec(),
    )
    .map_err(|_| AppError::ObjectEncoding)?;

    Ok(Render(ListProjectsPage::new(user, about, projects)))
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
    user: Option<User>,
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

    Ok(Render(GetProjectPage::new(
        user, project, content, comments,
    )))
}

#[derive(Copy, Clone, TypedPath)]
#[typed_path("/projects/form")]
pub struct GetProjectFormUrl;

async fn get_project_form(_: GetProjectFormUrl, user: Option<User>) -> Render<ProjectFormPage> {
    Render(ProjectFormPage::new(user, ValidationErrors::new(), None))
}

#[derive(Copy, Clone, TypedPath)]
#[typed_path("/projects/form")]
pub struct PostProjectFormUrl;

async fn post_project_form(
    _: PostProjectFormUrl,
    State(repo): State<ProjectsRepository>,
    State(client): State<Arc<aws_sdk_s3::Client>>,
    user: Option<User>,
    WithRejection(parts, _): WithPageRejection<Multipart>,
) -> PageResult<Result<Redirect, Render<ProjectFormPage>>> {
    #[derive(Validate)]
    struct FormData {
        name: String,
        #[validate(length(min = 1))]
        content: String,
        thumbnail: (Option<String>, Option<String>, Bytes),
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
        thumbnail: Option<(Option<String>, Option<String>, Bytes)>,
        project_url: Option<Option<String>>,
    }

    impl IncompleteFormData {
        async fn from_multipart(mut parts: Multipart) -> Result<Self, MultipartError> {
            let mut data = IncompleteFormData::default();

            while let Some(field) = parts.next_field().await? {
                match field.name() {
                    Some("name") => data.name = Some(field.text().await?),
                    Some("description") => data.content = Some(field.text().await?),
                    Some("thumbnail") => {
                        data.thumbnail = Some((
                            field.content_type().map(ToOwned::to_owned),
                            field.file_name().map(ToOwned::to_owned),
                            field.bytes().await?,
                        ))
                    }
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
        Err(errors) => return Ok(Err(Render(ProjectFormPage::new(user, errors, None)))),
    };
    if let Err(errors) = data.validate() {
        return Ok(Err(Render(ProjectFormPage::new(user, errors, None))));
    }
    let project = NewProject::new(data.name, data.project_url);

    match project.validate() {
        Ok(_) => {
            let (project, transaction) = repo
                .create(&project, &generate_preview(&data.content))
                .await?;

            let mut builder = client
                .put_object()
                .bucket(&AppConfig::get().buckets().thumbnails)
                .key(project.thumbnail_id.to_string());

            if let Some(content_type) = data.thumbnail.0 {
                builder = builder.content_type(content_type);
            }

            if let Some(file_name) = data.thumbnail.1 {
                builder = builder.content_disposition(format!(r#"filename="{file_name}""#))
            }

            builder
                .body(ByteStream::new(SdkBody::from(data.thumbnail.2)))
                .send()
                .await?;

            client
                .put_object()
                .bucket(&AppConfig::get().buckets().content)
                .key(project.content_id.to_string())
                .content_disposition(r#"filename="content.md""#)
                .content_length(data.content.len() as i64)
                .content_type("text/markdown")
                .body(ByteStream::new(SdkBody::from(data.content)))
                .send()
                .await?;

            transaction.commit().await?;

            Ok(Ok(Redirect::to(
                &GetProjectUrl { id: project.id }.to_string(),
            )))
        }
        Err(errors) => Ok(Err(Render(ProjectFormPage::new(user, errors, None)))),
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
