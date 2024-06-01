use sqlx::{query_as, PgPool};

use crate::{
    dto::users::{User, UserId},
    error::AppResult,
};

#[derive(Clone, Debug)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub async fn get(&self, id: &UserId) -> AppResult<User> {
        struct PartialUser {
            id: UserId,
            name: Option<String>,
        }

        let PartialUser { id, name } = query_as!(
            PartialUser,
            "SELECT id, username AS name FROM keycloak.user_entity WHERE id = $1;",
            id,
        )
        .fetch_one(&self.pool)
        .await?;

        let user = User {
            id,
            name,
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
