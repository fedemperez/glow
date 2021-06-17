use std::collections::BTreeMap;
use std::iter::repeat_with;

use anvil_nbt::{CompoundTag, Tag};
use anvil_region::position::RegionChunkPosition;
use anvil_region::position::RegionPosition;
use anvil_region::provider::RegionProvider;
use async_trait::async_trait;
use anvil_region::provider::FolderRegionProvider;
use tokio::task;
use crate::blocks::{Block, BlockType};

use crate::chunks::{
    ChunkData, ChunkCoords,
    data::Palette, data::Section,
};
use super::ChunkLoader;

const SECTIONS_PER_CHUNK: usize = 16;

pub struct AnvilChunkLoader;

impl AnvilChunkLoader {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ChunkLoader for AnvilChunkLoader {
    async fn load_chunk(&self, coords: ChunkCoords) -> Option<ChunkData> {
        let chunk = read_chunk(coords).await?;
        let section_tags = chunk
            .get_compound_tag("Level").unwrap()
            .get_compound_tag_vec("Sections").unwrap();
        let mut sections: Vec<Option<Section>> = 
            repeat_with(|| None)
            .take(SECTIONS_PER_CHUNK)
            .collect();
        for tag in section_tags {
            if let Ok(palette) = tag.get_compound_tag_vec("Palette") {
                let entries: Vec<&'static Block> = palette.iter().map(|block_tag| {
                    let name = block_tag.get_str("Name").unwrap();
                    let props = get_properties(block_tag);
                    BlockType::from_name(name).unwrap()
                        .with_props(&props).unwrap()
                }).collect();
                let palette = Palette::from_entries(entries.as_slice());
                let blocks = tag.get_i64_vec("BlockStates").unwrap();
                let section = Section::from_raw(blocks.clone(), palette);
                let y = tag.get_i8("Y").unwrap() as usize;
                sections[y] = Some(section);
            }
        }
        Some(ChunkData::from_sections(sections))
    }
}

async fn read_chunk(coords: ChunkCoords) -> Option<CompoundTag> {
    let ChunkCoords(chunk_x, chunk_z) = coords;
    let region_position = 
        RegionPosition::from_chunk_position(chunk_x, chunk_z);
    let chunk_position = 
        RegionChunkPosition::from_chunk_position(chunk_x, chunk_z);
    task::spawn_blocking(move || {
        let provider = FolderRegionProvider::new("world/region");
        let mut region = provider.get_region(region_position).ok()?;
        region.read_chunk(chunk_position).ok()
    }).await.ok()?
}

fn get_properties(tag: &CompoundTag) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    if let Ok(props) = tag.get_compound_tag("Properties") {
        for (name, tag) in props.iter() {
            if let Tag::String(value) = tag {
                map.insert(name.clone(), value.clone());
            }
        }
    }
    map
}
