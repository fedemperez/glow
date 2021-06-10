use std::sync::{Arc, RwLock};

use super::ChunkData;
use crate::common::block::Block;

#[derive(Clone)]
pub enum ChunkEvent {
    ChunkLoaded {
        chunk: Arc<RwLock<ChunkData>>,
    },
    BlockChanged {
        x: usize,
        y: usize,
        z: usize,
        new: Block,
    },
}