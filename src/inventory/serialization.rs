use std::convert::TryFrom;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::{Inventory, ItemStack, SlotIndex};
use crate::common::item_stack::ItemId;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all="PascalCase")] 
pub struct ItemStackPlayerData {
    pub count: i8,
    pub slot: i8,
    #[serde(rename="id")]
    pub id: String,
}

impl TryFrom<Vec<ItemStackPlayerData>> for Inventory {
    type Error = anyhow::Error;
    
    fn try_from(items: Vec<ItemStackPlayerData>)
        -> Result<Self, Self::Error> 
    {
        let mut slots = HashMap::new();
        for item in items {
            let index = SlotIndex::from_file(item.slot);
            let id = ItemId::from_str(item.id.as_str())?;
            slots.insert(index, 
                ItemStack {
                    id,
                    count: item.count as u8,
                    nbt: None,
                });
        }
        Ok(Self {
            held_slot: SlotIndex::from_hotbar(0),
            slots,
        })
    }
}

impl From<Inventory> for Vec<ItemStackPlayerData> {
    fn from(inventory: Inventory) -> Self {
        let mut result = vec![];
        for (slot, item) in inventory.slots {
            if let Ok(id) = item.id.to_str() {
                result.push(
                    ItemStackPlayerData {
                        slot: slot.to_file(),
                        id: id.into(),
                        count: item.count as i8,
                    })
            }
        }
        result
    }
}
