//! OpenID Connect Discovery endpoint implementation.
//! Provides the OpenID Provider configuration information as specified in the OpenID Connect Discovery specification.

use super::types::OpenIDConfiguration;
use crate::Config;
use crate::config::OIDCConfig;
use axum::extract::State;
use axum::{response::Json, routing::get, Router};

/// Handles the OpenID Configuration endpoint request.
/// Returns a JSON response containing the OpenID Provider configuration information.
pub async fn openid_configuration(State(config): State<OIDCConfig>) -> Json<OpenIDConfiguration> {
    let base_url = config.external_url.unwrap();

    Json(OpenIDConfiguration {
        // The Issuer Identifier for the OpenID Provider
        issuer: base_url.clone(),
        // URL of the OAuth 2.0 Authorization Endpoint
        authorization_endpoint: format!("{}/auth", base_url),
        // URL of the OAuth 2.0 Token Endpoint
        token_endpoint: format!("{}/token", base_url),
        // URL of the UserInfo Endpoint
        userinfo_endpoint: format!("{}/userinfo", base_url),
        // URL of the JSON Web Key Set document
        jwks_uri: format!("{}/jwks", base_url),
        // List of OAuth 2.0 response_type values supported
        response_types_supported: vec!["code".to_string()],
        // List of subject identifier types supported
        subject_types_supported: vec!["public".to_string()],
        // List of JWS signing algorithms supported for ID Token
        id_token_signing_alg_values_supported: vec!["RS256".to_string()],
        // List of OAuth 2.0 scope values supported
        scopes_supported: vec![
            "openid".to_string(),
            "profile".to_string(),
            "email".to_string(),
        ],
        // List of client authentication methods supported
        token_endpoint_auth_methods_supported: vec!["client_secret_basic".to_string()],
        // List of claim names supported
        claims_supported: vec![
            "sub".to_string(),
            "iss".to_string(),
            "name".to_string(),
            "email".to_string(),
        ],
        // List of PKCE code challenge methods supported
        code_challenge_methods_supported: vec!["S256".to_string()],
    })
}
