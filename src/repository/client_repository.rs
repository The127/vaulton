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
pub struct InMemoryClientRepository {
    #[shaku(default)]
    clients: Arc<RwLock<HashMap<String, Client>>>,
}

impl InMemoryClientRepository {
    fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl ClientRepository for InMemoryClientRepository {
    async fn create(&self, params: CreateClientParams) -> Result<Client, String> {
        unimplemented!()
    }

    async fn find_by_id(&self, id: &str) -> Option<Client> {
        let clients = self.clients.read().await;
        clients.get(id).cloned()
    }
}