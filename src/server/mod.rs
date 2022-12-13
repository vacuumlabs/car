use hex;
use rweb::*;
use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::net::SocketAddr;

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Default)]
pub struct AddressRef {
    hex: String,
    human: String,
    quantity: i32,
    tags: Vec<String>,
    services: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Schema, Default)]
pub struct Address {
    hex: String,
    human: String,
    inputs: Vec<AddressRef>,
    outputs: Vec<AddressRef>,
    mixed_in: Vec<AddressRef>,
    mixed_out: Vec<AddressRef>,
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

/// Only for internal look up
#[derive(Debug)]
pub struct PrivAddress {
    pub title: String,
    pub hash: Vec<u8>,
    pub tags: Vec<i32>,
    pub services: Vec<i32>,
}

/// Map adress ID to internal format
async fn map_addresses(
    db: &DatabaseConnection,
    address_list: &mut BTreeSet<i64>,
    address_map: &mut BTreeMap<i64, PrivAddress>,
    tag_list: &mut BTreeSet<i32>,
    service_list: &mut BTreeSet<i32>,
) {
    let statement = Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"SELECT id, title, hash, tags, services FROM address WHERE id = ANY($1);"#,
        vec![address_list
            .iter()
            .map(|a| a.clone())
            .collect::<Vec<i64>>()
            .into()],
    );

    if let Ok(query) = db.query_all(statement.clone()).await {
        for row in query.iter() {
            let id: i64 = row.try_get("", "id").unwrap();
            let tags: Vec<i32> = row.try_get("", "tags").unwrap_or(Vec::new());
            let services: Vec<i32> = row.try_get("", "services").unwrap_or(Vec::new());

            for tag in tags.iter() {
                tag_list.insert(*tag);
            }

            for service in services.iter() {
                service_list.insert(*service);
            }

            let hash = row.try_get("", "hash").unwrap();
            address_map.insert(
                id,
                PrivAddress {
                    title: row.try_get("", "title").unwrap_or(hex::encode(&hash)),
                    hash: hash,
                    tags: tags,
                    services: services,
                },
            );
        }
    } else {
        tracing::error!("QUERY ERROR: {:?}", statement);
    }
}

/// Colect information about tags
pub async fn map_tags(
    db: &DatabaseConnection,
    tag_list: &BTreeSet<i32>,
    tag_map: &mut BTreeMap<i32, String>,
) {
    let statement = Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"SELECT id, title from tag WHERE id = ANY($1);"#,
        vec![tag_list
            .iter()
            .map(|a| a.clone())
            .collect::<Vec<i32>>()
            .into()],
    );
    if let Ok(query) = db.query_all(statement).await {
        for row in query.iter() {
            tag_map.insert(
                row.try_get::<i32>("", "id").unwrap(),
                row.try_get::<String>("", "title").unwrap(),
            );
        }
    }
}

// Colect information about services
pub async fn map_services(
    db: &DatabaseConnection,
    service_list: &BTreeSet<i32>,
    service_map: &mut BTreeMap<i32, String>,
) {
    let statement = Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"SELECT id, title from service WHERE id = ANY($1);"#,
        vec![service_list
            .iter()
            .map(|a| a.clone())
            .collect::<Vec<i32>>()
            .into()],
    );
    if let Ok(query) = db.query_all(statement).await {
        for row in query.iter() {
            service_map.insert(
                row.try_get::<i32>("", "id").unwrap(),
                row.try_get::<String>("", "title").unwrap(),
            );
        }
    }
}

pub fn address_ref(
    address_map: &BTreeMap<i64, PrivAddress>,
    tag_map: &BTreeMap<i32, String>,
    service_map: &BTreeMap<i32, String>,
    addresses: BTreeMap<i64, i32>,
) -> Vec<AddressRef> {
    let mut result: Vec<AddressRef> = addresses
        .iter()
        .map(|(address_id, address_count)| {
            if let Some(address) = address_map.get(address_id) {
                return AddressRef {
                    hex: hex::encode(&address.hash),
                    human: address.title.clone(),
                    quantity: address_count.clone(),
                    tags: address
                        .tags
                        .iter()
                        .map(|t| tag_map.get(t).unwrap_or(&t.to_string()).clone())
                        .collect(),
                    services: address
                        .services
                        .iter()
                        .map(|s| service_map.get(s).unwrap_or(&s.to_string()).clone())
                        .collect(),
                };
            }
            AddressRef::default()
        })
        .collect();
    result.sort_by(|a, b| b.quantity.cmp(&a.quantity));
    result
}

#[get("/api/address/{address}/")] // TODO: Chain select?
#[openapi(description = "Test description?")]
async fn address(
    #[data] db: DatabaseConnection,
    address: String,
) -> Result<Json<Address>, Rejection> {
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
                map_addresses(
                    &db,
                    &mut address_list,
                    &mut address_map,
                    &mut tag_list,
                    &mut service_list,
                )
                .await;
                map_tags(&db, &tag_list, &mut tag_map).await;
                map_services(&db, &service_list, &mut service_map).await;

                // Transfer to json output
                let address_detail = address_map.get(&address_id).unwrap();
                Ok(Address {
                    hex: address.clone(),
                    human: address_detail.title.clone(),
                    inputs: address_ref(&address_map, &tag_map, &service_map, inputs),
                    outputs: address_ref(&address_map, &tag_map, &service_map, outputs),
                    mixed_in: address_ref(&address_map, &tag_map, &service_map, mixed_in),
                    mixed_out: address_ref(&address_map, &tag_map, &service_map, mixed_out),
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
    let (spec, filter) = openapi::spec().build(|| index(db.clone()).or(address(db.clone())));

    serve(filter.or(openapi_docs(spec))).run(bind.clone()).await;
}
