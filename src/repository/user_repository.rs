// src/repositories/user_repository.rs
use async_trait::async_trait;
use crate::domain::user::{User};

pub struct CreateUserParams {

}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, params: CreateUserParams) -> Result<User, String>;
}