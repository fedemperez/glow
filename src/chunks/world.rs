use crate::common::block::Block;
use super::ChunkData;
use super::chunk::Chunk;
use super::coords::ChunkCoords;
use super::events::ChunkEvent;
use super::saving::ChunkSaver;
use block_macro::block_id;
use legion::system;
use legion::systems::Builder;
use std::sync::Mutex;
use std::time::Duration;
use std::collections::HashMap;
use super::loading::ChunkLoader;
use std::sync::{Arc, RwLock};

const CHUNK_UNLOAD_TIME: Duration = Duration::from_secs(10);
const MAX_UNLOADS_PER_TICK: usize = 2;

pub fn register(schedule: &mut Builder) {
    schedule.add_thread_local(unload_chunks_system());
}

#[system]
fn unload_chunks(#[resource] world: &mut World) {
    let mut removed = vec![];
    for (coords, chunk) in world.chunks.read().unwrap().iter() {
        if chunk.time_unobserved() > CHUNK_UNLOAD_TIME {
            removed.push(*coords);
            if removed.len() == MAX_UNLOADS_PER_TICK {
                break;
            }
        }
    }
    let mut chunks = world.chunks.write().unwrap();
    let mut saver = world.saver.lock().unwrap();
    for coords in removed {
        if let Some(chunk) = chunks.remove(&coords) {
            chunk.save(coords, &mut saver);
        }
    }
}

pub struct World {
    chunks: Arc<RwLock<
        HashMap<ChunkCoords, Chunk>
    >>,
    chunk_loaders: Arc<Vec<Box<dyn ChunkLoader>>>,
    saver: Mutex<ChunkSaver>,
}

impl World {
    pub fn new(chunk_sources: Vec<Box<dyn ChunkLoader>>) -> Self {
        Self {
            chunks: Arc::new(RwLock::new(HashMap::new())),
            chunk_loaders: Arc::new(chunk_sources),
            saver: Mutex::new(ChunkSaver::new()),
        }
    }

    pub fn subscribe<F>(&self, coords: ChunkCoords, id: u32, callback: F)
        where F: Fn(ChunkEvent) + 'static + Send + Sync
    {
        let chunk = self.chunks.read().unwrap()
            .get(&coords).cloned();
        match chunk {
            Some(chunk) => {
                chunk.subscribe(id, callback);
            },
            None => {
                let mut chunk = Chunk::new();
                chunk.subscribe(id, callback);
                self.chunks.write().unwrap()
                    .insert(coords, chunk);
                let sources = self.chunk_loaders.clone();
                let world = self.chunks.clone();
                tokio::spawn(async move {
                    if let Some(data) = load_chunk(coords, &*sources).await {
                        if let Some(chunk) = world.write().unwrap().get_mut(&coords) {
                            chunk.load(data);
                        }
                    } else {
                        eprintln!("No chunk source could load chunk at {:?}", coords);
                    }
                });
            }
        }
    }

    pub fn unsubscribe(&self, coords: ChunkCoords, id: u32) {
        let chunk = self.chunks.read().unwrap()
            .get(&coords).cloned();
        if let Some(chunk) = chunk {
            chunk.unsubscribe(id);
        }
    }

    pub fn get_block(&self, x: i32, y: i32, z: i32) -> Block {
        let coords = ChunkCoords::from_block(x, z);
        let chunk = self.chunks.read().unwrap()
            .get(&coords).cloned();
        if let Some(chunk) = chunk {
            let (x, y, z) = coords.relative(x, y, z);
            chunk.get_block(x, y, z)
        } else {
            Block(block_id!(air))
        }
    }

    pub fn set_block(&self, x: i32, y: i32, z: i32, block: Block) {
        let coords = ChunkCoords::from_block(x, z);
        let chunk = self.chunks.read().unwrap()
            .get(&coords).cloned();
        if let Some(chunk) = chunk {
            let (x, y, z) = coords.relative(x, y, z);
            chunk.set_block(x, y, z, block);
        }
    }

    pub fn save_all(&mut self) {
        let mut saver = self.saver.lock().unwrap();
        let chunks = self.chunks.write().unwrap();
        for (coords, chunk) in chunks.iter() {
            chunk.save(*coords, &mut saver);
        }
        saver.wait_completion();
    }
}

async fn load_chunk(coords: ChunkCoords, sources: &Vec<Box<dyn ChunkLoader>>) 
    -> Option<ChunkData>
{
    for source in sources {
        if let Some(chunk) = source.load_chunk(coords).await {
            return Some(chunk);
        }
    }
    None
}