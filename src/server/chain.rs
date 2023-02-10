use crate::entity::chain;
use rweb::*;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, ModelTrait};

#[post("/api/chain/")]
#[openapi(description = "Create chain record")]
pub async fn create(
    #[data] db: DatabaseConnection,
    body: Json<shared::Chain>,
) -> Result<Json<shared::Chain>, Rejection> {
    let body = body.into_inner();

    let value = chain::ActiveModel {
        id: ActiveValue::NotSet,
        title: ActiveValue::Set(body.title.clone()),
        params: ActiveValue::Set(
            serde_json::to_value(shared::ChainParam::PolyScan(shared::AnyScan::default())).unwrap(),
        ),
    }
    .insert(&db)
    .await;

    match value {
        Ok(new) => Ok(shared::Chain {
            id: Some(new.id),
            title: new.title,
            params: shared::ChainParam::default(),
        }
        .into()),
        _ => Err(reject::not_found()),
    }
}

#[get("/api/chain/{id}")] // Create chain endpoint
#[openapi(description = "Read chain record")]
pub async fn detail(
    #[data] db: DatabaseConnection,
    id: i32,
) -> Result<Json<shared::Chain>, Rejection> {
    match chain::Entity::find_by_id(id).one(&db).await {
        Ok(Some(value)) => Ok(shared::Chain {
            id: Some(value.id),
            title: value.title.clone(),
            params: serde_json::from_value(value.params).unwrap(),
        }
        .into()),
        _ => Err(reject::not_found()),
    }
}

#[get("/api/chain/")]
#[openapi(description = "Read chain record list")]
pub async fn list(#[data] db: DatabaseConnection) -> Result<Json<Vec<shared::Chain>>, Rejection> {
    match chain::Entity::find().all(&db).await {
        Ok(chain_list) => Ok(chain_list
            .iter()
            .map(|ch| shared::Chain {
                id: Some(ch.id),
                title: ch.title.clone(),
                params: serde_json::from_value(ch.params.clone()).unwrap(),
            })
            .collect::<Vec<shared::Chain>>()
            .into()),
        _ => Err(reject::not_found()),
    }
}

#[post("/api/chain/{id}")]
#[openapi(description = "Update chain record")]
pub async fn update(
    #[data] db: DatabaseConnection,
    body: Json<shared::Chain>,
    id: i32,
) -> Result<Json<shared::Chain>, Rejection> {
    let body = body.into_inner();

    match chain::Entity::find_by_id(id).one(&db).await {
        Ok(Some(value)) => {
            let mut value: chain::ActiveModel = value.into();

            value.title = ActiveValue::Set(body.title.clone());
            let value: chain::Model = value.update(&db).await.unwrap();

            Ok(shared::Chain {
                title: value.title.clone(),
                id: Some(value.id),
                params: serde_json::from_value(value.params).unwrap(),
            }
            .into())
        }
        _ => Err(reject::not_found()),
    }
}

#[delete("/api/chain/{id}")]
#[openapi(description = "Remove chain record record")]
pub async fn delete(#[data] db: DatabaseConnection, id: i32) -> Result<Json<()>, Rejection> {
    match chain::Entity::find_by_id(id).one(&db).await {
        Ok(Some(value)) => {
            value.delete(&db).await.unwrap();
            Ok(().into())
        }
        _ => Err(reject::not_found()),
    }
}
