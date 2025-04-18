use std::collections::HashMap;
use std::sync::Arc;
use async_trait::async_trait;
use shaku::{Component, Interface};
use tokio::sync::RwLock;
use crate::oidc::auth::AuthorizationRequest;

#[async_trait]
pub trait AuthRequestRepository: Interface {
    async fn store_request(&self, request: &AuthorizationRequest) -> Result<(), String>;
    async fn find_by_id(&self, request_id: &str) -> Option<AuthorizationRequest>;
}

#[derive(Component)]
#[shaku(interface = AuthRequestRepository)]
pub struct InMemoryAuthRequestRepository {
    #[shaku(default)]
    requests: Arc<RwLock<HashMap<String, AuthorizationRequest>>>,
}

impl InMemoryAuthRequestRepository {
    fn new() -> Self {
        Self {
            requests: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl AuthRequestRepository for InMemoryAuthRequestRepository {
    async fn store_request(&self, request: &AuthorizationRequest) -> Result<(), String> {
        let mut requests = self.requests.write().await;
        requests.insert(request.request_id.clone(), request.clone());
        Ok(())
    }

    async fn find_by_id(&self, request_id: &str) -> Option<AuthorizationRequest> {
        let requests = self.requests.read().await;
        requests.get(request_id).cloned()
    }
}

// Define the module
use shaku::module;

module! {
    pub OAuthModule {
        components = [InMemoryAuthRequestRepository],
        providers = []
    }
}