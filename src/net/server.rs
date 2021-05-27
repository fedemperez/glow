use crate::config::Config;
use crate::net::status_gen::gen_status_str;
use tokio::net::{TcpListener, TcpStream};
use std::sync::mpsc::{Receiver, Sender, TryIter, channel};
use anyhow::{Result, anyhow};
use super::{PlayerConnection, connection::connection};
use super::handshaking::{handshaking, Intent};
use super::status::status;
use super::login::login;
use super::play::play;

pub struct Server {
    player_recv: Receiver<(String, PlayerConnection)>,
    list_send: Sender<String>,
}

impl Server {
    pub fn start(config: &Config) -> Self {
        let (player_send, player_recv) = channel();
        let (list_send, list_recv) = channel();
        tokio::spawn(listen(
            config.port, 
            config.motd.clone(), 
            player_send, 
            list_recv
        ));
        Self {
            player_recv,
            list_send,
        }
    }

    pub fn get_new_players(&mut self) -> TryIter<'_, (String, PlayerConnection)> {
        self.player_recv.try_iter()
    }

    pub fn add_player(&mut self, name: String) {
        self.list_send.send(name);
    }
}

async fn listen(port: u16, motd: String, player_sender: Sender<(String, PlayerConnection)>, 
    list_updates: Receiver<String>) -> Result<()> 
{
    let address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(address).await?;
    println!("Listening on port {}", port);
    let mut player_list = vec![];
    loop {
        let (conn, address) = listener.accept().await?;
        println!("New connection from {}", address);
        for name in list_updates.try_iter() {
            player_list.push(name);
        }
        let status = gen_status_str(&motd, &player_list);
        let player_sender = player_sender.clone();
        tokio::spawn(handle_to_end(conn, status, player_sender));
    }
}

async fn handle_to_end(mut conn: TcpStream, status_str: String, 
    player_sender: Sender<(String, PlayerConnection)>) -> Result<()> 
{
    let next_state = handshaking(&mut conn).await?;
    match next_state {
        Intent::Login => {
            let name = login(&mut conn).await?;
            let (player_conn, game_conn) = connection();
            player_sender.send((name, player_conn)).map_err(|e| {
                anyhow!("Couldn't send new player to the server: {:?}", e)
            })?;
            play(conn, game_conn).await?;
        },
        Intent::Status => status(&mut conn, status_str).await?,
    }
    Ok(())
}