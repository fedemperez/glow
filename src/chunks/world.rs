use crate::common::block::Block;
use super::WorldView;
use super::ChunkData;
use super::view::adjacent_coords;
use super::chunk::Chunk;
use super::coords::ChunkCoords;
use super::events::ChunkEvent;
use super::saving::ChunkSaver;
use legion::system;
use legion::systems::Builder;
use nalgebra::{vector, Vector3};
use std::mem::take;
use std::sync::Mutex;
use std::time::Duration;
use std::collections::HashMap;
use super::loading::ChunkLoader;
use std::sync::{Arc, RwLock};

const CHUNK_UNLOAD_TIME: Duration = Duration::from_secs(10);
const MAX_UNLOADS_PER_TICK: usize = 2;

pub fn register(schedule: &mut Builder) {
    schedule.add_system(update_changed_system());
    schedule.add_thread_local(unload_chunks_system());
}

#[system]
fn update_changed(#[resource] world: &mut World) {
    let last_changes = take(&mut world.changed)
        .into_inner().unwrap();
    let to_update = last_changes.iter()
        .map(|pos| adjacent_coords(pos))
        .flatten();
    for pos in to_update {
        let view = world.get_view(pos);
        let block = world.get_block(&pos);
        block.update(&view);
    }
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
    changed: Mutex<Vec<Vector3<i32>>>,
}

impl World {
    pub fn new(chunk_sources: Vec<Box<dyn ChunkLoader>>) -> Self {
        Self {
            chunks: Arc::new(RwLock::new(HashMap::new())),
            chunk_loaders: Arc::new(chunk_sources),
            saver: Mutex::new(ChunkSaver::new()),
            changed: Mutex::new(vec![]),
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
                let chunk = Chunk::new();
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

    pub fn get_block(&self, pos: &Vector3<i32>) -> &'static Block {
        let coords = ChunkCoords::from_block(pos);
        let chunk = self.chunks.read().unwrap()
            .get(&coords).cloned();
        if let Some(chunk) = chunk {
            let (x, y, z) = coords.relative(pos);
            chunk.get_block(x, y, z)
        } else {
            Block::air()
        }
    }

    pub fn set_block(&self, pos: &Vector3<i32>, block: &'static Block) {
        let coords = ChunkCoords::from_block(pos);
        let chunk = self.chunks.read().unwrap()
            .get(&coords).cloned();
        if let Some(chunk) = chunk {
            self.changed.lock().unwrap()
                .push(pos.clone());
            let (x, y, z) = coords.relative(pos);
            chunk.set_block(x, y, z, block);
        }
    }

    pub fn get_view(&self, center: Vector3<i32>) -> WorldView {
        WorldView::new(self, center)
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
