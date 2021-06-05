use num_derive::FromPrimitive;

#[derive(FromPrimitive)]
pub enum BlockFace {
    NegY = 0, PosY,
    NegZ, PosZ,
    NegX, PosX,
}

impl BlockFace {
    pub fn get_adjacent(&self, block: (i32, i32, i32)) 
        -> (i32, i32, i32)
    {
        let (x, y, z) = block;
        match self {
            BlockFace::NegY => (x, y - 1, z),
            BlockFace::PosY => (x, y + 1, z),
            BlockFace::NegZ => (x, y, z - 1),
            BlockFace::PosZ => (x, y, z + 1),
            BlockFace::NegX => (x - 1, y, z),
            BlockFace::PosX => (x + 1, y, z),
        }
    }
}
