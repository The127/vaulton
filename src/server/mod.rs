mod health;

use std::sync::Arc;
use crate::{oidc, Config};

use axum::{Router, routing::get};
use crate::db::DatabaseImplParameters;
use crate::di::MyModule;

#[derive(Clone)]
pub struct AppState {
    pub module: Arc<MyModule>,
    pub config: Config,
}

pub async fn create_server(config: Config) -> Router {
    // connect to the database
    let pool = crate::db::connect_to_db(&config.postgres).await.unwrap();

    // Create the DI module
    let module = Arc::new(
        MyModule::builder()
            .with_component_parameters::<crate::db::DatabaseImpl>(DatabaseImplParameters{
                pool,
            })
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