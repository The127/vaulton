// src/di/mod.rs
use shaku::{Component};


// Define our module
use shaku::module;

module! {
    pub MyModule {
        components = [
            crate::db::DatabaseImpl,
            crate::repository::client_repository::PostgresClientRepository
        ],
        providers = []
    }
}