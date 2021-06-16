mod placement;
mod updates;
mod destruction;
mod random_tick;

use super::{Block, BlockClass, BlockType};

fn can_place_plant_on(block: &Block) -> bool {
    block.material.name == "minecraft:soil" || 
    block.material.name == "minecraft:solid_organic"
}
