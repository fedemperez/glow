use legion::*;
use nalgebra::vector;
use systems::{CommandBuffer};
use crate::buckets::EntityTracker;
use crate::buckets::events::{EntityEvent, EntityEventData};
use crate::entities::EntityId;
use crate::net::PlayerConnection;
use crate::entities::{Position, Rotation};
use crate::net::packets::play::ServerboundPacket;
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
            ServerboundPacket::PlayerPosition { x, y, z, .. } => {
                let new_pos = vector!(x as f32, y as f32, z as f32);
                tracker.move_entity(id.0, *entity, position.0, new_pos);
                position.0 = new_pos;
            },
            ServerboundPacket::PlayerRotation { yaw, pitch, .. } => {
                tracker.send_event(&position.0, 
                    EntityEvent {
                        id: id.0,
                        data: EntityEventData::Rotate{ pitch, yaw },
                    }
                );
                *rotation = Rotation(yaw, pitch);
            },
            ServerboundPacket::PlayerPositionAndRotation {
                x, y, z, yaw, pitch, ..
            } => {

            },
            ServerboundPacket::PlayerDigging {
                status, position: (x, y, z), face
            } => {
                match status {
                    2 => chunks.set_block(x, y, z, Block::Air),
                    _ => (),
                }
            },
            ServerboundPacket::Disconnect { message } => {
                println!("{} disconnected, reason: {}", name.0, message);
                let entity = *entity;
                cmd.exec_mut(move |world, resources| {
                    remove_player(entity, world, resources);
                });
            },
        }
    }
}