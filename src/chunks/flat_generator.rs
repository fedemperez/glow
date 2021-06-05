use async_trait::async_trait;
use super::chunk_source::ChunkSource;
use super::{Chunk, ChunkCoords};
use crate::common::block::Block;

pub struct FlatGenerator;

#[async_trait]
impl ChunkSource for FlatGenerator {
    async fn load_chunk(&self, coords: ChunkCoords) -> Option<Chunk> {
        let mut chunk = Chunk::new();
        for x in 0..16 {
            for z in 0..16 {
                chunk.set_block(x, 0, z, 
                    Block::from_name("minecraft:grass_block").unwrap());
            }
        }
        Some(chunk)
    }
}