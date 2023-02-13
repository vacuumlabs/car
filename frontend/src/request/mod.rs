use seed::{prelude::*, *};

pub mod address;
pub mod analysis;
pub mod chain;
pub mod service;
pub mod tag;

pub async fn token_check(token: String) -> fetch::Result<bool> {
    Request::new("/api/token")
        .method(Method::Get)
        .header(Header::bearer(token))
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}
