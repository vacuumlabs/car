use rweb::rt::IndexMap;
use rweb::*;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
mod address;
mod analysis;
mod chain;
mod service;
mod tag;
mod transaction;
mod transform;

#[derive(Debug, Clone)]
pub struct Unauthorized;

#[derive(Debug, Clone)]
pub struct NotFound;

#[derive(Debug, Clone)]
pub struct InternalError;

impl warp::reject::Reject for Unauthorized {}
impl warp::reject::Reject for NotFound {}
impl warp::reject::Reject for InternalError {}

#[get("/api/token")]
#[openapi(description = "Check token")]
pub async fn token_check(
    #[header = "authorization"] authorization: String,
    #[data] token: String,
) -> Result<Json<bool>, Rejection> {
    if !authorization.ends_with(&token) {
        return Err(reject::custom(Unauthorized));
    }

    Ok(true.into())
}

pub async fn run(
    bind: &SocketAddr,
    db: &DatabaseConnection,
    feed_channel: crate::FeedChannel,
    token: String,
    frontend_path: String,
) {
    let (spec, filter) = openapi::spec().build(|| {
        token_check(token.clone())
            .or(tag::create(db.clone(), token.clone()))
            .or(tag::detail(db.clone()))
            .or(tag::list(db.clone()))
            .or(tag::update(db.clone(), token.clone()))
            .or(tag::delete(db.clone(), token.clone()))
            // Service
            .or(service::create(db.clone(), token.clone()))
            .or(service::detail(db.clone()))
            .or(service::list(db.clone()))
            .or(service::update(db.clone(), token.clone()))
            .or(service::delete(db.clone(), token.clone()))
            // Chain
            .or(chain::create(
                db.clone(),
                token.clone(),
                feed_channel.clone(),
            ))
            .or(chain::detail(db.clone()))
            .or(chain::list(db.clone()))
            .or(chain::update(db.clone(), token.clone()))
            .or(chain::delete(
                token.clone(),
                db.clone(),
                feed_channel.clone(),
            ))
            // Address
            .or(address::detail(db.clone()))
            .or(address::update(db.clone(), token.clone()))
            .or(address::delete(db.clone(), token.clone()))
            .or(address::create(db.clone(), token.clone()))
            .or(address::process(
                db.clone(),
                token.clone(),
                feed_channel.clone(),
            ))
            .or(address::list_by_address(db.clone()))
            .or(address::list_by_tag(db.clone()))
            .or(address::list_by_service(db.clone()))
            .or(address::list_by_transaction(db.clone()))
            // Transaction
            .or(transaction::create(db.clone(), token.clone()))
            .or(transaction::detail(db.clone()))
            .or(transaction::update(db.clone(), token.clone()))
            .or(transaction::delete(db.clone(), token.clone()))
            // Analysis
            .or(analysis::relation(db.clone()))
            .or(analysis::relation_human(db.clone()))
    });

    serve(
        openapi_docs(spec)
            .or(filter)
            // Rest of get request handle with static files
            .or(warp::get().and(warp::fs::dir(frontend_path.clone())))
            .or(warp::get().and(warp::fs::file(frontend_path.clone() + "/index.html")))
            .recover(|err: Rejection| async move {
                let (reply, code) = if let Some(_err) = err.find::<Unauthorized>() {
                    (
                        "UNAUTHORIZED".to_string(),
                        warp::http::StatusCode::UNAUTHORIZED,
                    )
                } else if let Some(_err) = err.find::<NotFound>() {
                    ("NOT FOUND".to_string(), warp::http::StatusCode::NOT_FOUND)
                } else {
                    (
                        "INTERNAL_SERVER_ERROR".to_string(),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    )
                };

                Ok::<_, std::convert::Infallible>(warp::reply::with_status(reply, code))
            }),
    )
    .run(bind.clone())
    .await;
}
