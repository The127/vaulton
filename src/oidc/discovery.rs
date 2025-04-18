//! OpenID Connect Discovery endpoint implementation.
//! Provides the OpenID Provider configuration information as specified in the OpenID Connect Discovery specification.

use super::types::OpenIDConfiguration;
use axum::{response::Json, routing::get, Router};

/// Creates a router with OpenID Connect Discovery endpoint.
/// Exposes the well-known OpenID configuration at /.well-known/openid-configuration
pub fn discovery_routes() -> Router {
    Router::new().route(
        "/.well-known/openid-configuration",
        get(openid_configuration),
    )
}

/// Handles the OpenID Configuration endpoint request.
/// Returns a JSON response containing the OpenID Provider configuration information.
async fn openid_configuration() -> Json<OpenIDConfiguration> {
    Json(OpenIDConfiguration {
        // The Issuer Identifier for the OpenID Provider
        issuer: "http://localhost:3000".to_string(),
        // URL of the OAuth 2.0 Authorization Endpoint
        authorization_endpoint: "http://localhost:3000/auth".to_string(),
        // URL of the OAuth 2.0 Token Endpoint
        token_endpoint: "http://localhost:3000/token".to_string(),
        // URL of the UserInfo Endpoint
        userinfo_endpoint: "http://localhost:3000/userinfo".to_string(),
        // URL of the JSON Web Key Set document
        jwks_uri: "http://localhost:3000/jwks".to_string(),
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
