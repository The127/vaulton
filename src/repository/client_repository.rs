use std::sync::Arc;
use async_trait::async_trait;
use shaku::{Component, Interface};
use crate::db::Database;
use crate::domain::client::{Client};

pub struct CreateClientParams {
    pub redirect_uris: Vec<String>,
    pub scopes: Vec<String>,
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
        unimplemented!()
    }

    async fn find_by_id(&self, id: &str) -> Option<Client> {
        unimplemented!()
    }
}