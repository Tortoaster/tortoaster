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
    dto::projects::{NewProject, ProjectPreview, UpdateProject},
    error::{AppError, PageResult, ToastResult, WithPageRejection, WithToastRejection},
    repository::{files::FileRepository, projects::ProjectsRepository},
    state::AppState,
    template::{
        projects::{GetProjectPage, ListProjectsPage, ProjectFormPage},
        Render,
    },
    user::User,
    utils::pagination::{Pager, PaginatedResponse},
};

pub fn public_router() -> Router<AppState> {
    Router::new()
        .typed_get(list_projects)
        .typed_get(get_project)
}

pub fn protected_router() -> Router<AppState> {
    Router::new()
        .typed_get(get_project_post_form)
        .typed_get(get_project_patch_form)
        .typed_post(post_project)
        .typed_post(post_patch_project)
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
) -> Render<ProjectFormPage<ListProjectsUrl>> {
    Render(ProjectFormPage::new(
        "Create Project",
        ListProjectsUrl,
        user,
        ValidationErrors::new(),
        None,
    ))
}

async fn get_project_patch_form(
    SingleProjectFormUrl { id }: SingleProjectFormUrl,
    State(repo): State<ProjectsRepository>,
    user: Option<User>,
) -> PageResult<Render<ProjectFormPage<PostPatchProjectUrl>>> {
    let project = repo.get_name_content_url(&id).await?;

    Ok(Render(ProjectFormPage::new(
        "Update Project",
        PostPatchProjectUrl { id },
        user,
        ValidationErrors::new(),
        Some(project),
    )))
}

// API Pages

#[derive(Copy, Clone, TypedPath)]
#[typed_path("/projects")]
pub struct ListProjectsUrl;

#[derive(Clone, Deserialize, TypedPath)]
#[typed_path("/projects/:id")]
pub struct GetProjectUrl {
    id: String,
}

#[derive(Clone, Deserialize, TypedPath)]
#[typed_path("/projects/:id/patch")]
pub struct PostPatchProjectUrl {
    id: String,
}

async fn list_projects(
    _: ListProjectsUrl,
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
    _: ListProjectsUrl,
    State(repo): State<ProjectsRepository>,
    user: Option<User>,
    WithRejection(parts, _): WithPageRejection<Multipart>,
) -> PageResult<Result<Redirect, Render<ProjectFormPage<ListProjectsUrl>>>> {
    let project = match NewProject::try_from_multipart(parts).await? {
        Ok(data) => data,
        Err(errors) => {
            return Ok(Err(Render(ProjectFormPage::new(
                "Create Project",
                ListProjectsUrl,
                user,
                errors,
                None,
            ))))
        }
    };

    let project = repo.create(project).await?;

    Ok(Ok(Redirect::to(
        &GetProjectUrl { id: project.id }.to_string(),
    )))
}

async fn post_patch_project(
    PostPatchProjectUrl { id }: PostPatchProjectUrl,
    State(repo): State<ProjectsRepository>,
    user: Option<User>,
    WithRejection(parts, _): WithPageRejection<Multipart>,
) -> PageResult<Result<Render<GetProjectPage>, Render<ProjectFormPage<GetProjectUrl>>>> {
    let project = match UpdateProject::try_from_multipart(parts).await? {
        Ok(data) => data,
        Err(errors) => {
            return Ok(Err(Render(ProjectFormPage::new(
                "Update Project",
                GetProjectUrl { id },
                user,
                errors,
                None,
            ))))
        }
    };

    repo.update(&id, project).await?;
    let project = repo
        .get_with_comments(&id, &Pager::default())
        .await?
        .ok_or(AppError::NotFound)?;

    Ok(Ok(Render(GetProjectPage::new(user, project))))
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

// TODO: Replace with derive macros
mod form_helper {
    use std::mem::MaybeUninit;

    use axum::extract::Multipart;
    use bytes::Bytes;
    use validator::{Validate, ValidationError, ValidationErrors};

    use crate::{
        config::AppBucket,
        dto::projects::{NewProject, UpdateProject},
        error::{AppError, AppResult},
        repository::files::AppFile,
    };

    #[derive(Default)]
    struct IncompleteProject {
        name: Option<String>,
        content: Option<String>,
        thumbnail: Option<AppFile<Bytes>>,
        project_url: Option<Option<String>>,
    }

    impl IncompleteProject {
        async fn from_multipart(mut parts: Multipart) -> AppResult<Self> {
            let mut data = IncompleteProject::default();

            while let Some(field) = parts.next_field().await? {
                match field.name() {
                    Some("name") => data.name = Some(field.text().await?),
                    Some("content") => data.content = Some(field.text().await?),
                    Some("thumbnail") => {
                        let content_type = field
                            .content_type()
                            .and_then(|content_type| content_type.parse().ok())
                            .ok_or(AppError::FileType);
                        let content = field.bytes().await?;

                        if !content.is_empty() {
                            data.thumbnail =
                                Some(AppFile::new(content, AppBucket::Thumbnails, content_type?))
                        }
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

    impl TryFrom<IncompleteProject> for NewProject {
        type Error = ValidationErrors;

        fn try_from(value: IncompleteProject) -> Result<Self, Self::Error> {
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
                Self {
                    name: name.assume_init(),
                    content: content.assume_init(),
                    thumbnail: thumbnail.assume_init(),
                    project_url: project_url.assume_init(),
                }
            };

            Ok(data)
        }
    }

    impl NewProject {
        pub async fn try_from_multipart(
            parts: Multipart,
        ) -> AppResult<Result<Self, ValidationErrors>> {
            let result: Result<Self, _> =
                IncompleteProject::from_multipart(parts).await?.try_into();

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

    impl TryFrom<IncompleteProject> for UpdateProject {
        type Error = ValidationErrors;

        fn try_from(value: IncompleteProject) -> Result<Self, Self::Error> {
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
                None => MaybeUninit::new(None),
                Some(thumbnail) => MaybeUninit::new(Some(thumbnail)),
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
                Self {
                    name: name.assume_init(),
                    content: content.assume_init(),
                    thumbnail: thumbnail.assume_init(),
                    project_url: project_url.assume_init(),
                }
            };

            Ok(data)
        }
    }

    impl UpdateProject {
        pub async fn try_from_multipart(
            parts: Multipart,
        ) -> AppResult<Result<Self, ValidationErrors>> {
            let result: Result<Self, _> =
                IncompleteProject::from_multipart(parts).await?.try_into();

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
}
