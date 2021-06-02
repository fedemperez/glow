mod bucket;
mod coords;
pub mod events;
mod entity_tracker;
mod observer;

use legion::{Resources, systems::Builder};

pub use entity_tracker::EntityTracker;
pub use observer::Observer;

pub fn register(schedule: &mut Builder, resources: &mut Resources) {
    resources.insert(EntityTracker::new());
}