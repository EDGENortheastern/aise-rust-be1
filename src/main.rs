use axum::{routing::get, Router};

// Handler for the "/" route. Returns a simple welcome message.
async fn home() -> &'static str {
    "Here is your Rust backend"
}

// Handler for the "/health" route. Used to check if the server is up.
async fn health() -> &'static str {
    "OK"
}

// Entry point. The macro lets main run as an async function on the Tokio runtime.
#[tokio::main]
async fn main() {
    // Build the app and connect each URL path to its handler function.
    let app = Router::new()
        .route("/", get(home))
        .route("/health", get(health));

    // Listen for incoming connections on localhost, port 3000.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    // Start the server and keep it running.
    axum::serve(listener, app).await.unwrap();
}
