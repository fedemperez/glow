use tokio::net::TcpStream;
use anyhow::Result;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, BufReader};
use std::sync::mpsc::Sender;
use tokio::sync::mpsc::UnboundedReceiver;
use super::dimension_codec::{gen_dimension_codec, gen_default_dim};
use super::connection::GameConnection;
use super::packets::errors::UnknownPacket;
use super::packets::play::{ClientboundPacket, ServerboundPacket};

const BRAND: &str = "Cane";

pub async fn play(conn: TcpStream, game: GameConnection) -> Result<()> {
    let (game_recv, mut game_send) = game.into_split();
    let (tcp_read, tcp_write) = conn.into_split();
    tokio::spawn(game_to_client(game_recv, tcp_write));
    client_to_game(tcp_read, &mut game_send).await;
    Ok(())
}

async fn client_to_game<R>(tcp: R, sender: &mut Sender<ServerboundPacket>)
    where R: AsyncRead + Unpin
{
    let mut tcp = BufReader::with_capacity(256, tcp);
    loop {
        match ServerboundPacket::read(&mut tcp).await {
            Ok(packet) => {
                sender.send(packet);
            }
            Err(error) => {
                if !error.is::<UnknownPacket>() {
                    sender.send(ServerboundPacket::Disconnect {
                        reason: error.to_string(),
                    });
                    break;
                }
            }
        }
    }
}

async fn game_to_client<W>(mut game: UnboundedReceiver<ClientboundPacket>, mut tcp: W) 
    -> Result<()> where W: AsyncWrite + Unpin
{
    send_initial_packets(&mut tcp).await;
    while let Some(packet) = game.recv().await {
        packet.send(&mut tcp).await?;
    }
    Ok(())
}

async fn send_initial_packets<W>(writer: &mut W) -> Result<()>
    where W: AsyncWrite + Unpin 
{
    ClientboundPacket::JoinGame {
        entity_id: 999,
        gamemode: 1,
        dimension_codec: gen_dimension_codec(),
        dimension: gen_default_dim(),
        view_distance: 6,
    }.send(writer).await?;
    ClientboundPacket::PluginMessage {
        channel: "minecraft:brand".into(),
        content: BRAND.into(),
    }.send(writer).await
}