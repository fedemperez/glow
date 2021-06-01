mod components;
mod entity_id;

use legion::*;
use systems::Builder;

pub use components::{Position, Rotation, Name};
pub use entity_id::{EntityId, EntityIdGenerator};

pub fn register(schedule: &mut Builder, resources: &mut Resources) {
    resources.insert(EntityIdGenerator::new());
}