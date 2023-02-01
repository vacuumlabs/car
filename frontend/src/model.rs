use serde::{Deserialize, Serialize};

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
    pub id: Option<i32>,
    pub title: Option<String>,
    pub hash: String,
    pub chain: i32,
    pub services: Vec<i32>,
    pub tags: Vec<i32>,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct StoredList {
    pub id: String,
    pub description: String,
    pub addresses: Vec<i64>,
}
