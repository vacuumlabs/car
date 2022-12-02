use pallas_addresses::Address;
use sea_orm::{prelude::*, Database};
use std::net::SocketAddr;

pub mod entity;
pub mod server;
pub mod service;
pub mod tag;

#[tokio::main]
async fn main() -> Result<(), String> {
    let db: DatabaseConnection =
        Database::connect("postgres://postgres:postgres@localhost/addresses")
            .await
            .unwrap();

    println!("Hello, world!");

    let address = entity::prelude::AddressEntity::find_by_id(36111266)
        .one(&db)
        .await
        .unwrap()
        .unwrap();

    let pallas = Address::from_bytes(&address.hash).unwrap();
    println!(
        "{} {} {} {} {:?}",
        pallas.has_script(),
        pallas.is_enterprise(),
        pallas.typeid(),
        pallas.to_bech32().unwrap(),
        pallas
    );
    /*
    for service in service::init_services(&db).await.iter() {
        service.mark_addresses(&db).await.unwrap();
    }
    */

    let bind: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 3030));
    server::run(&bind, &db).await;
    Ok(())
}
