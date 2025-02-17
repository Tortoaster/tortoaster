use axum::{extract::State, Form, Json, Router};
use axum_extra::{
    extract::WithRejection,
    routing::{RouterExt, TypedPath},
};
use axum_valid::Valid;
use serde::Deserialize;

use crate::{
    dto::{
        comments::{CommentWithUser, NewComment},
        users::User,
    },
    error::{AppError, AppResult, WithAppRejection},
    repository::comments::CommentRepository,
    state::AppState,
};

pub fn public_router() -> Router<AppState> {
    Router::new().typed_get(list_comments)
}

pub fn protected_router() -> Router<AppState> {
    Router::new()
        .typed_post(post_comment)
        .typed_put(put_comment)
        .typed_delete(delete_comment)
}

#[derive(Clone, Debug, Deserialize, TypedPath)]
#[typed_path("/projects/:project_id/comments")]
pub struct CommentsUrl {
    pub project_id: String,
}

#[derive(Clone, Debug, Deserialize, TypedPath)]
#[typed_path("/comments/:comment_id")]
pub struct CommentUrl {
    pub comment_id: i32,
}

async fn list_comments(
    CommentsUrl { project_id }: CommentsUrl,
    State(comment_repo): State<CommentRepository>,
) -> AppResult<Json<Vec<CommentWithUser>>> {
    let comments = comment_repo.list(&project_id).await?;
    Ok(Json(comments))
}

async fn post_comment(
    url: CommentsUrl,
    State(repo): State<CommentRepository>,
    user: User,
    WithRejection(Valid(new_comment), _): WithAppRejection<Valid<Form<NewComment>>>,
) -> AppResult<Json<CommentWithUser>> {
    let comment = repo.create(user.id, &url.project_id, &new_comment).await?;
    Ok(Json(comment))
}

async fn put_comment(
    CommentUrl { comment_id }: CommentUrl,
    State(repo): State<CommentRepository>,
    WithRejection(Valid(new_comment), _): WithAppRejection<Valid<Form<NewComment>>>,
) -> AppResult<Json<CommentWithUser>> {
    let comment = repo.update(comment_id, &new_comment).await?;
    Ok(Json(comment))
}

async fn delete_comment(
    CommentUrl { comment_id }: CommentUrl,
    State(repo): State<CommentRepository>,
    user: User,
) -> AppResult<()> {
    if user.is_admin || user.id == repo.read_user_id(comment_id).await? {
        repo.delete(comment_id).await?;
        Ok(())
    } else {
        Err(AppError::Unauthorized)
    }
}
