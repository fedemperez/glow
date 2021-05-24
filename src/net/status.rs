use tokio::net::TcpStream;
use anyhow::{anyhow, Result};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite};
use super::value_readers::{read_varint, read_str};
use super::packet_builder::PacketBuilder;
use serde_json::json;

enum Packet {
    Request,
    Ping(u64),
}

pub async fn status(conn: &mut TcpStream, motd: &String) -> Result<()> {
    loop {
        let pack = read_pack(conn).await?;
        match pack {
            Packet::Request => {
                send_status(conn, motd).await?;
            },
            Packet::Ping(time) => {
                send_pong(conn, time).await?;
                break;
            }
        }
    }
    Ok(())
}

async fn read_pack<R: AsyncRead>(reader: &mut R) -> Result<Packet>
    where R: Unpin
{
    let len = read_varint(reader).await?;
    let id = read_varint(reader).await?;
    match id {
        0x00 => Ok(Packet::Request),
        0x01 => Ok(Packet::Ping(reader.read_u64().await?)),
        _ => Err(anyhow!("Invalid packet")),
    }
}

async fn send_status<W: AsyncWrite>(writer: &mut W, motd: &String) -> Result<()>
    where W: Unpin
{
    PacketBuilder::new(0)
        .add_str(gen_status_json(motd).as_str())
        .write(writer).await
}

async fn send_pong<W: AsyncWrite>(writer: &mut W, time: u64) -> Result<()>
    where W: Unpin
{
    PacketBuilder::new(1)
        .add_bytes(&time.to_be_bytes())
        .write(writer).await
}

fn gen_status_json(motd: &String) -> String {
    json!({
        "version": {
            "name": "1.16.5",
            "protocol": 754
        },
        "players": {
            "max": 100,
            "online": 5,
            "sample": [
                {
                    "name": "thinkofdeath",
                    "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
                }
            ]
        },
        "description": {
            "text": motd
        },
        "favicon": "data:image/png;base64,<data>"
    }).to_string()
}