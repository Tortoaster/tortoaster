use axum::response::Redirect;
use axum_extra::{extract::WithRejection, routing::TypedPath};
use axum_oidc::OidcRpInitiatedLogout;

use crate::{api::projects::ProjectsUrl, config::AppConfig, error::WithAppRejection};

#[derive(Copy, Clone, Debug, Default, TypedPath)]
#[typed_path("/login")]
pub struct LoginUrl;

pub async fn login(_: LoginUrl) -> Redirect {
    Redirect::temporary(&ProjectsUrl.to_string())
}

#[derive(Copy, Clone, Debug, Default, TypedPath)]
#[typed_path("/logout")]
pub struct LogoutUrl;

pub async fn logout(
    _: LogoutUrl,
    WithRejection(logout, _): WithAppRejection<OidcRpInitiatedLogout>,
) -> OidcRpInitiatedLogout {
    logout.with_post_logout_redirect(AppConfig::get().oidc.redirect_url.parse().unwrap())
}
