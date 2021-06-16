use std::collections::BTreeMap;

use nalgebra::Vector3;

use crate::chunks::World;
use crate::common::block::{Block, BlockFace};
use super::BlockClass;
use super::BlockType;

impl BlockType {
    pub fn place(&self, pos: (i32, i32, i32), world: &World, face: BlockFace, 
        cursor: Vector3<f32>, angle: (f32, f32))
    {
        let (x, y, z) = pos;
        let replacing = world.get_block(x, y, z);
        if !replacing.material.replaceable {
            return;
        }
        let mut props = self.auto_fill_props(replacing, &face, cursor, angle);
        match self.class {
            BlockClass::StairsBlock => {
                props.insert("half".into(), calc_half(&face, &cursor));
                props.insert("facing".into(),
                    facing_from_angle(angle.0 + 180.0));
            },
            BlockClass::EndRodBlock => {
                props.insert("facing".into(), facing_from_face(&face));
            },
            BlockClass::LadderBlock => {
                match face {
                    BlockFace::PosY | BlockFace::NegY => return,
                    face => {
                        props.insert("facing".into(),
                            facing_from_face(&face));
                    }
                }
            },
            BlockClass::SlabBlock => {
                props.insert("type".into(), calc_half(&face, &cursor));
            },
            BlockClass::TrapdoorBlock => {
                props.insert("half".into(), calc_half(&face, &cursor));
            },
            BlockClass::DoorBlock => {
                if !world.get_block(x, y + 1, z).material.replaceable {
                    return;
                }
                props.insert("facing".into(), 
                facing_from_angle(angle.0 + 180.0));
                props.insert("half".into(), "upper".into());
                let block = self.with_props(&props).unwrap();
                world.set_block(x, y + 1, z, block);
                props.insert("half".into(), "lower".into());
            },
            BlockClass::TallFlowerBlock | BlockClass::TallPlantBlock => {
                if !can_place_plant_on(world.get_block(x, y - 1, z)) {
                    return;
                }
                if !world.get_block(x, y + 1, z).material.replaceable {
                    return;
                }
                props.insert("half".into(), "upper".into());
                let block = self.with_props(&props).unwrap();
                world.set_block(x, y + 1, z, block);
                props.insert("half".into(), "lower".into());
            },
            BlockClass::FlowerBlock | BlockClass::FernBlock => {
                if !can_place_plant_on(world.get_block(x, y - 1, z)) {
                    return;
                }
            },
            BlockClass::CropBlock => {
                if world.get_block(x, y - 1, z).btype.name != "minecraft:farmland" {
                    return;
                }
            },
            _ => (),
        }
        let block = self.with_props(&props).unwrap();
        world.set_block(x, y, z, block);
    }

    fn auto_fill_props(&self, replacing: &Block, face: &BlockFace, 
        cursor: Vector3<f32>, angle: (f32, f32)) -> BTreeMap<String, String>
    {
        let mut props = self.default_state.clone();
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
        if let Some(waterlogged) = props.get_mut("waterlogged") {
            if replacing.btype.name == "minecraft:water" {
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

fn can_place_plant_on(block: &Block) -> bool {
    block.material.name == "minecraft:soil" || 
    block.material.name == "minecraft:solid_organic"
}
