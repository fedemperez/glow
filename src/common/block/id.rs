use std::collections::HashMap;
use bimap::BiHashMap;
use serde_json::Value;
use lazy_static::lazy_static;

const BLOCKS_JSON: &str = include_str!("blocks.json");

lazy_static! {
    static ref BLOCK_ID_MAP: BiHashMap<String, u16> = gen_block_id_map();
}

pub fn get_default_state(name: &str) -> Option<u16> {
    BLOCK_ID_MAP.get_by_left(name).cloned()
}

pub fn get_block_name(state: u16) -> Option<String> {
    BLOCK_ID_MAP.get_by_right(&state).cloned()
}

fn gen_block_id_map() -> BiHashMap<String, u16> {
    let mut result = BiHashMap::new();
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