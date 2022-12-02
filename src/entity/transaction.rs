//! `SeaORM` Entity. Generated by sea-orm-codegen 0.10.4

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "transaction")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub chain: i32,
    pub hash: Vec<u8>,
    pub amount: Option<i64>,
    pub from: Vec<i64>,
    pub to: Vec<i64>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::chain::Entity",
        from = "Column::Chain",
        to = "super::chain::Column::Id",
        on_update = "NoAction",
        on_delete = "Cascade"
    )]
    Chain,
}

impl Related<super::chain::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Chain.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
