use tokio::net::TcpStream;
use anyhow::{anyhow, Result};
use tokio::io::{AsyncWrite, AsyncReadExt};
use super::packet_builder::PacketBuilder;
use crate::net::dimension_codec::{gen_dimension_codec, gen_default_dim};
use crate::net::connection::GameConnection;

pub async fn play(conn: &mut TcpStream, player: String, game: GameConnection) -> Result<()> {
    send_join_game(conn).await?;
    loop {
        conn.read_u64().await?;
    }
}

async fn send_join_game<W: AsyncWrite>(writer: &mut W) -> Result<()>
    where W: Unpin
{
    PacketBuilder::new(0x24)
        .add_bytes(&0u32.to_be_bytes()) // Entity ID
        .add_bytes(&[0]) // Is hardcore
        .add_bytes(&[0]) // Gamemode
        .add_bytes(&[0]) // Prev gamemode
        .add_varint(1) // Size of following array
        .add_str("world") // World names array
        .add_nbt(&gen_dimension_codec()) // Dimension codec
        .add_nbt(&gen_default_dim()) // Dimension
        .add_str("world") // World where the player is spawning
        .add_bytes(&[0; 8]) // First 8 bytes of the SHA-256 of the seed
        .add_varint(0) // Max players, unused
        .add_varint(12) // View distance
        .add_bytes(&[0]) // Should debug info be hidden (F3)
        .add_bytes(&[1]) // Show the "You died" screen instead of respawning immediately
        .add_bytes(&[0]) // Is debug world
        .add_bytes(&[0]) // Is superflat world
        .write(writer).await
}