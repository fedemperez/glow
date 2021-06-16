use std::collections::{BTreeMap, HashMap};
use lazy_static::lazy_static;
use serde::Deserialize;

use crate::common::block::BlockType;

use super::Block;

const STATES_JSON: &str = include_str!("states.json");

lazy_static! {
    static ref BLOCKS: Vec<Block> = {
        let mut result = vec![];
        let json: HashMap<String, BlockJson> = 
            serde_json::from_str(STATES_JSON).unwrap();
        for (name, block) in json {
            for state in block.states {
                result.push(Block {
                    btype: BlockType::from_name(&name).unwrap(),
                    props: state.properties,
                    id: state.id,
                });
            }
        }
        result.sort_unstable_by(|a, b| a.id.cmp(&b.id));
        result
    };
}

pub fn get_block(state: u16) -> Option<&'static Block> {
    BLOCKS.get(state as usize)
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
