use anyhow::Result;
use tokio::io::{AsyncWrite, AsyncWriteExt};

use super::clientbound::ClientboundPacket;
use super::super::builder::PacketBuilder;

impl ClientboundPacket {
    pub async fn send<W>(&self, writer: &mut W) -> Result<()>
        where W: AsyncWrite + Unpin
    {
        let bytes = match self {
            Self::JoinGame { 
                entity_id, gamemode, world_names, dimension_codec, dimension, 
                current_world, view_distance,
            } => {
                let mut pack = PacketBuilder::new(0x24);
                pack.add_bytes(&entity_id.to_be_bytes()) // Entity ID
                    .add_bytes(&[0]) // Is hardcore
                    .add_bytes(&[*gamemode]) // Gamemode
                    .add_bytes(&[*gamemode]) // Prev gamemode
                    .add_varint(world_names.len() as u32);
                for name in world_names {
                    pack.add_str(&name);
                }
                pack.add_bytes(dimension_codec)
                    .add_bytes(dimension)
                    .add_str(&current_world)
                    .add_bytes(&[0; 8]) // First 8 bytes of the SHA-256 of the seed
                    .add_varint(0) // Max players, unused
                    .add_varint(*view_distance as u32) // View distance
                    .add_bytes(&[0]) // Should debug info be hidden (F3)
                    .add_bytes(&[1]) // Show the "You died" screen instead of respawning immediately
                    .add_bytes(&[0]) // Is debug world
                    .add_bytes(&[0]) // Is superflat world
                    .build()
            }
            Self::PluginMessage { channel, content } => {
                PacketBuilder::new(0x17)
                    .add_str(channel.as_str())
                    .add_str(content.as_str())
                    .build()
            }
            Self::ChunkData
                { x, z, full, bitmask, heightmap, biomes, data, block_entities } => 
            {
                let mut packet = PacketBuilder::new(0x20);
                packet
                    .add_bytes(&x.to_be_bytes())
                    .add_bytes(&z.to_be_bytes())
                    .add_bytes(&[*full as u8])
                    .add_varint(*bitmask as u32)
                    .add_nbt(&heightmap);
                if let Some(biomes) = biomes {
                    packet.add_varint(biomes.len() as u32);
                    for biome in biomes {
                        packet.add_varint(*biome as u32);
                    }
                }
                packet
                    .add_varint(data.len() as u32)
                    .add_bytes(data)
                    .add_varint(block_entities.len() as u32);
                for entity in block_entities {
                    packet.add_nbt(entity);
                }
                packet.build()
            }
            Self::UpdateLight{ 
                x, z, trust_edges, sky_mask, block_mask, 
                empty_sky_mask, empty_block_mask, sky_light, block_light,
            } => {
                let mut pack = PacketBuilder::new(0x23);
                pack
                    .add_varint(*x as u32)
                    .add_varint(*z as u32)
                    .add_bytes(&[*trust_edges as u8])
                    .add_varint(*sky_mask)
                    .add_varint(*block_mask)
                    .add_varint(*empty_sky_mask)
                    .add_varint(*empty_block_mask);
                for array in sky_light {
                    pack.add_varint(array.len() as u32);
                    for value in array {
                        pack.add_bytes(&value.to_be_bytes());
                    }
                }
                for array in block_light {
                    pack.add_varint(array.len() as u32);
                    for value in array {
                        pack.add_bytes(&value.to_be_bytes());
                    }
                }
                pack.build()
            },
            Self::KeepAlive(id) => {
                PacketBuilder::new(0x1F)
                    .add_bytes(&id.to_be_bytes())
                    .build()
            }
            Self::PlayerPosition(x, y, z) => {
                PacketBuilder::new(0x34)
                    .add_bytes(&x.to_be_bytes())
                    .add_bytes(&y.to_be_bytes())
                    .add_bytes(&z.to_be_bytes())
                    .add_bytes(&0f32.to_be_bytes()) // Yaw
                    .add_bytes(&0f32.to_be_bytes()) // Pitch
                    .add_bytes(&[0b11000]) // Rotation relative, position absolute
                    .add_varint(0) // Teleport ID, used by client to confirm
                    .build()
            }
            Self::UpdateViewPosition(x, z) => {
                PacketBuilder::new(0x40)
                    .add_varint(*x as u32)
                    .add_varint(*z as u32)
                    .build()
            }
            Self::PlayerInfoAddPlayers(players) => {
                let mut packet = PacketBuilder::new(0x32);
                packet
                    .add_varint(0)
                    .add_varint(players.len() as u32);
                for (uuid, info) in players {
                    packet
                        .add_bytes(uuid.as_bytes())
                        .add_str(info.name.as_str())
                        .add_varint(info.properties.len() as u32);
                    for property in &info.properties {
                        packet
                            .add_str(property.name.as_str())
                            .add_str(property.value.as_str());
                        match &property.signature {
                            Some(signature) => {
                                packet.add_bytes(&[1])
                                      .add_str(signature.as_str());
                            }
                            None => { packet.add_bytes(&[0]); }
                        }
                    }
                    packet
                        .add_varint(info.gamemode as u32)
                        .add_varint(info.ping);
                    match &info.display_name {
                        Some(name) => {
                            packet.add_bytes(&[1])
                                  .add_str(name.as_str());
                        }
                        None => { packet.add_bytes(&[0]); }
                    }
                }
                packet.build()
            }
            Self::PlayerInfoUpdateGamemode(updates) => {
                unimplemented!()
            }
            Self::PlayerInfoUpdateLatency(updates) => {
                unimplemented!()
            }
            Self::PlayerInfoRemovePlayers(players) => {
                let mut packet = PacketBuilder::new(0x32);
                packet.add_varint(4)
                    .add_varint(players.len() as u32);
                for uuid in players {
                    packet.add_bytes(uuid.as_bytes());
                }
                packet.build()
            }
            Self::EntityTeleport{ id, x, y , z, yaw, pitch, on_ground } => {
                PacketBuilder::new(0x56)
                    .add_varint(*id)
                    .add_bytes(&x.to_be_bytes())
                    .add_bytes(&y.to_be_bytes())
                    .add_bytes(&z.to_be_bytes())
                    .add_angle(*yaw)
                    .add_angle(*pitch)
                    .add_bytes(&[*on_ground as u8])
                    .build()
            }
            Self::EntityPosition{ id, delta_x, delta_y, delta_z, on_ground } => {
                PacketBuilder::new(0x27)
                    .add_varint(*id)
                    .add_position_delta(*delta_x)
                    .add_position_delta(*delta_y)
                    .add_position_delta(*delta_z)
                    .add_bytes(&[*on_ground as u8])
                    .build()
            }
            Self::EntityPositionAndRotation { 
                id, delta_x, delta_y, delta_z, yaw, pitch, on_ground
            } => {
                PacketBuilder::new(0x28)
                    .add_varint(*id)
                    .add_position_delta(*delta_x)
                    .add_position_delta(*delta_y)
                    .add_position_delta(*delta_z)
                    .add_angle(*yaw)
                    .add_angle(*pitch)
                    .add_bytes(&[*on_ground as u8])
                    .build()
            }
            Self::EntityRotation{ id, yaw, pitch, on_ground } => {
                PacketBuilder::new(0x29)
                    .add_varint(*id)
                    .add_angle(*yaw)
                    .add_angle(*pitch)
                    .add_bytes(&[*on_ground as u8])
                    .build()
            }
            Self::EntityHeadLook{ id, yaw } => {
                PacketBuilder::new(0x3A)
                    .add_varint(*id)
                    .add_angle(*yaw)
                    .build()
            }
            Self::DestroyEntities(entities) => {
                let mut packet = PacketBuilder::new(0x36);
                packet.add_varint(entities.len() as u32);
                for entity in entities {
                    packet.add_varint(*entity);
                }
                packet.build()
            }
            Self::SpawnPlayer{ entity_id, uuid, x, y, z, yaw, pitch } => {
                PacketBuilder::new(0x04)
                    .add_varint(*entity_id)
                    .add_bytes(uuid.as_bytes())
                    .add_bytes(&x.to_be_bytes())
                    .add_bytes(&y.to_be_bytes())
                    .add_bytes(&z.to_be_bytes())
                    .add_angle(*yaw)
                    .add_angle(*pitch)
                    .build()
            }
            Self::BlockChange{ pos, block_state } => {
                PacketBuilder::new(0x0B)
                    .add_block_position(pos)
                    .add_varint(*block_state)
                    .build()
            }
            Self::WindowItems{ window, items } => {
                let mut pack = PacketBuilder::new(0x13);
                pack.add_bytes(&[*window])
                    .add_bytes(&(items.len() as u16).to_be_bytes());
                for slot in items {
                    pack.add_bytes(&[slot.is_some() as u8]);
                    if let Some(stack) = slot {
                        pack.add_varint(stack.item.to_numeric() as u32);
                        pack.add_bytes(&[stack.count]);
                        pack.add_bytes(&[0]);
                    }
                }
                pack.build()
            }
            Self::UnloadChunk(x, z) => {
                PacketBuilder::new(0x1C)
                    .add_bytes(&x.to_be_bytes())
                    .add_bytes(&z.to_be_bytes())
                    .build()
            }
            Self::Disconnect{ reason } => {
                PacketBuilder::new(0x19)
                    .add_str(&reason.to_string())
                    .build()
            }
            Self::Tags{ raw } => {
                PacketBuilder::new(0x5B)
                    .add_bytes(raw)
                    .build()
            }
        };
        writer.write_all(&bytes).await?;
        Ok(())
    }
}