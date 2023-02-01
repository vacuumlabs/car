use crate::entity::service;
use crate::server::Service;
use rweb::*;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, ModelTrait};

#[post("/api/service/")] // Create address endpoint
#[openapi(description = "Read address record")]
pub async fn create(
    #[data] db: DatabaseConnection,
    body: Json<Service>,
) -> Result<Json<Service>, Rejection> {
    let body = body.into_inner();

    let value = service::ActiveModel {
        id: ActiveValue::NotSet,
        title: ActiveValue::Set(body.title.clone()),
    }
    .insert(&db)
    .await;

    match value {
        Ok(new) => Ok(Service {
            id: new.id,
            title: new.title,
        }
        .into()),
        _ => Err(reject::not_found()),
    }
}

#[get("/api/service/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
pub async fn detail(#[data] db: DatabaseConnection, id: i32) -> Result<Json<Service>, Rejection> {
    match service::Entity::find_by_id(id).one(&db).await {
        Ok(Some(value)) => Ok(Service {
            id: value.id,
            title: value.title.clone(),
        }
        .into()),
        _ => Err(reject::not_found()),
    }
}

#[get("/api/service/")] // Create address endpoint
#[openapi(description = "Read address record")]
pub async fn list(#[data] db: DatabaseConnection) -> Result<Json<Vec<Service>>, Rejection> {
    match service::Entity::find().all(&db).await {
        Ok(service_list) => Ok(service_list
            .iter()
            .map(|s| Service {
                id: s.id,
                title: s.title.clone(),
            })
            .collect::<Vec<Service>>()
            .into()),
        _ => Err(reject::not_found()),
    }
}

#[post("/api/service/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
pub async fn update(
    #[data] db: DatabaseConnection,
    body: Json<Service>,
    id: i32,
) -> Result<Json<Service>, Rejection> {
    let body = body.into_inner();

    match service::Entity::find_by_id(id).one(&db).await {
        Ok(Some(value)) => {
            let mut value: service::ActiveModel = value.into();

            value.title = ActiveValue::Set(body.title.clone());
            let value: service::Model = value.update(&db).await.unwrap();

            Ok(Service {
                title: value.title.clone(),
                id: value.id,
            }
            .into())
        }
        _ => Err(reject::not_found()),
    }
}

#[delete("/api/service/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
pub async fn delete(#[data] db: DatabaseConnection, id: i32) -> Result<Json<()>, Rejection> {
    match service::Entity::find_by_id(id).one(&db).await {
        Ok(Some(value)) => {
            value.delete(&db).await.unwrap();
            Ok(().into())
        }
        _ => Err(reject::not_found()),
    }
}
