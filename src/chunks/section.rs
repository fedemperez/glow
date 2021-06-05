use block_macro::block_id;

use crate::common::block::Block;
use crate::serialization::{compacted_long, push_varint};

use super::palette::Palette;

pub const SECTION_LENGTH: usize = 16;
const BLOCKS_PER_SECTION: usize = SECTION_LENGTH.pow(3);
const GLOBAL_PALETTE_BITS_PER_BLOCK: u8 = 15;

pub struct Section {
    blocks: Vec<u16>,
    palette: Option<Palette>,
}

impl Section {
    pub fn new() -> Self {
        Self {
            blocks: vec![block_id!(air); BLOCKS_PER_SECTION],
            palette: Some(Palette::new()),
        }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Block {
        let block = self.blocks[Self::coords_to_index(x, y, z)];
        if let Some(palette) = &self.palette {
            palette.get_block(block)
        } else {
            Block(block)
        }
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: Block) {
        if let Some(palette) = &mut self.palette {
            if let Ok(id) = palette.get_or_add_id(block) {
                self.blocks[Self::coords_to_index(x, y, z)] = id;
            } else {
                self.convert_to_global_palette();
                self.blocks[Self::coords_to_index(x, y, z)] = block.0;
            }
        } else {
            self.blocks[Self::coords_to_index(x, y, z)] = block.0;
        }
    }

    fn convert_to_global_palette(&mut self) {
        for block in &mut self.blocks {
            *block = self.palette.as_ref().unwrap()
                .get_block(*block).0;
        }
        self.palette = None;
    }

    const fn coords_to_index(x: usize, y: usize, z: usize) -> usize {
        x + z * SECTION_LENGTH + y * SECTION_LENGTH * SECTION_LENGTH
    }

    fn count_non_air(&self) -> u16 {
        16
    }

    pub fn push_data(&self, data: &mut Vec<u8>) {
        data.extend_from_slice(
            &self.count_non_air().to_be_bytes());
        if let Some(palette) = &self.palette {
            data.push(palette.get_bits_per_block());
            push_varint(palette.entries.len() as u32,
                data);
            for entry in &palette.entries {
                push_varint(*entry as u32, data);
            }
        } else {
            data.push(GLOBAL_PALETTE_BITS_PER_BLOCK);
        }
        let blocks = self.get_compacted_blocks();
        push_varint(blocks.len() as u32, data);
        for long in blocks {
            data.extend_from_slice(&long.to_be_bytes());
        }
    }

    fn get_compacted_blocks(&self) -> Vec<i64> {
        if let Some(palette) = &self.palette {
            compacted_long(&self.blocks, 
                palette.get_bits_per_block() as u32)
        } else {
            compacted_long(&self.blocks, 
                GLOBAL_PALETTE_BITS_PER_BLOCK as u32)
        }
    }
}