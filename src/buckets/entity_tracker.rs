use std::collections::HashMap;
use nalgebra::Vector3;
use legion::*;
use tokio::sync::broadcast::Receiver;

use super::{bucket::Bucket, coords::BucketCoords, events::EntityEvent};

pub struct EntityTracker {
    buckets: HashMap<BucketCoords, Bucket>,
}

impl EntityTracker {
    pub fn new() -> Self {
        Self {
            buckets: HashMap::new(),
        }
    }

    pub fn add(&mut self, id: u32, entity: Entity, pos: &Vector3<f32>) {
        let coords = BucketCoords::from_pos(pos);
        let bucket = self.get_or_create(&coords);
        bucket.add(id, entity);
        bucket.send_event(EntityEvent::Appear{ entity });
    }

    pub fn remove(&mut self, id: u32, pos: &Vector3<f32>) {
        let coords = BucketCoords::from_pos(pos);
        if let Some(bucket) = self.buckets.get_mut(&coords) {
            bucket.remove(id);
            bucket.send_event(EntityEvent::Disappear{ id });
        }
    }

    pub fn move_entity(&mut self, id: u32, entity: Entity, 
        from: Vector3<f32>, to: Vector3<f32>) 
    {
        let old_coords = BucketCoords::from_pos(&from);
        let new_coords = BucketCoords::from_pos(&to);
        if old_coords == new_coords {
            self.get_or_create(&new_coords).send_event(
                EntityEvent::Move{ id, from, to }
            );
        } else {
            let old_bucket = self.get_or_create(&old_coords);
            old_bucket.remove(id);
            old_bucket.send_event(
                EntityEvent::MoveAway{ id, to: new_coords }
            );
            let new_bucket = self.get_or_create(&new_coords);
            new_bucket.add(id, entity);
            new_bucket.send_event(
                EntityEvent::MoveInto{
                    entity,
                    id,
                    old: old_coords,
                    from, to,
                }
            );
        }
    }

    pub fn send_event(&self, pos: &Vector3<f32>, event: EntityEvent) {
        let coords = BucketCoords::from_pos(pos);
        if let Some(bucket) = self.buckets.get(&coords) {
            bucket.send_event(event);
        }
    }

    pub fn subscribe(&self, coords: &BucketCoords) -> Receiver<EntityEvent> {
        self.get_or_create(coords).subscribe()
    }

    pub fn get_entities(&self, coords: &BucketCoords) -> Vec<(u32, Entity)> {
        if let Some(bucket) = self.buckets.get_mut(coords) {
            bucket.get_entities()
        } else {
            Vec::with_capacity(0)
        }
    }

    fn get_or_create(&mut self, coords: &BucketCoords) -> &mut Bucket {
        if !self.buckets.contains_key(coords) {
            self.buckets.insert(*coords, Bucket::new());
        }
        self.buckets.get_mut(coords).unwrap()
    }
}
