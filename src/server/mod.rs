use rweb::*;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
mod address;
mod analysis;
mod chain;
mod service;
mod tag;
mod transaction;
mod transform;

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Default)]
pub struct AddressRelationHuman {
    id: i64,
    hex: String,
    human: String,
    inputs: Vec<AddressRefHuman>,
    outputs: Vec<AddressRefHuman>,
    mixed_in: Vec<AddressRefHuman>,
    mixed_out: Vec<AddressRefHuman>,
    tags: Vec<String>,
    services: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Default)]
pub struct AddressRelation {
    id: i64,
    hex: String,
    human: String,
    inputs: Vec<AddressRef>,
    outputs: Vec<AddressRef>,
    mixed_in: Vec<AddressRef>,
    mixed_out: Vec<AddressRef>,
    tags: Vec<i32>,
    services: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Default)]
pub struct Address {
    id: Option<i64>,
    hash: String,
    title: Option<String>,
    chain: i32,
    services: Vec<i32>,
    tags: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Default)]
pub struct Chain {
    id: Option<i32>,
    title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Default)]
pub struct Service {
    id: i32,
    title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Default)]
pub struct Tag {
    id: i32,
    title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Default)]
pub struct Transaction {
    id: Option<i32>,
    chain: i32,
    hash: Vec<u8>,
    amount: i64,
    from: Vec<i64>,
    to: Vec<i64>,
}

/// Only for internal look up
#[derive(Debug)]
pub struct PrivAddress {
    pub title: String,
    pub chain: i32,
    pub hash: Vec<u8>,
    pub tags: Vec<i32>,
    pub services: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Default)]
pub struct AddressRef {
    id: i64,
    hex: String,
    human: String,
    quantity: i32,
    tags: Vec<i32>,
    services: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Default)]
pub struct AddressRefHuman {
    id: i64,
    hex: String,
    human: String,
    quantity: i32,
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

pub async fn run(bind: &SocketAddr, db: &DatabaseConnection) {
    let static_path = std::env::var("STATIC_PATH").unwrap_or(String::from("frontend/dist"));
    let (spec, filter) = openapi::spec().build(|| {
        tag::create(db.clone())
            .or(tag::detail(db.clone()))
            .or(tag::list(db.clone()))
            .or(tag::update(db.clone()))
            .or(tag::delete(db.clone()))
            // Service
            .or(service::create(db.clone()))
            .or(service::detail(db.clone()))
            .or(service::list(db.clone()))
            .or(service::update(db.clone()))
            .or(service::delete(db.clone()))
            // Chain
            .or(chain::create(db.clone()))
            .or(chain::detail(db.clone()))
            .or(chain::list(db.clone()))
            .or(chain::update(db.clone()))
            .or(chain::delete(db.clone()))
            // Address
            .or(address::detail(db.clone()))
            .or(address::update(db.clone()))
            .or(address::delete(db.clone()))
            .or(address::create(db.clone()))
            .or(address::list_by_address(db.clone()))
            .or(address::list_by_tag(db.clone()))
            .or(address::list_by_service(db.clone()))
            .or(address::list_by_transaction(db.clone()))
            // Transaction
            .or(transaction::create(db.clone()))
            .or(transaction::detail(db.clone()))
            .or(transaction::update(db.clone()))
            .or(transaction::delete(db.clone()))
            // Analysis
            .or(analysis::relation(db.clone()))
            .or(analysis::relation_human(db.clone()))
            .or(warp::fs::dir(static_path.clone()))
            .or(warp::fs::file(static_path + "/index.html"))
    });

    serve(openapi_docs(spec).or(filter)).run(bind.clone()).await;
}
