use super::chunk::CHUNK_WIDTH;
use nalgebra::Vector3;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub struct ChunkCoords(pub i32, pub i32);

impl ChunkCoords {
    pub fn from_pos(pos: Vector3<f32>) -> Self {
        let (x, _, z) = block_coords(pos);
        Self::from_block(x, z)
    }
    
    pub fn from_block(x: i32, z: i32) -> Self {
        Self(x / CHUNK_WIDTH as i32, z / CHUNK_WIDTH as i32)
    }
}

fn block_coords(pos: Vector3<f32>) -> (i32, i32, i32) {
    (pos.x as i32, pos.y as i32, pos.z as i32)
}