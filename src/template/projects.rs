use std::fmt::Display;

use askama::Template;
use validator::ValidationErrors;

use crate::{
    api::{
        auth::{LoginUrl, LogoutUrl},
        projects::ProjectsFormUrl,
    },
    dto::projects::{ProjectNameContentUrl, ProjectPreview, ProjectWithComments},
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

#[derive(Template)]
#[template(path = "projects/page_form.html")]
pub struct ProjectFormPage<Url: Display> {
    title: &'static str,
    action: Url,
    user: Option<User>,
    login_url: LoginUrl,
    logout_url: LogoutUrl,
    errors: ValidationErrors,
    project: Option<ProjectNameContentUrl>,
}

impl<Url: Display> ProjectFormPage<Url> {
    pub fn new(
        title: &'static str,
        action: Url,
        user: Option<User>,
        errors: ValidationErrors,
        project: Option<ProjectNameContentUrl>,
    ) -> Self {
        Self {
            title,
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
