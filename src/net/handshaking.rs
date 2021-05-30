use tokio::net::TcpStream;
use anyhow::{Result, anyhow};
use super::packets::handshaking::{read_packet, ServerboundPacket::Handshake};

pub enum Intent {
    Login, Status
}

pub async fn handshaking(conn: &mut TcpStream) -> Result<Intent> {
    let packet = read_packet(conn).await?;
    match packet {
        Handshake { intent, .. } => {
            match intent {
                1 => Ok(Intent::Status),
                2 => Ok(Intent::Login),
                _ => Err(anyhow!("Invalid packet")),
            }
        },
    }
}