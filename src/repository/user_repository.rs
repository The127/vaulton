use std::sync::Arc;
// src/repositories/user_repository.rs
use async_trait::async_trait;
use shaku::{Component, Interface};
use crate::db::Database;
use crate::domain::user::{User};

pub struct CreateUserParams {

}

#[async_trait]
pub trait UserRepository: Interface {
    async fn create(&self, params: CreateUserParams) -> Result<User, String>;
}

#[derive(Component)]
#[shaku(interface = UserRepository)]
pub struct PostgresUserRepository {
    #[shaku(inject)]
    pool: Arc<dyn Database>,
}

impl PostgresUserRepository {
    fn new(pool: Arc<dyn Database>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn create(&self, params: CreateUserParams) -> Result<User, String> {
        unimplemented!()
    }
}