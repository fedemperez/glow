use std::collections::{HashSet, HashMap};
use uuid::Uuid;
use legion::*;
use world::SubWorld;
use crate::buckets::{EntityTracker, Observer};
use crate::buckets::events::{EntityEvent, EntityEventData};
use crate::entities::{EntityId, Position, Rotation};
use crate::net::PlayerConnection;
use crate::net::packets::play::ClientboundPacket;

const VIEW_RANGE: u32 = 6 * 16;

#[system]
#[read_component(Uuid)]
#[read_component(EntityId)]
#[read_component(Position)]
#[read_component(Rotation)]
#[read_component(PlayerConnection)]
#[write_component(Observer)]
pub fn send_entity_events(world: &mut SubWorld, #[resource] tracker: &EntityTracker) {
    let mut pending_spawns = HashMap::new();
    let mut query = <(&EntityId, &Position, &PlayerConnection, &mut Observer)>::query();
    for (player_id, pos, conn, observer) in query.iter_mut(world) {
        let events = observer.update(&pos.0, tracker);
        for event in events {
            if event.id != player_id.0 {
                match event {
                    EntityEvent{
                        data: EntityEventData::Appear { entity }, ..
                    } => {
                        pending_spawns.entry(entity)
                            .or_insert(vec![])
                            .push(conn.get_sender());
                    },
                    event => { send_event(event, conn); }
                }
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
    let EntityEvent{ id, data } = event;
    match data {
        EntityEventData::Disappear => {
            conn.send(ClientboundPacket::DestroyEntities(vec![id]));
        },
        EntityEventData::Move { from, to } => {
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
        EntityEventData::Rotate { yaw, pitch } => {
            conn.send(ClientboundPacket::EntityRotation {
                id,
                yaw,
                pitch,
                on_ground: true,
            });
        },
        EntityEventData::RotateHead { yaw } => {
            conn.send(ClientboundPacket::EntityHeadLook { id, yaw });
        },
        _ => panic!("Invalid event")
    }
}