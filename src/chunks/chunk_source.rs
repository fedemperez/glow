use async_trait::async_trait;
use super::Chunk;
use super::ChunkCoords;

#[async_trait]
pub trait ChunkSource: Send + Sync {
    async fn load_chunk(&self, coords: ChunkCoords) -> Option<Chunk>;
}