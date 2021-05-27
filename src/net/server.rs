use crate::config::Config;
use tokio::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender, TryIter, channel};
use anyhow::{Result, anyhow};
use super::{PlayerConnection, connection::connection};
use super::handshaking::{handshaking, Intent};
use super::status::status;
use super::login::login;
use super::play::play;

pub struct Server {
    port: u16,
    motd: String,
    player_sender: Sender<(String, PlayerConnection)>,
    player_receiver: Receiver<(String, PlayerConnection)>,
}

impl Server {
    pub fn from_config(config: &Config) -> Self {
        let (player_sender, player_receiver) = channel();
        Self {
            port: config.port,
            motd: config.motd.clone(),
            player_sender,
            player_receiver,
        }
    }

    pub fn start(&self) {
        tokio::spawn(
            listen(self.port, self.motd.clone(), self.player_sender.clone())
        );
    }

    pub fn get_new_players(&mut self) -> TryIter<'_, (String, PlayerConnection)> {
        self.player_receiver.try_iter()
    }
}

async fn listen(port: u16, motd: String, sender: Sender<(String, PlayerConnection)>) -> Result<()> {
    let address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(address).await?;
    println!("Listening on port {}", port);
    loop {
        let (conn, address) = listener.accept().await?;
        println!("New connection from {}", address);
        let motd = motd.clone();
        let sender = sender.clone();
        tokio::spawn(async move {
            let result = handle_to_end(conn, &motd, sender).await;
            match result {
                Ok(_) => println!("{} disconnected", address),
                Err(e) => eprintln!("{} disconnected with error: {}", address, e)
            }
        });
    }
}

async fn handle_to_end(mut conn: TcpStream, motd: &String, sender: Sender<(String, PlayerConnection)>)
    -> Result<()> 
{
    let next_state = handshaking(&mut conn).await?;
    match next_state {
        Intent::Login => {
            let name = login(&mut conn).await?;
            let (player_conn, game_conn) = connection();
            sender.send((name, player_conn)).map_err(|e| {
                anyhow!("Couldn't send new player to the server: {:?}", e)
            })?;
            play(conn, game_conn).await?;
        },
        Intent::Status => status(&mut conn, motd).await?,
    }
    Ok(())
}