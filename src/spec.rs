//! TODO: how to sort struct fields with serde?
//! within this mod all the struct fields should be "sorted" statically to generate the correct
//! object hash, this is annoying but we have no way to find out how to do that with serde

// use std::collections::{BTreeMap, HashMap};
// use serde::{Serialize, Serializer};
use serde_json::Value;

// this is used to sort a HashMap struct
// #[allow(dead_code)]
// fn ordered_map<S>(value: &HashMap<String, String>, serializer: S) -> Result<S::Ok, S::Error>
// where
//     S: Serializer,
// {
//     let ordered: BTreeMap<_, _> = value.iter().collect();
//     ordered.serialize(serializer)
// }
// #[serde(serialize_with = "ordered_map")]

#[derive(Debug, Serialize, Deserialize)]
pub struct Authentifiers {
    pub r: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Authors {
    pub address: String,
    pub authentifiers: Authentifiers,
    pub definition: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Inputs {
    pub message_index: u64,
    pub output_index: u64,
    pub unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Messages {
    pub app: String,
    pub payload: Payload,
    pub payload_hash: String,
    pub payload_location: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Outputs {
    pub address: String,
    pub amount: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Payload {
    pub inputs: Vec<Inputs>,
    pub outputs: Vec<Outputs>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ball {
    // TODO: need a real definition
    pub unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Unit {
    pub alt: String,
    pub authors: Vec<Authors>,
    pub content_hash: Option<String>, // this may not exist
    pub headers_commission: u64,
    pub last_ball: String,
    pub last_ball_unit: String,
    pub messages: Vec<Messages>,
    pub parent_units: Vec<String>,
    pub payload_commission: u64,
    pub unit: Option<String>, // this may not exist
    pub version: String,
    pub witness_list_unit: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Joint {
    pub ball: Option<Ball>,
    pub unit: Unit,
}
