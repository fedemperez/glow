use std::sync::{Arc, RwLock};

use super::{Block, Chunk};

#[derive(Clone)]
pub enum ChunkEvent {
    ChunkLoaded {
        chunk: Arc<RwLock<Chunk>>,
    },
    BlockChanged {
        x: usize,
        y: usize,
        z: usize,
        new: Block,
    },
}