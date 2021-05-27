use std::{future::Future, sync::Arc};
use crate::game::Chunk;
use crate::game::ChunkCoords;
use anyhow::Result;
use anyhow::anyhow;

use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio::sync::RwLock;

pub enum PlayerEvent {

}

pub enum GameEvent {
    LoadChunk(ChunkCoords, Arc<RwLock<Chunk>>),
    KeepAlive(u64),
}

pub struct PlayerConnection {
    receiver: UnboundedReceiver<PlayerEvent>,
    sender: UnboundedSender<GameEvent>,
}

impl PlayerConnection {
    pub fn send(&self, ev: GameEvent) -> Result<()> {
        self.sender.send(ev).map_err(|e| {
            anyhow!("Tried to send game event to a closed connection")
        })
    }

    pub fn get_sender(&self) -> UnboundedSender<GameEvent> {
        self.sender.clone()
    }
}

pub struct GameConnection {
    receiver: UnboundedReceiver<GameEvent>,
    sender: UnboundedSender<PlayerEvent>,
}

impl GameConnection {
    pub fn into_split(self) -> (UnboundedReceiver<GameEvent>, UnboundedSender<PlayerEvent>) 
    {
        (self.receiver, self.sender)
    }
}

pub fn connection() -> (PlayerConnection, GameConnection) {
    let (game_send, game_recv) = unbounded_channel();
    let (player_send, player_recv) = unbounded_channel();
    (
        PlayerConnection {
            receiver: player_recv,
            sender: game_send,
        },
        GameConnection {
            receiver: game_recv,
            sender: player_send,
        },
    )
}