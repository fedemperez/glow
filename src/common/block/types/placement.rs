use std::collections::BTreeMap;

use nalgebra::Vector3;

use crate::common::block::states::get_state;
use crate::common::block::{Block, BlockFace};
use super::BlockClass;
use super::BlockType;

impl BlockType {
    pub fn place(&self, face: BlockFace, dir: Vector3<f64>) -> Block {
        let state = match self.class {
            BlockClass::PillarBlock => {
                let axis = match face {
                    BlockFace::NegX | BlockFace::PosX
                        => "x".to_string(),
                    BlockFace::NegY | BlockFace::PosY 
                        => "y".to_string(),
                    BlockFace::NegZ | BlockFace::PosZ
                        => "z".to_string(),
                };
                let mut props = BTreeMap::new();
                props.insert("axis".to_string(), axis);
                get_state(&self.name, &props).unwrap()
            }
            _ => self.default_state,
        };
        Block(state)
    }
}
