use std::sync::mpsc::TryIter;
use crate::events::{ClientEvent, ServerEvent};
use anyhow::{Result, anyhow};
use std::sync::Mutex;
use std::sync::mpsc::{Sender, Receiver, channel};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};

pub struct PlayerConnection {
    receiver: Mutex<Receiver<ClientEvent>>,
    sender: UnboundedSender<ServerEvent>,
}

impl PlayerConnection {
    pub fn send(&self, ev: ServerEvent) -> Result<()> {
        self.sender.send(ev).map_err(|_| {
            anyhow!("Tried to send an event to a closed connection")
        })
    }

    pub fn get_sender(&self) -> UnboundedSender<ServerEvent> {
        self.sender.clone()
    }

    pub fn receive(&mut self) -> TryIter<'_, ClientEvent> {
        self.receiver.get_mut().unwrap().try_iter()
    }
}

pub struct GameConnection {
    receiver: UnboundedReceiver<ServerEvent>,
    sender: Sender<ClientEvent>,
}

impl GameConnection {
    pub fn into_split(self) -> (UnboundedReceiver<ServerEvent>, Sender<ClientEvent>) 
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