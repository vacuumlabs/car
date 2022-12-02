use sea_orm::prelude::*;

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum Chain {
    #[sea_orm(num_value = 1)]
    Cardano,
    #[sea_orm(num_value = 2)]
    Ethereum,
    #[sea_orm(num_value = 3)]
    Polygon,
}
