use askama::Template;

use crate::dto::comments::Comment;

#[derive(Debug, Template)]
#[template(path = "comments/comment.html")]
pub struct CommentComponent {
    pub comment: Comment,
}

#[derive(Debug, Template)]
#[template(path = "comments/new_comment.html")]
pub struct NewCommentForm {
    pub project_id: u32,
}
