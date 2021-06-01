use legion::*;
use uuid::Uuid;
use crate::{buckets::EntityTracker, entities::{EntityId, Position}};

use super::PlayerList;

pub fn remove_player(entity: Entity, world: &mut World, resources: &mut Resources) {
    if let Some(entry) = world.entry(entity) {
        (|| {
            let mut list = resources.get_mut::<PlayerList>()?;
            let uuid = entry.get_component::<Uuid>().ok()?;
            list.remove(*uuid);
            Some(())
        })();
        (|| {
            let mut tracker = resources.get_mut::<EntityTracker>()?;
            let id = entry.get_component::<EntityId>().ok()?;
            let pos = entry.get_component::<Position>().ok()?;
            tracker.remove(id.0, &pos.0);
            Some(())
        })();
    }
    world.remove(entity);
}