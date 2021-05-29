mod player_list;
mod chunk_view;
mod new_players;
mod entity_viewer;

use legion::*;
use uuid::Uuid;
use world::SubWorld;
use systems::{Builder, CommandBuffer};
use crate::net::{ClientEvent, ServerEvent, PlayerConnection, Server};
use crate::util::get_time_millis;
use player_list::{PlayerList, PlayerListUpdate};
use chunk_view::update_chunk_view_system;
use new_players::accept_new_players_system;
use entity_viewer::send_visible_entities_system;
use crate::entities::{Position, SpatialHash, SpatialHashMap};

pub struct Name(pub String);

#[system(for_each)]
fn receive_events(entity: &Entity, conn: &mut PlayerConnection, uuid: &Uuid, name: &Name, 
                  space_hash: &mut SpatialHash, position: &mut Position, cmd: &mut CommandBuffer, 
                  #[resource] list: &mut PlayerList, #[resource] entity_map: &mut SpatialHashMap) 
{
    for event in conn.receive() {
        match event {
            ClientEvent::Disconnect(reason) => {
                println!("{} disconnected, reason: {}", name.0, reason);
                cmd.remove(*entity);
                list.remove(*uuid);
                entity_map.remove(entity, space_hash);
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

#[system]
#[read_component(PlayerConnection)]
fn update_player_list(world: &SubWorld, #[resource] list: &mut PlayerList, 
                      #[resource] server: &mut Server)
{
    let updates = list.flush_updates();
    if updates.len() > 0 {
        server.update_list(list.count(), list.get_sample());
        for update in updates {
            let mut query = <(&PlayerConnection,)>::query();
            query.for_each(world, |(conn,)| {
                match &update {
                    PlayerListUpdate::Add(uuid, name) => {
                        conn.send(ServerEvent::AddPlayer(*uuid, name.clone()));
                    }
                    PlayerListUpdate::Remove(uuid) => {
                        conn.send(ServerEvent::RemovePlayer(*uuid));
                    }
                }
            });
        }
    }
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