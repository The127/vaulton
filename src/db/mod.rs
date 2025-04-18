use sqlx::postgres::{PgPool, PgPoolOptions};
use shaku::{Component, Interface};
use std::time::Duration;

pub trait Database: Interface{
    fn get_pool(&self) -> &PgPool;
}

#[derive(Component)]
#[shaku(interface = Database)]
pub struct DatabaseImpl {
    pool: PgPool,
}

impl DatabaseImpl {
    pub async fn new(config: &crate::config::PostgresConfig) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .acquire_timeout(Duration::from_secs(3))
            .connect(&config.connection_string()).await?;
            
        Ok(Self { pool })
    }
}

impl Database for DatabaseImpl {
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}