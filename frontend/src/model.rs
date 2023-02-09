use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct Chain {
    pub id: Option<i32>,
    pub title: String,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct Tag {
    pub id: Option<i32>,
    pub title: String,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct Service {
    pub id: Option<i32>,
    pub title: String,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct Address {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub hash: String,
    pub chain: i32,
    pub services: Vec<i32>,
    pub tags: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
pub struct AddressRef {
    pub id: i64,
    pub hex: String,
    pub human: String,
    pub quantity: i32,
    pub tags: Vec<i32>,
    pub services: Vec<i32>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct StoredList {
    pub id: Uuid,
    pub description: String,
    pub addresses: Vec<i64>,
}
