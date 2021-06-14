use std::collections::{BTreeMap, HashMap};
use bimap::BiHashMap;
use lazy_static::lazy_static;
use serde::Deserialize;

const STATES_JSON: &str = include_str!("states.json");

struct Maps {
    state: BiHashMap<BlockData, u16>,
    defaults: HashMap<String, BTreeMap<String, String>>,
}

lazy_static! {
    static ref MAPS: Maps = gen_maps(STATES_JSON);
}

pub fn get_state(name: &str, props: &BTreeMap<String, String>) 
    -> Option<u16> 
{
    let block = BlockData {
        name: name.to_string(), 
        props: props.clone(),
    };
    MAPS.state.get_by_left(&block).cloned()
}

pub fn get_name_props(state: u16) 
    -> Option<(String, BTreeMap<String, String>)> 
{
    MAPS.state.get_by_right(&state).cloned()
        .map(|block| (block.name, block.props))
}

pub fn get_defaults(name: &str) 
    -> Option<&'static BTreeMap<String, String>> 
{
    MAPS.defaults.get(&name.to_string())
}

fn gen_maps(json: &str) -> Maps {
    let mut state_map = BiHashMap::new();
    let mut defaults = HashMap::new();
    let json: HashMap<String, BlockJson> = serde_json::from_str(json).unwrap();
    for (name, block) in json {
        for state in block.states {
            let block_data = BlockData {
                name: name.clone(),
                props: state.properties,
            };
            if state.default {
                defaults.insert(name.clone(), block_data.props.clone());
            }
            state_map.insert(block_data, state.id);
        }
    }
    Maps {
        state: state_map, defaults,
    }
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
