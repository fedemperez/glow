use legion::*;
use std::collections::HashSet;
use nalgebra::Vector3;
use crate::net::{ServerEvent, PlayerConnection};
use super::Position;
use super::super::chunks::{ChunkCoords, World as Chunks};

#[system(for_each)]
pub fn update_chunk_view(pos: &Position, view: &mut ChunkView, 
               conn: &mut PlayerConnection, #[resource] chunks: &mut Chunks) 
{
    if view.changed_chunk(pos.0) {
        conn.send(ServerEvent::ChunkPosition(ChunkCoords::from_pos(pos.0)));
        let needed = view.get_needed(pos.0);
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
}

pub struct ChunkView {
    already_sent: HashSet<ChunkCoords>,
    last_pos: Option<Vector3<f32>>,
    range: i32,
}

impl ChunkView {
    pub fn new(range: i32) -> Self {
        Self {
            last_pos: None,
            range,
            already_sent: HashSet::new(),
        }
    }

    pub fn changed_chunk(&self, new_pos: Vector3<f32>) -> bool {
        match self.last_pos {
            Some(last_pos) => {
                ChunkCoords::from_pos(last_pos) != ChunkCoords::from_pos(new_pos)
            }
            None => true,
        }
    }

    pub fn get_needed(&mut self, pos: Vector3<f32>) -> Vec<ChunkCoords> {
        let mut needed = vec![];
        let ChunkCoords(x, z) = ChunkCoords::from_pos(pos);
        for delta_x in -self.range..self.range {
            for delta_z in -self.range..self.range {
                let coords = ChunkCoords(x + delta_x, z + delta_z);
                if self.already_sent.insert(coords) {
                    needed.push(coords);
                }
            }
        }
        self.last_pos = Some(pos);
        needed
    }
}
