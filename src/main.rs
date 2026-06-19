use axum::{extract::Query, routing::get, Json, Router};
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

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/data", get(data))
        .route("/greet", get(greet));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
