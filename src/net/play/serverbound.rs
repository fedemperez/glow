use nalgebra::Vector3;
use nbt::Value as Nbt;
use crate::common::{
    item_stack::ItemStack,
    block::BlockFace,
};

pub enum ServerboundPacket {
    PlayerPosition {
        x: f64, 
        y: f64, 
        z: f64,
        on_ground: bool,
    },
    PlayerPositionAndRotation {
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },
    PlayerRotation {
        yaw: f32, 
        pitch: f32,
        on_ground: bool,
    },
    PlayerDigging {
        status: u8,
        position: (i32, i32, i32),
        face: u8,
    },
    HeldItemChange {
        slot: u16,
    },
    CreativeInventoryAction {
        slot: i16,
        stack: Option<ItemStack>,
    },
    PlayerBlockPlacement {
        hand: u8,
        location: (i32, i32, i32),
        face: BlockFace,
        cursor_position: Vector3<f32>,
        inside_block: bool,
    },
    Disconnect {
        reason: String,
    },
}
