use legion::*;
use systems::CommandBuffer;
use nalgebra::Vector3;

use super::chunk_viewer::ChunkViewer;
use super::PlayerList;
use super::entity_viewer::EntityViewer;
use crate::buckets::EntityTracker;
use crate::entities::{Name, Position};
use crate::entities::EntityIdGenerator;
use crate::entities::Rotation;
use crate::net::Server;
use crate::events::ServerEvent;

const SPAWN_POSITION: Vector3<f32> = Vector3::new(0.0, 2.0, 0.0);

#[system]
pub fn accept_new_players(cmd: &mut CommandBuffer, #[resource] server: &mut Server, 
                         #[resource] list: &mut PlayerList,
                         #[resource] tracker: &mut EntityTracker,
                         #[resource] entity_id_gen: &EntityIdGenerator)
{
    for (uuid, name, conn) in server.get_new_players() {
        let position = SPAWN_POSITION;
        conn.send(ServerEvent::PlayerPosition(position));
        for (uuid, name) in list.get_players() {
            conn.send(ServerEvent::AddPlayer(*uuid, name.clone()));
        }
        let id = entity_id_gen.get_new();
        let entity = cmd.push((
            id,
            uuid,
            Position(position),
            Rotation(0.0, 0.0),
            Name(name.clone()), 
            conn,
            ChunkViewer::new(8),
            EntityViewer::new(),
        ));
        list.add(uuid, name);
        tracker.add(id.0, entity, &position);
    }
}