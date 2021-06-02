use nbt::Value as Nbt;
use uuid::Uuid;

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
        dimension_codec: Nbt,
        dimension: Nbt,
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
        delta_x: f32,
        delta_y: f32,
        delta_z: f32,
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
        x: i32,
        y: i32,
        z: i32,
        block_state: u32,
    }
}
