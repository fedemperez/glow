use std::convert::TryFrom;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use super::{Inventory, ItemStack, SlotIndex};
use crate::items::ItemType;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all="PascalCase")] 
pub struct ItemStackPlayerData {
    pub count: i8,
    pub slot: i8,
    #[serde(rename="id")]
    pub item: String,
}

impl TryFrom<Vec<ItemStackPlayerData>> for Inventory {
    type Error = anyhow::Error;
    
    fn try_from(items: Vec<ItemStackPlayerData>)
        -> Result<Self, Self::Error> 
    {
        let mut slots = HashMap::new();
        for stack in items {
            let index = SlotIndex::from_file(stack.slot);
            let item = ItemType::from_str(stack.item.as_str())?;
            slots.insert(index, 
                ItemStack {
                    item,
                    count: stack.count as u8,
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
        inventory.slots.into_iter().map(|(slot, stack)| 
            ItemStackPlayerData {
                slot: slot.to_file(),
                item: stack.item.to_str().into(),
                count: stack.count as i8,
            }
        ).collect()
    }
}
