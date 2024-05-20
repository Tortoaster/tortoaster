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
use validator::ValidationErrors;

use crate::{
    config::AppBucket,
    dto::projects::ProjectPreview,
    error::{AppError, PageResult, ToastResult, WithPageRejection, WithToastRejection},
    pagination::{Pager, PaginatedResponse},
    repository::{files::FileRepository, projects::ProjectsRepository},
    state::AppState,
    template::{
        projects::{GetProjectPage, ListProjectsPage, ProjectForm, ProjectFormPage},
        Render,
    },
    user::User,
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
        .typed_put(put_project)
}

// Forms

#[derive(Copy, Clone, TypedPath)]
#[typed_path("/projects/form")]
pub struct ProjectsFormUrl;

#[derive(Clone, Deserialize, TypedPath)]
#[typed_path("/projects/:id/form")]
pub struct SingleProjectFormUrl {
    id: String,
}

async fn get_project_post_form(
    _: ProjectsFormUrl,
    user: Option<User>,
) -> Render<ProjectFormPage<ProjectsUrl>> {
    Render(ProjectFormPage::new(
        ProjectsUrl,
        user,
        ValidationErrors::new(),
        None,
    ))
}

async fn get_project_put_form(
    SingleProjectFormUrl { id }: SingleProjectFormUrl,
    user: Option<User>,
) -> Render<ProjectFormPage<SingleProjectUrl>> {
    Render(ProjectFormPage::new(
        SingleProjectUrl { id },
        user,
        ValidationErrors::new(),
        None,
    ))
}

// API Pages

#[derive(Copy, Clone, TypedPath)]
#[typed_path("/projects")]
pub struct ProjectsUrl;

#[derive(Clone, Deserialize, TypedPath)]
#[typed_path("/projects/:id")]
pub struct SingleProjectUrl {
    id: String,
}

async fn list_projects(
    _: ProjectsUrl,
    State(repo): State<ProjectsRepository>,
    State(file_repo): State<FileRepository>,
    user: Option<User>,
    WithRejection(Valid(Query(pager)), _): WithPageRejection<
        Valid<Query<Pager<(OffsetDateTime, String)>>>,
    >,
) -> PageResult<Render<ListProjectsPage>> {
    const ABOUT_KEY: &str = "projects";

    let projects = repo.list(&pager).await?;

    let about = file_repo
        .retrieve_markdown(ABOUT_KEY, AppBucket::System)
        .await?;

    Ok(Render(ListProjectsPage::new(user, about, projects)))
}

async fn get_project(
    SingleProjectUrl { id }: SingleProjectUrl,
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

async fn put_project(
    _: SingleProjectUrl,
    State(repo): State<ProjectsRepository>,
    user: Option<User>,
    WithRejection(parts, _): WithPageRejection<Multipart>,
) {
}

async fn post_project(
    _: ProjectsUrl,
    State(repo): State<ProjectsRepository>,
    user: Option<User>,
    WithRejection(parts, _): WithPageRejection<Multipart>,
) -> PageResult<Result<Redirect, Render<ProjectFormPage<ProjectsUrl>>>> {
    let project = match form_helper::parse_form_data(parts).await? {
        Ok(data) => data,
        Err(errors) => {
            return Ok(Err(Render(ProjectFormPage::new(
                ProjectsUrl,
                user,
                errors,
                None,
            ))))
        }
    };

    let project = repo.create(project).await?;

    Ok(Ok(Redirect::to(
        &SingleProjectUrl { id: project.id }.to_string(),
    )))
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
) -> ToastResult<PaginatedResponse<ProjectPreview, ListProjectsPartialUrl, (OffsetDateTime, String)>>
{
    let items = repo.list(&pager).await?;
    Ok(PaginatedResponse { items, url, pager })
}

#[derive(Copy, Clone, TypedPath)]
#[typed_path("/partial/projects/form")]
pub struct ProjectFormPartialUrl;

async fn project_form_partial(_: ProjectFormPartialUrl) -> Render<ProjectForm> {
    Render(ProjectForm {
        action: String::new(),
        errors: ValidationErrors::new(),
        project: None,
    })
}

mod form_helper {
    use std::mem::MaybeUninit;

    use axum::extract::Multipart;
    use bytes::Bytes;
    use validator::{Validate, ValidationError, ValidationErrors};

    use crate::{
        config::AppBucket,
        dto::projects::NewProject,
        error::{AppError, AppResult},
        repository::files::AppFile,
    };

    impl TryFrom<IncompleteFormData> for NewProject {
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
                NewProject {
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
        thumbnail: Option<AppFile<Bytes>>,
        project_url: Option<Option<String>>,
    }

    impl IncompleteFormData {
        async fn from_multipart(mut parts: Multipart) -> AppResult<Self> {
            let mut data = IncompleteFormData::default();

            while let Some(field) = parts.next_field().await? {
                match field.name() {
                    Some("name") => data.name = Some(field.text().await?),
                    Some("description") => data.content = Some(field.text().await?),
                    Some("thumbnail") => {
                        let content_type = field
                            .content_type()
                            .and_then(|content_type| content_type.parse().ok())
                            .ok_or(AppError::FileType)?;

                        data.thumbnail = Some(AppFile::new(
                            field.bytes().await?,
                            AppBucket::Thumbnails,
                            content_type,
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

    pub async fn parse_form_data(
        parts: Multipart,
    ) -> AppResult<Result<NewProject, ValidationErrors>> {
        let result: Result<NewProject, _> =
            IncompleteFormData::from_multipart(parts).await?.try_into();

        let data = match result {
            Ok(data) => data,
            Err(errors) => {
                return Ok(Err(errors));
            }
        };

        match data.validate() {
            Ok(_) => Ok(Ok(data)),
            Err(errors) => Ok(Err(errors)),
        }
    }
}
