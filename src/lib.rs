use serde_derive::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct Bundle {
    pub id: String,
    pub objects: Vec<Object>,
}

pub type Object = HashMap<String, serde_json::Value>;
