use nalgebra::Vector3;

#[derive(Clone, Copy, Debug, Default)]
pub struct Position(pub Vector3<f64>);

#[derive(Clone, Copy, Debug, Default)]
pub struct Rotation(pub f32, pub f32);

#[derive(Clone, Debug, Default)]
pub struct Name(pub String);