use std::env;

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Extension, Json, Router, Server};
use axum_extra::extract::cookie::Key;
use redis::{Client, Commands};
use thiserror::Error;
use tracing_subscriber::fmt;

use coloretto::Action;

use crate::database::Redis;
use crate::game::{Room, RoomId};
use crate::session::User;

mod database;
mod game;
mod session;

pub type Result<T> = std::result::Result<T, Error>;

async fn new(mut redis: Redis, user: User) -> Result<Json<RoomId>> {
    let room = Room::new(user);
    let room_id = room.id();
    redis.set(format!("game/{room_id}"), serde_json::to_string(&room)?)?;
    Ok(Json(room_id))
}

async fn join(mut redis: Redis, user: User, Path(room_id): Path<RoomId>) -> Result<Json<Room>> {
    let room_json: String = redis.get(format!("game/{room_id}"))?;
    let mut room: Room = serde_json::from_str(&room_json)?;
    room.enroll(user)?;
    redis.set(format!("game/{room_id}"), serde_json::to_string(&room)?)?;
    Ok(Json(room))
}

async fn leave(mut redis: Redis, user: User, Path(room_id): Path<RoomId>) -> Result<Json<Room>> {
    let room_json: String = redis.get(format!("game/{room_id}"))?;
    let mut room: Room = serde_json::from_str(&room_json)?;
    room.remove(&user)?;
    redis.set(format!("game/{room_id}"), serde_json::to_string(&room)?)?;
    Ok(Json(room))
}

async fn start(mut redis: Redis, user: User, Path(room_id): Path<RoomId>) -> Result<Json<Room>> {
    let room_json: String = redis.get(format!("game/{room_id}"))?;
    let mut room: Room = serde_json::from_str(&room_json)?;
    room.start(&user)?;
    redis.set(format!("game/{room_id}"), serde_json::to_string(&room)?)?;
    Ok(Json(room))
}

async fn status(mut redis: Redis, Path(room_id): Path<RoomId>) -> Result<Json<Room>> {
    let room_json: String = redis.get(format!("game/{room_id}"))?;
    let room: Room = serde_json::from_str(&room_json)?;
    Ok(Json(room))
}

async fn perform(
    mut redis: Redis,
    user: User,
    Path((room_id, action)): Path<(RoomId, Action)>,
) -> Result<Json<Room>> {
    let room_json: String = redis.get(format!("game/{room_id}"))?;
    let mut room: Room = serde_json::from_str(&room_json)?;
    room.perform(&user, action)?;
    redis.set(format!("game/{room_id}"), serde_json::to_string(&room)?)?;
    Ok(Json(room))
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

    let client = Client::open(redis).expect("cannot connect to redis server");

    let key = Key::generate();

    let app = Router::new()
        .route("/new", get(new))
        .route("/join/:room", get(join))
        .route("/leave/:room", get(leave))
        .route("/start/:room", get(start))
        .route("/status/:room", get(status))
        .route("/perform/:room/:action", get(perform))
        .layer(Extension(client))
        .layer(Extension(key));

    tracing::debug!("listening on http://{}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("failed to interact with database")]
    Redis(#[from] redis::RedisError),
    #[error("failed to (de)serialize")]
    Json(#[from] serde_json::Error),
    #[error("game produced an error")]
    Game(#[from] game::GameError),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let mut response = ().into_response();

        *response.status_mut() = match self {
            Error::Redis(_) => StatusCode::FAILED_DEPENDENCY,
            Error::Json(_) => StatusCode::BAD_REQUEST,
            Error::Game(_) => StatusCode::FORBIDDEN,
        };

        response
    }
}
