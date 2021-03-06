use nalgebra::Vector3;
use nbt::Value as Nbt;
use serde_json::Value as Json;
use uuid::Uuid;

use crate::items::ItemStack;

#[derive(Clone)]
pub struct PlayerInfo {
    pub name: String,
    pub properties: Vec<PlayerInfoProperty>,
    pub gamemode: u8,
    pub ping: u32,
    pub display_name: Option<String>,
}

#[derive(Clone)]
pub struct PlayerInfoProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

#[derive(Clone)]
pub enum ClientboundPacket {
    JoinGame {
        entity_id: u32,
        gamemode: u8,
        world_names: Vec<String>,
        dimension_codec: &'static[u8],
        dimension: &'static[u8],
        current_world: String,
        view_distance: u8,
    },
    PluginMessage {
        channel: String,
        content: String,
    },
    ChunkData {
        x: i32,
        z: i32,
        full: bool,
        bitmask: u16,
        heightmap: Nbt,
        biomes: Option<Vec<u16>>,
        data: Vec<u8>,
        block_entities: Vec<Nbt>,
    },
    UpdateLight {
        x: i32,
        z: i32,
        trust_edges: bool,
        sky_mask: u32,
        block_mask: u32,
        empty_sky_mask: u32,
        empty_block_mask: u32,
        sky_light: Vec<Vec<u8>>,
        block_light: Vec<Vec<u8>>,
    },
    KeepAlive(u64),
    PlayerPosition(f64, f64, f64),
    UpdateViewPosition(i32, i32),
    PlayerInfoAddPlayers(Vec<(Uuid, PlayerInfo)>),
    PlayerInfoUpdateGamemode(Vec<(Uuid, u8)>),
    PlayerInfoUpdateLatency(Vec<(Uuid, u16)>),
    PlayerInfoRemovePlayers(Vec<Uuid>),
    EntityTeleport {
        id: u32,
        x: f64,
        y: f64,
        z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },
    EntityPosition {
        id: u32, 
        delta_x: f64,
        delta_y: f64,
        delta_z: f64,
        on_ground: bool,
    },
    EntityPositionAndRotation {
        id: u32, 
        delta_x: f64,
        delta_y: f64,
        delta_z: f64,
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },
    EntityRotation {
        id: u32, 
        yaw: f32,
        pitch: f32,
        on_ground: bool,
    },
    EntityHeadLook {
        id: u32,
        yaw: f32,
    },
    DestroyEntities(Vec<u32>),
    SpawnPlayer {
       entity_id: u32,
       uuid: Uuid,
       x: f64,
       y: f64,
       z: f64,
       yaw: f32,
       pitch: f32,
    },
    BlockChange {
        pos: Vector3<i32>,
        block_state: u32,
    },
    WindowItems {
        window: u8,
        items: Vec<Option<ItemStack>>,
    },
    UnloadChunk(i32, i32),
    Disconnect {
        reason: Json,
    },
    Tags {
        raw: &'static [u8],
    }
}
