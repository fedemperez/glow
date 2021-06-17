use async_trait::async_trait;
use block_macro::block_id;
use super::ChunkLoader;
use super::{ChunkData, ChunkCoords};
use crate::blocks::Block;

pub struct FlatGenerator;

#[async_trait]
impl ChunkLoader for FlatGenerator {
    async fn load_chunk(&self, _coords: ChunkCoords) -> Option<ChunkData> {
        let mut chunk = ChunkData::new();
        let grass = Block::from_state_id(block_id!(grass_block)).unwrap();
        for x in 0..16 {
            for z in 0..16 {
                chunk.set_block(x, 0, z, grass);
            }
        }
        Some(chunk)
    }
}
