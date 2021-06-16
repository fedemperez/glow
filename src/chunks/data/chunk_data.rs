use std::iter::repeat_with;
use anyhow::{anyhow, Result};
use anvil_nbt::CompoundTag;
use anvil_region::{
    position::{RegionChunkPosition, RegionPosition}, 
    provider::{FolderRegionProvider, RegionProvider},
    error::ChunkWriteError};

use block_macro::block_id;

use crate::common::block::Block;
use crate::chunks::ChunkCoords;

use super::{
    CHUNK_HEIGHT, 
    heightmap::HeightMap, 
    section::{Section, SECTION_WIDTH}
};

pub struct ChunkData {
    sections: Vec<Option<Section>>,
    pub heightmap: HeightMap,
}

impl ChunkData {
    pub fn new() -> Self {
        Self {
            sections: repeat_with(|| None)
                .take(CHUNK_HEIGHT / SECTION_WIDTH)
                .collect(),
            heightmap: HeightMap::new(),
        }
    }
    
    pub fn from_sections(sections: Vec<Option<Section>>) -> Self {
        Self {
            sections,
            heightmap: HeightMap::new(),
        }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> &'static Block {
        let section = y / SECTION_WIDTH;
        match &self.sections[section] {
            Some(section) => {
                section.get_block(x, y % SECTION_WIDTH, z)
            }
            None => Block::from_state_id(block_id!(air)).unwrap(),
        }
    }

    pub fn set_block(&mut self, x: usize, y: usize, z: usize, block: &'static Block) {
        let section = y / SECTION_WIDTH;
        match &mut self.sections[section] {
            Some(section) => {
                section.set_block(x, y % SECTION_WIDTH, z, block)
            }
            None => {
                let mut new_sect = Section::new();
                new_sect.set_block(x, y % SECTION_WIDTH, z, block);
                self.sections[section] = Some(new_sect);
            }
        }
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

    pub fn save(&self, coords: ChunkCoords) -> Result<()> {
        let provider = FolderRegionProvider::new("world/region");
        let ChunkCoords(chunk_x, chunk_z) = coords;
        let region_position = 
            RegionPosition::from_chunk_position(chunk_x, chunk_z);
        let region_chunk_position = 
            RegionChunkPosition::from_chunk_position(chunk_x, chunk_z);

        let mut region = provider.get_region(region_position)?;

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

        region.write_chunk(region_chunk_position, chunk_tag)
            .map_err(|err| {
                match err {
                    ChunkWriteError::LengthExceedsMaximum { length } 
                    => anyhow!(format!("Too large ({} bytes)", length)),
                    ChunkWriteError::IOError { io_error } => io_error.into(),
                }
            })?;
        Ok(())
    }
}
