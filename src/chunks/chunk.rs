use block_macro::block_id;

use crate::common::block::Block;
use super::ChunkCoords;
use super::ChunkData;
use super::events::ChunkEvent;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;
use std::time::Instant;

#[derive(Clone)]
pub struct Chunk {
    data: Option<Arc<RwLock<ChunkData>>>,
    subscribers: Arc<RwLock<HashMap<u32, Box<dyn Fn(ChunkEvent) + Send + Sync>>>>,
    unobserved_since: Arc<RwLock<Option<Instant>>>,
}

impl Chunk {
    pub fn new() -> Self {
        Self {
            data: None,
            subscribers: Default::default(),
            unobserved_since: Arc::new(RwLock::new(Some(Instant::now()))),
        }
    }

    pub fn load(&mut self, data: ChunkData) {
        let data = Arc::new(RwLock::new(data));
        self.data = Some(data.clone());
        self.emit_event(ChunkEvent::ChunkLoaded {
            chunk: data,
        })
    }

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> Block {
        match &self.data {
            Some(data) => data.read().unwrap().get_block(x, y, z),
            None => Block(block_id!(air)),
        }
    }

    pub fn set_block(&self, x: usize, y: usize, z: usize, block: Block) {
        if let Some(data) = &self.data {
            data.write().unwrap().set_block(x, y, z, block);
            self.emit_event(ChunkEvent::BlockChanged {
                x, y, z, new: block,
            });
        }
    }

    pub fn subscribe<F>(&self, id: u32, callback: F)
        where F: Fn(ChunkEvent) + 'static + Send + Sync
    {
        if let Some(data) = &self.data {
            callback(ChunkEvent::ChunkLoaded { 
                chunk: data.clone(),
            });
        }
        self.subscribers.write().unwrap()
            .insert(id, Box::new(callback));
        *self.unobserved_since.write().unwrap() = None;
    }

    pub fn unsubscribe(&self, id: u32) {
        let mut subscribers = self.subscribers.write().unwrap();
        subscribers.remove(&id);
        if subscribers.len() == 0 {
            *self.unobserved_since.write().unwrap() = Some(Instant::now());
        }
    }

    pub fn time_unobserved(&self) -> Duration {
        let unobserved_since = self.unobserved_since.read().unwrap();
        match *unobserved_since {
            Some(time) => Instant::now() - time,
            None => Duration::from_secs(0),
        }
    }

    pub fn save(&self, coords: ChunkCoords) {
        if let Some(data) = &self.data {
            data.read().unwrap().save(coords);
        }
    }

    fn emit_event(&self, event: ChunkEvent) {
        let subscribers = self.subscribers.read().unwrap();
        for callback in subscribers.values() {
            callback(event.clone());
        }
    }
}