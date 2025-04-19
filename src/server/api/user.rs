use axum::extract::{State, Json};
use axum::response::IntoResponse;
use serde::Deserialize;
use crate::server::AppState;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequestDto {
    pub username: String,
    pub email: String,
    pub password: String,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(dto): Json<CreateUserRequestDto>,
) -> impl IntoResponse {
    dto.email
}