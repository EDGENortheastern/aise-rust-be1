use axum::{
    extract::Query, http::StatusCode, response::IntoResponse, routing::get, Json, Router,
};
use serde_json::{json, Value};
use std::collections::HashMap;

// Returns some data as JSON.
async fn home() -> Json<Value> {
    Json(json!({
        "name": "rust-backend-intro",
        "message": "Hello from Rust! 🦀",
        "version": "0.1",}))
}

// Returns some data as JSON.
async fn data() -> Json<Value> {
    Json(json!({ "text": "Hello from Rust", "count": 3 }))
}

// Reads "name" from the query string, e.g. /greet?name=Katia
async fn greet(Query(params): Query<HashMap<String, String>>) -> Json<Value> {
    // Use the name if given, otherwise "stranger".
    let name = params.get("name").map(|s| s.as_str()).unwrap_or("stranger");
    Json(json!({ "hello": name }))
}

// Reads countries from data/world_cup.json and serves it as JSON.
async fn world_cup() -> impl IntoResponse {
    match tokio::fs::read_to_string("data/world_cup.json").await {
        // Parse the file so we return real JSON (not a string) and catch bad JSON early.
        Ok(contents) => match serde_json::from_str::<Value>(&contents) {
            Ok(value) => (StatusCode::OK, Json(value)),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": format!("invalid JSON in file: {e}") })),
            ),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": format!("could not read file: {e}") })),
        ),
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/data", get(data))
        .route("/greet", get(greet))
        .route("/world-cup", get(world_cup));

    // Render (and most hosts) provide the port via the PORT env var and require
    // binding to 0.0.0.0 so the service is reachable from outside the container.
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
