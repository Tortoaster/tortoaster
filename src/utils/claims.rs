use std::ops::{Deref, DerefMut};

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
use axum_oidc::{error::ExtractorError, OidcClaims};
use openidconnect::AdditionalClaims;
use serde::{Deserialize, Serialize};
use sqlx::{query, PgPool};
use thiserror::Error;

use crate::dto::users::User;

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for User
where
    PgPool: FromRef<S>,
{
    type Rejection = UserRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let claims = OidcClaims::<AppClaims>::from_request_parts(parts, state).await?;
        let pool = PgPool::from_ref(state);

        let user = User {
            id: claims.subject().parse()?,
            name: claims
                .preferred_username()
                .map(|username| username.to_string()),
            is_admin: claims.additional_claims().is_admin(),
        };

        // Update user data in local table
        // TODO: Only do this once at login
        query!(
            "INSERT INTO users (id, name, is_admin) VALUES ($1, $2, $3) ON CONFLICT (id) DO \
             UPDATE SET name = $2, is_admin = $3;",
            user.id,
            user.name.as_deref().unwrap_or("<anonymous>"),
            user.is_admin,
        )
        .execute(&pool)
        .await?;

        Ok(user)
    }
}

#[derive(Debug)]
pub struct Admin(User);

impl Admin {
    pub fn into_user(self) -> User {
        self.0
    }
}

impl Deref for Admin {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Admin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for Admin
where
    PgPool: FromRef<S>,
{
    type Rejection = UserRejection;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let user = User::from_request_parts(parts, state).await?;

        if user.is_admin {
            Ok(Admin(user))
        } else {
            Err(UserRejection::Permission)
        }
    }
}

#[derive(Debug, Error)]
pub enum UserRejection {
    #[error("{0}")]
    Inner(#[from] ExtractorError),
    #[error("identity provider returned invalid id")]
    Id(#[from] uuid::Error),
    #[error("user not found")]
    Database(#[from] sqlx::Error),
    #[error("insufficient permissions")]
    Permission,
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
