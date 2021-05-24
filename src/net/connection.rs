use std::sync::mpsc::{channel, Receiver, Sender};

pub enum PlayerEvent {

}

pub enum GameEvent {

}

pub struct PlayerConnection {
    receiver: Receiver<PlayerEvent>,
    sender: Sender<GameEvent>,
}

pub struct GameConnection {
    receiver: Receiver<GameEvent>,
    sender: Sender<PlayerEvent>,
}

pub fn connection() -> (PlayerConnection, GameConnection) {
    let (game_send, game_recv) = channel();
    let (player_send, player_recv) = channel();
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