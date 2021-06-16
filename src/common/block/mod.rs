mod face;
mod maps;
mod types;
mod material;

use std::collections::BTreeMap;

use maps::BLOCK_STATES;
use material::BlockMaterial;

pub use face::BlockFace;
pub use types::BlockType;

pub struct Block {
    pub btype: &'static BlockType,
    pub props: BTreeMap<String, String>,
    pub material: &'static BlockMaterial,
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
