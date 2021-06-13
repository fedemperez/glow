use std::collections::HashMap;
use lazy_static::lazy_static;

const BLOCKS_JSON: &str = include_str!("../ids/blocks.json");
const ENTITY_TYPES_JSON: &str = include_str!("../ids/entity_types.json");
const FLUIDS_JSON: &str = include_str!("../ids/fluids.json");
const ITEMS_JSON: &str = include_str!("../ids/items.json");

lazy_static! {
    pub static ref BLOCK_IDS: HashMap<String, u32> = 
        serde_json::from_str(BLOCKS_JSON).unwrap();
    pub static ref ENTITY_IDS: HashMap<String, u32> = 
        serde_json::from_str(ENTITY_TYPES_JSON).unwrap();
    pub static ref FLUID_IDS: HashMap<String, u32> = 
        serde_json::from_str(FLUIDS_JSON).unwrap();
    pub static ref ITEM_IDS: HashMap<String, u32> = 
        serde_json::from_str(ITEMS_JSON).unwrap();
}
