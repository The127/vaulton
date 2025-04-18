// src/domain/client.rs
use std::collections::HashSet;

#[derive(Clone, Debug)]
pub struct ClientId(String);

#[derive(Clone, Debug)]
pub struct Client {
    id: ClientId,
    secret: String,
    redirect_uris: HashSet<String>,
    allowed_scopes: HashSet<String>,
}

impl Client {
    pub fn validate_redirect_uri(&self, uri: &str) -> bool {
        self.redirect_uris.contains(uri)
    }
    
    pub fn validate_scopes(&self, scopes: &Vec<&str>) -> bool {
        scopes.iter().all(|s| self.allowed_scopes.contains(&s.to_string()))
    }
}