use async_trait::async_trait;
use sea_orm::prelude::*;

#[async_trait]
pub trait Service {
    /// Mark address related with this service with service and tags flags
    async fn mark_addresses(&self, db: DatabaseConnection) -> Result<(), String>;
}
