use crate::blocks::Block;
use super::ChunkCoords;
use super::ChunkData;
use super::WorldView;
use super::events::ChunkEvent;
use super::saving::ChunkSaver;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;
use std::time::Instant;
use nalgebra::{Vector3, vector};
use rand::{Rng, thread_rng};

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

    pub fn get_block(&self, x: usize, y: usize, z: usize) -> &'static Block {
        match &self.data {
            Some(data) => data.read().unwrap().get_block(x, y, z),
            None => Block::air(),
        }
    }

    pub fn set_block(&self, x: usize, y: usize, z: usize, block: &'static Block) {
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

    pub fn random_tick(&self, view: &WorldView) {
        let mut rng = thread_rng();
        let bitmask = self.data.as_ref().map(|data| 
            data.read().unwrap()
            .get_sections_bitmask());
        if let Some(mut bitmask) = bitmask {
            for section in 0..16 {
                if bitmask & 1 == 1 {
                    for _tick in 0..3 {
                        let y_delta = section * 16;
                        let x = rng.gen_range(0..16);
                        let y = rng.gen_range(0..16);
                        let z = rng.gen_range(0..16);
                        let coords = vector!(x, y + y_delta, z);
                        let mut view = view.clone();
                        view.displace(coords);
                        self.get_block(
                            x as usize, 
                            (y + y_delta) as usize, 
                            z as usize
                        ).random_tick(&view);
                    }
                }
                bitmask >>= 1;
            }
        }
    }

    pub fn save(&self, coords: ChunkCoords, saver: &mut ChunkSaver) {
        if let Some(data) = &self.data {
            saver.save(coords, data.clone());
        }
    }

    fn emit_event(&self, event: ChunkEvent) {
        let subscribers = self.subscribers.read().unwrap();
        for callback in subscribers.values() {
            callback(event.clone());
        }
    }
}