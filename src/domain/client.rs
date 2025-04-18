use std::collections::HashSet;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug)]
pub struct ClientId(pub String);

#[derive(Clone, Debug)]
pub struct Client {
    pub uuid: Uuid,
    pub id: ClientId,
    pub secret_hash: Option<Vec<u8>>,
    pub redirect_uris: Vec<String>,
    pub allowed_scopes: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Client {
    pub fn validate_redirect_uri(&self, uri: &str) -> bool {
        self.redirect_uris.contains(&uri.to_string())
    }
    
    pub fn validate_scopes(&self, scopes: &Vec<&str>) -> bool {
        scopes.iter().all(|s| self.allowed_scopes.contains(&s.to_string()))
    }
}