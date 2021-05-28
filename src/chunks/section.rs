use super::block::Block;
use crate::util::compacted_long;

pub const SECTION_LENGTH: usize = 16;
const BLOCKS_PER_SECTION: usize = SECTION_LENGTH.pow(3);
const BITS_PER_BLOCK: u32 = 15;

pub struct Section {
    blocks: Vec<Block>,
}

impl Section {
    pub fn new() -> Self {
        Self {
            blocks: vec![Block::Air; BLOCKS_PER_SECTION],
        }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Block {
        self.blocks[Self::coords_to_index(x, y, z)]
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: Block) {
        self.blocks[Self::coords_to_index(x, y, z)] = block;
    }

    const fn coords_to_index(x: usize, y: usize, z: usize) -> usize {
        x + z * SECTION_LENGTH + y * SECTION_LENGTH * SECTION_LENGTH
    }

    fn get_data(&self) -> Vec<u16> {
        (&self.blocks).into_iter().map(|block| {
            block.get_id()
        }).collect()
    }

    pub fn get_compacted_data(&self) -> (u32, Vec<i64>) {
        (BITS_PER_BLOCK, compacted_long(self.get_data(), BITS_PER_BLOCK))
    }
}