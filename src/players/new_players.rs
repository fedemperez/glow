use legion::*;
use systems::CommandBuffer;
use nalgebra::Vector3;

use super::chunk_view::ChunkView;
use super::PlayerList;
use super::{Position, Name};
use crate::entities::SpatialHashMap;
use crate::net::{Server, ServerEvent};

const SPAWN_POSITION: Vector3<f32> = Vector3::new(0.0, 2.0, 0.0);

#[system]
pub fn accept_new_players(cmd: &mut CommandBuffer, #[resource] server: &mut Server, 
                         #[resource] list: &mut PlayerList,
                         #[resource] entity_map: &mut SpatialHashMap)
{
    for (uuid, name, conn) in server.get_new_players() {
        let position = SPAWN_POSITION;
        conn.send(ServerEvent::PlayerPosition(position));
        for (uuid, name) in list.get_players() {
            conn.send(ServerEvent::AddPlayer(*uuid, name.clone()));
        }
        let entity = cmd.push((
            uuid,
            Position(SPAWN_POSITION),
            Name(name.clone()), 
            conn,
            ChunkView::new(8),
        ));
        list.add(uuid, name);
        let space_hash = entity_map.add(entity, &position);
        cmd.add_component(entity, space_hash);
    }
}