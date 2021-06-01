use std::collections::{HashSet, HashMap};
use uuid::Uuid;
use legion::*;
use world::SubWorld;
use crate::buckets::{EntityTracker, Observer};
use crate::entities::{EntityId, Position, Rotation};
use crate::net::PlayerConnection;
use crate::buckets::EntityEvent;
use crate::net::packets::play::ClientboundPacket;

const VIEW_RANGE: u32 = 6 * 16;

#[system]
#[read_component(Uuid)]
#[read_component(EntityId)]
#[read_component(Position)]
#[read_component(Rotation)]
#[read_component(PlayerConnection)]
#[write_component(Observer)]
pub fn send_visible_entities(world: &mut SubWorld, #[resource] tracker: &EntityTracker) {
    let mut pending_spawns = HashMap::new();
    let mut query = <(&Position, &PlayerConnection, &mut Observer)>::query();
    for (pos, conn, observer) in query.iter_mut(world) {
        let events = observer.update(&pos.0, tracker);
        for event in events {
            match event {
                EntityEvent::Appear { entity } => {
                    pending_spawns.entry(entity)
                        .or_insert(vec![])
                        .push(conn.get_sender());
                },
                event => { send_event(event, conn); }
            }
        }
    }
    for (entity, senders) in pending_spawns.into_iter() {
        let entry = world.entry_ref(entity).unwrap();
        let entity_id = entry.get_component::<EntityId>().unwrap().0;
        let uuid = *entry.get_component::<Uuid>().unwrap();
        let position = entry.get_component::<Position>().unwrap().0;
        let rotation = entry.get_component::<Rotation>().unwrap();
        for sender in senders {
            sender.send(ClientboundPacket::SpawnPlayer {
                entity_id,
                uuid,
                x: position.x as f64,
                y: position.y as f64,
                z: position.z as f64,
                yaw: rotation.0,
                pitch: rotation.1,
            });
        }
    }
}

fn send_event(event: EntityEvent, conn: &PlayerConnection) {
    match event {
        EntityEvent::Disappear { id } => {
            conn.send(ClientboundPacket::DestroyEntities(vec![id]));
        },
        EntityEvent::Move { id, from, to } => {
            conn.send(ClientboundPacket::EntityTeleport {
                id,
                x: to.x as f64,
                y: to.y as f64,
                z: to.z as f64,
                yaw: 0.0,
                pitch: 0.0,
                on_ground: true,
            });
        },
        EntityEvent::Rotate { id, yaw, pitch } => {
            conn.send(ClientboundPacket::EntityRotation {
                id,
                yaw,
                pitch,
                on_ground: true,
            });
        },
        EntityEvent::RotateHead { id, yaw } => {
            conn.send(ClientboundPacket::EntityHeadLook { id, yaw });
        },
        _ => panic!("Invalid event")
    }
}