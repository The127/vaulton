mod health;

use crate::{oidc, Config};

use axum::{Router, routing::get};

pub async fn create_server(config: Config) -> Router {
    // Create a new router with a single route
    Router::new()
        .route("/health", get(health::health_check))
        .merge(oidc::oidc_routes(config))

}