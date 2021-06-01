use anyhow::Result;
use tokio::io::AsyncWrite;

use super::clientbound::ClientboundPacket;
use super::super::builder::PacketBuilder;

impl ClientboundPacket {
    pub async fn send<W>(&self, writer: &mut W) -> Result<()>
        where W: AsyncWrite + Unpin
    {
        match self {
            Self::JoinGame { entity_id, gamemode, dimension_codec, dimension, view_distance } => {
                PacketBuilder::new(0x24)
                    .add_bytes(&entity_id.to_be_bytes()) // Entity ID
                    .add_bytes(&[0]) // Is hardcore
                    .add_bytes(&[*gamemode]) // Gamemode
                    .add_bytes(&[*gamemode]) // Prev gamemode
                    .add_varint(1) // Size of following array
                    .add_str("world") // World names array
                    .add_nbt(dimension_codec) // Dimension codec
                    .add_nbt(dimension) // Dimension
                    .add_str("world") // World where the player is spawning
                    .add_bytes(&[0; 8]) // First 8 bytes of the SHA-256 of the seed
                    .add_varint(0) // Max players, unused
                    .add_varint(*view_distance as u32) // View distance
                    .add_bytes(&[0]) // Should debug info be hidden (F3)
                    .add_bytes(&[1]) // Show the "You died" screen instead of respawning immediately
                    .add_bytes(&[0]) // Is debug world
                    .add_bytes(&[0]) // Is superflat world
                    .write(writer).await
            }
            Self::PluginMessage { channel, content } => {
                PacketBuilder::new(0x17)
                    .add_str("minecraft:brand")
                    .add_bytes(content)
                    .write(writer).await
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
                packet.write(writer).await
            }
            Self::KeepAlive(id) => {
                PacketBuilder::new(0x1F)
                    .add_bytes(&id.to_be_bytes())
                    .write(writer).await
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
                    .write(writer).await
            }
            Self::UpdateViewPosition(x, z) => {
                PacketBuilder::new(0x40)
                    .add_varint(*x as u32)
                    .add_varint(*z as u32)
                    .write(writer).await
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
                packet.write(writer).await
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
                packet.write(writer).await
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
                    .write(writer).await
            }
            Self::EntityPosition{ id, delta_x, delta_y, delta_z, on_ground } => {
                unimplemented!()
            }
            Self::EntityRotation{ id, yaw, pitch, on_ground } => {
                PacketBuilder::new(0x29)
                    .add_varint(*id)
                    .add_angle(*yaw)
                    .add_angle(*pitch)
                    .add_bytes(&[*on_ground as u8])
                    .write(writer).await
            }
            Self::EntityHeadLook{ id, yaw } => {
                PacketBuilder::new(0x3A)
                    .add_varint(*id)
                    .add_angle(*yaw)
                    .write(writer).await
            }
            Self::DestroyEntities(entities) => {
                let mut packet = PacketBuilder::new(0x36);
                packet.add_varint(entities.len() as u32);
                for entity in entities {
                    packet.add_varint(*entity);
                }
                packet.write(writer).await
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
                    .write(writer).await
            }
        }
    }
}