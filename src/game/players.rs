use std::{collections::HashSet, time::Instant};

use legion::*;
use world::SubWorld;
use systems::{Builder, CommandBuffer};
use crate::net::{ClientEvent, ServerEvent, PlayerConnection, Server};
use nalgebra::Vector3;
use super::{chunks::{Chunk, ChunkCoords, ChunkWorld}, player_list::{PlayerList, PlayerListUpdate}};
use crate::util::get_time_millis;

const SPAWN_POSITION: Vector3<f32> = Vector3::new(0.0, 2.0, 0.0);

#[derive(Clone, Copy, Debug, Default)]
struct Position(Vector3<f32>);

struct Name(String);

struct ChunkRequester {
    already_sent: HashSet<ChunkCoords>,
    last_pos: Option<Vector3<f32>>,
    range: u8,
}

impl ChunkRequester {
    pub fn new(range: u8) -> Self {
        Self {
            last_pos: None,
            range,
            already_sent: HashSet::new(),
        }
    }

    fn changed_chunk(&self, new_pos: Vector3<f32>) -> bool {
        match self.last_pos {
            Some(last_pos) => {
                ChunkCoords::from_pos(last_pos) != ChunkCoords::from_pos(new_pos)
            }
            None => true,
        }
    }

    pub fn get_needed(&mut self, pos: Vector3<f32>) -> Vec<ChunkCoords> {
        let mut needed = vec![];
        if self.changed_chunk(pos) {
            for x in -8..8 {
                for z in -8..8 {
                    let coords = ChunkCoords(x, z);
                    if self.already_sent.insert(coords) {
                        needed.push(coords);
                    }
                }
            }
        }
        self.last_pos = Some(pos);
        needed
    }
}

#[system(for_each)]
fn receive_events(entity: &Entity, conn: &mut PlayerConnection, 
                name: &Name, cmd: &mut CommandBuffer) {
    for event in conn.receive() {
        match event {
            ClientEvent::Disconnect(reason) => {
                println!("{} disconnected, reason: {}", name.0, reason);
                cmd.remove(*entity);
            }
        }
    }
}

#[system]
fn accept_new_players(cmd: &mut CommandBuffer, #[resource] server: &mut Server, 
    #[resource] list: &mut PlayerList)
{
    for (name, conn) in server.get_new_players() {
        conn.send(ServerEvent::PlayerPosition(SPAWN_POSITION));
        cmd.push((
            Position(SPAWN_POSITION),
            Name(name.clone()), 
            conn,
            ChunkRequester::new(8),
        ));
        list.add(name);
    }
}

#[system(for_each)]
fn keepalive(conn: &PlayerConnection) {
    conn.send(ServerEvent::KeepAlive(get_time_millis()));
}

#[system(for_each)]
fn send_chunks(pos: &Position, requester: &mut ChunkRequester, 
                conn: &mut PlayerConnection, #[resource] chunks: &mut ChunkWorld) 
{
    let needed = requester.get_needed(pos.0);
    for coords in needed {
        let future = chunks.get_chunk(coords);
        let sender = conn.get_sender();
        tokio::spawn(async move {
            match future.await {
                Ok(chunk) => {
                    sender.send(ServerEvent::LoadChunk(coords, chunk));
                }
                Err(e) => eprintln!("Error loading chunk: {:?}", e),
            }
        });
    }
}

#[system]
#[read_component(PlayerConnection)]
fn update_player_list(world: &SubWorld, #[resource] list: &mut PlayerList, 
    #[resource] server: &mut Server)
{
    for update in list.flush_updates() {
        let mut query = <(&PlayerConnection,)>::query();
        query.for_each(world, |(conn,)| {
            match &update {
                PlayerListUpdate::Add(name) => {
                    conn.send(ServerEvent::PlayerJoined(name.clone()));
                }
                PlayerListUpdate::Remove(name) => {

                }
            }
        });
        match update {
            PlayerListUpdate::Add(name) => {
                server.add_player(name)
            }
            PlayerListUpdate::Remove(name) => {

            }
        }
    }
}

pub fn register_systems(schedule: &mut Builder) {
    schedule
        .add_system(receive_events_system())
        .add_system(accept_new_players_system())
        .add_system(keepalive_system())
        .add_system(update_player_list_system())
        .add_thread_local(send_chunks_system());
}