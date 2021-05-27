use tokio::net::TcpStream;
use anyhow::Result;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite};
use std::sync::mpsc::Sender;
use tokio::sync::mpsc::UnboundedReceiver;
use super::{ClientEvent, ServerEvent};
use super::packet_builder::PacketBuilder;
use crate::net::dimension_codec::{gen_dimension_codec, gen_default_dim};
use crate::net::connection::GameConnection;
use super::value_readers::read_varint;

const BRAND: &str = "Cane";

pub async fn play(mut conn: TcpStream, game: GameConnection) -> Result<()> {
    let (game_recv, mut game_send) = game.into_split();
    let (tcp_read, tcp_write) = conn.into_split();
    tokio::spawn(game_to_client(game_recv, tcp_write));
    client_to_game(tcp_read, &mut game_send).await?;
    Ok(())
}

async fn client_to_game<R: AsyncRead>(mut tcp: R, game: &mut Sender<ClientEvent>)
    -> Result<()> where R: Unpin
{
    loop {
        match read_packet(&mut tcp).await {
            Ok(_) => {}
            Err(e) => {
                game.send(ClientEvent::Disconnect(e.to_string()));
                break;
            }
        }
    }
    Ok(())
}

async fn game_to_client<W: AsyncWrite>(mut game: UnboundedReceiver<ServerEvent>, mut tcp: W)
    -> Result<()> where W: Unpin
{
    send_join_game(&mut tcp).await?;
    send_brand(&mut tcp).await?;
    while let Some(event) = game.recv().await {
        event.write_to(&mut tcp).await?;
    }
    Ok(())
}

async fn read_packet<R: AsyncRead>(tcp: &mut R)
    -> Result<()> where R: Unpin
{
    let length = read_varint(tcp).await? as usize;
    let mut buffer = vec![0; length];
    tcp.read_exact(buffer.as_mut()).await?;
    Ok(())
}

async fn send_join_game<W: AsyncWrite>(writer: &mut W) -> Result<()>
    where W: Unpin
{
    PacketBuilder::new(0x24)
        .add_bytes(&0u32.to_be_bytes()) // Entity ID
        .add_bytes(&[0]) // Is hardcore
        .add_bytes(&[1]) // Gamemode
        .add_bytes(&[1]) // Prev gamemode
        .add_varint(1) // Size of following array
        .add_str("world") // World names array
        .add_nbt(&gen_dimension_codec()) // Dimension codec
        .add_nbt(&gen_default_dim()) // Dimension
        .add_str("world") // World where the player is spawning
        .add_bytes(&[0; 8]) // First 8 bytes of the SHA-256 of the seed
        .add_varint(0) // Max players, unused
        .add_varint(6) // View distance
        .add_bytes(&[0]) // Should debug info be hidden (F3)
        .add_bytes(&[1]) // Show the "You died" screen instead of respawning immediately
        .add_bytes(&[0]) // Is debug world
        .add_bytes(&[0]) // Is superflat world
        .write(writer).await
}

async fn send_brand<W: AsyncWrite>(writer: &mut W) -> Result<()> where W: Unpin {
    PacketBuilder::new(0x17)
        .add_str("minecraft:brand")
        .add_str(BRAND)
        .write(writer).await
}
