use seed::{prelude::*, *};

pub async fn create(address: shared::Address) -> fetch::Result<shared::Address> {
    Request::new("/api/address/")
        .method(Method::Post)
        .json(&address)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn list_by_address(address: String) -> fetch::Result<Vec<shared::Address>> {
    Request::new(format!("/api/address/by_address/{}", address))
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn list_by_tag(tag: i32) -> fetch::Result<Vec<shared::Address>> {
    Request::new(format!("/api/address/by_tag_id/{}", tag))
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn list_by_service(service: i32) -> fetch::Result<Vec<shared::Address>> {
    Request::new(format!("/api/address/by_service_id/{}", service))
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn detail(id: i64) -> fetch::Result<shared::Address> {
    Request::new(format!("/api/address/{}", id))
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn save(address: shared::Address) -> fetch::Result<shared::Address> {
    Request::new(format!("/api/address/{}", address.id.unwrap()))
        .method(Method::Post)
        .json(&address)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn delete(id: i64) -> fetch::Result<i64> {
    Request::new(format!("/api/address/{}", id))
        .method(Method::Delete)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await?;

    Ok(id)
}
