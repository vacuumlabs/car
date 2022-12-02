use async_trait::async_trait;
use rweb::*;
use sea_orm::{Database, DatabaseConnection};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
struct AddressRef {
    hex: String,
    human: String,
    tags: Vec<String>,
    services: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
struct Address {
    hex: String,
    human: String,
    inputs: Vec<AddressRef>,
    outputs: Vec<AddressRef>,
    mixed: Vec<AddressRef>,
    tags: Vec<String>,
    services: Vec<String>,
}

#[get("/")]
#[openapi(description = "Test description?")]
async fn index(#[data] db: DatabaseConnection) -> Result<String, Rejection> {
    Ok(String::from(
        "content type will be 'text/plain' as you return String",
    ))
}

#[get("/api/address/{hex}/")]
#[openapi(description = "Test description?")]
async fn address(hex: String) -> Result<Json<Address>, Rejection> {
    Ok(Address {
        hex: hex.clone(),
        human: String::from("test"),
        inputs: Vec::new(),
        outputs: Vec::new(),
        mixed: Vec::new(),
        tags: Vec::new(),
        services: Vec::new(),
    }
    .into())
}

pub async fn run(bind: &SocketAddr, db: &DatabaseConnection) {
    let (spec, filter) = openapi::spec().build(|| index(db.clone()).or(address()));

    serve(filter.or(openapi_docs(spec))).run(bind.clone()).await;
}
