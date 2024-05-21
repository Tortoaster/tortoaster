use askama::Template;
use validator::ValidationErrors;

use crate::{
    api::{
        auth::{LoginUrl, LogoutUrl},
        files::PostImageUrl,
        projects::{GetProjectPostFormUrl, PostProjectUrl, PostPutProjectUrl},
    },
    dto::projects::{ProjectPreview, ProjectView, ProjectWithComments},
    user::User,
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
    project: ProjectView,
}

impl UpdateProjectFormPage {
    pub fn new(
        user: Option<User>,
        post_put_project_url: PostPutProjectUrl,
        errors: ValidationErrors,
        project: ProjectView,
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

// Pages

#[derive(Debug, Template)]
#[template(path = "projects/page_list.html")]
pub struct ListProjectsPage {
    user: Option<User>,
    login_url: LoginUrl,
    logout_url: LogoutUrl,
    new_project_url: GetProjectPostFormUrl,
    about: String,
    projects: Vec<ProjectPreview>,
}

impl ListProjectsPage {
    pub fn new(user: Option<User>, about: String, projects: Vec<ProjectPreview>) -> Self {
        Self {
            user,
            login_url: LoginUrl,
            logout_url: LogoutUrl,
            new_project_url: GetProjectPostFormUrl,
            about,
            projects,
        }
    }
}

#[derive(Debug, Template)]
#[template(path = "projects/page_get.html")]
pub struct GetProjectPage {
    user: Option<User>,
    login_url: LoginUrl,
    logout_url: LogoutUrl,
    project: ProjectWithComments,
}

impl GetProjectPage {
    pub fn new(user: Option<User>, project: ProjectWithComments) -> Self {
        Self {
            user,
            login_url: LoginUrl,
            logout_url: LogoutUrl,
            project,
        }
    }
}

// Partials

#[derive(Debug, Template)]
#[template(path = "projects/component.html")]
pub struct ProjectComponent {
    pub project: ProjectWithComments,
}

mod filters {
    use std::convert::Infallible;

    use askama::Result;
    use sqlx::types::time::OffsetDateTime;
    use time_humanize::HumanTime;

    pub fn humantime(time: &OffsetDateTime) -> Result<String> {
        let human_time = HumanTime::from_duration_since_timestamp(time.unix_timestamp() as u64)
            - HumanTime::now();

        Ok(human_time.to_string())
    }

    pub fn markdown(s: impl AsRef<str>) -> Result<String, Infallible> {
        let parser = pulldown_cmark::Parser::new(s.as_ref());
        let mut html = String::new();

        pulldown_cmark::html::push_html(&mut html, parser);

        Ok(html)
    }
}
