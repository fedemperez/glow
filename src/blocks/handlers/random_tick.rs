use block_macro::block_id;
use nalgebra::{Vector3, vector};
use rand::Rng;
use rand::prelude::SliceRandom;
use rand::thread_rng;

use crate::blocks::classes::BlockClass;
use crate::chunks::WorldView;

use crate::blocks::Block;

const GRASS_GROW_DIRECTIONS: &[Vector3<i32>] = &[
    vector!(1, -1, 0), vector!(-1, -1, 0), 
    vector!(0, -1, 1), vector!(0, -1, -1), 
    vector!(1, 0, 0), vector!(-1, 0, 0), 
    vector!(0, 0, 1), vector!(0, 0, -1), 
    vector!(1, 1, 0), vector!(-1, 1, 0), 
    vector!(0, 1, 1), vector!(0, 1, -1), 
];

impl Block {
    pub fn random_tick(&self, view: &WorldView) {
        match self.btype.class {
            BlockClass::GrassBlock | BlockClass::MyceliumBlock => {
                let mut rng = thread_rng();
                for dir in GRASS_GROW_DIRECTIONS
                    .choose_multiple(&mut rng, 3) 
                {
                    let dir = vector!(dir.x, rng.gen_range(-1..=1), dir.z);
                    let to = view.get(dir.x, dir.y, dir.z);
                    let above_to = view.get(dir.x, dir.y + 1, dir.z);
                    if to.id == block_id!(dirt) && !above_to.opaque {
                        let new = self.btype
                            .with_props(&self.btype.default_state)
                            .unwrap();
                        view.set(dir.x, dir.y, dir.z, new);
                        break;
                    }
                }
            },
            _ => (),
        }
    }
}
