use std::collections::HashMap;
use anyhow::{anyhow, Result};
use block_macro::block_id;

use crate::common::block::Block;

const MIN_BITS: u8 = 4;
const MAX_BITS: u32 = 8;

pub struct Palette {
    pub entries: Vec<u16>,
    block_to_entry: HashMap<Block, u16>,
}

impl Palette {
    pub fn new() -> Self {
        Self {
            entries: vec![block_id!(air)],
            block_to_entry: HashMap::new(),
        }
    }

    pub fn get_bits_per_block(&self) -> u8 {
        ((self.entries.len() as f32).log2().ceil() as u8)
            .max(MIN_BITS)
    }

    pub fn get_block(&self, id: u16) -> Block {
        Block(self.entries[id as usize])
    }

    pub fn get_or_add_id(&mut self, block: Block) -> Result<u16> {
        if let Some(id) = self.block_to_entry.get(&block) {
            Ok(*id)
        } else {
            let id = self.entries.len() as u16;
            if id < 2u16.pow(MAX_BITS) {
                self.entries.push(block.0);
                self.block_to_entry.insert(block, id);
                Ok(id)
            } else {
                Err(anyhow!("Palette is too big"))
            }
        }
    }
}
