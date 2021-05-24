mod block;
mod chunk;
mod section;
mod chunk_source;

pub use chunk::Chunk;
pub use block::Block;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use chunk_source::ChunkSource;
use crate::game::world::chunk::CHUNK_WIDTH;
use tokio::sync::{RwLock, oneshot, oneshot::Receiver, Mutex};
use std::sync::Arc;

#[derive(Clone)]
pub struct World {
    chunks: Arc<RwLock<
        HashMap<(i32, i32), Arc<Mutex<Chunk>>>
    >>,
    chunk_sources: Arc<Vec<Box<dyn ChunkSource>>>,
}

impl World {
    pub fn new(chunk_sources: Vec<Box<dyn ChunkSource>>) -> Self {
        Self {
            chunks: Arc::new(RwLock::new(HashMap::new())),
            chunk_sources: Arc::new(chunk_sources),
        }
    }

    async fn load_chunk(&mut self, x: i32, z: i32) -> Result<()> {
        for source in &*self.chunk_sources {
            if let Some(chunk) = source.load_chunk(x, z).await {
                let chunk = Arc::new(Mutex::new(chunk));
                self.chunks.write().await.insert((x, z), chunk);
                return Ok(());
            }
        }
        Err(anyhow!("No chunk source could load chunk x: {} z: {}", x, z))
    }
}