use std::{collections::HashMap, sync::{Arc, RwLock}, time::Duration};
use nalgebra::Vector3;
use legion::*;
use tokio::sync::broadcast::Receiver;

use super::{bucket::Bucket, coords::BucketCoords, events::{EntityEvent, EntityEventData}};

const UNLOAD_TIME: Duration = Duration::from_secs(10);

#[system]
pub fn unload_buckets(#[resource] tracker: &mut EntityTracker) {
    let buckets = tracker.buckets.get_mut().unwrap();
    buckets.retain(|_, bucket| {
        let mut bucket = bucket.write().unwrap();
        bucket.time_unobserved() < UNLOAD_TIME
    });
}

pub struct EntityTracker {
    buckets: RwLock<HashMap<BucketCoords, Arc<RwLock<Bucket>>>>,
}

impl EntityTracker {
    pub fn new() -> Self {
        Self {
            buckets: RwLock::new(HashMap::new()),
        }
    }

    pub fn add(&self, id: u32, entity: Entity, pos: &Vector3<f64>) {
        let coords = BucketCoords::from_pos(pos);
        let bucket = self.get_or_create(&coords);
        let mut bucket = bucket.write().unwrap();
        bucket.add(id, entity);
        bucket.send_event(EntityEvent { 
            id,
            data: EntityEventData::Appear{ entity }
        });
    }

    pub fn remove(&self, id: u32, pos: &Vector3<f64>) {
        let coords = BucketCoords::from_pos(pos);
        if let Some(bucket) = self.buckets.read().unwrap().get(&coords) {
            let mut bucket = bucket.write().unwrap();
            bucket.remove(id);
            bucket.send_event(EntityEvent {
                id,
                data: EntityEventData::Disappear,
            });
        }
    }

    pub fn move_entity(&self, id: u32, entity: Entity, from: Vector3<f64>, to: Vector3<f64>) {
        let old_coords = BucketCoords::from_pos(&from);
        let new_coords = BucketCoords::from_pos(&to);
        if old_coords != new_coords {
            {
                let old_bucket = self.get_or_create(&old_coords);
                let mut old_bucket = old_bucket.write().unwrap();
                old_bucket.remove(id);
                old_bucket.send_event(
                    EntityEvent {
                        id,
                        data: EntityEventData::MoveAway{ to: new_coords },
                    });
            }
            let new_bucket = self.get_or_create(&new_coords);
            let mut new_bucket = new_bucket.write().unwrap();
            new_bucket.add(id, entity);
            new_bucket.send_event(
                EntityEvent {
                    id,
                    data: EntityEventData::MoveInto{ 
                        entity, 
                        from: old_coords,
                    }
                });
        }
    }

    pub fn send_event(&self, pos: &Vector3<f64>, event: EntityEvent) {
        let coords = BucketCoords::from_pos(pos);
        if let Some(bucket) = self.buckets.read().unwrap().get(&coords) {
            bucket.read().unwrap().send_event(event);
        }
    }

    pub fn subscribe(&self, coords: &BucketCoords) -> Receiver<EntityEvent> {
        self.get_or_create(coords).read().unwrap().subscribe()
    }

    pub fn get_entities(&self, coords: &BucketCoords) -> Vec<(u32, Entity)> {
        if let Some(bucket) = self.buckets.read().unwrap().get(coords) {
            bucket.read().unwrap().get_entities()
        } else {
            Vec::with_capacity(0)
        }
    }

    fn get_or_create(&self, coords: &BucketCoords) -> Arc<RwLock<Bucket>> {
        if !self.buckets.read().unwrap().contains_key(coords) {
            let bucket = Arc::new(RwLock::new(Bucket::new()));
            self.buckets.write().unwrap()
                .insert(*coords, bucket.clone());
            bucket
        } else {
            self.buckets.read().unwrap().get(coords).unwrap().clone()
        }
    }
}
