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
    Disconnect {
        reason: String,
    },
}