use std::sync::Arc;
// src/repositories/user_repository.rs
use crate::db::Database;
use crate::domain::user::User;
use async_trait::async_trait;
use shaku::{Component, Interface};

pub struct CreateUserParams {
    pub username: String,
    pub password_hash: Vec<u8>,
    pub email: String,
}

#[async_trait]
pub trait UserRepository: Interface {
    async fn create(&self, params: CreateUserParams) -> Result<User, String>;
    async fn find_by_username_or_email(&self, username_or_email: &str) -> Option<User>;
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
        let result = sqlx::query!(
            r#"
            insert into users(username, password_hash, email)
            values($1, $2, $3)
            returning *;
            "#,
            params.username,
            params.password_hash,
            params.email,
        )
        .fetch_one(self.pool.get_pool())
        .await
        .map_err(|e| e.to_string())?;

        Ok(User{
            uuid: result.id,
            username: result.username,
            password_hash: result.password_hash,
            email: result.email,
            created_at: result.created_at,
            updated_at: result.updated_at,
        })
    }

    async fn find_by_username_or_email(&self, username_or_email: &str) -> Option<User> {
        let result = sqlx::query!(
            r#"
            select * from users where username = $1 or email = $1;
            "#,
            username_or_email,
        )
            .fetch_optional(self.pool.get_pool())
            .await
            .ok()??;


        Some(User{
            uuid: result.id,
            username: result.username,
            password_hash: result.password_hash,
            email: result.email,
            created_at: result.created_at,
            updated_at: result.updated_at,
        })
    }
}
