use pallas_addresses::Address;
use sea_orm::{prelude::*, ConnectOptions, Database};
use std::{collections::BTreeSet, net::SocketAddr, time::Duration};
use tracing_subscriber::prelude::*;

pub mod common;
pub mod entity;
pub mod server;
pub mod service;
pub mod tag;

#[tokio::main]
async fn main() -> Result<(), String> {
    let fmt_layer = tracing_subscriber::fmt::layer();
    let filter = tracing_subscriber::filter::Targets::new()
        .with_target("sqlx", tracing::Level::DEBUG)
        .with_target("addresses", tracing::Level::TRACE);

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(filter)
        .init();

    let mut opt = ConnectOptions::new(String::from(std::env::var("DATABASE_URL").unwrap()));
    opt.max_connections(2)
        .min_connections(1)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .set_schema_search_path("public".into());
    let db: DatabaseConnection = Database::connect(opt).await.unwrap();

    // Some test queries
    /*
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

    for service in service::init_services(&db.clone()).await.iter() {
        service.clone().mark_addresses(db.clone()).await.unwrap();
    }

    let tasks = vec![
        tokio::spawn(common::address_interacting(
            db.clone(),
            vec![common::Chain::Cardano],
            None,
            None,
            Some(vec![1]),
            None,
            common::DirectionOfInteraction::From,
        )),
        tokio::spawn(common::address_interacting(
            db.clone(),
            vec![common::Chain::Cardano],
            None,
            None,
            Some(vec![2]),
            None,
            common::DirectionOfInteraction::From,
        )),
        tokio::spawn(common::address_interacting(
            db.clone(),
            vec![common::Chain::Cardano],
            None,
            None,
            Some(vec![3]),
            None,
            common::DirectionOfInteraction::From,
        )),
    ];

    let mut sets = Vec::new();
    for task in tasks {
        sets.push(task.await.unwrap().unwrap());
    }

    let mut intersection: Option<BTreeSet<i64>> = None;

    for set in sets.iter() {
        intersection = match &intersection {
            Some(current) => Some(current & set),
            _ => Some(set.clone()),
        }
    }

    tracing::info!("addresses: {:?}", intersection);
    */
    let bind: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3030));
    server::run(&bind, &db).await;
    Ok(())
}
