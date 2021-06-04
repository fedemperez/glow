use anyhow::{Result, anyhow};
use bimap::BiHashMap;
use lazy_static::lazy_static;

const ITEMS_JSON: &str = include_str!("items.json");

lazy_static! {
    static ref ITEM_ID_MAP: BiHashMap<String, u32> = 
        serde_json::from_str(&ITEMS_JSON).unwrap();
}

pub fn network_to_file_id(id: u32) -> Result<&'static str> {
    ITEM_ID_MAP.get_by_right(&id).map(|s| s.as_str())
        .ok_or(anyhow!("Invalid network item id"))
}

pub fn file_to_network_id(id: &str) -> Result<u32> {
    ITEM_ID_MAP.get_by_left(id).map(|v| *v)
        .ok_or(anyhow!("Invalid file item id"))
}