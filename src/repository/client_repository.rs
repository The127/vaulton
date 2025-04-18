use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use shaku::{Component, Interface};
use tokio::sync::RwLock;
use crate::domain::client::{Client, ClientId};

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
}

impl PostgresClientRepository {
    fn new() -> Self {
        Self {
        }
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