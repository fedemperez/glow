use tokio::sync::RwLock;
use nalgebra::Vector3;
use uuid::Uuid;
use std::sync::Arc;

use crate::chunks::{Chunk, ChunkCoords};
use crate::entities::EntityId;

pub enum ServerEvent {
    LoadChunk(ChunkCoords, Arc<RwLock<Chunk>>),
    KeepAlive(u64),
    PlayerPosition(Vector3<f32>),
    ChunkPosition(ChunkCoords),
    AddPlayer(Uuid, String),
    RemovePlayer(Uuid),
    EntityTeleported(EntityId, Vector3<f32>, (f32, f32)),
    EntityMoved(EntityId, Vector3<f32>),
    EntityRotated(EntityId, (f32, f32)),
    EntityHeadRotated(EntityId, f32),
    DestroyEntities(Vec<EntityId>),
    SpawnPlayer(Uuid, EntityId, Vector3<f32>),
}