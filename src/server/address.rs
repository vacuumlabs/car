use crate::entity::address;
use crate::server::Address;
use rweb::*;
use sea_orm::{
    ActiveModelTrait, ActiveValue, Condition, DatabaseConnection, DbBackend, EntityTrait,
    ModelTrait, Statement,
};

/// Create address endpoint
#[post("/api/address/")]
#[openapi(description = "Create address record")]
pub async fn create(
    #[data] db: DatabaseConnection,
    body: Json<Address>,
) -> Result<Json<Address>, Rejection> {
    let body = body.into_inner();

    let value = address::ActiveModel {
        title: ActiveValue::Set(body.title.clone()),
        chain: ActiveValue::Set(body.chain),
        services: ActiveValue::Set(body.services.clone()),
        tags: ActiveValue::Set(body.tags.clone()),
        hash: ActiveValue::Set(hex::decode(body.hash).unwrap()),
        ..Default::default()
    }
    .insert(&db)
    .await;

    match value {
        Ok(new) => Ok(Address {
            id: Some(new.id),
            title: new.title,
            chain: new.chain.clone(),
            hash: hex::encode(new.hash),
            services: body.services.clone(),
            tags: body.tags.clone(),
        }
        .into()),
        _ => Err(reject::not_found()),
    }
}

/// Get address endpoint
#[get("/api/address/{id}")]
#[openapi(description = "Read address record")]
pub async fn detail(
    #[data] db: DatabaseConnection,
    id: String,
) -> Result<Json<Address>, Rejection> {
    match address::Entity::find_by_id(id.parse::<i64>().unwrap())
        .one(&db)
        .await
    {
        Ok(Some(a)) => {
            tracing::info!("output: {:?}", a);
            Ok(Address {
                id: Some(a.id),
                title: a.title.clone(),
                hash: hex::encode(&a.hash),
                services: a
                    .services
                    .iter()
                    //.flatten()
                    .map(|s| s.clone())
                    .collect::<Vec<i32>>(),
                tags: a
                    .tags
                    .iter()
                    //.flatten()
                    .map(|s| s.clone())
                    .collect::<Vec<i32>>(),
                chain: a.chain.clone(),
            }
            .into())
        }
        _ => Err(reject::not_found()),
    }
}

pub fn address_list_query(list: Vec<address::Model>) -> Vec<Address> {
    list.iter()
        .map(|a| Address {
            id: Some(a.id),
            title: a.title.clone(),
            hash: hex::encode(&a.hash),
            services: a
                .services
                .iter()
                //.flatten()
                .map(|s| s.clone())
                .collect::<Vec<i32>>(),
            tags: a
                .tags
                .iter()
                //.flatten()
                .map(|s| s.clone())
                .collect::<Vec<i32>>(),
            chain: a.chain.clone(),
        })
        .collect::<Vec<Address>>()
}

/// Get address list by tag id endpoint
#[get("/api/address/by_address/{address}")]
#[openapi(description = "Read address list by address")]
pub async fn list_by_address(
    #[data] db: DatabaseConnection,
    address: String,
) -> Result<Json<Vec<Address>>, Rejection> {
    tracing::info!("By address");

    let mut bin_address: Option<Vec<u8>> = None;
    let mut address = address.to_lowercase();

    if address.starts_with("0x") {
        address = String::from(&address[2..]);
    } else {
        tracing::info!("not match 0x");
    }

    if let Ok(bytes) = hex::decode(address) {
        bin_address = Some(bytes);
    } else {
        tracing::info!("Address is not convertable");
    }

    if let Some(address) = bin_address {
        match address::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"SELECT * from address WHERE hash = $1;"#,
                vec![address.into()],
            ))
            .all(&db)
            .await
        {
            Ok(list) => return Ok(address_list_query(list).into()),
            _ => {
                tracing::error!("Failed2");
                return Err(reject::not_found());
            }
        }
    } else {
        tracing::error!("Failed1");
    }
    tracing::error!("Failed3");
    Err(reject::not_found())
}

/// Get address list by tag id endpoint
#[get("/api/address/by_tag_id/{id}")]
#[openapi(description = "Read address list by tag id")]
pub async fn list_by_tag(
    #[data] db: DatabaseConnection,
    id: i32,
) -> Result<Json<Vec<Address>>, Rejection> {
    match address::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"SELECT * from address WHERE tags @> ARRAY[$1];"#,
            vec![id.into()],
        ))
        .all(&db)
        .await
    {
        Ok(list) => Ok(address_list_query(list).into()),
        _ => Err(reject::not_found()),
    }
}

/// Get address list by service id endpoint
#[get("/api/address/by_service_id/{id}")]
#[openapi(description = "Read address list by tag id")]
pub async fn list_by_service(
    #[data] db: DatabaseConnection,
    id: i32,
) -> Result<Json<Vec<Address>>, Rejection> {
    match address::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"SELECT * from address WHERE services @> ARRAY[$1];"#,
            vec![id.into()],
        ))
        .all(&db)
        .await
    {
        Ok(list) => Ok(address_list_query(list).into()),
        _ => Err(reject::not_found()),
    }
}

/// Get address list by tag id endpoint
#[get("/api/address/by_transaction_id/{id}")]
#[openapi(description = "Read address transaction id")]
pub async fn list_by_transaction(
    #[data] db: DatabaseConnection,
    id: i32,
) -> Result<Json<Vec<Address>>, Rejection> {
    match address::Entity::find()
        .from_raw_sql(Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"SELECT * from address WHERE tags @> ARRAY[$1];"#,
            vec![id.into()],
        ))
        .all(&db)
        .await
    {
        Ok(list) => Ok(address_list_query(list).into()),
        _ => Err(reject::not_found()),
    }
}

#[post("/api/address/{id}")] // Create address endpoint
#[openapi(description = "Update address record")]
pub async fn update(
    #[data] db: DatabaseConnection,
    id: i64,
    body: Json<Address>,
) -> Result<Json<Address>, Rejection> {
    let body = body.into_inner();

    match address::Entity::find_by_id(id).one(&db).await {
        Ok(Some(value)) => {
            let mut value: address::ActiveModel = value.into();

            value.title = ActiveValue::Set(body.title.clone());
            value.tags = ActiveValue::Set(body.tags.clone());
            value.services = ActiveValue::Set(body.services.clone());
            let value: address::Model = value.update(&db).await.unwrap();

            Ok(Address {
                id: Some(value.id),
                title: value.title.clone(),
                chain: value.chain,
                hash: hex::encode(&value.hash),
                tags: value.tags.clone(),
                services: value.services.clone(),
            }
            .into())
        }
        _ => Err(reject::not_found()),
    }
}

#[delete("/api/address/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
pub async fn delete(
    #[data] db: DatabaseConnection,
    id: String,
    body: Json<Address>,
) -> Result<Json<Address>, Rejection> {
    let address = Address {
        id: Some(1),
        title: Some(String::from("test")),
        hash: hex::encode(vec![0, 0]),
        services: vec![1],
        tags: vec![1],
        chain: 1,
    };
    Ok(address.into())
}
