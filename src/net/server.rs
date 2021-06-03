use crate::config::Config;
use crate::net::status_gen::gen_status_str;
use tokio::net::{TcpListener, TcpStream};
use uuid::Uuid;
use std::sync::mpsc::{Receiver, Sender, TryIter, channel};
use anyhow::{Result, anyhow};
use super::{PlayerConnection, connection::connection};
use super::handshaking::{handshaking, Intent};
use super::status::status;
use super::login::login;
use super::play::play;

pub struct Server {
    player_recv: Receiver<(Uuid, String, PlayerConnection)>,
    list_send: Sender<(usize, Vec<String>)>,
}

impl Server {
    pub fn start(config: &Config) -> Self {
        let (player_send, player_recv) = channel();
        let (list_send, list_recv) = channel();
        tokio::spawn(listen(
            config.port, 
            config.motd.clone(), 
            player_send, 
            list_recv,
        ));
        Self {
            player_recv,
            list_send,
        }
    }

    pub fn get_new_players(&mut self) -> TryIter<'_, (Uuid, String, PlayerConnection)> {
        self.player_recv.try_iter()
    }

    pub fn update_list(&mut self, count: usize, sample: Vec<String>) {
        self.list_send.send((count, sample));
    }
}

async fn listen(port: u16, motd: String, player_sender: Sender<(Uuid, String, PlayerConnection)>, 
    list_updates: Receiver<(usize, Vec<String>)>) -> Result<()> 
{
    let address = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(address).await?;
    println!("Listening on port {}", port);
    let mut player_count = 0;
    let mut player_list = vec![];
    loop {
        let (conn, address) = listener.accept().await?;
        conn.set_nodelay(true).unwrap();
        println!("New connection from {}", address);
        for (count, list) in list_updates.try_iter() {
            player_list = list;
            player_count = count;
        }
        let status = gen_status_str(&motd, player_count, &player_list);
        let player_sender = player_sender.clone();
        tokio::spawn(handle_to_end(conn, status, player_sender));
    }
}

async fn handle_to_end(mut conn: TcpStream, status_str: String, 
    player_sender: Sender<(Uuid, String, PlayerConnection)>) -> Result<()> 
{
    let next_state = handshaking(&mut conn).await?;
    match next_state {
        Intent::Login => {
            let (uuid, name) = login(&mut conn).await?;
            let (player_conn, game_conn) = connection();
            player_sender.send((uuid, name, player_conn)).map_err(|e| {
                anyhow!("Couldn't send new player to the server: {:?}", e)
            })?;
            play(conn, game_conn).await?;
        },
        Intent::Status => status(&mut conn, status_str).await?,
    }
    Ok(())
}