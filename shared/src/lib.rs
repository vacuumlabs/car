#[cfg(feature = "schema")]
use rweb::Schema;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg(feature = "schema")]
use strum_macros::EnumIter;

mod address;

pub use address::{Address, AddressRef, AddressRefHuman, AddressRelation, AddressRelationHuman};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(Schema))]
pub struct Chain {
    pub id: Option<i32>,
    pub title: String,
    pub params: ChainParam,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(Schema))]
pub struct Service {
    pub id: Option<i32>,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(Schema))]
pub struct Tag {
    pub id: Option<i32>,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(Schema))]
pub struct Transaction {
    pub id: Option<i32>,
    pub chain: i32,
    pub hash: Vec<u8>,
    pub amount: i64,
    pub from: Vec<i64>,
    pub to: Vec<i64>,
}

/// Only for internal look up
#[derive(Debug)]
pub struct PrivAddress {
    pub title: String,
    pub chain: i32,
    pub hash: Vec<u8>,
    pub tags: Vec<i32>,
    pub services: Vec<i32>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(Schema, EnumIter))]
pub enum ChainParam {
    #[default]
    None,
    EtherScan(AnyScan),
    PolyScan(AnyScan),
    ArbiScan(AnyScan),
    Cardano(Cardano),
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(Schema))]
pub struct AnyScan {
    pub base_url: String,
    pub token: String,
    pub last: u64,
    pub delay: u64,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "schema", derive(Schema))]
pub struct Cardano {
    pub address: String,
    pub block_hash: String,
    pub slot: u64,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct StoredList {
    pub id: Uuid,
    pub description: String,
    pub addresses: Vec<i64>,
}
