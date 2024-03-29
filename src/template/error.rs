use askama::Template;
use axum::http::StatusCode;

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorPage {
    pub status_code: StatusCode,
    pub message: String,
}

#[derive(Template)]
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
