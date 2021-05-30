use tokio::net::TcpStream;
use anyhow::Result;
use super::packets::status::ServerboundPacket::{Ping, Request};
use super::packets::status::ClientboundPacket;
use super::packets::status::read_packet;

pub async fn status(conn: &mut TcpStream, status_str: String) -> Result<()> {
    loop {
        let pack = read_packet(conn).await?;
        match pack {
            Request => {
                ClientboundPacket::Response(status_str.clone())
                    .send(conn).await?;
            },
            Ping(time) => {
                ClientboundPacket::Pong(time)
                    .send(conn).await?;
                break;
            }
        }
    }
    Ok(())
}