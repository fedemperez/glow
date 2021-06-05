use anyhow::{Result, anyhow};
use serde_json::Value;
use std::collections::HashMap;
use lazy_static::lazy_static;

const BLOCKS_JSON: &str = include_str!("blocks.json");

lazy_static! {
    static ref BLOCK_ID_MAP: HashMap<String, u16> = gen_block_id_map();
}

pub fn get_default_state(name: &str) -> Option<u16> {
    BLOCK_ID_MAP.get(name).map(|v| *v)
}

fn gen_block_id_map() -> HashMap<String, u16> {
    let mut result = HashMap::new();
    let json: HashMap<String, HashMap<String, Value>> = 
        serde_json::from_str(&BLOCKS_JSON).unwrap();
    for (name, block) in json {
        let states = block["states"].as_array().unwrap();
        for state in states {
            if let Some(Value::Bool(true)) = state.get("default") {
                let id = state["id"].as_u64().unwrap() as u16;
                result.insert(name, id);
                break;
            }
        }
    }
    result
}