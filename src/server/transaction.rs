use crate::entity::transaction;
use rweb::*;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, EntityTrait, Statement};

#[post("/api/transaction/")] // Create address endpoint
#[openapi(description = "Read address record")]
pub async fn create(
    #[data] db: DatabaseConnection,
    #[data] token: String,
    body: Json<shared::Transaction>,
) -> Result<Json<shared::Transaction>, Rejection> {
    Ok(body.into())
}

#[get("/api/transaction/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
pub async fn detail(
    #[data] db: DatabaseConnection,
    id: String,
) -> Result<Json<shared::Transaction>, Rejection> {
    Ok(shared::Transaction {
        id: Some(1),
        chain: 1,
        amount: 0,
        from: vec![1],
        to: vec![1],
        hash: vec![0],
    }
    .into())
}

#[post("/api/transaction/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
pub async fn update(
    #[data] token: String,
    #[data] db: DatabaseConnection,
    body: Json<shared::Transaction>,
    id: String,
) -> Result<Json<shared::Transaction>, Rejection> {
    Ok(shared::Transaction {
        id: Some(1),
        chain: 1,
        amount: 0,
        from: vec![1],
        to: vec![1],
        hash: vec![0],
    }
    .into())
}

#[delete("/api/transaction/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
pub async fn delete(
    #[data] token: String,
    #[data] db: DatabaseConnection,
    id: String,
) -> Result<Json<shared::Transaction>, Rejection> {
    Ok(shared::Transaction {
        id: Some(1),
        chain: 1,
        amount: 0,
        from: vec![1],
        to: vec![1],
        hash: vec![0],
    }
    .into())
}
