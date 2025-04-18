use axum::{Router, routing::get};

pub async fn create_server() -> Router {
    // Create a new router with a single route
    Router::new()
        .route("/health", get(health_check))
}

// A simple health check handler that returns "OK"
async fn health_check() -> &'static str {
    "OK"
}