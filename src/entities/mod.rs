mod spatial_hash;
mod components;

use legion::*;
use systems::Builder;
use spatial_hash::update_spatial_hash_system;

pub use components::Position;
pub use spatial_hash::{SpatialHashMap, SpatialHash};

pub fn register(schedule: &mut Builder, resources: &mut Resources) {
    schedule
        .add_system(update_spatial_hash_system());
    resources
        .insert(SpatialHashMap::new());
}