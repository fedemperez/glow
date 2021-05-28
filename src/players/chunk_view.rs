use legion::*;
use std::collections::HashSet;
use nalgebra::Vector3;
use crate::net::{ServerEvent, PlayerConnection};
use super::Position;
use super::super::chunks::{ChunkCoords, World as Chunks};

#[system(for_each)]
pub fn send_chunks(pos: &Position, requester: &mut ChunkView, 
               conn: &mut PlayerConnection, #[resource] chunks: &mut Chunks) 
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

pub struct ChunkView {
    already_sent: HashSet<ChunkCoords>,
    last_pos: Option<Vector3<f32>>,
    range: u8,
}

impl ChunkView {
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
