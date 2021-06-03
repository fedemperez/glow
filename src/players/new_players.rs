use legion::*;
use systems::CommandBuffer;
use nalgebra::{vector, Vector3};

use super::chunk_viewer::ChunkViewer;
use super::PlayerList;
use crate::buckets::EntityTracker;
use crate::buckets::Observer;
use crate::entities::{Name, Position};
use crate::entities::EntityIdGenerator;
use crate::entities::Rotation;
use crate::net::Server;
use crate::net::packets::play::{ClientboundPacket, PlayerInfo};

const SPAWN_POSITION: Vector3<f32> = vector!(0.0, 2.0, 0.0);

#[system]
pub fn accept_new_players(cmd: &mut CommandBuffer, #[resource] server: &mut Server, 
                         #[resource] list: &mut PlayerList,
                         #[resource] tracker: &mut EntityTracker,
                         #[resource] entity_id_gen: &EntityIdGenerator)
{
    for (uuid, name, conn) in server.get_new_players() {
        let position = SPAWN_POSITION;
        conn.send(ClientboundPacket::PlayerPosition(
            position.x as f64, position.y as f64, position.z as f64));
        let players = list.get_players().into_iter().map(|(uuid, name)| (
            *uuid,
            PlayerInfo {
                name: name.clone(),
                properties: Vec::new(),
                gamemode: 0,
                ping: 0,
                display_name: None,
            }
        )).collect();
        conn.send(ClientboundPacket::PlayerInfoAddPlayers(players));
        let id = entity_id_gen.get_new();
        let entity = cmd.push((
            id,
            uuid,
            Position(position),
            Rotation(0.0, 0.0),
            Name(name.clone()), 
            conn,
            ChunkViewer::new(8),
            Observer::new(16*6),
        ));
        list.add(uuid, name);
        tracker.add(id.0, entity, &position);
    }
}