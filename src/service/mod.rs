use sea_orm::{prelude::*, DatabaseConnection, TryGetError, TryGetable};
use sea_query::value::with_array::NotU8;
use serde::{Deserialize, Serialize};
pub mod common;
pub mod wingriders;

#[derive(Debug, Clone, Eq, PartialEq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum Service {
    WingRiders = 1,
}

impl NotU8 for Service {}
//impl TryGetableFromJson for Service {}

pub async fn init_services(db: &DatabaseConnection) -> Vec<Box<dyn common::Service>> {
    vec![Box::new(wingriders::WingRiders::init(db, 1).await)]
}
