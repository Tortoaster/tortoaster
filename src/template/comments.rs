use askama::Template;
use validator::ValidationErrors;

use crate::{
    api::{
        comments::{PostCommentUrl, PostPutCommentUrl},
        users::LoginUrl,
    },
    dto::{
        comments::{CommentMessage, CommentWithUser},
        users::User,
    },
    template::filters,
};

// Forms

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

#[derive(Debug, Template)]
#[template(path = "comments/form/update_form_partial.html")]
pub struct UpdateCommentFormPartial {
    post_put_comment_url: PostPutCommentUrl,
    comment: CommentMessage,
    errors: ValidationErrors,
}

impl UpdateCommentFormPartial {
    pub fn new(comment: CommentMessage, post_put_comment_url: PostPutCommentUrl) -> Self {
        Self {
            post_put_comment_url,
            comment,
            errors: ValidationErrors::new(),
        }
    }

    pub fn with_errors(mut self, errors: ValidationErrors) -> Self {
        self.errors = errors;
        self
    }
}

// Pages

#[derive(Debug, Template)]
#[template(path = "comments/component.html")]
pub struct CommentPartial {
    pub user: Option<User>,
    pub comment: CommentWithUser,
}
