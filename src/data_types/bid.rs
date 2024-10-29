use crate::data_types::Parse;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Bid {
    pub price: Option<f64>,
    pub size: Option<HashMap<String, f64>>,
}

impl Parse for Bid {
    fn parse(map: &mut serde_json::Map<String, serde_json::Value>) -> Self {
        let price = map.get("price").and_then(|v| v.as_f64());
        let size_objects = map.get("size").and_then(|v| v.as_object());
        let size = match size_objects {
            Some(size_object) => {
                let mut bid_hash_map = HashMap::new(); 
                for key in size_object.keys(){ 
                    match size_object.get(key).and_then(|v| v.as_f64()){ 
                        Some(value) => {bid_hash_map.insert(key.clone(),value);},
                        None => continue,
                    }
                } 
                Some(bid_hash_map)},
            None => None,
        };
        Bid { price, size }
    }
}