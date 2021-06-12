mod bucket;
mod coords;
pub mod events;
mod entity_tracker;
mod observer;

use legion::{Resources, systems::Builder};
use entity_tracker::unload_buckets_system;
pub use entity_tracker::EntityTracker;
pub use observer::Observer;

pub fn register(schedule: &mut Builder, resources: &mut Resources) {
    schedule.add_system(unload_buckets_system());
    resources.insert(EntityTracker::new());
}