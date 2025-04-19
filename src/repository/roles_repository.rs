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
    async fn create(&self, role: CreateRoleParams) -> Result<Role, String>;
    async fn find_by_id(&self, id: Uuid) -> Option<Role>;
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
    async fn create(&self, params: CreateRoleParams) -> Result<Role, String> {
        let result = sqlx::query!(
            r#"
            insert into roles (name, description)
            values ($1, $2)
            returning id as "uuid", name, description, created_at, updated_at
            "#,
            params.name.clone(),
            params.description,
        )
        .fetch_one(self.pool.get_pool())
        .await
        .map_err(|e| e.to_string())?;

        Ok(Role {
            uuid: result.uuid,
            name: result.name,
            description: result.description,
            created_at: result.created_at,
            updated_at: result.updated_at,
        })
    }

    async fn find_by_id(&self, id: Uuid) -> Option<Role> {
        let result = sqlx::query!(
            r#"
            select id as "uuid", name, description, created_at, updated_at
            from roles
            where id = $1
            "#,
            id.clone()
        )
        .fetch_optional(self.pool.get_pool())
        .await
        .ok()??;

        Some(Role {
            uuid: result.uuid,
            name: result.name,
            description: result.description,
            created_at: result.created_at,
            updated_at: result.updated_at,
        })
    }
}