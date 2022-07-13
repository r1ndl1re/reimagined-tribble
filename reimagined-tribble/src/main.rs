use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};
use uuid::Uuid;

#[derive(Debug, Serialize)]
struct User {
    id: String,
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
    let user_id = Uuid::new_v4();
    let user = User {
        id: user_id.to_string(),
        username: payload.username,
    };
    tracing::info!("new_user_created: {}", &user.username);
    (StatusCode::CREATED, Json(user))
}

#[tokio::main]
async fn main() {
    LogTracer::init().expect("Failed to set logger");
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("sample".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).expect("Failed to set subscriber");

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
