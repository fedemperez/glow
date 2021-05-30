use std::collections::{HashSet, HashMap};
use uuid::Uuid;
use legion::*;
use world::SubWorld;
use crate::entities::{EntityId, Position, Rotation, SpatialHashMap};
use crate::net::PlayerConnection;
use crate::events::ServerEvent;

const VIEW_RANGE: u32 = 6 * 16;

#[system]
#[read_component(Position)]
#[read_component(Rotation)]
#[read_component(Uuid)]
#[read_component(EntityId)]
#[write_component(EntityViewer)]
#[write_component(PlayerConnection)]
pub fn send_visible_entities(world: &mut SubWorld, #[resource] map: &SpatialHashMap) {
    let mut pending = vec![];
    let mut query = <(Entity, &Position, &mut EntityViewer, &mut PlayerConnection)>::query();
    for (this_entity, pos, viewer, conn) in query.iter_mut(world) {
        let mut visible = map.get_close_entities(&pos.0, VIEW_RANGE);
        visible.remove(this_entity);
        pending.push((*this_entity, visible));
    }
    for (viewer, entities) in pending {
        let viewer_entry = world.entry_ref(viewer).unwrap();
        let connection = viewer_entry.get_component::<PlayerConnection>().unwrap()
                            .get_sender();
        let mut already_seen = viewer_entry.get_component::<EntityViewer>().unwrap()
                                .last_seen.clone();
        let mut new_map = HashMap::new();
        for entity in entities {
            let entry = world.entry_ref(entity).unwrap();
            let id = entry.get_component::<EntityId>().unwrap();
            let position = entry.get_component::<Position>().unwrap();
            let rotation = entry.get_component::<Rotation>().unwrap();
            if already_seen.contains_key(&entity) {
                connection.send(ServerEvent::EntityTeleported(*id, position.0, (rotation.0, rotation.1)));
                connection.send(ServerEvent::EntityHeadRotated(*id, rotation.0));
                already_seen.remove(&entity);
            } else {
                let uuid = entry.get_component::<Uuid>().unwrap();
                connection.send(ServerEvent::SpawnPlayer(*uuid, *id, position.0));
            }
            new_map.insert(entity, *id);
        }
        let removed = already_seen.values().map(|v| *v).collect();
        connection.send(ServerEvent::DestroyEntities(removed));
        world.entry_mut(viewer).unwrap()
            .get_component_mut::<EntityViewer>().unwrap()
            .last_seen = new_map;
    }
}

pub struct EntityViewer {
    last_seen: HashMap<Entity, EntityId>,
}

impl EntityViewer {
    pub fn new() -> Self {
        Self {
            last_seen: HashMap::new(),
        }
    }
}