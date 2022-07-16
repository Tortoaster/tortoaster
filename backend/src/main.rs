use axum::routing::get;
use axum::{Router, Server};
use tracing_subscriber::fmt;

async fn index() -> &'static str {
    "Hello, world!"
}

#[tokio::main]
async fn main() {
    fmt::init();

    let host = std::env::var("HOST").unwrap_or("0.0.0.0".to_owned());
    let port = std::env::var("PORT").unwrap_or("8000".to_owned());
    let addr = format!("{host}:{port}")
        .parse()
        .expect(&format!("invalid address: {host}:{port}"));

    let app = Router::new().route("/", get(index));

    tracing::debug!("listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
