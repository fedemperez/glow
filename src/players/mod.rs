mod player_list;
mod chunk_viewer;
mod new_players;
mod entity_viewer;
mod event_receiver;
mod player_data;
mod disconnections;

use legion::*;
use systems::Builder;
use crate::net::PlayerConnection;
use crate::net::packets::play::ClientboundPacket;
use crate::util::get_time_millis;
use player_list::{PlayerList, update_player_list_system};
use chunk_viewer::update_chunk_view_system;
use new_players::{JoiningPlayerQueue, join_players_system, load_player_data_system};
use entity_viewer::send_entity_events_system;
use event_receiver::receive_events_system;
use disconnections::{DisconnectionQueue, handle_disconnections_system};

#[system(for_each)]
fn keepalive(conn: &PlayerConnection) {
    conn.send(ClientboundPacket::KeepAlive(get_time_millis()));
}

pub fn register(schedule: &mut Builder, resources: &mut Resources) {
    schedule
        .add_system(update_player_list_system())
        .add_system(receive_events_system())
        .add_system(keepalive_system())
        .add_system(send_entity_events_system())
        .add_thread_local(update_chunk_view_system())
        .add_system(join_players_system())
        .add_thread_local(load_player_data_system())
        .add_thread_local(handle_disconnections_system());
    resources.insert(PlayerList::new());
    resources.insert(JoiningPlayerQueue::new());
    resources.insert(DisconnectionQueue::new());
}