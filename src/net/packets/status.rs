use anyhow::{anyhow, Result};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite};
use super::value_readers::read_varint;
use super::builder::PacketBuilder;

pub enum ServerboundPacket {
    Request,
    Ping(u64),
}

pub async fn read_packet<R>(reader: &mut R) -> Result<ServerboundPacket>
    where R: AsyncRead + Unpin
{
    let _len = read_varint(reader).await?;
    let id = read_varint(reader).await?;
    match id {
        0x00 => Ok(ServerboundPacket::Request),
        0x01 => Ok(ServerboundPacket::Ping(reader.read_u64().await?)),
        _ => Err(anyhow!("Invalid packet")),
    }
}

pub enum ClientboundPacket {
    Response(String),
    Pong(u64),
}

impl ClientboundPacket {
    pub async fn send<W>(&self, writer: &mut W) -> Result<()>
        where W: AsyncWrite + Unpin
    {
        match self {
            ClientboundPacket::Response(status) => {
                PacketBuilder::new(0)
                    .add_str(status.as_str())
                    .write(writer).await
            }
            ClientboundPacket::Pong(time) => {
                PacketBuilder::new(1)
                    .add_bytes(&time.to_be_bytes())
                    .write(writer).await
            }
        }
        
    }
}