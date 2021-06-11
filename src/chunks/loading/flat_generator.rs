use async_trait::async_trait;
use block_macro::block_id;
use super::ChunkLoader;
use super::{ChunkData, ChunkCoords};
use crate::common::block::Block;

pub struct FlatGenerator;

#[async_trait]
impl ChunkLoader for FlatGenerator {
    async fn load_chunk(&self, _coords: ChunkCoords) -> Option<ChunkData> {
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