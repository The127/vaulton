use axum::Router;
use axum::routing::post;
use crate::server::AppState;

pub mod user;

pub fn api_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/api/users", post(user::create_user))
        .with_state(app_state.clone())
}