use nalgebra::{Vector3, vector};
use num_derive::FromPrimitive;

#[derive(FromPrimitive)]
pub enum BlockFace {
    NegY = 0, PosY,
    NegZ, PosZ,
    NegX, PosX,
}

impl BlockFace {
    pub fn get_adjacent(&self, block: (i32, i32, i32)) 
        -> Vector3<i32>
    {
        let (x, y, z) = block;
        match self {
            BlockFace::NegY => vector!(x, y - 1, z),
            BlockFace::PosY => vector!(x, y + 1, z),
            BlockFace::NegZ => vector!(x, y, z - 1),
            BlockFace::PosZ => vector!(x, y, z + 1),
            BlockFace::NegX => vector!(x - 1, y, z),
            BlockFace::PosX => vector!(x + 1, y, z),
        }
    }
}
