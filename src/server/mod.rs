mod health;

use axum::{Router, routing::get};

pub async fn create_server() -> Router {
    // Create a new router with a single route
    Router::new()
        .route("/health", get(health::health_check))
}