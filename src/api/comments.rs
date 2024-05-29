use axum::{extract::State, Form};
use axum_extra::{extract::WithRejection, routing::TypedPath};
use serde::Deserialize;
use validator::Validate;

use crate::{
    dto::{comments::NewComment, users::User},
    error::{ToastResult, WithToastRejection},
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

    let comment = repo.create(user.id, &url.project_id, &new_comment).await?;

    Ok(Ok(RenderBoth(
        CreateCommentFormPartial::new(Some(user), url),
        CommentPartial { comment },
    )))
}
