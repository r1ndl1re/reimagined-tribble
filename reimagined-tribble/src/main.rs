use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing;
use tracing_subscriber;

#[derive(Debug, Serialize)]
struct User {
    id: u64,
    username: String,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    username: String,
}

async fn root() -> &'static str {
    "Hello, world!!"
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    let user = User {
        id: 1,
        username: payload.username,
    };
    (StatusCode::CREATED, Json(user))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}
