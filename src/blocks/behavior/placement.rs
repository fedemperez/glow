use std::collections::BTreeMap;

use nalgebra::Vector3;

use crate::chunks::WorldView;
use super::stairs::get_stair_shape;
use super::can_place_plant_on;
use crate::blocks::{BlockFace, Block, BlockType, BlockClass};

impl BlockType {
    pub fn place(&self, view: &WorldView, face: BlockFace, 
        cursor: Vector3<f32>, angle: (f32, f32))
    {
        let mut view = view.clone();
        if !view.get(0, 0, 0).material.replaceable {
            view.displace(face.get_direction());
            if !view.get(0, 0, 0).material.replaceable {
                return;
            }
        }
        let mut props = self.auto_fill_props(
            view.get(0, 0, 0), &face, cursor, angle);
        match self.class {
            BlockClass::StairsBlock => {
                props.insert("half".into(), 
                    calc_half(&face, &cursor));
                props.insert("facing".into(),
                    facing_from_angle(angle.0 + 180.0));
                props.insert("shape".into(), 
                    get_stair_shape(&props, &view));
            },
            BlockClass::EndRodBlock => {
                props.insert("facing".into(), 
                    facing_from_face(&face));
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
                if !view.get(0, 1, 0).material.replaceable {
                    return;
                }
                props.insert("facing".into(), 
                facing_from_angle(angle.0 + 180.0));
                props.insert("half".into(), "upper".into());
                let block = self.with_props(&props).unwrap();
                view.set(0, 1, 0, block);
                props.insert("half".into(), "lower".into());
            },
            BlockClass::TallFlowerBlock | BlockClass::TallPlantBlock => {
                if !can_place_plant_on(view.get(0, -1, 0)) {
                    return;
                }
                if !view.get(0, 1, 0).material.replaceable {
                    return;
                }
                props.insert("half".into(), "upper".into());
                let block = self.with_props(&props).unwrap();
                view.set(0, 1, 0, block);
                props.insert("half".into(), "lower".into());
            },
            BlockClass::FlowerBlock | BlockClass::FernBlock => {
                if !can_place_plant_on(view.get(0, -1, 0)) {
                    return;
                }
            },
            BlockClass::CropBlock => {
                if view.get(0, -1, 0).btype.name != "minecraft:farmland" {
                    return;
                }
            },
            _ => (),
        }
        let block = self.with_props(&props).unwrap();
        view.set(0, 0, 0, block);
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
