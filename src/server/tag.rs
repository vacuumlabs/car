use crate::entity::tag;
use crate::server::Tag;
use rweb::*;
use sea_orm::{ActiveModelTrait, ActiveValue, DatabaseConnection, EntityTrait, ModelTrait};

#[post("/api/tag/")] // Create address endpoint
#[openapi(description = "Read address record")]
pub async fn create(
    #[data] db: DatabaseConnection,
    body: Json<Tag>,
) -> Result<Json<Tag>, Rejection> {
    let body = body.into_inner();

    let value = tag::ActiveModel {
        id: ActiveValue::NotSet,
        title: ActiveValue::Set(body.title.clone()),
    }
    .insert(&db)
    .await;

    match value {
        Ok(new) => Ok(Tag {
            id: new.id,
            title: new.title,
        }
        .into()),
        _ => Err(reject::not_found()),
    }
}

#[get("/api/tag/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
pub async fn detail(#[data] db: DatabaseConnection, id: i32) -> Result<Json<Tag>, Rejection> {
    match tag::Entity::find_by_id(id).one(&db).await {
        Ok(Some(value)) => Ok(Tag {
            id: value.id,
            title: value.title.clone(),
        }
        .into()),
        _ => Err(reject::not_found()),
    }
}

#[get("/api/tag/")] // Create address endpoint
#[openapi(description = "Read address record")]
pub async fn list(#[data] db: DatabaseConnection) -> Result<Json<Vec<Tag>>, Rejection> {
    match tag::Entity::find().all(&db).await {
        Ok(chain_list) => Ok(chain_list
            .iter()
            .map(|t| Tag {
                id: t.id,
                title: t.title.clone(),
            })
            .collect::<Vec<Tag>>()
            .into()),
        _ => Err(reject::not_found()),
    }
}

#[post("/api/tag/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
pub async fn update(
    #[data] db: DatabaseConnection,
    body: Json<Tag>,
    id: i32,
) -> Result<Json<Tag>, Rejection> {
    let body = body.into_inner();

    match tag::Entity::find_by_id(id).one(&db).await {
        Ok(Some(value)) => {
            let mut value: tag::ActiveModel = value.into();

            value.title = ActiveValue::Set(body.title.clone());
            let value: tag::Model = value.update(&db).await.unwrap();

            Ok(Tag {
                title: value.title.clone(),
                id: value.id,
            }
            .into())
        }
        _ => Err(reject::not_found()),
    }
}

#[delete("/api/tag/{id}")] // Create address endpoint
#[openapi(description = "Read address record")]
pub async fn delete(#[data] db: DatabaseConnection, id: i32) -> Result<Json<()>, Rejection> {
    match tag::Entity::find_by_id(id).one(&db).await {
        Ok(Some(value)) => {
            value.delete(&db).await.unwrap();
            Ok(().into())
        }
        _ => Err(reject::not_found()),
    }
}
