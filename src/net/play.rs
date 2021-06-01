use nalgebra::Vector3;
use tokio::net::TcpStream;
use anyhow::Result;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite};
use std::sync::mpsc::Sender;
use tokio::sync::mpsc::UnboundedReceiver;
use crate::events::ClientEvent;
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

async fn client_to_game<R>(mut tcp: R, game: &mut Sender<ClientEvent>)
    where R: AsyncRead + Unpin
{
    loop {
        match ServerboundPacket::read(&mut tcp).await {
            Ok(packet) => {
                send_events(&packet, game);
            }
            Err(error) => {
                if !error.is::<UnknownPacket>() {
                    game.send(ClientEvent::Disconnect(error.to_string()));
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

fn send_events(packet: &ServerboundPacket, sender: &mut Sender<ClientEvent>) {
    let event = match *packet {
        ServerboundPacket::Move(x, y, z) => ClientEvent::Move(Vector3::new(x, y, z)),
        ServerboundPacket::Rotate(yaw, pitch) => ClientEvent::Rotate(yaw, pitch),
        ServerboundPacket::BreakBlock(x, y, z) => ClientEvent::BreakBlock(x, y, z),
    };
    sender.send(event);
}

/*async fn send_packets<W>(event: &ServerEvent, sender: &mut W) -> Result<()>
    where W: AsyncWrite + Unpin 
{
    match event {
        ServerEvent::LoadChunk(_, _) => {}
        ServerEvent::KeepAlive(_) => {}
        ServerEvent::PlayerPosition(_) => {}
        ServerEvent::ChunkPosition(_) => {}
        ServerEvent::AddPlayer(_, _) => {}
        ServerEvent::RemovePlayer(_) => {}
        ServerEvent::EntityTeleported(_, _, _) => {}
        ServerEvent::EntityMoved(_, _) => {}
        ServerEvent::EntityRotated(_, _) => {}
        ServerEvent::EntityHeadRotated(_, _) => {}
        ServerEvent::DestroyEntities(_) => {}
        ServerEvent::SpawnPlayer(_, _, _) => {}
    }
    Ok(())
}*/

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
        content: BRAND.as_bytes().into(),
    }.send(writer).await
}