//! OpenID Connect Authorization endpoint implementation.
//! Handles the authentication requests and initiates the authorization flow.

use axum::{
    extract::Query,
    response::Redirect,
};
use serde::Deserialize;

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

/// Handles the authorization request and initiates the authentication flow.
/// Returns a redirect to either the login page or the error page depending on the validation result.
pub async fn authorize(Query(params): Query<AuthRequest>) -> Redirect {
    // TODO: Implement authorization flow
    todo!()
}
