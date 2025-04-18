//! OpenID Connect Authorization endpoint implementation.
//! Handles the authentication requests and initiates the authorization flow.

use std::sync::Arc;
use super::error::OAuthError;
use axum::{extract::Query, response::Redirect};
use axum::extract::State;
use serde::Deserialize;
use shaku::HasComponent;
use crate::repository::client_repository::ClientRepository;
use crate::repository::auth_request_repository::AuthRequestRepository;
use crate::server::AppState;use chrono::{DateTime, Utc};

/// Represents an OpenID Connect authorization request.
/// Contains the parameters required for initiating the authentication flow.
#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    /// The client identifier issued to the client during the registration process
    client_id: String,
    /// URL to which the response will be sent after the authorization
    redirect_uri: String,
    /// OAuth 2.0 response type value. Must be "code" for Authorization Code Flow
    response_type: String,
    /// Space-separated list of requested scope values
    scope: Option<String>,
    /// Opaque value to maintain state between the request and callback
    state: Option<String>,
    /// PKCE code challenge value
    code_challenge: Option<String>,
    /// PKCE code challenge method (e.g., "S256")
    code_challenge_method: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AuthorizationRequest {
    pub request_id: String,
    pub client_id: String,
    pub redirect_uri: String,
    pub scope: String,
    pub state: Option<String>,
    pub code_challenge: Option<String>,
    pub code_challenge_method: Option<String>,
    pub created_at: DateTime<Utc>,
}

/// Handles the authorization request and initiates the authentication flow.
/// Returns a redirect to either the login page or the error page depending on the validation result.
pub async fn authorize(
    State(state): State<AppState>,
    Query(params): Query<AuthRequest>,
) -> Redirect {
    if params.response_type != "code" {
        return OAuthError::UnsupportedResponseType(
            "Only 'code' response type is supported".to_string(),
        )
        .to_redirect_response(&params.redirect_uri, params.state.as_deref());
    }

    if let Some(method) = &params.code_challenge_method {
        if method != "S256" {
            return OAuthError::InvalidRequest(
                "Only 'S256' code challenge method is supported".to_string(),
            )
            .to_redirect_response(&params.redirect_uri, params.state.as_deref());
        }
    }

    let requested_scopes = params.scope
        .as_deref()
        .unwrap_or("openid")
        .split_whitespace()
        .collect::<Vec<_>>();

    if !requested_scopes.contains(&"openid") {
        return OAuthError::InvalidScope("Missing 'openid' scope".to_string())
            .to_redirect_response(&params.redirect_uri, params.state.as_deref());
    }

    let client_repository: Arc<dyn ClientRepository> = state.module.as_ref().resolve();

    let client = match client_repository.find_by_id(&params.client_id).await {
        Some(client) => client,
        None => {
            return OAuthError::InvalidClient("Client not found".to_string())
                .to_redirect_response(&params.redirect_uri, params.state.as_deref());
        }
    };

    if !client.validate_redirect_uri(&params.redirect_uri) {
        return OAuthError::InvalidRequest("Invalid redirect URI".to_string())
            .to_redirect_response(&params.redirect_uri, params.state.as_deref());
    }

    if !client.validate_scopes(&requested_scopes) {
        return OAuthError::InvalidScope("Requested scopes not allowed for this client".to_string())
            .to_redirect_response(&params.redirect_uri, params.state.as_deref());
    }

    let auth_req = AuthorizationRequest {
        client_id: params.client_id,
        redirect_uri: params.redirect_uri.clone(),
        scope: params.scope.unwrap_or_else(|| "openid".to_string()),
        state: params.state.clone(),
        code_challenge: params.code_challenge,
        code_challenge_method: params.code_challenge_method,
        // Generate a unique request ID
        request_id: generate_request_id(),
        // Set creation time
        created_at: chrono::Utc::now(),
    };

    let auth_request_repository: Arc<dyn AuthRequestRepository> = state.module.resolve();

    // Store the authorization request
    match auth_request_repository.store_request(&auth_req).await {
        Ok(_) => {
            Redirect::temporary(&format!("/login?request_id={}", auth_req.request_id))
        }
        Err(_) => {
            OAuthError::ServerError("Failed to store authorization request".to_string())
                .to_redirect_response(&params.redirect_uri, params.state.clone().as_deref())
        }
    }

}

fn generate_request_id() -> String {
    use rand::{thread_rng, Rng};
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    const REQUEST_ID_LEN: usize = 32;

    let mut rng = thread_rng();
    let request_id: String = (0..REQUEST_ID_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    request_id
}

