use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, Statement};
use std::collections::{BTreeMap, BTreeSet};

fn map_address_query(address_list: &BTreeSet<i64>) -> Statement {
    Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"SELECT id, chain, title, hash, tags, services FROM address WHERE id = ANY($1);"#,
        vec![address_list
            .iter()
            .map(|a| a.clone())
            .collect::<Vec<i64>>()
            .into()],
    )
}

pub async fn map_addresses(
    db: &DatabaseConnection,
    address_list: &mut BTreeSet<i64>,
    address_map: &mut BTreeMap<i64, shared::PrivAddress>,
) {
    let statement = map_address_query(address_list);

    if let Ok(query) = db.query_all(statement.clone()).await {
        for row in query.iter() {
            let id: i64 = row.try_get("", "id").unwrap();
            let hash = row.try_get("", "hash").unwrap();
            address_map.insert(
                id,
                shared::PrivAddress {
                    title: row.try_get("", "title").unwrap_or(String::new()),
                    chain: row.try_get("", "chain").unwrap(),
                    hash: hash,
                    tags: row.try_get("", "tags").unwrap_or(Vec::new()),
                    services: row.try_get("", "services").unwrap_or(Vec::new()),
                },
            );
        }
    }
}
/// Map adress ID to internal format
pub async fn map_addresses_extended(
    db: &DatabaseConnection,
    address_list: &mut BTreeSet<i64>,
    address_map: &mut BTreeMap<i64, shared::PrivAddress>,
    tag_list: &mut BTreeSet<i32>,
    service_list: &mut BTreeSet<i32>,
) {
    let statement = map_address_query(address_list);

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
                shared::PrivAddress {
                    title: row.try_get("", "title").unwrap_or(hex::encode(&hash)),
                    chain: row.try_get("", "chain").unwrap(),
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

pub fn address_ref_human(
    address_map: &BTreeMap<i64, shared::PrivAddress>,
    tag_map: &BTreeMap<i32, String>,
    service_map: &BTreeMap<i32, String>,
    addresses: BTreeMap<i64, i32>,
) -> Vec<shared::AddressRefHuman> {
    let mut result: Vec<shared::AddressRefHuman> = addresses
        .iter()
        .map(|(address_id, address_count)| {
            if let Some(address) = address_map.get(address_id) {
                return shared::AddressRefHuman {
                    id: address_id.clone(),
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
            shared::AddressRefHuman::default()
        })
        .collect();
    result.sort_by(|a, b| b.quantity.cmp(&a.quantity));
    result
}

pub fn address_ref(
    address_map: &BTreeMap<i64, shared::PrivAddress>,
    addresses: BTreeMap<i64, i32>,
) -> Vec<shared::AddressRef> {
    addresses
        .iter()
        .map(|(address_id, address_count)| {
            if let Some(address) = address_map.get(address_id) {
                return shared::AddressRef {
                    id: address_id.clone(),
                    hex: hex::encode(&address.hash),
                    human: address.title.clone(),
                    quantity: address_count.clone(),
                    tags: address.tags.clone(),
                    services: address.services.clone(),
                };
            }
            shared::AddressRef::default()
        })
        .collect()
}
