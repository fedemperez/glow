use std::collections::BTreeMap;

use nalgebra::Vector3;

use crate::common::block::states::get_state;
use crate::common::block::{Block, BlockFace};
use super::BlockClass;
use super::BlockType;

impl BlockType {
    pub fn place(&self, face: BlockFace, cursor: Vector3<f32>, angle: (f32, f32)) 
        -> Block 
    {
        let state = match self.class {
            BlockClass::PillarBlock => {
                let axis = axis_from_face(&face);
                let mut props = BTreeMap::new();
                props.insert("axis".to_string(), axis);
                get_state(&self.name, &props).unwrap()
            },
            BlockClass::StairsBlock => {
                let mut props = BTreeMap::new();
                props.insert("facing".into(), facing_from_yaw(angle.0));
                props.insert("half".into(), calc_half(&face, &cursor));
                props.insert("shape".into(), "straight".into());
                props.insert("waterlogged".into(), "false".into());
                get_state(&self.name, &props).unwrap()
            },
            BlockClass::SlabBlock => {
                let mut props = BTreeMap::new();
                props.insert("type".into(), calc_half(&face, &cursor));
                props.insert("waterlogged".into(), "false".into());
                get_state(&self.name, &props).unwrap()
            },
            _ => self.default_state,
        };
        Block(state)
    }
}

fn axis_from_face(face: &BlockFace) -> String {
    match face {
        BlockFace::NegX | BlockFace::PosX
            => "x".to_string(),
        BlockFace::NegY | BlockFace::PosY 
            => "y".to_string(),
        BlockFace::NegZ | BlockFace::PosZ
            => "z".to_string(),
    }
}

fn facing_from_yaw(yaw: f32) -> String {
    match (yaw + 45.0).rem_euclid(360.0) {
        x if x < 90.0 => "south",
        x if x < 180.0 => "west",
        x if x < 270.0 => "north",
        _ => "east",
    }.into()
}

fn calc_half(face: &BlockFace, cursor: &Vector3<f32>) -> String {
    match face {
        BlockFace::PosY => "bottom",
        BlockFace::NegY => "top",
        _ => if cursor.y < 0.5 { 
            "bottom" 
        } else { 
            "top" 
        }
    }.into()
}
