use crate::model;
use seed::{prelude::*, *};

pub async fn create(tag: model::Tag) -> fetch::Result<model::Tag> {
    Request::new("/api/tag/")
        .method(Method::Post)
        .json(&tag)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn list() -> fetch::Result<Vec<model::Tag>> {
    Request::new("/api/tag")
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn detail(id: i32) -> fetch::Result<model::Tag> {
    Request::new(format!("/api/tag/{}", id))
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn save(tag: model::Tag) -> fetch::Result<model::Tag> {
    Request::new(format!("/api/tag/{}", tag.id.unwrap()))
        .method(Method::Post)
        .json(&tag)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn delete(id: i32) -> fetch::Result<i32> {
    Request::new(format!("/api/tag/{}", id))
        .method(Method::Delete)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await?;

    Ok(id)
}
