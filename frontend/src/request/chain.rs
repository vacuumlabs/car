use seed::{prelude::*, *};

pub async fn create(chain: shared::Chain) -> fetch::Result<shared::Chain> {
    Request::new("/api/chain/")
        .method(Method::Post)
        .json(&chain)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn list() -> fetch::Result<Vec<shared::Chain>> {
    Request::new("/api/chain")
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn detail(id: i32) -> fetch::Result<shared::Chain> {
    Request::new(format!("/api/chain/{}", id))
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn save(chain: shared::Chain) -> fetch::Result<shared::Chain> {
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
