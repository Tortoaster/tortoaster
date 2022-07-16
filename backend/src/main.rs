use std::env;

use axum::routing::get;
use axum::{Extension, Router, Server};
use axum_extra::extract::cookie::Key;
use tracing_subscriber::fmt;

use redis::{Client, Commands};

use crate::database::{Redis, RedisError};
use crate::session::UserId;

mod database;
mod session;

const COUNT: &str = "count";

async fn index(mut redis: Redis, user_id: UserId) -> Result<String, RedisError> {
    let count: i32 = redis.get(COUNT)?;
    redis.set(COUNT, count + 1)?;
    Ok(format!("{:?}, {count}", user_id))
}

#[tokio::main]
async fn main() {
    fmt::init();

    let host = env::var("HOST").unwrap_or("0.0.0.0".to_owned());
    let port = env::var("PORT").unwrap_or("8000".to_owned());
    let redis = env::var("REDIS_SERVER").expect("no redis server specified");

    let addr = format!("{host}:{port}")
        .parse()
        .expect(&format!("invalid address: {host}:{port}"));

    let client = Client::open(redis).unwrap();
    let mut con = client
        .get_connection()
        .expect("cannot connect to redis server");
    let _: () = con.set(COUNT, 0).expect("cannot set value");

    let key = Key::generate();

    let app = Router::new()
        .route("/", get(index))
        .layer(Extension(client))
        .layer(Extension(key));

    tracing::debug!("listening on http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
