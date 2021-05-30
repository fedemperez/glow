use legion::*;
use uuid::Uuid;
use super::PlayerList;
use crate::entities::{SpatialHash, SpatialHashMap};

pub fn remove_player(entity: Entity, world: &mut World, resources: &mut Resources) {
    if let Some(entry) = world.entry(entity) {
        (|| {
            let mut list = resources.get_mut::<PlayerList>()?;
            let uuid = entry.get_component::<Uuid>().ok()?;
            list.remove(*uuid);
            Some(())
        })();
        (|| {
            let mut map = resources.get_mut::<SpatialHashMap>()?;
            let hash = entry.get_component::<SpatialHash>().ok()?;
            map.remove(&entity, hash);
            Some(())
        })();
    }
    world.remove(entity);
}