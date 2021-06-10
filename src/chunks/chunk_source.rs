use async_trait::async_trait;
use super::ChunkData;
use super::ChunkCoords;

#[async_trait]
pub trait ChunkSource: Send + Sync {
    async fn load_chunk(&self, coords: ChunkCoords) -> Option<ChunkData>;
}