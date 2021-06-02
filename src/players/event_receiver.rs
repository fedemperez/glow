use legion::*;
use systems::{CommandBuffer};
use crate::buckets::EntityTracker;
use crate::buckets::events::{EntityEvent, EntityEventData};
use crate::entities::EntityId;
use crate::net::PlayerConnection;
use crate::entities::{Position, Rotation};
use crate::events::ClientEvent;
use crate::chunks::{Block, World as ChunkWorld};
use super::remove_player::remove_player;
use crate::entities::Name;

#[system(for_each)]
pub fn receive_events(entity: &Entity, id: &EntityId, conn: &mut PlayerConnection, 
    name: &Name, position: &mut Position, rotation: &mut Rotation, cmd: &mut CommandBuffer, 
    #[resource] chunks: &ChunkWorld, #[resource] tracker: &mut EntityTracker) 
{
    for event in conn.receive() {
        match event {
            ClientEvent::Disconnect(reason) => {
                println!("{} disconnected, reason: {}", name.0, reason);
                let entity = *entity;
                cmd.exec_mut(move |world, resources| {
                    remove_player(entity, world, resources);
                });
            }
            ClientEvent::Move(new_pos) => {
                tracker.move_entity(id.0, *entity, position.0, new_pos);
                position.0 = new_pos;
            }
            ClientEvent::Rotate(yaw, pitch) => {
                tracker.send_event(&position.0, 
                    EntityEvent {
                        id: id.0,
                        data: EntityEventData::Rotate{ pitch, yaw },
                    }
                );
                *rotation = Rotation(yaw, pitch);
            }
            ClientEvent::BreakBlock(x, y, z) => {
                chunks.set_block(x, y, z, Block::Air);
            }
        }
    }
}