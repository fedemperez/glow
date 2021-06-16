mod face;
mod maps;
mod types;

use std::collections::BTreeMap;

use maps::BLOCK_STATES;

pub use face::BlockFace;
pub use types::BlockType;

pub struct Block {
    pub btype: &'static BlockType,
    pub props: BTreeMap<String, String>,
    pub id: u16,
}

impl Block {
    pub fn from_state_id(state: u16) -> Option<&'static Self> {
        BLOCK_STATES.get(state as usize)
    }

    pub fn air() -> &'static Self {
        &BLOCK_STATES[0]
    }
}
