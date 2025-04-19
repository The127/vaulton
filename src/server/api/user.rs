use std::sync::Arc;
use crate::repository::user_repository::{CreateUserParams, UserRepository};
use crate::server::AppState;
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use axum::extract::{Json, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use shaku::HasComponent;

#[derive(Debug, Deserialize)]
pub struct CreateUserRequestDto {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
struct UserResponseDto {
    id: String,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(dto): Json<CreateUserRequestDto>,
) -> impl IntoResponse {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = match argon2.hash_password(dto.password.as_bytes(), &salt) {
        Ok(hash) => hash,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to hash password").into_response();
        }
    };

    let create_params = CreateUserParams {
        username: dto.username,
        email: dto.email,
        password_hash: password_hash.to_string(),
    };

    let user_repository: Arc<dyn UserRepository> = state.module.resolve();

    match user_repository.create(create_params).await {
        Ok(user) => {
            let response = UserResponseDto {
                id: user.uuid.to_string(),
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}
