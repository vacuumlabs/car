use seed::{prelude::*, *};

pub async fn relations(address: String) -> fetch::Result<shared::AddressRelation> {
    Request::new(format!("/api/analysis/address/{}", address))
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}
