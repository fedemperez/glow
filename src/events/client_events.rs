use nalgebra::Vector3;

pub enum ClientEvent {
    Disconnect(String),
    Move(Vector3<f32>),
    Rotate(f32, f32),
    BreakBlock(i32, i32, i32),
}