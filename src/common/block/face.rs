use nalgebra::{Vector3, vector};
use num_derive::FromPrimitive;

#[derive(FromPrimitive)]
pub enum BlockFace {
    NegY = 0, PosY,
    NegZ, PosZ,
    NegX, PosX,
}

impl BlockFace {
    pub fn get_adjacent(&self, block: Vector3<i32>) 
        -> Vector3<i32>
    {
        block + self.get_direction()
    }

    pub const fn get_direction(&self) -> Vector3<i32> {
        match self {
            BlockFace::NegY => vector!(0, -1, 0),
            BlockFace::PosY => vector!(0, 1, 0),
            BlockFace::NegZ => vector!(0, 0, -1),
            BlockFace::PosZ => vector!(0, 0, 1),
            BlockFace::NegX => vector!(-1, 0, 0),
            BlockFace::PosX => vector!(1, 0, 0),
        }
    }
}
