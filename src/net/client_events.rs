use nalgebra::Vector3;
use tokio::io::{AsyncRead, AsyncReadExt};
use anyhow::{Result, anyhow};
use thiserror::Error;

use super::value_readers::read_varint;

pub enum ClientEvent {
    Disconnect(String),
    Move(Vector3<f32>),
    Rotate(f32, f32),
}

impl ClientEvent {
    pub async fn read<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Self> {
        let length = read_varint(reader).await? as usize;
        if length == 0 {
            return Err(anyhow!("Packet length cannot be 0"));
        }
        let id = reader.read_u8().await?;
        match id {
            0x12 => {
                let x = f64::from_bits(reader.read_u64().await?) as f32;
                let y = f64::from_bits(reader.read_u64().await?) as f32;
                let z = f64::from_bits(reader.read_u64().await?) as f32;
                let on_ground = reader.read_u8().await? != 0;
                Ok(ClientEvent::Move(Vector3::new(x, y, z)))
            }
            0x14 => {
                let yaw = f32::from_bits(reader.read_u32().await?);
                let pitch = f32::from_bits(reader.read_u32().await?);
                let on_ground = reader.read_u8().await? != 0;
                Ok(ClientEvent::Rotate(yaw, pitch))
            }
            id => {
                let mut buffer = vec![0; length - 1];
                reader.read_exact(buffer.as_mut()).await?;
                Err(UnknownPacket(id).into())
            }
        }
    }
}

#[derive(Error, Debug)]
#[error("Unknown packet id: {0}")]
pub struct UnknownPacket(pub u8);