use axum::{
    extract::Query,
    response::Redirect,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    client_id: String,
    redirect_uri: String,
    response_type: String,
    scope: Option<String>,
    state: Option<String>,
    code_challenge: Option<String>,
    code_challenge_method: Option<String>,
}

pub async fn authorize(Query(params): Query<AuthRequest>) -> Redirect {
    // TODO: Implement authorization flow
    todo!()
}
