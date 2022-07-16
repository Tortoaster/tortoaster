use std::ops::{Deref, DerefMut};

use axum::extract::{FromRequest, RequestParts};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use redis::{Client, Connection};
use thiserror::Error;

pub struct Redis {
    conn: Connection,
}

#[async_trait::async_trait]
impl<B> FromRequest<B> for Redis
where
    B: Send,
{
    type Rejection = RedisError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(client) = Extension::<Client>::from_request(req)
            .await
            .expect("`redis::Client` extension missing");
        let redis = Redis {
            conn: client.get_connection()?,
        };
        Ok(redis)
    }
}

impl Deref for Redis {
    type Target = Connection;

    fn deref(&self) -> &Self::Target {
        &self.conn
    }
}

impl DerefMut for Redis {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.conn
    }
}

#[derive(Debug, Error)]
#[error("failed to interact with database")]
pub struct RedisError(#[from] redis::RedisError);

impl IntoResponse for RedisError {
    fn into_response(self) -> Response {
        let mut res = ().into_response();
        *res.status_mut() = StatusCode::FAILED_DEPENDENCY;
        res
    }
}
