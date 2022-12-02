use sea_orm::entity::prelude::*;
use sea_query::value::with_array::NotU8;
use serde::{Deserialize, Serialize};

// https://bitpay.com/blog/who-accepts-ethereum/

#[derive(Debug, Clone, Eq, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum Tag {
    Unknown = 0,

    Dex = 1,
    Exchange = 2,
    Atm = 3,

    Game = 100,
    Eshop = 101,
    Gamble = 102,
    Bet = 103,
    #[sea_orm(num_value = 104)]
    Travel,
    #[sea_orm(num_value = 105)]
    Sport,
    #[sea_orm(num_value = 106)]
    Entertainment,

    #[sea_orm(num_value = 201)]
    Drogs,
    #[sea_orm(num_value = 202)]
    Food,
    #[sea_orm(num_value = 203)]
    Information,
    #[sea_orm(num_value = 204)]
    Stream,
    #[sea_orm(num_value = 205)]
    Podcast,
    #[sea_orm(num_value = 206)]
    Video,
    #[sea_orm(num_value = 207)]
    Audio,
    #[sea_orm(num_value = 208)]
    Image,
}

impl NotU8 for Tag {}
