use legion::*;
use systems::{CommandBuffer};
use crate::net::PlayerConnection;
use crate::entities::{Position, Rotation};
use crate::events::ClientEvent;
use super::remove_player::remove_player;
use crate::entities::Name;

#[system(for_each)]
pub fn receive_events(entity: &Entity, conn: &mut PlayerConnection, name: &Name, 
                  position: &mut Position, rotation: &mut Rotation,
                  cmd: &mut CommandBuffer) 
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
                position.0 = new_pos;
            }
            ClientEvent::Rotate(yaw, pitch) => {
                *rotation = Rotation(yaw, pitch);
            }
            ClientEvent::BreakBlock(x, y, z) => {
                println!("Alguien rompio un bloquesito en {}, {}, {}", x, y, z);
            }
        }
    }
}