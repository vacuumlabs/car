use seed::{prelude::*, *};

pub async fn create(service: shared::Service) -> fetch::Result<shared::Service> {
    Request::new("/api/service/")
        .method(Method::Post)
        .json(&service)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn list() -> fetch::Result<Vec<shared::Service>> {
    Request::new("/api/service")
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn detail(id: i32) -> fetch::Result<shared::Service> {
    Request::new(format!("/api/service/{}", id))
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn save(service: shared::Service) -> fetch::Result<shared::Service> {
    Request::new(format!("/api/service/{}", service.id.unwrap()))
        .method(Method::Post)
        .json(&service)?
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}

pub async fn delete(id: i32) -> fetch::Result<i32> {
    Request::new(format!("/api/service/{}", id))
        .method(Method::Delete)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await?;

    Ok(id)
}
