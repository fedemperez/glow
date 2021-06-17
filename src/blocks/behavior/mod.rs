mod placement;
mod updates;
mod destruction;
mod random_tick;
mod interaction;
mod stairs;

use super::Block;
pub use interaction::InteractionResult;

fn can_place_plant_on(block: &Block) -> bool {
    block.material.name == "minecraft:soil" || 
    block.material.name == "minecraft:solid_organic"
}
