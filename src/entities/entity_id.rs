use std::sync::atomic::{Ordering, AtomicU32};

pub struct EntityIdGenerator {
    counter: AtomicU32,
}

impl EntityIdGenerator {
    pub fn new() -> Self {
        Self {
            counter: 0.into(),
        }
    }

    pub fn get_new(&self) -> EntityId {
        let id = self.counter.fetch_add(1, Ordering::Relaxed);
        EntityId(id)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EntityId(pub u32);