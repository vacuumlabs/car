use crate::entity::chain;
use crate::server::Chain;
use rweb::*;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, ModelTrait};

#[post("/api/chain/")]
#[openapi(description = "Create chain record")]
pub async fn create(
    #[data] db: DatabaseConnection,
    body: Json<Chain>,
) -> Result<Json<Chain>, Rejection> {
    let body = body.into_inner();

    let value = chain::ActiveModel {
        id: ActiveValue::NotSet,
        title: ActiveValue::Set(body.title.clone()),
    }
    .insert(&db)
    .await;

    match value {
        Ok(new) => Ok(Chain {
            id: Some(new.id),
            title: new.title,
        }
        .into()),
        _ => Err(reject::not_found()),
    }
}

#[get("/api/chain/{id}")] // Create chain endpoint
#[openapi(description = "Read chain record")]
pub async fn detail(#[data] db: DatabaseConnection, id: i32) -> Result<Json<Chain>, Rejection> {
    match chain::Entity::find_by_id(id).one(&db).await {
        Ok(Some(value)) => Ok(Chain {
            id: Some(value.id),
            title: value.title.clone(),
        }
        .into()),
        _ => Err(reject::not_found()),
    }
}

#[get("/api/chain/")]
#[openapi(description = "Read chain record list")]
pub async fn list(#[data] db: DatabaseConnection) -> Result<Json<Vec<Chain>>, Rejection> {
    match chain::Entity::find().all(&db).await {
        Ok(chain_list) => Ok(chain_list
            .iter()
            .map(|ch| Chain {
                id: Some(ch.id),
                title: ch.title.clone(),
            })
            .collect::<Vec<Chain>>()
            .into()),
        _ => Err(reject::not_found()),
    }
}

#[post("/api/chain/{id}")]
#[openapi(description = "Update chain record")]
pub async fn update(
    #[data] db: DatabaseConnection,
    body: Json<Chain>,
    id: i32,
) -> Result<Json<Chain>, Rejection> {
    let body = body.into_inner();

    match chain::Entity::find_by_id(id).one(&db).await {
        Ok(Some(value)) => {
            let mut value: chain::ActiveModel = value.into();

            value.title = ActiveValue::Set(body.title.clone());
            let value: chain::Model = value.update(&db).await.unwrap();

            Ok(Chain {
                title: value.title.clone(),
                id: Some(value.id),
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
