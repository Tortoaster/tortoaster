use sqlx::{query_as, PgPool};
use uuid::Uuid;

use crate::{dto::users::User, error::AppResult};

#[derive(Clone, Debug)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub async fn get(&self, id: Uuid) -> AppResult<User> {
        struct Username {
            username: Option<String>,
        }

        let Username { username } = query_as!(
            Username,
            "SELECT username FROM keycloak.user_entity WHERE id = $1;",
            id.to_string()
        )
        .fetch_one(&self.pool)
        .await?;

        let user = User {
            id,
            name: username.unwrap_or_else(|| "[unknown user]".to_owned()),
            // TODO: Retrieve admin status
            is_admin: false,
        };

        Ok(user)
    }
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
