use axum::{extract::State, Form};
use axum_extra::{extract::WithRejection, routing::TypedPath};
use serde::Deserialize;
use validator::Validate;

use crate::{
    dto::{comments::NewComment, users::User},
    error::{AppError, ToastResult, WithToastRejection},
    repository::comments::CommentRepository,
    template::{
        comments::{CommentPartial, CreateCommentFormPartial},
        Render, RenderBoth,
    },
};

#[derive(Clone, Debug, Deserialize, TypedPath)]
#[typed_path("/projects/:project_id/comments")]
pub struct PostCommentUrl {
    pub project_id: String,
}

#[derive(Clone, Debug, Deserialize, TypedPath)]
#[typed_path("/comments/:comment_id/delete")]
pub struct PostDeleteCommentUrl {
    pub comment_id: i32,
}

pub async fn post_comment(
    url: PostCommentUrl,
    State(repo): State<CommentRepository>,
    user: User,
    WithRejection(new_comment, _): WithToastRejection<Form<NewComment>>,
) -> ToastResult<
    Result<RenderBoth<CreateCommentFormPartial, CommentPartial>, Render<CreateCommentFormPartial>>,
> {
    if let Err(errors) = new_comment.validate() {
        return Ok(Err(Render(
            CreateCommentFormPartial::new(Some(user), url).with_errors(errors),
        )));
    }

    let comment = repo.create(&user.id, &url.project_id, &new_comment).await?;

    Ok(Ok(RenderBoth(
        CreateCommentFormPartial::new(Some(user.clone()), url),
        CommentPartial {
            user: Some(user),
            comment,
        },
    )))
}

pub async fn post_delete_comment(
    PostDeleteCommentUrl { comment_id }: PostDeleteCommentUrl,
    State(repo): State<CommentRepository>,
    user: User,
) -> ToastResult<()> {
    if user.is_admin || user.id == repo.read_user_id(comment_id).await? {
        repo.delete(comment_id).await?;
        Ok(())
    } else {
        Err(AppError::Unauthorized.into())
    }
}
