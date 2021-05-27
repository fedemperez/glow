#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Block {
    Air,
    Grass,
}

impl Block {
    pub const fn get_id(&self) -> u16 {
        match self {
            Block::Air => 0x0,
            Block::Grass => 0x9,
        }
    }
}