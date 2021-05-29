use nalgebra::Vector3;

#[derive(Clone, Copy, Debug, Default)]
pub struct Position(pub Vector3<f32>);

#[derive(Clone, Copy, Debug, Default)]
pub struct Rotation(pub f32, pub f32);