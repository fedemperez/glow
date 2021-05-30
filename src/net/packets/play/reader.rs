use tokio::io::{AsyncRead, AsyncReadExt};
use anyhow::{Result, anyhow};
use super::super::errors::UnknownPacket;
use super::serverbound::ServerboundPacket;
use super::super::value_readers::{read_block_pos, read_varint};

impl ServerboundPacket {
    pub async fn read<R>(reader: &mut R) -> Result<Self> 
    where R: AsyncRead + Unpin
    {
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
                Ok(Self::Move(x, y, z))
            }
            0x14 => {
                let yaw = f32::from_bits(reader.read_u32().await?);
                let pitch = f32::from_bits(reader.read_u32().await?);
                let on_ground = reader.read_u8().await? != 0;
                Ok(Self::Rotate(yaw, pitch))
            }
            0x1B => {
                let status = reader.read_u8().await?;
                let (x, y, z) = read_block_pos(reader).await?;
                let face = reader.read_u8().await?;
                match status {
                    0 => {
                        Ok(Self::BreakBlock(x, y, z))
                    }
                    _ => Err(UnknownPacket(id).into()),
                }
            }
            id => {
                let mut buffer = vec![0; length - 1];
                reader.read_exact(buffer.as_mut()).await?;
                Err(UnknownPacket(id).into())
            }
        }
    }
}