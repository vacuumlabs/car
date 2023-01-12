use hex;
use rweb::*;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::net::SocketAddr;

mod transform;

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Default)]
pub struct AddressRef {
    hex: String,
    human: String,
    quantity: i32,
    tags: Vec<String>,
    services: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Default)]
pub struct AddressRelation {
    hex: String,
    human: String,
    inputs: Vec<AddressRef>,
    outputs: Vec<AddressRef>,
    mixed_in: Vec<AddressRef>,
    mixed_out: Vec<AddressRef>,
    tags: Vec<String>,
    services: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Default)]
pub struct Address {
    id: Option<i64>,
    chain: i32,
    hash: Vec<u8>,
    title: String,
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
    id: Option<i32>,
    title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Default)]
pub struct Tag {
    id: Option<i32>,
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
    pub hash: Vec<u8>,
    pub tags: Vec<i32>,
    pub services: Vec<i32>,
}

#[get("/")]
#[openapi(description = "Test description?")]
async fn index(#[data] db: DatabaseConnection) -> Result<String, Rejection> {
    Ok(String::from(
        "content type will be 'text/plain' as you return String",
    ))
}

/// Create address endpoint
#[post("/api/address/")]
#[openapi(description = "Create address record")]
async fn address_create(
    #[data] db: DatabaseConnection,
    #[json] body: Address,
) -> Result<Json<Address>, Rejection> {
    Ok(body.into())
}

/// Create address endpoint
#[get("/api/address/{address}")]
#[openapi(description = "Read address record")]
async fn address_detail(
    #[data] db: DatabaseConnection,
    address: String,
) -> Result<Json<Address>, Rejection> {
    let address = Address {
        chain: 1,
        id: Some(1),
        hash: vec![0, 0],
        services: vec![1],
        tags: vec![1],
        title: String::from("test"),
    };
    Ok(address.into())
}

#[post("/api/address/{address}")] // Create address endpoint
#[openapi(description = "Read address record")]
async fn address_update(
    #[data] db: DatabaseConnection,
    address: String,
) -> Result<Json<Address>, Rejection> {
    let address = Address {
        chain: 1,
        id: Some(1),
        hash: vec![0, 0],
        services: vec![1],
        tags: vec![1],
        title: String::from("test"),
    };
    Ok(address.into())
}

#[delete("/api/address/{address}")] // Create address endpoint
#[openapi(description = "Read address record")]
async fn address_delete(
    #[data] db: DatabaseConnection,
    address: String,
) -> Result<Json<Address>, Rejection> {
    let address = Address {
        chain: 1,
        id: Some(1),
        hash: vec![0, 0],
        services: vec![1],
        tags: vec![1],
        title: String::from("test"),
    };
    Ok(address.into())
}

#[post("/api/tag/")] // Create address endpoint
#[openapi(description = "Read address record")]
async fn tag_create(
    #[data] db: DatabaseConnection,
    #[json] body: Tag,
) -> Result<Json<Tag>, Rejection> {
    Ok(body.into())
}

#[get("/api/tag/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
async fn tag_detail(#[data] db: DatabaseConnection, id: String) -> Result<Json<Tag>, Rejection> {
    Ok(Tag {
        id: Some(1),
        title: String::from("test"),
    }
    .into())
}

#[get("/api/tag/")] // Create address endpoint
#[openapi(description = "Read address record")]
async fn tag_list(#[data] db: DatabaseConnection) -> Result<Json<Vec<Tag>>, Rejection> {
    Ok(vec![Tag {
        id: Some(1),
        title: String::from("test"),
    }]
    .into())
}

#[post("/api/tag/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
async fn tag_update(
    #[data] db: DatabaseConnection,
    #[json] body: Tag,
    id: String,
) -> Result<Json<Tag>, Rejection> {
    Ok(Tag {
        id: Some(1),
        title: String::from("test"),
    }
    .into())
}

#[delete("/api/tag/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
async fn tag_delete(#[data] db: DatabaseConnection, id: String) -> Result<Json<Tag>, Rejection> {
    Ok(Tag {
        id: Some(1),
        title: String::from("test"),
    }
    .into())
}

/////

#[post("/api/service/")] // Create address endpoint
#[openapi(description = "Read address record")]
async fn service_create(
    #[data] db: DatabaseConnection,
    #[json] body: Service,
) -> Result<Json<Service>, Rejection> {
    Ok(body.into())
}

