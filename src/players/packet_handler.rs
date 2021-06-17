use legion::*;
use nalgebra::vector;
use crate::buckets::EntityTracker;
use crate::buckets::events::{EntityEvent, EntityEventData};
use crate::common::block::InteractionResult;
use crate::entities::EntityId;
use crate::net::PlayerConnection;
use crate::entities::{Position, Rotation};
use crate::net::ServerboundPacket;
use crate::chunks::World as ChunkWorld;
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
                *rotation = Rotation(yaw, pitch);
            },
            ServerboundPacket::PlayerDigging {
                status, position, face
            } => {
                match status {
                    0 => {
                        let view = chunks.get_view(position);
                        chunks.get_block(&position).destroy(&view);
                    },
                    2 => {
                        
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
                if slot != -1 {
                    let index = SlotIndex::from_network(slot as u8);
                    inventory.set_slot(index, stack);
                }
            },
            ServerboundPacket::PlayerBlockPlacement {
                hand, location, face, cursor_position, ..
            } => {
                let view = chunks.get_view(location);
                match chunks.get_block(&location).interact(&view) {
                    InteractionResult::None => {
                        if let Some(stack) = inventory.get_held() {
                            if let Some(block_type) = stack.item.get_block() {
                                block_type.place(&view, face, cursor_position, 
                                    (rotation.0, rotation.1));
                            }
                        }
                    },
                    InteractionResult::PreventPlacing => (),
                }
            },
            ServerboundPacket::Disconnect { reason } => {
                disconnections.send(*entity, reason);
            },
        }
    }
}
