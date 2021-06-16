use lazy_static::lazy_static;
use serde_json::Value;
use serde::Deserialize;
use std::collections::{HashMap, BTreeMap};

use super::{Block, BlockType};
use super::types::BlockClass;

const BLOCKS_JSON: &str = include_str!("blocks.json");

lazy_static! {
    pub static ref BLOCK_TYPES: Vec<BlockType> = {
        let mut result: Vec<BlockType> = 
            serde_json::from_str::<HashMap<String, BlockTypeJson>>(BLOCKS_JSON).unwrap()
            .into_iter()
            .map(|(name, json)| {
                let default_state = json.states[&json.default_state]
                    .clone().into_props();
                let states = json.states.into_iter()
                    .map(|(id, state)| {
                        (state.into_props(), id)
                    }).collect();
                BlockType {
                    id: json.id,
                    name,
                    item: json.item,
                    class: json.class,
                    default_state,
                    states,
                }
            })
            .collect();
        result.sort_unstable_by_key(|a| a.id);
        result
    };

    pub static ref NAME_TO_TYPE: HashMap<String, &'static BlockType> = 
        BLOCK_TYPES.iter()
            .map(|btype| (btype.name.clone(), btype))
            .collect();

    pub static ref BLOCK_STATES: Vec<Block> = {
        let mut blocks: Vec<Block> = BLOCK_TYPES.iter()
            .map(|btype| 
                btype.states.iter()
                .map(move |(props, id)| Block {
                    id: *id,
                    props: props.clone(),
                    btype,
                }))
            .flatten()
            .collect();
        blocks.sort_unstable_by_key(|a| a.id);
        blocks
    };
}

#[derive(Deserialize)]
pub struct BlockTypeJson {
    id: u16,
    item: u16,
    class: BlockClass,
    default_state: u16,
    states: HashMap<u16, BlockStateJson>,
}

#[derive(Clone, Deserialize)]
pub struct BlockStateJson {
    #[serde(default)]
    properties: BTreeMap<String, Value>,
}

impl BlockStateJson {
    fn into_props(self) -> BTreeMap<String, String> {
        self.properties.into_iter().map(|(name, value)|
            (name, match value {
                Value::String(s) => s,
                v => v.to_string(),
            })
        ).collect()
    }
}
