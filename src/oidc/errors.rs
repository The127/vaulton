// src/oidc/error.rs
use axum::response::Redirect;
use serde::Serialize;
use std::fmt;
use url::Url;

#[derive(Debug, Serialize)]
pub enum OAuthError {
    InvalidRequest(String),
    UnauthorizedClient(String),
    AccessDenied(String),
    UnsupportedResponseType(String),
    InvalidScope(String),
    ServerError(String),
    TemporarilyUnavailable(String),
}

impl fmt::Display for OAuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRequest(desc) => write!(f, "invalid_request: {}", desc),
            Self::UnauthorizedClient(desc) => write!(f, "unauthorized_client: {}", desc),
            Self::AccessDenied(desc) => write!(f, "access_denied: {}", desc),
            Self::UnsupportedResponseType(desc) => write!(f, "unsupported_response_type: {}", desc),
            Self::InvalidScope(desc) => write!(f, "invalid_scope: {}", desc),
            Self::ServerError(desc) => write!(f, "server_error: {}", desc),
            Self::TemporarilyUnavailable(desc) => write!(f, "temporarily_unavailable: {}", desc),
        }
    }
}

impl OAuthError {
    pub fn to_redirect_response(&self, redirect_uri: &str, state: Option<&str>) -> Redirect {
        let mut url = Url::parse(redirect_uri).expect("valid redirect URI");
        
        let (error, description) = match self {
            Self::InvalidRequest(desc) => ("invalid_request", desc),
            Self::UnauthorizedClient(desc) => ("unauthorized_client", desc),
            Self::AccessDenied(desc) => ("access_denied", desc),
            Self::UnsupportedResponseType(desc) => ("unsupported_response_type", desc),
            Self::InvalidScope(desc) => ("invalid_scope", desc),
            Self::ServerError(desc) => ("server_error", desc),
            Self::TemporarilyUnavailable(desc) => ("temporarily_unavailable", desc),
        };

        url.query_pairs_mut()
            .append_pair("error", error)
            .append_pair("error_description", description);

        if let Some(state) = state {
            url.query_pairs_mut().append_pair("state", state);
        }

        Redirect::to(url.as_str())
    }
}