use crate::common::block::Block;
use super::events::ChunkEvent;
use super::section::{Section, SECTION_LENGTH};
use std::iter::repeat_with;
use std::sync::{Arc, Mutex};
use nbt::{Value, Map};
use crate::util::{compacted_long, push_varint};

pub const CHUNK_HEIGHT: usize = 256;
pub const CHUNK_WIDTH: usize = SECTION_LENGTH;

pub struct Chunk {
    sections: Vec<Option<Section>>,
    subscribers: Vec<Box<dyn Fn(ChunkEvent) + Send + Sync>>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            sections: repeat_with(|| None)
                .take(CHUNK_HEIGHT / SECTION_LENGTH)
                .collect(),
            subscribers: vec![],
        }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Block {
        let section = y / SECTION_LENGTH;
        match &self.sections[section] {
            Some(section) => {
                section.get_block(x, y % SECTION_LENGTH, z)
            }
            None => Block::from_name("minecraft:air").unwrap(),
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
        self.emit_event(ChunkEvent::BlockChanged {
            x, y, z, new: block,
        });
    }

    pub fn subscribe<F>(&mut self, callback: F)
        where F: Fn(ChunkEvent) + 'static + Send + Sync
    {
        self.subscribers.push(Box::new(callback));
    }

    fn emit_event(&self, event: ChunkEvent) {
        for callback in &self.subscribers {
            callback(event.clone());
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
        let heights = compacted_long(heights, 9);
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
                bytes.extend_from_slice(&16u16.to_be_bytes());
                let (bits_per_block, data) = section.get_compacted_data();
                bytes.push(bits_per_block as u8);
                push_varint(data.len() as u32, &mut bytes);
                for long in data {
                    bytes.extend_from_slice(&long.to_be_bytes());
                }
            }
        }
        bytes
    }
}