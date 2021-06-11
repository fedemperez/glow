mod file;
mod flat_generator;

use async_trait::async_trait;
use super::ChunkData;
use super::ChunkCoords;

pub use file::AnvilChunkLoader;
pub use flat_generator::FlatGenerator;

#[async_trait]
pub trait ChunkLoader: Send + Sync {
    async fn load_chunk(&self, coords: ChunkCoords) -> Option<ChunkData>;
}
