use crate::chunks::WorldView;
use block_macro::block_id;
use super::{Block, BlockClass, can_place_plant_on, stairs::get_stair_shape};

impl Block {
    pub fn update(&'static self, view: &WorldView) {
        match self.btype.class {
            BlockClass::StairsBlock => {
                let shape = get_stair_shape(&self.props, view);
                if shape != self.props["shape"] {
                    let mut props = self.props.clone();
                    props.insert("shape".into(), shape);
                    let block = self.btype.with_props(&props).unwrap();
                    view.set(0, 0, 0, block);
                }
            },
            BlockClass::DoorBlock => {
                double_block_update(self, view);
            },
            BlockClass::TallFlowerBlock | BlockClass::TallPlantBlock => {
                double_block_update(self, view);
                if self.props.get("half").unwrap() == "lower" {
                    plant_root_update(self, view);
                }
            },
            BlockClass::FlowerBlock | BlockClass::FernBlock => {
                plant_root_update(self, view);
            },
            BlockClass::SugarCaneBlock | BlockClass::BambooBlock => {
                tall_plant_update(self, view);
            },
            _ => (),
        }
    }
}

fn double_block_update(block: &'static Block, view: &WorldView) {
    let other = if block.props.get("half").unwrap() == "lower" {
        view.get(0, 1, 0)
    } else {
        view.get(0, -1, 0)
    };
    if other.btype != block.btype {
        view.set(0, 0, 0, Block::air());
    }
}

fn tall_plant_update(block: &'static Block, view: &WorldView) {
    let below = view.get(0, -1, 0);
    if below.btype != block.btype {
        plant_root_update(block, view);
    }
}

fn plant_root_update(block: &'static Block, view: &WorldView) {
    let below = view.get(0, -1, 0);
    if !can_place_plant_on(below) {
        view.set(0, 0, 0, Block::air());
    }
}
