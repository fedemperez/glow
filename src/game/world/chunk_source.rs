use async_trait::async_trait;
use super::Chunk;

#[async_trait]
pub trait ChunkSource {
    async fn load_chunk(&self, x: i32, z: i32) -> Option<Chunk>;
}