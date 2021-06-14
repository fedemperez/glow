use std::collections::BTreeMap;

use block_macro::block_id;
use nalgebra::Vector3;

use crate::chunks::World;
use crate::common::block::states::{get_defaults, get_state};
use crate::common::block::{Block, BlockFace};
use super::BlockClass;
use super::BlockType;

impl BlockType {
    pub fn place(&self, pos: (i32, i32, i32), world: &World, face: BlockFace, 
        cursor: Vector3<f32>, angle: (f32, f32))
    {
        let (x, y, z) = pos;
        let mut props = self.auto_fill_props(pos, world, &face, cursor, angle);
        match self.class {
            BlockClass::StairsBlock => {
                *props.get_mut("facing").unwrap() = 
                    facing_from_angle(angle.0 + 180.0);
            },
            BlockClass::EndRodBlock => {
                *props.get_mut("facing").unwrap() = 
                    facing_from_face(&face);
            },
            BlockClass::LadderBlock => {
                match face {
                    BlockFace::PosY | BlockFace::NegY => return,
                    face => {
                        *props.get_mut("facing").unwrap() = 
                            facing_from_face(&face);
                    }
                }
            },
            BlockClass::SlabBlock => {
                *props.get_mut("type").unwrap() = calc_half(&face, &cursor);
            },
            BlockClass::DoorBlock => {
                if world.get_block(x, y + 1, z).0 != block_id!(air) {
                    return;
                }
                *props.get_mut("facing").unwrap() = 
                    facing_from_angle(angle.0 + 180.0);
                *props.get_mut("half").unwrap() = "upper".into();
                let block = Block(get_state(&self.name, &props).unwrap());
                world.set_block(x, y + 1, z, block);
                *props.get_mut("half").unwrap() = "lower".into();
            },
            _ => (),
        }
        let block = Block(get_state(&self.name, &props).unwrap());
        world.set_block(x, y, z, block);
    }

    fn auto_fill_props(&self, pos: (i32, i32, i32), world: &World, face: &BlockFace, 
        cursor: Vector3<f32>, angle: (f32, f32)) -> BTreeMap<String, String>
    {
        let (x, y, z) = pos;
        let mut props = get_defaults(&self.name).unwrap().clone();
        if let Some(axis) = props.get_mut("axis") {
            *axis = match face {
                BlockFace::NegX | BlockFace::PosX
                    => "x".to_string(),
                BlockFace::NegY | BlockFace::PosY 
                    => "y".to_string(),
                BlockFace::NegZ | BlockFace::PosZ
                    => "z".to_string(),
            };
        }
        if let Some(facing) = props.get_mut("facing") {
            *facing = facing_from_angle(angle.0);
        }
        if let Some(half) = props.get_mut("half") {
            *half = calc_half(&face, &cursor);
        }
        if let Some(waterlogged) = props.get_mut("waterlogged") {
            if world.get_block(x, y, z).0 == block_id!(water) {
                *waterlogged = "true".into();
            }
        }
        props
    }
}

fn facing_from_angle(yaw: f32) -> String {
    match (yaw + 45.0).rem_euclid(360.0) {
        x if x < 90.0 => "north",
        x if x < 180.0 => "east",
        x if x < 270.0 => "south",
        _ => "west",
    }.into()
}

fn facing_from_face(face: &BlockFace) -> String {
    match face {
        BlockFace::PosX => "east",
        BlockFace::NegX => "west",
        BlockFace::PosY => "up",
        BlockFace::NegY => "down",
        BlockFace::PosZ => "south",
        BlockFace::NegZ => "north",
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
