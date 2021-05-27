mod block;
mod chunk;
mod section;
mod chunk_source;
mod coords;
mod flat_generator;

pub use chunk::Chunk;
pub use block::Block;
pub use coords::ChunkCoords;
pub use flat_generator::FlatGenerator;
use anyhow::{anyhow, Result};
use std::{collections::HashMap, future::Future};
use chunk_source::ChunkSource;
use tokio::sync::RwLock;
use std::sync::Arc;

pub struct ChunkWorld {
    chunks: Arc<RwLock<
        HashMap<ChunkCoords, Arc<RwLock<Chunk>>>
    >>,
    chunk_sources: Arc<Vec<Box<dyn ChunkSource>>>,
}

impl ChunkWorld {
    pub fn new(chunk_sources: Vec<Box<dyn ChunkSource>>) -> Self {
        Self {
            chunks: Arc::new(RwLock::new(HashMap::new())),
            chunk_sources: Arc::new(chunk_sources),
        }
    }

    pub fn get_chunk(&self, coords: ChunkCoords) -> impl Future<Output=Result<Arc<RwLock<Chunk>>>> 
    {
        let chunks = self.chunks.clone();
        let sources = self.chunk_sources.clone();
        async move {
            let chunk = chunks.read().await
                .get(&coords).map(|c| c.clone());
            match chunk {
                Some(chunk) => Ok(chunk),
                None => {
                    for source in &*sources {
                        if let Some(chunk) = source.load_chunk(coords).await {
                            let chunk = Arc::new(RwLock::new(chunk));
                            chunks.write().await.insert(coords, chunk.clone());
                            return Ok(chunk);
                        }
                    }
                    Err(anyhow!("No chunk source could load chunk at {:?}", coords))
                }
            }
        }
    }
}