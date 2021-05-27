use nalgebra::Vector3;
use tokio::net::TcpStream;
use anyhow::{anyhow, Result};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite};
use std::sync::mpsc::Sender;
use tokio::sync::mpsc::UnboundedReceiver;
use super::{GameEvent, PlayerEvent};
use super::packet_builder::PacketBuilder;
use crate::game::{Chunk, ChunkCoords};
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

async fn client_to_game<R: AsyncRead>(mut tcp: R, game: &mut Sender<PlayerEvent>)
    -> Result<()> where R: Unpin
{
    loop {
        match read_packet(&mut tcp).await {
            Ok(_) => {}
            Err(e) => {
                game.send(PlayerEvent::Disconnect(e.to_string()));
                break;
            }
        }
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

async fn game_to_client<W: AsyncWrite>(mut game: UnboundedReceiver<GameEvent>, mut tcp: W)
    -> Result<()> where W: Unpin
{
    send_join_game(&mut tcp).await?;
    send_brand(&mut tcp).await?;
    send_position(&mut tcp, Vector3::new(0.0, 50.0, 0.0)).await?;
    while let Some(event) = game.recv().await {
        match event {
            GameEvent::LoadChunk(coords, chunk) => {
                send_chunk(&mut tcp, coords, &*chunk.read().await).await?;
            }
            GameEvent::KeepAlive(id) => {
                send_keepalive(&mut tcp, id).await?;
            }
        }
    }
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

async fn send_chunk<W: AsyncWrite>(writer: &mut W, coords: ChunkCoords, chunk: &Chunk)
    -> Result<()> where W: Unpin
{
    let mut packet = PacketBuilder::new(0x20);
    packet
        .add_bytes(&coords.0.to_be_bytes())
        .add_bytes(&coords.1.to_be_bytes())
        .add_bytes(&[1])
        .add_varint(chunk.get_sections_bitmask() as u32)
        .add_nbt(&chunk.get_heightmap())
        .add_varint(1024);
    for biome in chunk.get_biome_map() {
        packet.add_varint(biome as u32);
    }
    let chunk_data = chunk.get_data();
    packet
        .add_varint(chunk_data.len() as u32)
        .add_bytes(chunk_data.as_slice())
        .add_varint(0)
        .write(writer).await
}

async fn send_position<W: AsyncWrite>(writer: &mut W, pos: Vector3<f32>)
    -> Result<()> where W: Unpin
{
    PacketBuilder::new(0x34)
        .add_bytes(&(pos.x as f64).to_be_bytes())
        .add_bytes(&(pos.y as f64).to_be_bytes())
        .add_bytes(&(pos.z as f64).to_be_bytes())
        .add_bytes(&0f32.to_be_bytes()) // Yaw
        .add_bytes(&0f32.to_be_bytes()) // Pitch
        .add_bytes(&[0b11000]) // Rotation relative, position absolute
        .add_varint(0) // Teleport ID, used by client to confirm
        .write(writer).await
}

async fn send_brand<W: AsyncWrite>(writer: &mut W) -> Result<()> where W: Unpin {
    PacketBuilder::new(0x17)
        .add_str("minecraft:brand")
        .add_str(BRAND)
        .write(writer).await
}

async fn send_keepalive<W: AsyncWrite>(writer: &mut W, id: u64)
    -> Result<()> where W: Unpin 
{
    PacketBuilder::new(0x1F)
        .add_bytes(&id.to_be_bytes())
        .write(writer).await
}