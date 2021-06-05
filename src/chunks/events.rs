use std::sync::{Arc, RwLock};

use super::Chunk;
use crate::common::block::Block;

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