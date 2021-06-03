use legion::*;
use tokio::sync::mpsc::UnboundedSender;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use nalgebra::Vector3;
use crate::chunks::Chunk;
use crate::chunks::events::ChunkEvent;
use crate::net::PlayerConnection;
use crate::entities::Position;
use crate::chunks::{ChunkCoords, World as Chunks};
use crate::net::packets::play::ClientboundPacket;

#[system(for_each)]
pub fn update_chunk_view(pos: &Position, view: &mut ChunkViewer, 
               conn: &mut PlayerConnection, #[resource] chunks: &Chunks) 
{
    if view.changed_chunk(pos.0) {
        let ChunkCoords(chunk_x, chunk_y) = ChunkCoords::from_pos(pos.0);
        conn.send(ClientboundPacket::UpdateViewPosition(chunk_x, chunk_y));
        let needed = view.get_needed(pos.0);
        for coords in needed {
            let sender = conn.get_sender();
            tokio::spawn(chunks.subscribe(coords, 
                move |event| {
                    handle_chunk_event(&sender, coords, event);
                }
            ));
        }
    }
}

fn handle_chunk_event(sender: &UnboundedSender<ClientboundPacket>, 
    coords: ChunkCoords, event: ChunkEvent)
{
    match event {
        ChunkEvent::ChunkLoaded { chunk } 
            => send_chunk(&sender, coords, chunk),
        ChunkEvent::BlockChanged { x, y, z, new } => {
            let (x, y, z) = coords.global(x, y, z);
            sender.send(ClientboundPacket::BlockChange {
                x, y, z, block_state: new.get_id() as u32,
            });
        },
    }
}

fn send_chunk(sender: &UnboundedSender<ClientboundPacket>, 
    coords: ChunkCoords, chunk: Arc<RwLock<Chunk>>)
{
    let chunk = chunk.read().unwrap();
    sender.send(ClientboundPacket::ChunkData{
        x: coords.0,
        z: coords.1,
        full: true,
        bitmask: chunk.get_sections_bitmask(),
        heightmap: chunk.get_heightmap(),
        biomes: Some(chunk.get_biome_map()),
        data: chunk.get_data(),
        block_entities: vec![],
    });
}

pub struct ChunkViewer {
    already_sent: HashSet<ChunkCoords>,
    last_pos: Option<Vector3<f64>>,
    range: i32,
}

impl ChunkViewer {
    pub fn new(range: i32) -> Self {
        Self {
            last_pos: None,
            range,
            already_sent: HashSet::new(),
        }
    }

    pub fn changed_chunk(&self, new_pos: Vector3<f64>) -> bool {
        match self.last_pos {
            Some(last_pos) => {
                ChunkCoords::from_pos(last_pos) != ChunkCoords::from_pos(new_pos)
            }
            None => true,
        }
    }

    pub fn get_needed(&mut self, pos: Vector3<f64>) -> Vec<ChunkCoords> {
        let mut needed = vec![];
        let around = ChunkCoords::from_pos(pos).get_close(self.range);
        for coords in around {
            if self.already_sent.insert(coords) {
                needed.push(coords);
            }
        }
        self.last_pos = Some(pos);
        needed
    }
}
