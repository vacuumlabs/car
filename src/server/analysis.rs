use crate::entity::address;
use crate::server::{transform, AddressRef, AddressRelation, AddressRelationHuman, PrivAddress};
use rweb::*;
use sea_orm::{
    ConnectionTrait, DatabaseConnection, DbBackend, EntityTrait, QueryResult, Statement,
};
use std::collections::{BTreeMap, BTreeSet};

fn process_query(
    query: Vec<QueryResult>,
    address_id: &i64,
    address_list: &mut BTreeSet<i64>,
    inputs: &mut BTreeMap<i64, i32>,
    outputs: &mut BTreeMap<i64, i32>,
    mixed_in: &mut BTreeMap<i64, i32>,
    mixed_out: &mut BTreeMap<i64, i32>,
) {
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
                mixed_out.insert(address, mixed_out.get(&address).unwrap_or(&0) + 1);
            }
        }
    }
}

#[get("/api/analysis/address/{address}")] // TODO: Chain select?
#[openapi(description = "Test description?")]
pub async fn relation(
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
                let mut inputs: BTreeMap<i64, i32> = BTreeMap::new();
                let mut outputs: BTreeMap<i64, i32> = BTreeMap::new();
                let mut mixed_in: BTreeMap<i64, i32> = BTreeMap::new();
                let mut mixed_out: BTreeMap<i64, i32> = BTreeMap::new();

                let address_id: i64 = result.try_get("", "id").unwrap();
                address_list.insert(address_id);

                let statement = Statement::from_sql_and_values(
                    DbBackend::Postgres,
                    r#"SELECT "from", "to" FROM transaction WHERE $1 = ANY("from") or $1 = ANY("to");"#,
                    vec![address_id.into()],
                );
                if let Ok(query) = db.query_all(statement).await {
                    process_query(
                        query,
                        &address_id,
                        &mut address_list,
                        &mut inputs,
                        &mut outputs,
                        &mut mixed_in,
                        &mut mixed_out,
                    );
                }

                let address_detail = address_map.get(&address_id).unwrap();
                Ok(AddressRelation {
                    hex: address.clone(),
                    human: address_detail.title.clone(),
                    inputs: transform::address(&address_map, inputs),
                    outputs: transform::address(&address_map, outputs),
                    mixed_in: transform::address(&address_map, mixed_in),
                    mixed_out: transform::address(&address_map, mixed_out),
                    tags: address_detail.tags.clone(),
                    services: address_detail.services.clone(),
                }
                .into())
            }
            _ => Err(warp::reject::not_found()),
        };
    }
    Err(warp::reject::not_found())
}

#[get("/api/analysis/address/human/{address}")] // TODO: Chain select?
#[openapi(description = "Test description?")]
pub async fn relation_human(
    #[data] db: DatabaseConnection,
    address: String,
) -> Result<Json<AddressRelationHuman>, Rejection> {
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
                    process_query(
                        query,
                        &address_id,
                        &mut address_list,
                        &mut inputs,
                        &mut outputs,
                        &mut mixed_in,
                        &mut mixed_out,
                    );
                }

                // Map DB resources
                transform::map_addresses_extended(
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
                Ok(AddressRelationHuman {
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
