use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::sync::mpsc::channel;

use legion::*;
use systems::CommandBuffer;
use nalgebra::vector;
use uuid::Uuid;

use super::chunk_viewer::ChunkViewer;
use super::PlayerList;
use super::player_data::PlayerData;
use crate::buckets::EntityTracker;
use crate::buckets::Observer;
use crate::entities::{Name, Position};
use crate::entities::EntityIdGenerator;
use crate::entities::Rotation;
use crate::net::PlayerConnection;
use crate::net::Server;
use crate::net::packets::play::ClientboundPacket;

pub struct JoiningPlayer {
    uuid: Uuid,
    name: String,
    conn: PlayerConnection,
    data: PlayerData,
}

pub struct JoiningPlayerQueue {
    sender: Sender<JoiningPlayer>,
    receiver: Receiver<JoiningPlayer>,
}

impl JoiningPlayerQueue {
    pub fn new() -> Self {
        let (sender, receiver) = channel();
        Self {
            sender, receiver
        }
    }
}

#[system]
pub fn join_players(cmd: &mut CommandBuffer, #[resource] queue: &mut JoiningPlayerQueue, 
    #[resource] list: &mut PlayerList, #[resource] tracker: &mut EntityTracker,
    #[resource] entity_id_gen: &EntityIdGenerator)
{
    for JoiningPlayer{ uuid, name, conn, data } in queue.receiver.try_iter() {
        conn.send(ClientboundPacket::PlayerPosition(
            data.pos.x, data.pos.y, data.pos.z));
        list.send_player(&conn.get_sender());
        let id = entity_id_gen.get_new();
        list.add(uuid, name.clone());
        let entity = cmd.push((
            id,
            uuid,
            Position(data.pos),
            Rotation(data.rotation.0, data.rotation.1),
            Name(name), 
            conn,
            ChunkViewer::new(8),
            Observer::new(16*6),
        ));
        tracker.add(id.0, entity, &data.pos);
    }
}

#[system]
pub fn load_player_data(#[resource] server: &mut Server, 
    #[resource] queue: &JoiningPlayerQueue)
{
    for (uuid, name, conn) in server.get_new_players() {
        let sender = queue.sender.clone();
        tokio::spawn(async move {
            let data = PlayerData::load(uuid).await
                .unwrap_or_else(|_| { 
                    gen_new_player() 
                });
            sender.send(JoiningPlayer {
                uuid, name, conn, data
            })
        });
    }
}

fn gen_new_player() -> PlayerData {
    PlayerData {
        pos: vector!(0.0, 2.0, 0.0),
        rotation: (0.0, 0.0),
    }
}
