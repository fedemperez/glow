use std::{collections::HashMap, time::{Duration, Instant}};
use legion::Entity;
use tokio::sync::broadcast::{Sender, Receiver, channel};

use super::events::EntityEvent;

pub struct Bucket {
    entities: HashMap<u32, Entity>,
    events: Sender<EntityEvent>,
    last_observed: Option<Instant>,
}

impl Bucket {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            events: channel(64).0,
            last_observed: None,
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

    pub fn subscribe(&self, ) -> Receiver<EntityEvent> {
        self.events.subscribe()
    }

    pub fn time_unobserved(&mut self) -> Duration {
        self.update_observer_count();
        let time = self.last_observed.unwrap_or(Instant::now());
        Instant::now() - time
    }

    fn update_observer_count(&mut self) {
        if self.events.receiver_count() == 0 {
            if self.last_observed.is_none() {
                self.last_observed = Some(Instant::now());
            }
        } else {
            self.last_observed = None;
        }
    }
}
