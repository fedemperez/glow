use legion::*;
use tokio::sync::mpsc::UnboundedSender;
use std::collections::HashSet;
use std::sync::{Arc, RwLock};
use nalgebra::{Vector3, vector};
use crate::chunks::Chunk;
use crate::chunks::events::ChunkEvent;
use crate::net::PlayerConnection;
use crate::entities::{EntityId, Position};
use crate::chunks::{ChunkCoords, World as Chunks};
use crate::net::packets::play::ClientboundPacket;

#[system(for_each)]
pub fn update_chunk_view(id: &EntityId, pos: &Position, view: &mut ChunkViewer, 
               conn: &mut PlayerConnection, #[resource] chunks: &Chunks) 
{
    let changes = view.move_to(pos.0);
    if changes.changed_chunk {
        let ChunkCoords(chunk_x, chunk_y) = ChunkCoords::from_pos(pos.0);
        conn.send(ClientboundPacket::UpdateViewPosition(chunk_x, chunk_y));
    }
    for coords in changes.added {
        let sender = conn.get_sender();
        tokio::spawn(chunks.subscribe(coords, id.0,
            move |event| {
                handle_chunk_event(&sender, coords, event);
            }
        ));
    }
    for coords in changes.removed {
        chunks.unsubscribe(coords, id.0);
        conn.send(ClientboundPacket::UnloadChunk(coords.0, coords.1));
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
                x, y, z, block_state: new.0 as u32,
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
    in_view: HashSet<ChunkCoords>,
    last_pos: Option<Vector3<f64>>,
    range: i32,
}

impl ChunkViewer {
    pub fn new(range: i32) -> Self {
        Self {
            last_pos: None,
            range,
            in_view: HashSet::new(),
        }
    }

    pub fn move_to(&mut self, new_pos: Vector3<f64>) -> ViewMoveResult {
        let changed_chunk = match self.last_pos {
            Some(last_pos) => {
                ChunkCoords::from_pos(last_pos) != ChunkCoords::from_pos(new_pos)
            }
            None => true,
        };
        let new_view: HashSet<ChunkCoords> = 
            ChunkCoords::near(new_pos, self.range)
            .into_iter().collect();
        let added = new_view.difference(&self.in_view)
            .cloned().collect();
        let removed = self.in_view.difference(&new_view)
            .cloned().collect();
        self.last_pos = Some(new_pos);
        self.in_view = new_view;
        ViewMoveResult {
            added, removed, changed_chunk
        }
    }
}

struct ViewMoveResult {
    added: Vec<ChunkCoords>,
    removed: Vec<ChunkCoords>,
    changed_chunk: bool,
}
