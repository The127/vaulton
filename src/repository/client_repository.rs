use crate::db::Database;
use crate::domain::client::{Client, ClientId};
use async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;

pub struct CreateClientParams {
    pub client_id: String,
    pub redirect_uris: Vec<String>,
    pub scopes: Vec<String>,
    pub client_secret_hash: Option<Vec<u8>>,
}

#[async_trait]
pub trait ClientRepository: Interface {
    async fn create(&self, client: CreateClientParams) -> Result<Client, String>;
    async fn find_by_id(&self, id: &str) -> Option<Client>;
}

#[derive(Component)]
#[shaku(interface = ClientRepository)]
pub struct PostgresClientRepository {
    #[shaku(inject)]
    pool: Arc<dyn Database>,
}

impl PostgresClientRepository {
    fn new(pool: Arc<dyn Database>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ClientRepository for PostgresClientRepository {
    async fn create(&self, params: CreateClientParams) -> Result<Client, String> {
        let result = sqlx::query!(
            r#"
            insert into clients(client_id, client_secret_hash, redirect_uris, scopes)
            values ($1, $2, $3, $4)
            returning *;
            "#,
            params.client_id,
            params.client_secret_hash,
            params.redirect_uris.as_slice(),
            params.scopes.as_slice(),
        )
        .fetch_one(self.pool.get_pool())
        .await
        .map_err(|e| e.to_string())?;

        Ok(Client {
            uuid: result.id,
            id: ClientId(result.client_id),
            secret_hash: result.client_secret_hash,
            redirect_uris: result.redirect_uris.unwrap(),
            allowed_scopes: result.scopes.unwrap(),
            created_at: result.created_at,
            updated_at: result.updated_at,
        })
    }

    async fn find_by_id(&self, id: &str) -> Option<Client> {
        unimplemented!()
    }
}
