use askama::Template;
use axum::http::StatusCode;

use crate::{
    api::auth::{LoginUrl, LogoutUrl},
    user::User,
};

#[derive(Debug, Template)]
#[template(path = "error.html")]
pub struct ErrorPage {
    user: Option<User>,
    login_url: LoginUrl,
    logout_url: LogoutUrl,
    status_code: StatusCode,
    message: String,
}

impl ErrorPage {
    pub fn new(user: Option<User>, status_code: StatusCode, message: String) -> Self {
        Self {
            user,
            login_url: LoginUrl,
            logout_url: LogoutUrl,
            status_code,
            message,
        }
    }
}

#[derive(Debug, Template)]
#[template(path = "error_toast.html")]
pub struct ErrorToast {
    pub message: String,
}

mod filters {
    use askama::Result;
    use axum::http::StatusCode;

    pub fn with_reason(status_code: &StatusCode) -> Result<String> {
        Ok(format!(
            "{} {}",
            status_code.as_u16(),
            status_code
                .canonical_reason()
                .unwrap_or("No idea what went wrong")
        ))
    }
}
