use block_macro::block_id;

use crate::common::block::Block;
use crate::serialization::{CompactedLong, push_varint};

use super::palette::Palette;

pub const SECTION_LENGTH: usize = 16;
const BLOCKS_PER_SECTION: usize = SECTION_LENGTH.pow(3);
const GLOBAL_PALETTE_BITS: u8 = 15;
const MAX_PALETTE_BITS: u8 = 8;

pub struct Section {
    blocks: CompactedLong,
    palette: Option<Palette>,
}

impl Section {
    pub fn new() -> Self {
        let mut palette = Palette::new();
        palette.get_or_add_id(Block(block_id!(air)));
        Self {
            blocks: CompactedLong::new(vec![0; BLOCKS_PER_SECTION / (64 / 4)], 4),
            palette: Some(palette),
        }
    }

    pub fn from_raw(blocks: Vec<i64>, palette: Palette) -> Self {
        Self {
            blocks: CompactedLong::new(blocks, palette.get_bits_per_block()),
            palette: Some(palette),
        }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Block {
        let index = Self::coords_to_index(x, y, z);
        let block = self.blocks.get(index) as u16;
        if let Some(palette) = &self.palette {
            palette.get_block(block)
        } else {
            Block(block)
        }
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: Block) {
        let index = Self::coords_to_index(x, y, z);
        let block = if let Some(palette) = &mut self.palette {
            let id = palette.get_or_add_id(block);
            if palette.get_bits_per_block() > MAX_PALETTE_BITS {
                self.convert_to_global_palette();
                block.0
            } else {
                self.blocks.set_bits(palette.get_bits_per_block());
                id
            }
        } else {
            block.0
        };
        self.blocks.set(index, block as i64);
    }

    fn convert_to_global_palette(&mut self) {
        self.blocks.set_bits(GLOBAL_PALETTE_BITS);
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
            data.push(GLOBAL_PALETTE_BITS);
        }
        let blocks = &self.blocks.longs;
        push_varint(blocks.len() as u32, data);
        for long in blocks {
            data.extend_from_slice(&long.to_be_bytes());
        }
    }
}