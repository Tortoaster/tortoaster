use std::fmt::Display;

use askama::Template;
use validator::ValidationErrors;

use crate::{
    api::{
        auth::{LoginUrl, LogoutUrl},
        projects::ProjectsFormUrl,
    },
    dto::projects::{ProjectPreview, ProjectWithComments},
    user::User,
};

// Pages

#[derive(Template)]
#[template(path = "projects/page_list.html")]
pub struct ListProjectsPage {
    user: Option<User>,
    login_url: LoginUrl,
    logout_url: LogoutUrl,
    new_project_url: ProjectsFormUrl,
    about: String,
    projects: Vec<ProjectPreview>,
}

impl ListProjectsPage {
    pub fn new(user: Option<User>, about: String, projects: Vec<ProjectPreview>) -> Self {
        Self {
            user,
            login_url: LoginUrl,
            logout_url: LogoutUrl,
            new_project_url: ProjectsFormUrl,
            about,
            projects,
        }
    }
}

#[derive(Template)]
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

#[derive(Default, Template)]
#[template(path = "projects/page_form.html")]
pub struct ProjectFormPage<Url: Display> {
    action: Url,
    user: Option<User>,
    login_url: LoginUrl,
    logout_url: LogoutUrl,
    errors: ValidationErrors,
    project: Option<ProjectWithComments>,
}

impl<Url: Display> ProjectFormPage<Url> {
    pub fn new(
        action: Url,
        user: Option<User>,
        errors: ValidationErrors,
        project: Option<ProjectWithComments>,
    ) -> Self {
        Self {
            action,
            user,
            login_url: LoginUrl,
            logout_url: LogoutUrl,
            errors,
            project,
        }
    }
}

// Partials

#[derive(Template)]
#[template(path = "projects/component.html")]
pub struct ProjectComponent {
    pub project: ProjectWithComments,
}

#[derive(Default, Template)]
#[template(path = "projects/form.html")]
pub struct ProjectForm {
    pub action: String,
    pub errors: ValidationErrors,
    pub project: Option<ProjectWithComments>,
}

mod filters {
    use askama::Result;
    use sqlx::types::time::OffsetDateTime;
    use time_humanize::HumanTime;

    pub fn humantime(time: &OffsetDateTime) -> Result<String> {
        let human_time = HumanTime::from_duration_since_timestamp(time.unix_timestamp() as u64)
            - HumanTime::now();

        Ok(human_time.to_string())
    }
}
