use std::collections::{BTreeMap, HashMap};
use bimap::BiHashMap;
use lazy_static::lazy_static;
use serde::Deserialize;

const STATES_JSON: &str = include_str!("states.json");

lazy_static! {
    static ref STATE_MAP: BiHashMap<BlockData, u16> = gen_state_map(STATES_JSON);
}

pub fn get_state(name: &str, props: &BTreeMap<String, String>) 
    -> Option<u16> 
{
    let block = BlockData {
        name: name.to_string(), 
        props: props.clone(),
    };
    STATE_MAP.get_by_left(&block).cloned()
}

pub fn get_name_props(state: u16) 
    -> Option<(String, BTreeMap<String, String>)> 
{
    STATE_MAP.get_by_right(&state).cloned()
        .map(|block| (block.name, block.props))
}

fn gen_state_map(json: &str) -> BiHashMap<BlockData, u16> {
    let mut result = BiHashMap::new();
    let json: HashMap<String, BlockJson> = serde_json::from_str(json).unwrap();
    for (name, block) in json {
        for state in block.states {
            let block_data = BlockData {
                name: name.clone(),
                props: state.properties,
            };
            result.insert(block_data, state.id);
        }
    }
    result
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct BlockData {
    name: String,
    props: BTreeMap<String, String>,
}

#[derive(Deserialize)]
struct BlockJson {
    states: Vec<BlockStateJson>,
}

#[derive(Deserialize)]
struct BlockStateJson {
    #[serde(default)]
    properties: BTreeMap<String, String>,
    id: u16,
    #[serde(default)]
    default: bool,
}
