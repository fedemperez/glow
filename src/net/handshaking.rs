use tokio::net::TcpStream;
use tokio::io::{AsyncRead, AsyncReadExt};
use crate::net::value_readers::{read_varint, read_str};
use anyhow::{Result, anyhow};

pub enum Intent {
    Login, Status
}

enum Packet {
    Handshake {
        proto_version: u32,
        host_name: String,
        port: u16,
        intent: Intent,
    }
}

pub async fn handshaking(conn: &mut TcpStream) -> Result<Intent> {
    let packet = read_pack(conn).await?;
    match packet {
        Packet::Handshake { intent, .. } => Ok(intent),
    }
}

async fn read_pack<R: AsyncRead>(reader: &mut R) -> Result<Packet>
    where R: Unpin
{
    let _len = read_varint(reader).await?;
    let id = read_varint(reader).await?;
    match id {
        0x00 => Ok(
            Packet::Handshake {
                proto_version: read_varint(reader).await?,
                host_name: read_str(reader).await?,
                port: reader.read_u16().await?,
                intent: match read_varint(reader).await? {
                    1 => Intent::Status,
                    2 => Intent::Login,
                    _ => return Err(anyhow!("Invalid packet")),
                }
            }),
        _ => Err(anyhow!("Invalid packet"))
    }
}