use crate::Config;
use axum::routing::get;
use axum::Router;
use crate::server::AppState;

mod auth;
pub mod discovery;
pub mod types;
pub mod error;

pub fn oidc_routes(app_state: AppState) -> Router {
    Router::new()
        .route(
            "/.well-known/openid-configuration",
            get(crate::oidc::discovery::openid_configuration),
        ).with_state(app_state.clone())
        .route("/authorize", get(auth::authorize))
}