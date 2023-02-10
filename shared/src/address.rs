#[cfg(feature = "schema")]
use rweb::Schema;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(Schema))]
pub struct AddressRelationHuman {
    pub id: i64,
    pub hex: String,
    pub human: String,
    pub inputs: Vec<AddressRefHuman>,
    pub outputs: Vec<AddressRefHuman>,
    pub mixed_in: Vec<AddressRefHuman>,
    pub mixed_out: Vec<AddressRefHuman>,
    pub tags: Vec<String>,
    pub services: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(Schema))]
pub struct AddressRelation {
    pub id: i64,
    pub hex: String,
    pub human: String,
    pub inputs: Vec<AddressRef>,
    pub outputs: Vec<AddressRef>,
    pub mixed_in: Vec<AddressRef>,
    pub mixed_out: Vec<AddressRef>,
    pub tags: Vec<i32>,
    pub services: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(Schema))]
pub struct Address {
    pub id: Option<i64>,
    pub hash: String,
    pub title: Option<String>,
    pub chain: i32,
    pub services: Vec<i32>,
    pub tags: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(Schema))]
pub struct AddressRef {
    pub id: i64,
    pub hex: String,
    pub human: String,
    pub quantity: i32,
    pub tags: Vec<i32>,
    pub services: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "schema", derive(Schema))]
pub struct AddressRefHuman {
    pub id: i64,
    pub hex: String,
    pub human: String,
    pub quantity: i32,
    pub tags: Vec<String>,
    pub services: Vec<String>,
}
