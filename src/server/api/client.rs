use std::collections::HashSet;
use crate::server::AppState;
use axum::extract::{Json, State};
use axum::response::IntoResponse;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateClientRequestDto {
    pub redirect_uris: Vec<String>,
    pub scopes: Vec<String>,
}

pub async fn create_client(
    State(state): State<AppState>,
    Json(dto): Json<CreateClientRequestDto>,
) -> impl IntoResponse {
    unimplemented!()
}
