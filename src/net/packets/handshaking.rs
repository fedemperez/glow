use super::value_readers::{read_varint, read_str};
use anyhow::{Result, anyhow};
use tokio::io::{AsyncRead, AsyncReadExt};

pub enum ServerboundPacket {
    Handshake {
        proto_version: u32,
        host_name: String,
        port: u16,
        intent: u32,
    }
}

pub async fn read_packet<R>(reader: &mut R) -> Result<ServerboundPacket>
    where R: AsyncRead + Unpin
{
    let _len = read_varint(reader).await?;
    let id = read_varint(reader).await?;
    match id {
        0x00 => Ok(
            ServerboundPacket::Handshake {
                proto_version: read_varint(reader).await?,
                host_name: read_str(reader).await?,
                port: reader.read_u16().await?,
                intent: read_varint(reader).await?,
            }),
        _ => Err(anyhow!("Invalid packet"))
    }
}