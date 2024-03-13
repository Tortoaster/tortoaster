use askama::Template;

use crate::model::Comment;

#[derive(Template)]
#[template(path = "comments/comment.html")]
pub struct CommentPartial {
    pub comment: Comment,
}

#[derive(Template)]
#[template(path = "comments/new_comment.html")]
pub struct NewCommentPartial {
    pub project_id: u32,
}
