//! Types for OpenID Connect (OIDC) configuration and metadata.
//! These types represent the standard OIDC discovery document structure.

use serde::{Deserialize, Serialize};

/// Represents the OpenID Provider configuration information as defined by OpenID Connect Discovery 1.0
#[derive(Debug, Serialize, Deserialize)]
pub struct OpenIDConfiguration {
    /// URL using the https scheme with no query or fragment component that the OP asserts as its Issuer Identifier
    pub issuer: String,
    /// URL of the OP's OAuth 2.0 Authorization Endpoint
    pub authorization_endpoint: String,
    /// URL of the OP's OAuth 2.0 Token Endpoint
    pub token_endpoint: String,
    /// URL of the OP's UserInfo Endpoint
    pub userinfo_endpoint: String,
    /// URL of the OP's JSON Web Key Set document
    pub jwks_uri: String,
    /// List of the OAuth 2.0 response_type values that this OP supports
    pub response_types_supported: Vec<String>,
    /// List of the Subject Identifier types that this OP supports
    pub subject_types_supported: Vec<String>,
    /// List of the JWS signing algorithms supported by the OP for ID Token signatures
    pub id_token_signing_alg_values_supported: Vec<String>,
    /// List of the OAuth 2.0 scope values that this server supports
    pub scopes_supported: Vec<String>,
    /// List of Client Authentication methods supported by this Token Endpoint
    pub token_endpoint_auth_methods_supported: Vec<String>,
    /// List of the Claim Names of the Claims that the OpenID Provider MAY be able to supply values for
    pub claims_supported: Vec<String>,
    /// List of the supported Code Challenge methods
    pub code_challenge_methods_supported: Vec<String>,
}
