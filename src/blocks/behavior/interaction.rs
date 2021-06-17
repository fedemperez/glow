use crate::chunks::WorldView;
use crate::blocks::{Block, BlockClass};

impl Block {
    pub fn interact(&self, view: &WorldView) -> InteractionResult {
        match self.btype.class {
            BlockClass::DoorBlock => {
                if self.material.name != "minecraft:metal" {
                    view.set(0, 0, 0, toggle_open(self));
                    if self.props["half"] == "upper" {
                        let other = view.get(0, -1, 0);
                        view.set(0, -1, 0, toggle_open(other));
                    } else {
                        let other = view.get(0, 1, 0);
                        view.set(0, 1, 0, toggle_open(other));
                    }
                    InteractionResult::PreventPlacing
                } else {
                    InteractionResult::None
                }
            },
            BlockClass::TrapdoorBlock | BlockClass::FenceGateBlock => {
                if self.material.name != "minecraft:metal" {
                    view.set(0, 0, 0, toggle_open(self));
                    InteractionResult::PreventPlacing
                } else {
                    InteractionResult::None
                }
            },
            BlockClass::CraftingTableBlock => {
                InteractionResult::PreventPlacing
            },
            _ => InteractionResult::None,
        }
    }
}

pub enum InteractionResult {
    None,
    PreventPlacing,
}

fn toggle_open(block: &Block) -> &'static Block {
    let mut props = block.props.clone();
    if let Some(open) = props.get_mut("open") {
        *open = if open == "true" {
            "false".into()
        } else {
            "true".into()
        }
    }
    block.btype.with_props(&props).unwrap()
}
