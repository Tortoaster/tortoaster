use askama::Template;
use validator::ValidationErrors;

use crate::{
    api::{comments::PostCommentUrl, users::LoginUrl},
    dto::{comments::CommentWithUser, users::User},
    template::filters,
};

#[derive(Debug, Template)]
#[template(path = "comments/component.html")]
pub struct CommentPartial {
    pub user: Option<User>,
    pub comment: CommentWithUser,
}

#[derive(Debug, Template)]
#[template(path = "comments/form/create_form_partial.html")]
pub struct CreateCommentFormPartial {
    user: Option<User>,
    login_url: LoginUrl,
    post_comment_url: PostCommentUrl,
    errors: ValidationErrors,
}

impl CreateCommentFormPartial {
    pub fn new(user: Option<User>, post_comment_url: PostCommentUrl) -> Self {
        Self {
            user,
            login_url: LoginUrl,
            post_comment_url,
            errors: ValidationErrors::new(),
        }
    }

    pub fn with_errors(mut self, errors: ValidationErrors) -> Self {
        self.errors = errors;
        self
    }
}
