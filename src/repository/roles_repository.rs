use crate::db::Database;
use crate::domain::role::{Role, UserRole};
use async_trait::async_trait;
use shaku::{Component, Interface};
use std::sync::Arc;
use uuid::Uuid;

pub struct CreateRoleParams {
    pub name: String,
    pub description: Option<String>,
}

#[async_trait]
pub trait RoleRepository: Interface {
}

#[derive(Component)]
#[shaku(interface = RoleRepository)]
pub struct PostgresRoleRepository {
    #[shaku(inject)]
    pool: Arc<dyn Database>,
}

impl PostgresRoleRepository {
    fn new(pool: Arc<dyn Database>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RoleRepository for PostgresRoleRepository {
    
}