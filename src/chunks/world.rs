use crate::common::block::Block;
use super::chunk::Chunk;
use super::coords::ChunkCoords;
use super::events::ChunkEvent;
use anyhow::{anyhow, Result};
use block_macro::block_id;
use std::{collections::HashMap, future::Future};
use super::chunk_source::ChunkSource;
use std::sync::{Arc, RwLock};

pub struct World {
    chunks: Arc<RwLock<
        HashMap<ChunkCoords, Arc<RwLock<Chunk>>>
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

    pub fn subscribe<F>(&self, coords: ChunkCoords, id: u32, callback: F)
        -> impl Future<Output=()> where F: Fn(ChunkEvent) + 'static + Send + Sync
    {
        let world = self.chunks.clone();
        let sources = self.chunk_sources.clone();
        async move {
            let chunk = world.read().unwrap()
                .get(&coords).cloned();
            match chunk {
                Some(chunk) => {
                    callback(ChunkEvent::ChunkLoaded{ chunk: chunk.clone() });
                    chunk.write().unwrap().subscribe(id, callback);
                },
                None => {
                    for source in &*sources {
                        if let Some(chunk) = source.load_chunk(coords).await {
                            let chunk = Arc::new(RwLock::new(chunk));
                            world.write().unwrap()
                                .insert(coords, chunk.clone());
                            callback(ChunkEvent::ChunkLoaded{ chunk: chunk.clone() });
                            chunk.write().unwrap().subscribe(id, callback);
                            return;
                        }
                    }
                    eprintln!("No chunk source could load chunk at {:?}", coords);
                }
            }
        }
    }

    pub fn unsubscribe(&self, coords: ChunkCoords, id: u32) {
        let chunk = self.chunks.read().unwrap()
            .get(&coords).cloned();
        if let Some(chunk) = chunk {
            chunk.write().unwrap().unsubscribe(id);
        }
    }

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Block {
        let coords = ChunkCoords::from_block(x, z);
        let chunk = self.chunks.read().unwrap()
            .get(&coords)
            .map(|c| c.clone());
        if let Some(chunk) = chunk {
            let (x, y, z) = coords.relative(x, y, z);
            chunk.read().unwrap().get_block(x, y, z)
        } else {
            Block(block_id!(air))
        }
    }

    pub fn set_block(&self, x: i32, y: i32, z: i32, block: Block) {
        let coords = ChunkCoords::from_block(x, z);
        let chunk = self.chunks.read().unwrap()
            .get(&coords)
            .map(|c| c.clone());
        if let Some(chunk) = chunk {
            let (x, y, z) = coords.relative(x, y, z);
            chunk.write().unwrap().set_block(x, y, z, block);
        }
    }
}