use super::block::Block;
use super::section::{Section, SECTION_LENGTH};
use std::iter::repeat_with;

pub const CHUNK_HEIGHT: usize = 256;
pub const CHUNK_WIDTH: usize = SECTION_LENGTH;

pub struct Chunk {
    sections: Vec<Option<Section>>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            sections: repeat_with(|| None)
                .take(CHUNK_HEIGHT / SECTION_LENGTH)
                .collect(),
        }
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Block {
        let section = y / SECTION_LENGTH;
        match &self.sections[section] {
            Some(section) => {
                section.get_block(x, y % SECTION_LENGTH, z)
            }
            None => Block::Air
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
}