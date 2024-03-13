use sqlx::{query, query_as, PgPool};

use crate::model::auth::{NewUser, User};

#[derive(Debug)]
pub struct AuthRepository(PgPool);

impl AuthRepository {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }

    pub async fn add_user(&self, user: &NewUser) -> sqlx::Result<()> {
        query!(
            "INSERT INTO users (username, email_address, password_hash) VALUES ($1, $2, $3);",
            &user.username,
            &user.email_address,
            &user.password_hash,
        )
        .execute(&self.0)
        .await?;

        Ok(())
    }

    pub async fn get_user_by_username(&self, username: &str) -> sqlx::Result<User> {
        query_as!(User, "SELECT * FROM users WHERE username = $1;", username)
            .fetch_one(&self.0)
            .await
    }
}
