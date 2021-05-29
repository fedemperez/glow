use super::chunk::CHUNK_WIDTH;
use nalgebra::Vector3;
use num_integer::Integer;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub struct ChunkCoords(pub i32, pub i32);

impl ChunkCoords {
    pub fn from_pos(pos: Vector3<f32>) -> Self {
        let (x, _, z) = block_coords(pos);
        Self::from_block(x, z)
    }
    
    pub fn from_block(x: i32, z: i32) -> Self {
        Self(x.div_floor(&(CHUNK_WIDTH as i32)), z.div_floor(&(CHUNK_WIDTH as i32)))
    }
}

pub fn block_coords(pos: Vector3<f32>) -> (i32, i32, i32) {
    (pos.x.floor() as i32, pos.y.floor() as i32, pos.z.floor() as i32)
}