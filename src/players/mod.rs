mod player_list;
mod chunk_view;
mod new_players;
mod entity_viewer;

use legion::*;
use uuid::Uuid;
use systems::{Builder, CommandBuffer};
use crate::net::{ClientEvent, ServerEvent, PlayerConnection};
use crate::util::get_time_millis;
use player_list::{PlayerList, update_player_list_system};
use chunk_view::update_chunk_view_system;
use new_players::accept_new_players_system;
use entity_viewer::send_visible_entities_system;
use crate::entities::{Position, SpatialHash, SpatialHashMap};

pub struct Name(pub String);

#[system(for_each)]
fn receive_events(entity: &Entity, conn: &mut PlayerConnection, name: &Name, 
                  position: &mut Position, cmd: &mut CommandBuffer) 
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
        }
    }
}

#[system(for_each)]
fn keepalive(conn: &PlayerConnection) {
    conn.send(ServerEvent::KeepAlive(get_time_millis()));
}

pub fn register(schedule: &mut Builder, resources: &mut Resources) {
    schedule
        .add_system(update_player_list_system())
        .add_system(receive_events_system())
        .add_system(keepalive_system())
        .add_system(send_visible_entities_system())
        .add_thread_local(update_chunk_view_system())
        .add_system(accept_new_players_system());
    resources
        .insert(PlayerList::new());
}

fn remove_player(entity: Entity, world: &mut World, resources: &mut Resources) {
    if let Some(entry) = world.entry(entity) {
        (|| {
            let mut list = resources.get_mut::<PlayerList>()?;
            let uuid = entry.get_component::<Uuid>().ok()?;
            list.remove(*uuid);
            Some(())
        })();
        (|| {
            let mut map = resources.get_mut::<SpatialHashMap>()?;
            let hash = entry.get_component::<SpatialHash>().ok()?;
            map.remove(&entity, hash);
            Some(())
        })();
    }
    world.remove(entity);
}