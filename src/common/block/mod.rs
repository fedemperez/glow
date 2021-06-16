mod face;
mod map;
mod types;

use std::collections::BTreeMap;

pub use face::BlockFace;

use self::map::{get_block};
pub use types::BlockType;

pub struct Block {
    pub btype: &'static BlockType,
    pub props: BTreeMap<String, String>,
    pub id: u16,
}

impl Block {
    pub fn from_state_id(state: u16) -> Option<&'static Self> {
        get_block(state)
    }
}