#[get("/api/service/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
async fn service_detail(
    #[data] db: DatabaseConnection,
    id: String,
) -> Result<Json<Service>, Rejection> {
    Ok(Service {
        id: Some(1),
        title: String::from("test"),
    }
    .into())
}

#[get("/api/service/")] // Create address endpoint
#[openapi(description = "Read address record")]
async fn service_list(#[data] db: DatabaseConnection) -> Result<Json<Vec<Service>>, Rejection> {
    Ok(vec![Service {
        id: Some(1),
        title: String::from("test"),
    }]
    .into())
}

#[post("/api/service/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
async fn service_update(
    #[data] db: DatabaseConnection,
    #[json] body: Tag,
    id: String,
) -> Result<Json<Tag>, Rejection> {
    Ok(Tag {
        id: Some(1),
        title: String::from("test"),
    }
    .into())
}

#[delete("/api/service/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
async fn service_delete(
    #[data] db: DatabaseConnection,
    id: String,
) -> Result<Json<Service>, Rejection> {
    Ok(Service {
        id: Some(1),
        title: String::from("test"),
    }
    .into())
}

/////

#[post("/api/chain/")] // Create address endpoint
#[openapi(description = "Read address record")]
async fn chain_create(
    #[data] db: DatabaseConnection,
    #[json] body: Chain,
) -> Result<Json<Chain>, Rejection> {
    Ok(body.into())
}

#[get("/api/chain/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
async fn chain_detail(
    #[data] db: DatabaseConnection,
    id: String,
) -> Result<Json<Chain>, Rejection> {
    Ok(Chain {
        id: Some(1),
        title: String::from("test"),
    }
    .into())
}

#[get("/api/chain/")] // Create address endpoint
#[openapi(description = "Read address record")]
async fn chain_list(#[data] db: DatabaseConnection) -> Result<Json<Vec<Chain>>, Rejection> {
    Ok(vec![Chain {
        id: Some(1),
        title: String::from("test"),
    }]
    .into())
}

#[post("/api/chain/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
async fn chain_update(
    #[data] db: DatabaseConnection,
    #[json] body: Tag,
    id: String,
) -> Result<Json<Tag>, Rejection> {
    Ok(Tag {
        id: Some(1),
        title: String::from("test"),
    }
    .into())
}

#[delete("/api/chain/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
async fn chain_delete(
    #[data] db: DatabaseConnection,
    id: String,
) -> Result<Json<Chain>, Rejection> {
    Ok(Chain {
        id: Some(1),
        title: String::from("test"),
    }
    .into())
}

#[post("/api/transaction/")] // Create address endpoint
#[openapi(description = "Read address record")]
async fn transaction_create(
    #[data] db: DatabaseConnection,
    #[json] body: Transaction,
) -> Result<Json<Transaction>, Rejection> {
    Ok(body.into())
}

