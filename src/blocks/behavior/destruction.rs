use block_macro::block_id;

use crate::chunks::WorldView;
use crate::blocks::Block;

impl Block {
    pub fn destroy(&self, view: &WorldView) {
        let waterlogged = self.props.get("waterlogged")
            .map(|wl| wl == "true")
            .unwrap_or(false);
        if waterlogged {
            let water = Block::from_state_id(block_id!(water)).unwrap();
            view.set(0, 0, 0, water);
        } else {
            view.set(0, 0, 0, Block::air());
        }
    }
}
