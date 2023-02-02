use crate::model;
use seed::{prelude::*, *};

pub async fn relations(address: String) -> fetch::Result<model::AddressRelation> {
    Request::new(format!("/api/analysis/address/{}", address))
        .method(Method::Get)
        .fetch()
        .await?
        .check_status()?
        .json()
        .await
}
