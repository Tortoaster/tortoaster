use askama::Template;
use validator::ValidationErrors;

use crate::{
    api::projects::GetProjectFormUrl,
    dto::{comments::Comment, projects::Project},
};

// Pages

#[derive(Template)]
#[template(path = "projects/page_list.html")]
pub struct ListProjectsPage {
    new_project_url: GetProjectFormUrl,
    pub projects: Vec<Project>,
}

impl ListProjectsPage {
    pub fn new(projects: Vec<Project>) -> Self {
        ListProjectsPage {
            new_project_url: GetProjectFormUrl,
            projects,
        }
    }
}

#[derive(Template)]
#[template(path = "projects/page_get.html")]
pub struct GetProjectPage {
    pub project: Project,
    pub comments: Vec<Comment>,
}

#[derive(Default, Template)]
#[template(path = "projects/page_form.html")]
pub struct ProjectFormPage {
    pub errors: ValidationErrors,
    pub project: Option<Project>,
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
