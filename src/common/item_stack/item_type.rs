use anyhow::{Result, anyhow};
use bimap::BiHashMap;
use std::collections::HashMap;
use serde::Deserialize;
use lazy_static::lazy_static;

use crate::common::block::BlockType;

const ITEMS_JSON: &str = include_str!("items.json");

lazy_static! {
    static ref ITEM_ID_MAP: BiHashMap<u16, String> = TYPE_MAP.iter()
        .map(|(name, props)| (props.id, name.clone())).collect();
    static ref TYPE_MAP: HashMap<String, ItemType> =
        serde_json::from_str(&ITEMS_JSON).unwrap();
}

#[derive(Clone, Debug, Deserialize)]
pub struct ItemType {
    id: u16,
    block: Option<u16>,
}

impl ItemType {
    pub fn from_numeric(id: u16) -> Result<&'static Self> {
        let id = ITEM_ID_MAP.get_by_left(&id)
            .ok_or(anyhow!(format!("Invalid item id: {}", id)))?;
        Ok(TYPE_MAP.get(id).unwrap())
    }

    pub fn from_str(id: &str) -> Result<&'static Self> {
        TYPE_MAP.get(id)
            .ok_or(anyhow!(format!("Invalid item id: {}", id)))
    }

    pub fn to_str(&self) -> &'static str {
        ITEM_ID_MAP.get_by_left(&self.id).unwrap()
    }

    pub fn to_numeric(&self) -> u16 {
        self.id
    }

    pub fn get_block(&self) -> Option<&BlockType> {
        self.block.map(|block| BlockType::from_id(block).unwrap())
    }
}
