use legion::*;
use nalgebra::vector;
use crate::buckets::EntityTracker;
use crate::buckets::events::{EntityEvent, EntityEventData};
use crate::entities::EntityId;
use crate::net::PlayerConnection;
use crate::entities::{Position, Rotation};
use crate::net::packets::play::ServerboundPacket;
use crate::chunks::World as ChunkWorld;
use crate::common::block::Block;
use super::disconnections::DisconnectionQueue;
use crate::inventory::{Inventory, SlotIndex};

#[system(for_each)]
pub fn receive_events(entity: &Entity, id: &EntityId, conn: &mut PlayerConnection, 
    position: &mut Position, rotation: &mut Rotation, inventory: &mut Inventory,
    #[resource] chunks: &ChunkWorld, #[resource] disconnections: &DisconnectionQueue, 
    #[resource] tracker: &mut EntityTracker) 
{
    for event in conn.receive() {
        match event {
            ServerboundPacket::PlayerPosition { x, y, z, .. } => {
                let new_position = vector!(x, y, z);
                tracker.move_entity(id.0, *entity, position.0, new_position);
                tracker.send_event(&new_position, EntityEvent {
                    id: id.0,
                    data: EntityEventData::Move {
                        delta: new_position - position.0,
                    }
                });
                position.0 = new_position;
            },
            ServerboundPacket::PlayerRotation { yaw, pitch, .. } => {
                tracker.send_event(&position.0, 
                    EntityEvent {
                        id: id.0,
                        data: EntityEventData::Rotate{ pitch, yaw },
                    }
                );
                tracker.send_event(&position.0, 
                    EntityEvent {
                        id: id.0,
                        data: EntityEventData::RotateHead{ yaw },
                    }
                );
                *rotation = Rotation(yaw, pitch);
            },
            ServerboundPacket::PlayerPositionAndRotation {
                x, y, z, yaw, pitch, ..
            } => {
                let new_position = vector!(x, y, z);
                tracker.move_entity(id.0, *entity, position.0, new_position);
                tracker.send_event(&new_position, 
                    EntityEvent {
                        id: id.0,
                        data: EntityEventData::MoveRotate {
                            delta: new_position - position.0,
                            yaw,
                            pitch,
                        }
                    }
                );
                tracker.send_event(&position.0, 
                    EntityEvent {
                        id: id.0,
                        data: EntityEventData::RotateHead{ yaw },
                    }
                );
                position.0 = new_position;
            },
            ServerboundPacket::PlayerDigging {
                status, position: (x, y, z), face
            } => {
                match status {
                    0 => {
                        chunks.set_block(x, y, z, 
                            Block::from_name("minecraft:air").unwrap());
                    },
                    2 => {
                        chunks.set_block(x, y, z, 
                            Block::from_name("minecraft:air").unwrap());
                    },
                    _ => (),
                }
            },
            ServerboundPacket::HeldItemChange { slot } => {
                inventory.set_held_slot(
                    SlotIndex::from_hotbar(slot as u8));
            },
            ServerboundPacket::CreativeInventoryAction {
                slot, stack
            } => {
                let index = SlotIndex::from_network(slot as u8);
                inventory.set_slot(index, stack);
            },
            ServerboundPacket::PlayerBlockPlacement {
                hand, location, face, ..
            } => {
                let (x, y, z) = face.get_adjacent(location);
                if let Some(item) = inventory.get_held() {
                    if let Some(block) = 
                        Block::from_name(item.id.to_str().unwrap()) 
                    {
                        chunks.set_block(x, y, z, block)
                    }
                }
            },
            ServerboundPacket::Disconnect { reason } => {
                disconnections.send(*entity, reason);
            },
        }
    }
}