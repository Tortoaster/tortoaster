use askama::Template;
use validator::ValidationErrors;

use crate::{
    api::{
        comments::PostCommentUrl,
        files::PostImageUrl,
        projects::{
            GetProjectDeleteFormUrl, GetProjectPostFormUrl, GetProjectPutFormUrl, GetProjectsUrl,
            PostDeleteProjectUrl, PostProjectUrl, PostPutProjectUrl,
        },
        users::{LoginUrl, LogoutUrl},
    },
    config::AppConfig,
    dto::{
        comments::CommentWithUser,
        projects::{Project, ProjectData, ProjectName, ProjectPreview},
        users::User,
    },
    template::filters,
    utils::pagination::Page,
};
// Forms

#[derive(Debug, Template)]
#[template(path = "projects/form/create_form_page.html")]
pub struct CreateProjectFormPage {
    user: Option<User>,
    login_url: LoginUrl,
    logout_url: LogoutUrl,
    post_image_url: PostImageUrl,
    post_project_url: PostProjectUrl,
    errors: ValidationErrors,
}

impl CreateProjectFormPage {
    pub fn new(user: Option<User>, errors: ValidationErrors) -> Self {
        Self {
            user,
            login_url: LoginUrl,
            logout_url: LogoutUrl,
            post_image_url: PostImageUrl,
            post_project_url: PostProjectUrl,
            errors,
        }
    }
}

#[derive(Debug, Template)]
#[template(path = "projects/form/update_form_page.html")]
pub struct UpdateProjectFormPage {
    user: Option<User>,
    login_url: LoginUrl,
    logout_url: LogoutUrl,
    post_image_url: PostImageUrl,
    post_put_project_url: PostPutProjectUrl,
    thumbnail_bucket_url: String,
    errors: ValidationErrors,
    project: ProjectData,
}

impl UpdateProjectFormPage {
    pub fn new(
        user: Option<User>,
        post_put_project_url: PostPutProjectUrl,
        errors: ValidationErrors,
        project: ProjectData,
    ) -> Self {
        Self {
            user,
            login_url: LoginUrl,
            logout_url: LogoutUrl,
            post_image_url: PostImageUrl,
            post_put_project_url,
            thumbnail_bucket_url: AppConfig::get().s3_thumbnail_bucket_url().to_owned(),
            errors,
            project,
        }
    }
}

#[derive(Debug, Template)]
#[template(path = "projects/form/delete_form_page.html")]
pub struct DeleteProjectFormPage {
    user: Option<User>,
    login_url: LoginUrl,
    logout_url: LogoutUrl,
    post_delete_project_url: PostDeleteProjectUrl,
    project: ProjectName,
}

impl DeleteProjectFormPage {
    pub fn new(
        user: Option<User>,
        post_delete_project_url: PostDeleteProjectUrl,
        project: ProjectName,
    ) -> Self {
        Self {
            user,
            login_url: LoginUrl,
            logout_url: LogoutUrl,
            post_delete_project_url,
            project,
        }
    }
}

// Pages

#[derive(Debug, Template)]
#[template(path = "projects/list_page.html")]
pub struct ListProjectsPage {
    user: Option<User>,
    login_url: LoginUrl,
    logout_url: LogoutUrl,
    get_project_post_form_url: GetProjectPostFormUrl,
    get_projects_url: GetProjectsUrl,
    thumbnail_bucket_url: String,
    about: String,
    page: Page<ProjectPreview>,
}

impl ListProjectsPage {
    pub fn new(user: Option<User>, about: String, page: Page<ProjectPreview>) -> Self {
        Self {
            user,
            login_url: LoginUrl,
            logout_url: LogoutUrl,
            get_project_post_form_url: GetProjectPostFormUrl,
            get_projects_url: GetProjectsUrl,
            thumbnail_bucket_url: AppConfig::get().s3_thumbnail_bucket_url().to_owned(),
            about,
            page,
        }
    }
}

#[derive(Debug, Template)]
#[template(path = "projects/read_page.html")]
pub struct GetProjectPage {
    user: Option<User>,
    login_url: LoginUrl,
    logout_url: LogoutUrl,
    get_project_put_form_url: GetProjectPutFormUrl,
    get_project_delete_form_url: GetProjectDeleteFormUrl,
    post_comment_url: PostCommentUrl,
    thumbnail_bucket_url: String,
    project: Project,
    content: String,
    comments: Vec<CommentWithUser>,
}

impl GetProjectPage {
    pub fn new(
        user: Option<User>,
        project: Project,
        content: String,
        comments: Vec<CommentWithUser>,
    ) -> Self {
        Self {
            user,
            login_url: LoginUrl,
            logout_url: LogoutUrl,
            get_project_put_form_url: GetProjectPutFormUrl {
                id: project.id.clone(),
            },
            get_project_delete_form_url: GetProjectDeleteFormUrl {
                id: project.id.clone(),
            },
            post_comment_url: PostCommentUrl {
                project_id: project.id.clone(),
            },
            thumbnail_bucket_url: AppConfig::get().s3_thumbnail_bucket_url().to_owned(),
            project,
            content,
            comments,
        }
    }
}
