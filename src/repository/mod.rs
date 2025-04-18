pub mod client_repository;
pub mod auth_request_repository;

// Define the module
use shaku::module;

module! {
    pub OAuthModule {
        components = [
            client_repository::InMemoryClientRepository,
            auth_request_repository::InMemoryAuthRequestRepository,
        ],
        providers = []
    }
}