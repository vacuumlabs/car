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

#[get("/")]
#[openapi(description = "Test description?")]
async fn index(#[data] db: DatabaseConnection) -> Result<String, Rejection> {
    Ok(String::from(
        "content type will be 'text/plain' as you return String",
    ))
}

pub async fn run(bind: &SocketAddr, db: &DatabaseConnection, frontend_path: String) {
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
            .or(warp::fs::dir(frontend_path.clone()))
            .or(warp::fs::file(frontend_path.clone() + "/index.html"))
    });

    serve(openapi_docs(spec).or(filter)).run(bind.clone()).await;
}
