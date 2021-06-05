mod face;
mod id;

pub use face::BlockFace;

use self::id::get_default_state;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Block(pub u16);

impl Block {
    pub fn from_name(name: &str) -> Option<Self> {
        get_default_state(name)
            .map(|state| Self(state))
    }
}