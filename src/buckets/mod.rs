mod bucket;
mod coords;
mod events;
mod entity_tracker;
mod observer;

use legion::{Resources, systems::Builder};

pub use entity_tracker::EntityTracker;
pub use observer::Observer;
pub use events::EntityEvent;

pub fn register(schedule: &mut Builder, resources: &mut Resources) {
    resources.insert(EntityTracker::new());
}