#[get("/api/transaction/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
async fn transaction_detail(
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
async fn transaction_update(
    #[data] db: DatabaseConnection,
    #[json] body: Tag,
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
async fn transaction_delete(
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

////

#[get("/api/analysis/address/{address}")] // TODO: Chain select?
#[openapi(description = "Test description?")]
async fn address_relation(
    #[data] db: DatabaseConnection,
    address: String,
) -> Result<Json<AddressRelation>, Rejection> {
    if let Ok(address_hex) = hex::decode(&address) {
        // Get address ID
        let statement = Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"SELECT id FROM address WHERE hash = $1;"#,
            vec![address_hex.clone().into()],
        );
        return match db.query_one(statement).await {
            Ok(Some(result)) => {
                let mut address_list: BTreeSet<i64> = BTreeSet::new();
                let mut address_map: BTreeMap<i64, PrivAddress> = BTreeMap::new();
                let mut tag_list: BTreeSet<i32> = BTreeSet::new();
                let mut tag_map: BTreeMap<i32, String> = BTreeMap::new();

                let mut service_list: BTreeSet<i32> = BTreeSet::new();
                let mut service_map: BTreeMap<i32, String> = BTreeMap::new();
                let mut inputs: BTreeMap<i64, i32> = BTreeMap::new();
                let mut outputs: BTreeMap<i64, i32> = BTreeMap::new();
                let mut mixed_in: BTreeMap<i64, i32> = BTreeMap::new();
                let mut mixed_out: BTreeMap<i64, i32> = BTreeMap::new();

                let address_id: i64 = result.try_get("", "id").unwrap();
                address_list.insert(address_id);

                // Get relations
                let statement = Statement::from_sql_and_values(
                    DbBackend::Postgres,
                    r#"SELECT "from", "to" FROM transaction WHERE $1 = ANY("from") or $1 = ANY("to");"#,
                    vec![address_id.into()],
                );
                if let Ok(query) = db.query_all(statement).await {
                    for row in query.iter() {
                        let addresses_from: Vec<i64> = row.try_get("", "from").unwrap();
                        let addresses_to: Vec<i64> = row.try_get("", "to").unwrap();

                        let mixed_in_state = addresses_from.contains(&address_id);
                        let mixed_out_state = addresses_to.contains(&address_id);

                        // Proces input adress
                        for address in addresses_from {
                            // Add input only, if the original address is present in output
                            address_list.insert(address);
                            if mixed_out_state {
                                inputs.insert(address, inputs.get(&address).unwrap_or(&0) + 1);
                            }
                            // Add mixin only if is original address present in input
                            if mixed_in_state {
                                mixed_in.insert(address, mixed_in.get(&address).unwrap_or(&0) + 1);
                            }
                        }

                        // Proces output adress
                        for address in addresses_to {
                            address_list.insert(address);
                            // Add output only if the original address is present in input
                            if mixed_in_state {
                                outputs.insert(address, outputs.get(&address).unwrap_or(&0) + 1);
                            }
                            // Add mixin only if is original address present in output
                            if mixed_out_state {
                                mixed_out
                                    .insert(address, mixed_out.get(&address).unwrap_or(&0) + 1);
                            }
                        }
                    }
                }

                // Map DB resources
                transform::map_addresses(
                    &db,
                    &mut address_list,
                    &mut address_map,
                    &mut tag_list,
                    &mut service_list,
                )
                .await;
                transform::map_tags(&db, &tag_list, &mut tag_map).await;
                transform::map_services(&db, &service_list, &mut service_map).await;

                // Transfer to json output
                let address_detail = address_map.get(&address_id).unwrap();
                Ok(AddressRelation {
                    hex: address.clone(),
                    human: address_detail.title.clone(),
                    inputs: transform::address_ref(&address_map, &tag_map, &service_map, inputs),
                    outputs: transform::address_ref(&address_map, &tag_map, &service_map, outputs),
                    mixed_in: transform::address_ref(
                        &address_map,
                        &tag_map,
                        &service_map,
                        mixed_in,
                    ),
                    mixed_out: transform::address_ref(
                        &address_map,
                        &tag_map,
                        &service_map,
                        mixed_out,
                    ),
                    tags: address_detail
                        .tags
                        .iter()
                        .map(|t| tag_map.get(t).unwrap_or(&t.to_string()).clone())
                        .collect(),
                    services: address_detail
                        .services
                        .iter()
                        .map(|s| service_map.get(s).unwrap_or(&s.to_string()).clone())
                        .collect(),
                }
                .into())
            }
            _ => Err(warp::reject::not_found()),
        };
    }
    Err(warp::reject::not_found())
}

pub async fn run(bind: &SocketAddr, db: &DatabaseConnection) {
    let (spec, filter) = openapi::spec().build(|| {
        index(db.clone())
            // Tag
            .or(tag_create(db.clone()))
            .or(tag_detail(db.clone()))
            .or(tag_list(db.clone()))
            .or(tag_update(db.clone()))
            .or(tag_delete(db.clone()))
            // Service
            .or(service_create(db.clone()))
            .or(service_detail(db.clone()))
            .or(service_list(db.clone()))
            .or(service_update(db.clone()))
            .or(service_delete(db.clone()))
            // Chain
            .or(chain_create(db.clone()))
            .or(chain_detail(db.clone()))
            .or(chain_list(db.clone()))
            .or(chain_update(db.clone()))
            .or(chain_delete(db.clone()))
            // Address
            .or(address_create(db.clone()))
            .or(address_detail(db.clone()))
            .or(address_update(db.clone()))
            .or(address_delete(db.clone()))
            // Transaction
            .or(transaction_create(db.clone()))
            .or(transaction_detail(db.clone()))
            .or(transaction_update(db.clone()))
            .or(transaction_delete(db.clone()))
            // Analytics
            .or(address_relation(db.clone()))
    });

    serve(filter.or(openapi_docs(spec))).run(bind.clone()).await;
}
