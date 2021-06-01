use std::sync::mpsc::TryIter;
use crate::events::ClientEvent;
use anyhow::{Result, anyhow};
use std::sync::Mutex;
use std::sync::mpsc::{Sender, Receiver, channel};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

use super::packets::play::ClientboundPacket;

pub struct PlayerConnection {
    receiver: Mutex<Receiver<ClientEvent>>,
    sender: UnboundedSender<ClientboundPacket>,
}

impl PlayerConnection {
    pub fn send(&self, ev: ClientboundPacket) -> Result<()> {
        self.sender.send(ev).map_err(|_| {
            anyhow!("Tried to send a packet to a closed connection")
        })
    }

    pub fn get_sender(&self) -> UnboundedSender<ClientboundPacket> {
        self.sender.clone()
    }

    pub fn receive(&mut self) -> TryIter<'_, ClientEvent> {
        self.receiver.get_mut().unwrap().try_iter()
    }
}

pub struct GameConnection {
    receiver: UnboundedReceiver<ClientboundPacket>,
    sender: Sender<ClientEvent>,
}

impl GameConnection {
    pub fn into_split(self) -> (UnboundedReceiver<ClientboundPacket>, Sender<ClientEvent>) 
    {
        (self.receiver, self.sender)
    }
}

pub fn connection() -> (PlayerConnection, GameConnection) {
    let (game_send, game_recv) = unbounded_channel();
    let (player_send, player_recv) = channel();
    (
        PlayerConnection {
            receiver: Mutex::new(player_recv),
            sender: game_send,
        },
        GameConnection {
            receiver: game_recv,
            sender: player_send,
        },
    )
}