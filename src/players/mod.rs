mod player_list;
mod chunk_viewer;
mod new_players;
mod entity_viewer;
mod event_receiver;
mod remove_player;

use legion::*;
use systems::Builder;
use crate::net::PlayerConnection;
use crate::net::packets::play::ClientboundPacket;
use crate::util::get_time_millis;
use crate::events::ServerEvent;
use player_list::{PlayerList, update_player_list_system};
use chunk_viewer::update_chunk_view_system;
use new_players::accept_new_players_system;
use entity_viewer::send_visible_entities_system;
use event_receiver::receive_events_system;

#[system(for_each)]
fn keepalive(conn: &PlayerConnection) {
    conn.send(ClientboundPacket::KeepAlive(get_time_millis()));
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