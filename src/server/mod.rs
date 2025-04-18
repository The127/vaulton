mod health;

use std::sync::Arc;
use crate::{oidc, Config};

use axum::{Router, routing::get};
use crate::repository::OAuthModule;

#[derive(Clone)]
pub struct AppState {
    pub module: Arc<OAuthModule>,
    pub config: Config,
}

pub async fn create_server(config: Config) -> Router {
    // Create the DI module
    let module = Arc::new(
        OAuthModule::builder()
            .build()
    );

    // Create the app state
    let state = AppState {
        module,
        config,
    };

    // Create a new router with a single route
    Router::new()
        .route("/health", get(health::health_check))
        .merge(oidc::oidc_routes(state))
}