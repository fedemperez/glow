use std::collections::HashMap;
use legion::Entity;
use tokio::sync::broadcast::{Sender, Receiver, channel};

use super::events::EntityEvent;

pub struct Bucket {
    entities: HashMap<u32, Entity>,
    events: Sender<EntityEvent>,
}

impl Bucket {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            events: channel(128).0,
        }
    }

    pub fn add(&mut self, id: u32, entity: Entity) {
        self.entities.insert(id, entity);
    }

    pub fn remove(&mut self, id: u32) {
        self.entities.remove(&id);
    }

    pub fn get_entities(&self) -> Vec<(u32, Entity)> {
        self.entities.iter()
            .map(|(k, v)| (*k, *v))
            .collect()
    }

    pub fn send_event(&self, event: EntityEvent) {
        self.events.send(event);
    }

    pub fn subscribe(&self) -> Receiver<EntityEvent> {
        self.events.subscribe()
    }
}