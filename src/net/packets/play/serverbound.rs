pub enum ServerboundPacket {
    Move(f32, f32, f32),
    Rotate(f32, f32),
    BreakBlock(i32, i32, i32),
}