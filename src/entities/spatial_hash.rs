use std::collections::{HashMap, HashSet};
use nalgebra::Vector3;
use num_integer::Integer;
use legion::*;

use super::Position;

const BUCKET_SIZE: u32 = 16;

#[system(for_each)]
pub fn update_spatial_hash(entity: &Entity, pos: &Position, hash: &mut SpatialHash, 
                       #[resource] map: &mut SpatialHashMap) 
{
    let new_hash = SpatialHash::from_pos(&pos.0);
    if new_hash != *hash {
        map.remove(entity, hash);
        map.add(*entity, &pos.0);
        *hash = new_hash;
    }
}

pub struct SpatialHashMap {
    buckets: HashMap<SpatialHash, Bucket>,
}

impl SpatialHashMap {
    pub fn new() -> Self {
        Self {
            buckets: HashMap::new(),
        }
    }

    pub fn add(&mut self, entity: Entity, pos: &Vector3<f32>) -> SpatialHash {
        let hash = SpatialHash::from_pos(pos);
        if !self.buckets.contains_key(&hash) {
            self.buckets.insert(hash, Bucket::new());
        }
        let bucket = self.buckets.get_mut(&hash).unwrap();
        bucket.add(entity);
        hash
    }

    pub fn remove(&mut self, entity: &Entity, hash: &SpatialHash) {
        if let Some(bucket) = self.buckets.get_mut(hash) {
            bucket.remove(entity);
        }
    }

    pub fn get_close_entities(&self, from: &Vector3<f32>, aprox_distance: u32)
     -> HashSet<Entity> 
    {
        let mut result = HashSet::new();
        let bucket_distance = aprox_distance.div_ceil(&BUCKET_SIZE) as i32;
        let SpatialHash(center_x, center_z) = SpatialHash::from_pos(from);
        for delta_x in -bucket_distance..bucket_distance {
            for delta_z in -bucket_distance..bucket_distance {
                let hash = SpatialHash(center_x + delta_x, center_z + delta_z);
                if let Some(bucket) = self.buckets.get(&hash) {
                    bucket.add_to(&mut result);
                }
            }
        }
        result
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct SpatialHash(i32, i32);

impl SpatialHash {
    fn from_pos(pos: &Vector3<f32>) -> Self {
        Self((pos.x.floor() as i32).div_floor(&(BUCKET_SIZE as i32)),
             (pos.z.floor() as i32).div_floor(&(BUCKET_SIZE as i32)))
    }
}

struct Bucket(HashSet<Entity>);

impl Bucket {
    fn new() -> Self {
        Self(HashSet::new())
    }

    fn add(&mut self, entity: Entity) {
        self.0.insert(entity);
    }

    fn remove(&mut self, entity: &Entity) {
        self.0.remove(entity);
    }

    fn add_to(&self, set: &mut HashSet<Entity>) {
        set.extend(self.0.iter());
    }
}