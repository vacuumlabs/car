use crate::model;
use seed::{prelude::*, *};

pub async fn create(chain: model::Chain) -> fetch::Result<model::Chain> {
    Request::new("/api/chain/")
        .method(Method::Post)
        .json(&chain)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn list() -> fetch::Result<Vec<model::Chain>> {
    Request::new("/api/chain")
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn detail(id: i32) -> fetch::Result<model::Chain> {
    Request::new(format!("/api/chain/{}", id))
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn save(chain: model::Chain) -> fetch::Result<model::Chain> {
    Request::new(format!("/api/chain/{}", chain.id.unwrap()))
        .method(Method::Post)
        .json(&chain)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn delete(id: i32) -> fetch::Result<i32> {
    Request::new(format!("/api/chain/{}", id))
        .method(Method::Delete)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await?;

    Ok(id)
}
