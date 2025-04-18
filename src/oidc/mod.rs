use crate::server::AppState;
use crate::Config;
use axum::routing::get;
use axum::Router;

mod auth;
pub mod discovery;
pub mod error;
pub mod types;

pub fn oidc_routes(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/.well-known/openid-configuration",
            get(crate::oidc::discovery::openid_configuration),
        )
        .with_state(app_state.clone())
        .route("/authorize", get(auth::authorize))
        .with_state(app_state.clone())
}
