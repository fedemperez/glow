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
        let bedrock = Block::from_state_id(block_id!(bedrock)).unwrap();
        let dirt = Block::from_state_id(block_id!(dirt)).unwrap();
        let grass = Block::from_state_id(block_id!(grass_block)).unwrap();
        set_layer(&mut chunk, 0, bedrock);
        set_layer(&mut chunk, 1, dirt);
        set_layer(&mut chunk, 2, grass);
        Some(chunk)
    }
}

fn set_layer(chunk: &mut ChunkData, y: usize, block: &'static Block) {
    for x in 0..16 {
        for z in 0..16 {
            chunk.set_block(x, y, z, block);
        }
    }
}
