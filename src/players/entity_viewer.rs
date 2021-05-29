use std::collections::HashSet;
use uuid::Uuid;
use legion::*;
use world::SubWorld;
use crate::entities::{Position, SpatialHashMap};
use crate::net::{PlayerConnection, ServerEvent};

const VIEW_RANGE: u32 = 6 * 16;

#[system]
#[read_component(Position)]
#[read_component(Uuid)]
#[write_component(EntityViewer)]
#[write_component(PlayerConnection)]
pub fn send_visible_entities(world: &mut SubWorld, #[resource] map: &SpatialHashMap) {
    let mut pending = vec![];
    let mut query = <(Entity, &Position, &mut EntityViewer, &mut PlayerConnection)>::query();
    for (this_entity, pos, viewer, conn) in query.iter_mut(world) {
        let mut visible = map.get_close_entities(&pos.0, VIEW_RANGE);
        visible.remove(this_entity);
        let seen = &visible & &viewer.last_seen;
        let new = &visible - &viewer.last_seen;
        viewer.last_seen = visible;
        pending.push((conn.get_sender(), seen, new));
    }
    for (conn, seen, new) in pending {
        for entity in seen {
            if let Ok(entity) = world.entry_ref(entity) {
                let uuid = entity.get_component::<Uuid>().unwrap();
                let position = entity.get_component::<Position>().unwrap();
                conn.send(ServerEvent::PlayerMoved(*uuid, position.0));
            }
        }
        for entity in new {
            if let Ok(entity) = world.entry_ref(entity) {
                let uuid = entity.get_component::<Uuid>().unwrap();
                let position = entity.get_component::<Position>().unwrap();
                conn.send(ServerEvent::SpawnPlayer(*uuid, position.0));
            }
        }
    }
}

pub struct EntityViewer {
    last_seen: HashSet<Entity>,
}

impl EntityViewer {
    pub fn new() -> Self {
        Self {
            last_seen: HashSet::new(),
        }
    }
}