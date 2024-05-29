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
    dto::{
        projects::{ProjectData, ProjectName, ProjectPreview, ProjectWithComments},
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
    project: ProjectWithComments,
}

impl GetProjectPage {
    pub fn new(user: Option<User>, project: ProjectWithComments) -> Self {
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
            project,
        }
    }
}
