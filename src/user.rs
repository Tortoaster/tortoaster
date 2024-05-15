use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
use axum_oidc::{error::ExtractorError, OidcClaims};
use openidconnect::AdditionalClaims;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug)]
pub struct User {
    pub name: String,
    pub is_admin: bool,
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for User {
    type Rejection = UserRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let claims = OidcClaims::<AppClaims>::from_request_parts(parts, state).await?;

        let user = User {
            name: claims
                .preferred_username()
                .ok_or(UserRejection::NoName)?
                .to_string(),
            is_admin: claims.additional_claims().is_admin(),
        };

        Ok(user)
    }
}

#[derive(Debug, Error)]
pub enum UserRejection {
    #[error("{0}")]
    Inner(#[from] ExtractorError),
    #[error("user has no name")]
    NoName,
}

impl IntoResponse for UserRejection {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, self.to_string()).into_response()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppClaims {
    resource_access: ResourceAccess,
}

impl AppClaims {
    fn is_admin(&self) -> bool {
        self.resource_access.tortoaster.roles.contains(&Role::Admin)
    }
}

impl AdditionalClaims for AppClaims {}

impl axum_oidc::AdditionalClaims for AppClaims {}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct ResourceAccess {
    tortoaster: Tortoaster,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Tortoaster {
    roles: Vec<Role>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Role {
    Admin,
    #[serde(untagged)]
    Other(String),
}
