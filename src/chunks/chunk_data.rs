use std::iter::repeat_with;

use anvil_nbt::CompoundTag;
use anvil_region::{
    position::{RegionChunkPosition, RegionPosition}, 
    provider::{FolderRegionProvider, RegionProvider}};

use block_macro::block_id;
use nbt::{Map, Value};

use crate::{
    common::block::Block, 
    serialization::CompactLong};

use super::{ChunkCoords, section::{Section, SECTION_LENGTH}};

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
        let heights = CompactLong::from_values(&heights, 9);
        let mut map = Map::new();
        map.insert("MOTION_BLOCKING".into(), Value::LongArray(heights.longs));
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

    pub fn save(&self, coords: ChunkCoords) {
        let provider = FolderRegionProvider::new("world/region");
        let ChunkCoords(chunk_x, chunk_z) = coords;
        let region_position = 
            RegionPosition::from_chunk_position(chunk_x, chunk_z);
        let region_chunk_position = 
            RegionChunkPosition::from_chunk_position(chunk_x, chunk_z);

        let mut region = provider.get_region(region_position).unwrap();

        let mut chunk_tag = CompoundTag::new();
        let mut level_tag = CompoundTag::new();
        level_tag.insert_i32("xPos", chunk_x);
        level_tag.insert_i32("zPos", chunk_z);
        let mut section_tags = vec![];
        for (y, section) in self.sections.iter().enumerate() {
            if let Some(section) = section {
                section_tags.push(section.get_nbt(y as i8));
            }
        }
        level_tag.insert_compound_tag_vec("Sections", section_tags);
        chunk_tag.insert_compound_tag("Level", level_tag);

        region.write_chunk(region_chunk_position, chunk_tag);
    }
}