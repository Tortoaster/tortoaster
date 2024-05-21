use axum::{
    extract::{DefaultBodyLimit, Multipart, State},
    Router,
};
use axum_extra::{
    extract::WithRejection,
    routing::{RouterExt, TypedPath},
};
use uuid::Uuid;

use crate::{
    config::AppBucket,
    dto::projects::ProjectThumbnailId,
    error::{AppError, ToastResult, WithToastRejection},
    repository::files::FileRepository,
    state::AppState,
    template::{files::ImageWithId, Render},
};

pub fn public_router() -> Router<AppState> {
    Router::new()
        .typed_post(upload_image)
        .layer(DefaultBodyLimit::max(1024 * 1024 * 10))
}

#[derive(Copy, Clone, Default, TypedPath)]
#[typed_path("/uploads")]
pub struct PostImageUrl;

async fn upload_image(
    _: PostImageUrl,
    State(file_repo): State<FileRepository>,
    WithRejection(mut parts, _): WithToastRejection<Multipart>,
) -> ToastResult<Render<ImageWithId>> {
    let field = parts.next_field().await?.ok_or(AppError::FileMissing)?;
    let content_type = field
        .content_type()
        .and_then(|content_type| content_type.parse().ok())
        .ok_or(AppError::FileType)?;
    let bytes = field.bytes().await?;

    if !bytes.is_empty() {
        let thumbnail_id = Uuid::new_v4();
        file_repo
            .store_image(thumbnail_id, AppBucket::Thumbnails, bytes, content_type)
            .await?;
        Ok(Render(ImageWithId {
            project: ProjectThumbnailId { thumbnail_id },
        }))
    } else {
        Err(AppError::FileMissing.into())
    }
}
