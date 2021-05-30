use nbt::Value as Nbt;
use uuid::Uuid;

pub struct PlayerInfo {
    pub name: String,
    pub properties: Vec<PlayerInfoProperty>,
    pub gamemode: u8,
    pub ping: u32,
    pub display_name: Option<String>,
}

pub struct PlayerInfoProperty {
    pub name: String,
    pub value: String,
    pub signature: Option<String>,
}

pub enum ClientboundPacket<'a> {
    JoinGame {
        entity_id: u32,
        gamemode: u8,
        dimension_codec: Nbt,
        dimension: Nbt,
        view_distance: u8,
    },
    PluginMessage {
        channel: &'a str,
        content: &'a[u8],
    },
    ChunkData {
        x: i32,
        z: i32,
        full: bool,
        bitmask: u16,
        heightmap: Nbt,
        biomes: Option<&'a[u32]>,
        data: &'a[u8],
        block_entities: &'a[Nbt],
    },
    KeepAlive(u64),
    PlayerPosition(f32, f32, f32),
    UpdateViewPosition(i32, i32),
    PlayerInfoAddPlayers(&'a[(Uuid, PlayerInfo)]),
    PlayerInfoUpdateGamemode(&'a[(Uuid, u8)]),
    PlayerInfoUpdateLatency(&'a[(Uuid, u16)]),
    PlayerInfoRemovePlayers(&'a[Uuid]),
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
        delta_x: u16,
        delta_y: u16,
        delta_z: u16,
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
    DestroyEntities(&'a[u32]),
    SpawnPlayer {
       entity_id: u32,
       uuid: Uuid,
       x: f64,
       y: f64,
       z: f64,
       yaw: f32,
       pitch: f32,
    },
}
