use sea_orm::entity::prelude::*;
use sea_query::value::with_array::NotU8;
use serde::{Deserialize, Serialize};

// https://bitpay.com/blog/who-accepts-ethereum/

#[derive(Debug, Clone, Eq, PartialEq, EnumIter, DeriveActiveEnum, Deserialize, Serialize)]
#[sea_orm(rs_type = "i32", db_type = "Integer")]
pub enum Tag {
    Unknown = 0,

    // finance
    Finance = 1,
    Dex = 301,
    Exchange = 302,
    Atm = 303,

    // Activity
    Game = 100,
    Eshop = 101,
    Gamble = 102,
    Bet = 103,
    Travel = 104,
    Sport = 105,
    Entertainment = 106,
    Trade = 107,

    // Subject
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

    // Tags for dex
    Pool = 1001,
    Address = 1002,
    Worker = 1003,
    Order = 1004,
}

impl NotU8 for Tag {}
