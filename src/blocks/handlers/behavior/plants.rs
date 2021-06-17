use crate::blocks::Block;

pub fn can_survive_on(block: &Block) -> bool {
    block.material.name == "minecraft:soil" || 
    block.material.name == "minecraft:solid_organic"
}
