mod face;
mod id;

use std::collections::BTreeMap;

pub use face::BlockFace;

use self::id::{get_state, get_name_props};

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
pub struct Block(pub u16);

impl Block {
    pub fn from_props(name: &str, props: &BTreeMap<String, String>) 
        -> Option<Self> 
    {
        get_state(name, props)
            .map(|state| Self(state))
    }

    pub fn get_props(&self) -> (String, BTreeMap<String, String>) {
        get_name_props(self.0).unwrap()
    }
}