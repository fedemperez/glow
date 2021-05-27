use std::{collections::HashSet, time::Instant};

use legion::*;
use systems::{Builder, CommandBuffer};
use crate::net::{GameEvent, PlayerConnection, Server};
use nalgebra::Vector3;
use super::chunks::{Chunk, ChunkCoords, ChunkWorld};
use crate::util::get_time_millis;

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

#[system]
fn accept_new_players(cmd: &mut CommandBuffer, #[resource] server: &mut Server) {
    for (name, conn) in server.get_new_players() {
        cmd.push((
            Position::default(),
            Name(name), 
            conn,
            ChunkRequester::new(8),
        ));
    }
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
                    sender.send(GameEvent::LoadChunk(coords, chunk));
                }
                Err(e) => eprintln!("Error loading chunk: {:?}", e),
            }
        });
    }
}

#[system(for_each)]
fn keepalive(conn: &PlayerConnection) {
    conn.send(GameEvent::KeepAlive(get_time_millis()));
}

pub fn register_systems(schedule: &mut Builder) {
    schedule
        .add_system(accept_new_players_system())
        .add_system(keepalive_system())
        .add_thread_local(send_chunks_system());
}