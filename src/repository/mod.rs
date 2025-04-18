use crate::domain::client::{Client, ClientId};

pub trait ClientRepository: Send + Sync + 'static {
    async fn find_by_id(&self, id: &ClientId) -> Option<Client>;
}
