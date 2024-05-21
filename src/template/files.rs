use askama::Template;

use crate::dto::projects::ProjectThumbnailId;

#[derive(Debug, Template)]
#[template(path = "files/image_with_id.html")]
pub struct ImageWithId {
    pub project: ProjectThumbnailId,
}
