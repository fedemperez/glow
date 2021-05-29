mod spatial_hash;
mod components;
mod entity_id;

use legion::*;
use systems::Builder;
use spatial_hash::update_spatial_hash_system;

pub use components::{Position, Rotation};
pub use spatial_hash::{SpatialHashMap, SpatialHash};
pub use entity_id::{EntityId, EntityIdGenerator};

pub fn register(schedule: &mut Builder, resources: &mut Resources) {
    schedule
        .add_system(update_spatial_hash_system());
    resources.insert(SpatialHashMap::new());
    resources.insert(EntityIdGenerator::new());
}