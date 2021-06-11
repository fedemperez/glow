use crate::serialization::CompactLong;
use super::CHUNK_WIDTH;
use std::collections::HashMap;

pub struct HeightMap {
    motion_blocking: CompactLong,
}

impl HeightMap {
    pub fn new() -> Self {
        let heights = vec![0; CHUNK_WIDTH * CHUNK_WIDTH];
        Self {
            motion_blocking: 
                CompactLong::from_values(&heights, 9),
        }
    }

    pub fn get_nbt(&self) -> nbt::Value {
        let mut map = HashMap::new();
        map.insert("MOTION_BLOCKING".into(), 
            nbt::Value::LongArray(self.motion_blocking.longs.clone()));
        nbt::Value::Compound(map)
    }
}