mod serialization;

use crate::common::item_stack::ItemStack;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serialization::ItemStackPlayerData;

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(try_from = "Vec<ItemStackPlayerData>")]
#[serde(into = "Vec<ItemStackPlayerData>")]
pub struct Inventory {
    held_slot: SlotIndex,
    slots: HashMap<SlotIndex, ItemStack>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            held_slot: SlotIndex::from_hotbar(0),
            slots: HashMap::new(),
        }
    }

    pub fn set_slot(&mut self, index: SlotIndex, 
        stack: Option<ItemStack>)     
    {
        if let Some(stack) = stack {
            self.slots.insert(index, stack);
        } else {
            self.slots.remove(&index);
        }
    }

    pub fn set_held_slot(&mut self, slot: SlotIndex) {
        self.held_slot = slot;
    }

    pub fn get_held(&self) -> Option<&ItemStack> {
        self.slots.get(&self.held_slot)
    }

    pub fn get_window(&self) -> Vec<Option<ItemStack>> {
        let mut window = vec![None; 46];
        for (slot, item) in &self.slots {
            window[slot.to_network() as usize] = Some(item.clone());
        }
        window
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct SlotIndex(u8);

impl SlotIndex {
    pub fn from_network(index: u8) -> Self {
        if index >= 36 && index <= 44 {
            Self(index - 36) 
        } else {
            Self(index)
        }
    }

    pub fn from_file(index: i8) -> Self {
        Self(index as u8)
    }

    pub fn from_hotbar(index: u8) -> Self {
        Self(index)
    }

    pub fn to_file(&self) -> i8 {
        self.0 as i8
    }

    pub fn to_network(&self) -> u16 {
        if self.0 < 9 {
            (self.0 + 36) as u16
        } else {
            self.0 as u16
        }
    }
}
