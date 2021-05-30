use anyhow::Result;
use super::packets::login::*;
use tokio::net::TcpStream;
use uuid::Uuid;

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
