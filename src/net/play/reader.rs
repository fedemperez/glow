use std::io::Cursor;

use nalgebra::vector;
use tokio::io::{AsyncRead, AsyncReadExt};
use anyhow::{Result, anyhow};
use crate::common::item_stack::{ItemStack, ItemType};
use num_traits::FromPrimitive;

use super::errors::UnknownPacket;
use super::serverbound::ServerboundPacket;
use crate::net::value_readers::{read_block_pos, read_varint};

impl ServerboundPacket {
    pub async fn read<R>(reader: &mut R) -> Result<Self> 
    where R: AsyncRead + Unpin
    {
        let length = read_varint(reader).await? as usize;
        if length == 0 {
            return Err(anyhow!("Packet length cannot be 0"));
        }
        let mut buffer = vec![0; length];
        reader.read_exact(buffer.as_mut()).await?;
        let mut payload = Cursor::new(buffer);
        match payload.read_u8().await? {
            0x12 => {
                let x = f64::from_bits(payload.read_u64().await?);
                let y = f64::from_bits(payload.read_u64().await?);
                let z = f64::from_bits(payload.read_u64().await?);
                let on_ground = payload.read_u8().await? != 0;
                Ok(Self::PlayerPosition {
                    x, y, z, on_ground
                })
            }
            0x13 => {
                let x = f64::from_bits(payload.read_u64().await?);
                let y = f64::from_bits(payload.read_u64().await?);
                let z = f64::from_bits(payload.read_u64().await?);
                let yaw = f32::from_bits(payload.read_u32().await?);
                let pitch = f32::from_bits(payload.read_u32().await?);
                let on_ground = payload.read_u8().await? != 0;
                Ok(Self::PlayerPositionAndRotation {
                    x, y, z, yaw, pitch, on_ground
                })
            }
            0x14 => {
                let yaw = f32::from_bits(payload.read_u32().await?);
                let pitch = f32::from_bits(payload.read_u32().await?);
                let on_ground = payload.read_u8().await? != 0;
                Ok(Self::PlayerRotation {
                    yaw, pitch, on_ground
                })
            }
            0x1B => {
                let status = payload.read_u8().await?;
                let position = read_block_pos(&mut payload).await?;
                let face = payload.read_u8().await?;
                Ok(Self::PlayerDigging {
                    status, position, face
                })
            }
            0x25 => {
                let slot = payload.read_u16().await?;
                Ok(Self::HeldItemChange {
                    slot,
                })
            }
            0x28 => {
                let slot = payload.read_i16().await?;
                let stack = if payload.read_u8().await? == 0 {
                    None
                } else {
                    let id = read_varint(&mut payload).await?;
                    let count = payload.read_u8().await?;
                    Some(ItemStack {
                        item: ItemType::from_numeric(id as u16)?,
                        count,
                        nbt: None,
                    })
                };
                Ok(Self::CreativeInventoryAction {
                    slot, stack
                })
            }
            0x2E => {
                let hand = read_varint(&mut payload).await? as u8;
                let location = read_block_pos(&mut payload).await?;
                let face = read_varint(&mut payload).await?;
                let cursor_x = f32::from_bits(payload.read_u32().await?);
                let cursor_y = f32::from_bits(payload.read_u32().await?);
                let cursor_z = f32::from_bits(payload.read_u32().await?);
                let inside_block = payload.read_u8().await? != 0;
                Ok(Self::PlayerBlockPlacement {
                    hand, location, inside_block,
                    face: FromPrimitive::from_u32(face)
                        .ok_or(anyhow!("Invalid packet"))?, 
                    cursor_position: vector!(cursor_x, cursor_y, cursor_z),
                })
            }
            id => Err(UnknownPacket(id).into()),
        }
    }
}
