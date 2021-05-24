use anyhow::{anyhow, Result};
use super::packet_builder::PacketBuilder;
use super::value_readers::read_varint;
use super::value_readers::read_str;
use tokio::net::TcpStream;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite};
use uuid::Uuid;

const UUID_NAMESPACE: &Uuid = &Uuid::nil();

enum Packet {
    Login {
        name: String,
    }
}

pub async fn login(conn: &mut TcpStream) -> Result<String> {
    let packet = read_pack(conn).await?;
    match packet {
        Packet::Login { name } => {
            send_success(
                name.as_str(),
                Uuid::new_v3(UUID_NAMESPACE, name.as_bytes()),
                conn,
            ).await?;
            Ok(name)
        }
    }
}

async fn read_pack<R: AsyncRead>(reader: &mut R) -> Result<Packet>
    where R: Unpin
{
    let len = read_varint(reader).await?;
    let id = read_varint(reader).await?;
    match id {
        0x00 => Ok(
            Packet::Login {
                name: read_str(reader).await?,
            }),
        _ => Err(anyhow!("Invalid packet"))
    }
}

async fn send_success<W: AsyncWrite>(name: &str, uuid: Uuid, writer: &mut W) -> Result<()>
    where W: Unpin
{
    PacketBuilder::new(0x02)
        .add_bytes(uuid.as_bytes())
        .add_str(name)
        .write(writer).await
}
