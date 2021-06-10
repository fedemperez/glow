mod player_list;
mod chunk_viewer;
mod new_players;
mod entity_viewer;
mod packet_handler;
mod player_data;
mod disconnections;

use std::io::Write;

use legion::*;
use serde_json::json;
use systems::Builder;
use uuid::Uuid;
use crate::entities::Position;
use crate::entities::Rotation;
use crate::inventory::Inventory;
use crate::net::PlayerConnection;
use crate::net::packets::play::ClientboundPacket;
use crate::util::get_time_millis;
use player_list::{PlayerList, update_player_list_system};
use chunk_viewer::update_chunk_view_system;
use new_players::{JoiningPlayerQueue, join_players_system, load_player_data_system};
use entity_viewer::send_entity_events_system;
use packet_handler::receive_events_system;
use disconnections::{DisconnectionQueue, handle_disconnections_system};

use self::player_data::PlayerData;

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

pub async fn on_stop(world: &mut World, resources: &mut Resources) {
    print!("Saving players...        ");
    let _ = std::io::stdout().flush();
    let mut query = <(&Uuid, &Position, &Rotation, &Inventory, &PlayerConnection)>::query();
    for (uuid, pos, rot, inv, conn) in query.iter(world) {
        conn.send(ClientboundPacket::Disconnect {
            reason: json!({
                "text": "Server is closing :(",
            }),
        });
        PlayerData {
            pos: pos.0,
            rotation: (rot.0, rot.1),
            inventory: inv.clone(),
        }.save(*uuid).await
        .unwrap_or_else(|err| {
            eprintln!("Error saving player {}: {}", uuid, err);
        });
    }
    println!("Done");
}