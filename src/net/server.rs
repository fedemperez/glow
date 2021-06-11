use crate::config::Config;
use tokio::net::{TcpListener, TcpStream};
use uuid::Uuid;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, TryIter, channel};
use anyhow::{Result, anyhow};
use super::initial_handling::initial_handling;
use super::server_info::ServerInfo;
use super::{PlayerConnection, connection::connection};
use super::play::play;
use std::sync::RwLock;

pub struct Server {
    player_recv: Receiver<(Uuid, String, PlayerConnection)>,
    info: Arc<RwLock<ServerInfo>>,
}

impl Server {
    pub fn start(config: &Config) -> Self {
        let info = Arc::new(RwLock::new(ServerInfo::new(&config.motd)));
        let (player_send, player_recv) = channel();
        tokio::spawn(listen(
            config.port, 
            player_send, 
            info.clone(),
        ));
        Self {
            player_recv,
            info,
        }
    }

    pub fn get_new_players(&mut self) -> Vec<(Uuid, String, PlayerConnection)> {
        self.player_recv.try_iter().collect()
    }

    pub fn update_list(&mut self, count: usize, sample: Vec<String>) {
        self.info.write().unwrap().update_players(sample, count);
    }
}

async fn listen(port: u16, player_send: Sender<(Uuid, String, PlayerConnection)>, 
    info: Arc<RwLock<ServerInfo>>) -> Result<()> 
{
    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on port {}", port);
    loop {
        let (conn, address) = listener.accept().await?;
        conn.set_nodelay(true).unwrap();
        let status = info.read().unwrap().to_status_str();
        tokio::spawn(handle_to_end(
            conn, 
            status, 
            player_send.clone()));
    }
}

async fn handle_to_end(mut conn: TcpStream, status: String, 
    player_send: Sender<(Uuid, String, PlayerConnection)>) -> Result<()> 
{
    if let Some(player) = initial_handling(&mut conn, status).await {
        let (player_conn, game_conn) = connection();
        player_send.send((player.0, player.1, player_conn)).map_err(|e| {
            anyhow!("Couldn't send new player to the server: {:?}", e)
        })?;
        play(conn, game_conn).await?;
    }
    Ok(())
}
