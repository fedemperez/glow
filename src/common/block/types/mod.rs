mod placement;
mod classes;

use lazy_static::lazy_static;
use std::collections::HashMap;
use serde::Deserialize;

use classes::BlockClass;

const BLOCKS_JSON: &str = include_str!("blocks.json");

lazy_static! {
    static ref FROM_ID_MAP: HashMap<u16, BlockType> = gen_from_id_map(BLOCKS_JSON);
}

fn gen_from_id_map(json: &str) -> HashMap<u16, BlockType> {
    serde_json::from_str::<HashMap<String, BlockType>>(json).unwrap()
        .into_iter()
        .map(|(name, mut btype)| {
            btype.name = name;
            (btype.id, btype)
        })
        .collect()
}

#[derive(Deserialize)]
pub struct BlockType {
    id: u16,
    #[serde(skip)] 
    name: String,
    item: u16,
    class: BlockClass,
    default_state: u16,
}

impl BlockType {
    pub fn from_id(id: u16) -> Option<&'static Self> {
        FROM_ID_MAP.get(&id)
    }

    pub fn get_default(&self) -> u16 {
        self.default_state
    }
}
