#![recursion_limit = "256"]

use feed::Feed;
use pallas_addresses::Address;
use sea_orm::{prelude::*, ConnectOptions, Database};
use std::{
    collections::{BTreeSet, HashMap},
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};
use tokio::sync::RwLock;
use tracing_subscriber::prelude::*;

pub mod common;
pub mod entity;
pub mod feed;
pub mod server;
pub mod service;
pub mod tag;

type FeedChannel = Arc<RwLock<HashMap<i32, tokio::sync::mpsc::Sender<feed::FeedCommand>>>>;

#[tokio::main]
async fn main() -> Result<(), String> {
    let fmt_layer = tracing_subscriber::fmt::layer();
    let filter = tracing_subscriber::filter::Targets::new()
        .with_target("sqlx", tracing::Level::DEBUG)
        .with_target("rweb", tracing::Level::DEBUG)
        .with_target("car", tracing::Level::DEBUG);

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(filter)
        .init();

    let mut opt = ConnectOptions::new(String::from(std::env::var("DATABASE_URL").unwrap()));
    let address: SocketAddr = std::env::var("ADDRESS")
        .unwrap_or(String::from("0.0.0.0:3030"))
        .parse()
        .unwrap();
    let frontend_path = String::from(std::env::var("STATIC").unwrap_or(String::from("./dist")));
    let token = String::from(std::env::var("TOKEN").unwrap_or(String::from("token")));

    //
    opt.max_connections(2)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .set_schema_search_path("public".into());
    let db: DatabaseConnection = Database::connect(opt).await.unwrap();

    /*
    // Some test queries
    let address = entity::prelude::AddressEntity::find_by_id(36111266)
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    let pallas = Address::from_bytes(&address.hash).unwrap();
    tracing::info!(
        "{} {} {} {} {:?}",
        pallas.has_script(),
        pallas.is_enterprise(),
        pallas.typeid(),
        pallas.to_bech32().unwrap(),
        pallas
    );
    */

    /*
    for service in service::init_services(&db.clone()).await.iter() {
        service.clone().mark_addresses(db.clone()).await.unwrap();
    }
    */

    /*
    let set1 = common::address_interacting(
        db.clone(),
        vec![common::Chain::Cardano],
        None,
        None,
        Some(vec![1]),
        None,
        common::DirectionOfInteraction::From,
    )
    .await
    .unwrap();

    let set2 = common::address_interacting(
        db.clone(),
        vec![common::Chain::Cardano],
        None,
        None,
        Some(vec![2]),
        None,
        common::DirectionOfInteraction::From,
    )
    .await
    .unwrap();

    let set3 = common::address_interacting(
        db.clone(),
        vec![common::Chain::Cardano],
        None,
        None,
        Some(vec![3]),
        None,
        common::DirectionOfInteraction::From,
    )
    .await
    .unwrap();

    //let mut intersection: Option<BTreeSet<i64>> = None;

    let set4 = &set1 & &set2;
    let intersection = &set4 & &set3;
    tracing::debug!("addresses: {:?}", intersection);
    */
    //let bind: SocketAddr = ;

    let mut feed_channel: FeedChannel = Arc::new(RwLock::new(HashMap::new()));

    if let Ok(chains) = entity::chain::Entity::find().all(&db).await {
        for chain in chains {
            start_feeder(db.clone(), feed_channel.clone(), chain).await;
        }
    }

    tracing::info!(
        "Serving: {}, static: {}, database: {}",
        address,
        frontend_path,
        "ratata"
    );
    server::run(&address, &db, feed_channel, token, frontend_path).await;
    Ok(())
}

pub async fn start_feeder(
    db: DatabaseConnection,
    feed_channel: FeedChannel,
    chain: entity::chain::Model,
) {
    let (sender, receiver) = tokio::sync::mpsc::channel::<feed::FeedCommand>(16);
    let db = db.clone();
    let mut feed_channel = feed_channel.write().await;
    let params: shared::ChainParam = serde_json::from_value(chain.params).unwrap();
    match params {
        shared::ChainParam::ArbiScan(mut anyscan) => {
            feed_channel.insert(chain.id.clone(), sender);
            tokio::task::spawn(async move {
                anyscan.run(db, receiver, chain.id).await;
            });
        }
        shared::ChainParam::EtherScan(mut anyscan) => {
            feed_channel.insert(chain.id.clone(), sender);
            tokio::task::spawn(async move {
                anyscan.run(db, receiver, chain.id).await;
            });
        }
        shared::ChainParam::PolyScan(mut anyscan) => {
            feed_channel.insert(chain.id.clone(), sender);
            tokio::task::spawn(async move {
                anyscan.run(db, receiver, chain.id).await;
            });
        }
        shared::ChainParam::None => {}
    }
}
