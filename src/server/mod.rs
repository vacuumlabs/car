use crate::common::Chain;
use rweb::*;
use sea_orm::{ActiveEnum, ConnectionTrait, DatabaseConnection, DbBackend, Set, Statement};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::net::SocketAddr;

#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
pub struct AddressRef {
    hex: String,
    human: String,
    quantity: i32,
    tags: Vec<String>,
    services: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema)]
pub struct Address {
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

#[derive(Deserialize)]
struct AddressRequest {
    tags: Vec<i32>,
    services: Vec<i32>,
}

#[derive(Deserialize)]
struct AddressFilter {
    tags: Vec<Vec<i32>>,
    services: Vec<Vec<i32>>,
}

#[get("/api/address/")]
#[openapi(description = "Test description?")]
async fn address_list(#[json] body: AddressRequest) -> Result<Json<Vec<AddressRef>>, Rejection> {
    Ok(vec![AddressRef {
        hex: String::from("test"),
        human: String::from("test"),
        tags: Vec::new(),
        services: Vec::new(),
        quantity: 0,
    }]
    .into())
}

pub async fn addresse_filter(
    db: &DatabaseConnection,
    chains: Vec<Chain>,
    services: Option<Vec<i32>>,
    tags: Option<Vec<i32>>,
) -> Result<Vec<i64>, String> {
    match (services, tags) {
        (Some(s), Some(t)) if !s.is_empty() && !t.is_empty() => Ok(vec![1]),
        (None, Some(t)) => Ok(vec![1]),
        (Some(s), None) => Ok(vec![1]),
        _ => Err(String::from("empty query")),
    }
}

pub fn fill_address(
    db: &DatabaseConnection,
    address_list: &[i64],
) -> Result<BTreeMap<i64, Address>, String> {
    Err(String::from("Na kokot"))
}

pub fn fill_address_ref(
    db: &DatabaseConnection,
    address_list: &[i64],
) -> Result<BTreeMap<i64, AddressRef>, String> {
    Err(String::from("Na kokot"))
}

pub async fn run(bind: &SocketAddr, db: &DatabaseConnection) {
    let (spec, filter) =
        openapi::spec().build(|| index(db.clone()).or(address().or(address_list())));

    serve(filter.or(openapi_docs(spec))).run(bind.clone()).await;
}
