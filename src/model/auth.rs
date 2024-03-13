use sqlx::types::time::OffsetDateTime;

#[derive(Debug)]
pub struct NewUser {
    pub username: String,
    pub email_address: String,
    pub password_hash: String,
}

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email_address: String,
    pub email_verified: bool,
    pub password_hash: String,
    pub account_created_date: OffsetDateTime,
    pub last_online_date: OffsetDateTime,
}
