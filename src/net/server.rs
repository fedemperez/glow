use crate::config::Config;
use tokio::net::{TcpListener, TcpStream};
use std::sync::Arc;
use anyhow::Result;
use super::handshaking::{handshaking, Intent};
use super::status::status;
use super::login::login;
use super::play::play;
use super::connection::connection;

pub struct Server {
    port: u16,
    motd: Arc<String>,
}

impl Server {
    pub fn from_config(config: &Config) -> Self {
        Self {
            port: config.port,
            motd: Arc::new(config.motd.clone()),
        }
    }

    pub async fn serve(mut self) -> Result<()> {
        let address = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(address).await?;
        println!("Listening on port {}", self.port);
        loop {
            let (conn, address) = listener.accept().await?;
            println!("New connection from {}", address);
            let motd = self.motd.clone();
            tokio::spawn(async move {
                let result = handle_to_end(conn, &motd).await;
                match result {
                    Ok(_) => println!("{} disconnected", address),
                    Err(e) => eprintln!("{} disconnected with error: {}", address, e)
                }
            });
        }
    }
}

async fn handle_to_end(mut conn: TcpStream, motd: &String) -> Result<()> {
    let next_state = handshaking(&mut conn).await?;
    match next_state {
        Intent::Login => {
            let player = login(&mut conn).await?;
            println!("{} logged in", player);
            let (player_conn, game_conn) = connection();
            play(&mut conn, player, game_conn).await?;
        },
        Intent::Status => status(&mut conn, motd).await?,
    }
    Ok(())
}