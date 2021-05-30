use tokio::io::{AsyncRead, AsyncWrite};
use uuid::Uuid;
use super::{builder::PacketBuilder, value_readers::{read_str, read_varint}};
use anyhow::{anyhow, Result};

pub enum ServerboundPacket {
    Login {
        name: String,
    }
}

pub async fn read_pack<R>(reader: &mut R) -> Result<ServerboundPacket>
    where R: AsyncRead + Unpin
{
    let _length = read_varint(reader).await?;
    let id = read_varint(reader).await?;
    match id {
        0x00 => Ok(
            ServerboundPacket::Login {
                name: read_str(reader).await?,
            }),
        _ => Err(anyhow!("Invalid packet"))
    }
}

pub enum ClientboundPacket {
    Success(Uuid, String),
}

impl ClientboundPacket {
    pub async fn send<W>(&self, writer: &mut W) -> Result<()>
        where W: AsyncWrite + Unpin
    {
        match self {
            ClientboundPacket::Success(uuid, name) => {
                PacketBuilder::new(0x02)
                    .add_bytes(uuid.as_bytes())
                    .add_str(name.as_str())
                    .write(writer).await
            }
        }
    }
}