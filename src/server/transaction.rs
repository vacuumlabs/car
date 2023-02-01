use crate::entity::transaction;
use crate::server::Transaction;
use rweb::*;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, EntityTrait, Statement};

#[post("/api/transaction/")] // Create address endpoint
#[openapi(description = "Read address record")]
pub async fn create(
    #[data] db: DatabaseConnection,
    body: Json<Transaction>,
) -> Result<Json<Transaction>, Rejection> {
    Ok(body.into())
}

#[get("/api/transaction/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
pub async fn detail(
    #[data] db: DatabaseConnection,
    id: String,
) -> Result<Json<Transaction>, Rejection> {
    Ok(Transaction {
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
    #[data] db: DatabaseConnection,
    body: Json<Transaction>,
    id: String,
) -> Result<Json<Transaction>, Rejection> {
    Ok(Transaction {
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
    #[data] db: DatabaseConnection,
    id: String,
) -> Result<Json<Transaction>, Rejection> {
    Ok(Transaction {
        id: Some(1),
        chain: 1,
        amount: 0,
        from: vec![1],
        to: vec![1],
        hash: vec![0],
    }
    .into())
}
