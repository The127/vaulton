use crate::Config;
use axum::routing::get;
use axum::Router;

mod auth;
pub mod discovery;
pub mod types;
pub mod error;

pub fn oidc_routes(config: Config) -> Router {
    Router::new()
        .route(
            "/.well-known/openid-configuration",
            get(crate::oidc::discovery::openid_configuration),
        )
        .with_state(config.oidc.clone())
        .route("/auth", get(auth::authorize))
        .with_state(config.oidc.clone())
}