use askama::Template;

use crate::dto::{comments::Comment, projects::Project};

#[derive(Template)]
#[template(path = "projects/card.html")]
pub struct ProjectComponent {
    pub project: Project,
}

#[derive(Template)]
#[template(path = "projects/list.html")]
pub struct ProjectListPage {
    pub projects: Vec<Project>,
}

#[derive(Template)]
#[template(path = "projects/get.html")]
pub struct ProjectPage {
    pub project: Project,
    pub comments: Vec<Comment>,
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
