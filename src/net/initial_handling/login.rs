use tokio::net::TcpStream;
use uuid::Uuid;
use crate::net::{
    builder::PacketBuilder, 
    value_readers::{read_str, read_varint}};
use tokio::io::{AsyncRead, AsyncWrite};
use anyhow::{anyhow, Result};

const UUID_NAMESPACE: &Uuid = &Uuid::nil();

pub async fn login(conn: &mut TcpStream) -> Result<(Uuid, String)> {
    let packet = read_pack(conn).await?;
    match packet {
        ServerboundPacket::Login { name } => {
            let uuid = Uuid::new_v3(UUID_NAMESPACE, name.as_bytes());
            let response = ClientboundPacket::Success(uuid, name.clone());
            response.send(conn).await?;
            Ok((uuid, name))
        }
    }
}

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