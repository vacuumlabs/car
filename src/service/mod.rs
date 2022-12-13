use sea_orm::{prelude::*, DatabaseConnection};
use sea_query::value::with_array::NotU8;
use serde::{Deserialize, Serialize};
pub mod common;
pub mod dex;

#[derive(Debug, Clone, Eq, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum Service {
    WingRiders = 1,
}

impl NotU8 for Service {}
//impl TryGetableFromJson for Service {}

pub async fn init_services(db: &DatabaseConnection) -> Vec<Box<dyn common::Service>> {
    vec![
        Box::new(dex::WingRiders::init(db, 1).await),
        Box::new(dex::SundaeSwap::init(db, 2).await),
        Box::new(dex::MinSwap::init(db, 3).await),
    ]
}
