use std::iter::repeat_with;

use block_macro::block_id;
use nbt::{Map, Value};

use crate::{common::block::Block, serialization::write_compacted_long};

use super::section::{Section, SECTION_LENGTH};

pub const CHUNK_HEIGHT: usize = 256;
pub const CHUNK_WIDTH: usize = SECTION_LENGTH;

pub struct ChunkData {
    sections: Vec<Option<Section>>,
}

impl ChunkData {
    pub fn new() -> Self {
        Self {
            sections: repeat_with(|| None)
            .take(CHUNK_HEIGHT / SECTION_LENGTH)
            .collect(),
        }
    }
    
    pub fn from_sections(sections: Vec<Option<Section>>) -> Self {
        Self { sections }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Block {
        let section = y / SECTION_LENGTH;
        match &self.sections[section] {
            Some(section) => {
                section.get_block(x, y % SECTION_LENGTH, z)
            }
            None => Block(block_id!(air)),
        }
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: Block) {
        let section = y / SECTION_LENGTH;
        match &mut self.sections[section] {
            Some(section) => {
                section.set_block(x, y % SECTION_LENGTH, z, block)
            }
            None => {
                let mut new_sect = Section::new();
                new_sect.set_block(x, y % SECTION_LENGTH, z, block);
                self.sections[section] = Some(new_sect);
            }
        }
    }

    fn get_height(&self, x: usize, z: usize) -> u16 {
        1
    }

    pub fn get_heightmap(&self) -> Value {
        let mut heights = Vec::with_capacity(CHUNK_WIDTH * CHUNK_WIDTH);
        for x in 0..CHUNK_WIDTH {
            for z in 0..CHUNK_WIDTH {
                heights.push(self.get_height(x, z));
            }
        }
        let heights = write_compacted_long(&heights, 9);
        let mut map = Map::new();
        map.insert("MOTION_BLOCKING".into(), Value::LongArray(heights));
        Value::Compound(map)
    }

    pub fn get_biome_map(&self) -> Vec<u16> {
        vec![0; 1024]
    }

    pub fn get_sections_bitmask(&self) -> u16 {
        let mut mask = 0;
        let mut current_bit = 1;
        for section in &self.sections {
            if section.is_some() {
                mask |= current_bit;
            }
            current_bit <<= 1;
        }
        mask
    }

    pub fn get_data(&self) -> Vec<u8> {
        let mut bytes = vec![];
        for section in &self.sections {
            if let Some(section) = section {
                section.push_data(&mut bytes);
            }
        }
        bytes
    }
}