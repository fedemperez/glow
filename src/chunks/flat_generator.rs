use async_trait::async_trait;
use block_macro::block_id;
use super::chunk_source::ChunkSource;
use super::{ChunkData, ChunkCoords};
use crate::common::block::Block;

pub struct FlatGenerator;

#[async_trait]
impl ChunkSource for FlatGenerator {
    async fn load_chunk(&self, coords: ChunkCoords) -> Option<ChunkData> {
        let mut chunk = ChunkData::new();
        for x in 0..16 {
            for z in 0..16 {
                chunk.set_block(x, 0, z, 
                    Block(block_id!(grass_block)));
            }
        }
        Some(chunk)
    }
}