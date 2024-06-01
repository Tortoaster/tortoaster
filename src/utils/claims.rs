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

use crate::dto::users::User;

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for User {
    type Rejection = UserRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let claims = OidcClaims::<AppClaims>::from_request_parts(parts, state).await?;

        let user = User {
            id: claims.subject().to_string(),
            name: claims
                .preferred_username()
                .map(|username| username.to_string()),
            is_admin: claims.additional_claims().is_admin(),
        };

        Ok(user)
    }
}

#[derive(Debug, Error)]
pub enum UserRejection {
    #[error("{0}")]
    Inner(#[from] ExtractorError),
}

impl IntoResponse for UserRejection {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, self.to_string()).into_response()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppClaims {
    #[serde(default)]
    resource_access: ResourceAccess,
}

impl AppClaims {
    fn is_admin(&self) -> bool {
        self.resource_access.tortoaster.roles.contains(&Role::Admin)
    }
}

impl AdditionalClaims for AppClaims {}

impl axum_oidc::AdditionalClaims for AppClaims {}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct ResourceAccess {
    tortoaster: Tortoaster,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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
