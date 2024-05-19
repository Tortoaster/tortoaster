use std::fmt::Display;

use askama::Template;
use validator::ValidationErrors;

use crate::{
    api::{
        auth::{LoginUrl, LogoutUrl},
        projects::ProjectsFormUrl,
    },
    dto::{comments::Comment, projects::Project},
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
    projects: Vec<Project>,
}

impl ListProjectsPage {
    pub fn new(user: Option<User>, about: String, projects: Vec<Project>) -> Self {
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
    project: Project,
    content: String,
    comments: Vec<Comment>,
}

impl GetProjectPage {
    pub fn new(
        user: Option<User>,
        project: Project,
        content: String,
        comments: Vec<Comment>,
    ) -> Self {
        Self {
            user,
            login_url: LoginUrl,
            logout_url: LogoutUrl,
            project,
            content,
            comments,
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
    project: Option<Project>,
}

impl<Url: Display> ProjectFormPage<Url> {
    pub fn new(
        action: Url,
        user: Option<User>,
        errors: ValidationErrors,
        project: Option<Project>,
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
    pub project: Project,
}

#[derive(Default, Template)]
#[template(path = "projects/form.html")]
pub struct ProjectForm {
    pub action: String,
    pub errors: ValidationErrors,
    pub project: Option<Project>,
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
