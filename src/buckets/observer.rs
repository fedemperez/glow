use std::collections::HashMap;

use nalgebra::Vector3;
use tokio::sync::broadcast::Receiver;

use super::{
    coords::BucketCoords, 
    events::EntityEvent, 
    entity_tracker::EntityTracker
};

pub struct Observer {
    coords: BucketCoords,
    distance: u32,
    observed: HashMap<BucketCoords, Receiver<EntityEvent>>,
}

impl Observer {
    pub fn new(pos: &Vector3<f32>, distance: u32) -> Self {
        Self {
            coords: BucketCoords::from_pos(pos),
            distance,
            observed: HashMap::new(),
        }
    }

    pub fn update(&mut self, pos: &Vector3<f32>, tracker: &EntityTracker)
        -> Vec<EntityEvent>
    {
        let mut events = vec![];
        for receiver in self.observed.values_mut() {
            while let Ok(event) = receiver.try_recv() {
                events.push(event);
            }
        }
        events = events.into_iter().filter_map(|event|
            match event {
                EntityEvent::MoveInto{ entity, id, old, from, to } => {
                    if !self.observed.contains_key(&old) {
                        Some(EntityEvent::Appear{ entity })
                    } else {
                        Some(EntityEvent::Move{ id, from, to })
                    }
                }
                EntityEvent::MoveAway{ id, to } => {
                    if !self.observed.contains_key(&to) {
                        Some(EntityEvent::Disappear{ id })
                    } else {
                        None
                    }
                }
                event => Some(event),
            }
        ).collect();
        self.move_to(pos, tracker, &mut events);
        events
    }

    fn move_to(&mut self, pos: &Vector3<f32>, tracker: &EntityTracker, 
        events: &mut Vec<EntityEvent>) 
    {
        let new_coords = BucketCoords::from_pos(pos);
        if new_coords != self.coords {
            let mut new_observed = HashMap::new();
            for coords in new_coords.get_close(self.distance) {
                let receiver = match self.observed.remove(&coords) {
                    Some(r) => r,
                    None => {
                        self.add_bucket(&coords, tracker, events);
                        tracker.subscribe(&coords)
                    },
                };
                new_observed.insert(coords, receiver);
            }
            for coords in self.observed.keys() {
                self.remove_bucket(coords, tracker, events);
            }
            self.coords = new_coords;
            self.observed = new_observed;
        }
    }

    fn add_bucket(&self, coords: &BucketCoords, tracker: &EntityTracker,
        events: &mut Vec<EntityEvent>) 
    {
        for (_, entity) in tracker.get_entities(coords) {
            events.push(EntityEvent::Appear { entity });
        }
    }

    fn remove_bucket(&self, coords: &BucketCoords, tracker: &EntityTracker,
        events: &mut Vec<EntityEvent>) 
    {
        for (id, _) in tracker.get_entities(coords) {
            events.push(EntityEvent::Disappear { id });
        }
    }
}