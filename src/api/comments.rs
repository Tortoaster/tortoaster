use axum::{extract::State, Form, Router};
use axum_extra::{
    extract::WithRejection,
    routing::{RouterExt, TypedPath},
};
use serde::Deserialize;
use validator::Validate;

use crate::{
    dto::{comments::NewComment, users::User},
    error::{AppError, ToastResult, WithToastRejection},
    repository::comments::CommentRepository,
    state::AppState,
    template::{
        comments::{CommentPartial, CreateCommentFormPartial, UpdateCommentFormPartial},
        Render, RenderBoth,
    },
};

pub fn protected_router() -> Router<AppState> {
    Router::new()
        .typed_get(get_comment_put_form_partial)
        .typed_post(post_comment)
        .typed_post(post_put_comment)
        .typed_post(post_delete_comment)
}

// Forms

#[derive(Clone, Debug, Deserialize, TypedPath)]
#[typed_path("/comments/:comment_id/update-form")]
pub struct GetCommentPutFormPartialUrl {
    pub comment_id: i32,
}

async fn get_comment_put_form_partial(
    GetCommentPutFormPartialUrl { comment_id }: GetCommentPutFormPartialUrl,
    State(repo): State<CommentRepository>,
) -> ToastResult<Render<UpdateCommentFormPartial>> {
    let comment = repo.read_message(comment_id).await?;
    Ok(Render(UpdateCommentFormPartial::new(
        comment,
        PostPutCommentUrl { comment_id },
    )))
}

// API Pages

#[derive(Clone, Debug, Deserialize, TypedPath)]
#[typed_path("/projects/:project_id/comments")]
pub struct PostCommentUrl {
    pub project_id: String,
}

#[derive(Clone, Debug, Deserialize, TypedPath)]
#[typed_path("/comments/:comment_id/put")]
pub struct PostPutCommentUrl {
    pub comment_id: i32,
}

#[derive(Clone, Debug, Deserialize, TypedPath)]
#[typed_path("/comments/:comment_id/delete")]
pub struct PostDeleteCommentUrl {
    pub comment_id: i32,
}

async fn post_comment(
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

async fn post_put_comment(
    PostPutCommentUrl { comment_id }: PostPutCommentUrl,
    State(repo): State<CommentRepository>,
    user: User,
    WithRejection(new_comment, _): WithToastRejection<Form<NewComment>>,
) -> ToastResult<Result<Render<CommentPartial>, Render<UpdateCommentFormPartial>>> {
    if let Err(errors) = new_comment.validate() {
        return Ok(Err(Render(
            UpdateCommentFormPartial::new(new_comment.0.into(), PostPutCommentUrl { comment_id })
                .with_errors(errors),
        )));
    }

    let comment = repo.update(comment_id, &new_comment).await?;

    Ok(Ok(Render(CommentPartial {
        user: Some(user),
        comment,
    })))
}

async fn post_delete_comment(
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
