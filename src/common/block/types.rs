use anyhow::{anyhow, Result};
use std::collections::{BTreeMap, HashMap};

use super::{
    Block, 
    BlockClass,
    maps::{BLOCK_TYPES, NAME_TO_TYPE}};

pub struct BlockType {
    pub id: u16,
    pub name: String,
    pub item: u16,
    pub class: BlockClass,
    pub default_state: BTreeMap<String, String>,
    pub states: HashMap<BTreeMap<String, String>, u16>,
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
        NAME_TO_TYPE.get(name).map(|x| *x)
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
