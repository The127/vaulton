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

pub async fn connect_to_db(config: &crate::config::PostgresConfig) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config.connection_string()).await?;

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect(
            "Failed to run migrations",
        );

    Ok(pool)
}

impl Database for DatabaseImpl {
    fn get_pool(&self) -> &PgPool {
        &self.pool
    }
}