mod placement;
mod classes;

use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use serde_json::Value;
use std::collections::{BTreeMap, HashMap};
use serde::Deserialize;

use classes::BlockClass;

use super::Block;

const BLOCKS_JSON: &str = include_str!("blocks.json");

lazy_static! {
    static ref BLOCK_TYPES: Vec<BlockType> = {
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

    static ref NAME_TYPE_MAP: HashMap<String, &'static BlockType> = 
        BLOCK_TYPES.iter()
            .map(|btype| (btype.name.clone(), btype))
            .collect();
}

pub struct BlockType {
    pub id: u16,
    pub name: String,
    pub item: u16,
    pub class: BlockClass,
    pub default_state: BTreeMap<String, String>,
    states: HashMap<BTreeMap<String, String>, u16>,
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

impl PartialEq for BlockType {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for BlockType {}

impl BlockType {
    pub fn from_id(id: u16) -> Option<&'static Self> {
        BLOCK_TYPES.get(id as usize)
    }

    pub fn from_name(name: &str) -> Option<&'static Self> {
        NAME_TYPE_MAP.get(name).map(|x| *x)
    }

    pub fn with_props(&self, props: &BTreeMap<String, String>) 
        -> Result<&'static Block>
    {
        let state = self.states.get(&props).ok_or(
            anyhow!(format!("No block with id {} and properties {:?}", self.name, props))
        )?;
        Ok(Block::from_state_id(*state).unwrap())
    }
}
