use sqlx::{query_as, PgPool};
use uuid::Uuid;

use crate::dto::users::User;

#[derive(Clone, Debug)]
pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub async fn get(&self, id: Uuid) -> sqlx::Result<User> {
        query_as!(
            User,
            "SELECT id, name, is_admin FROM users WHERE id = $1;",
            id,
        )
        .fetch_one(&self.pool)
        .await
    }
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
