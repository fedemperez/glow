use crate::blocks::Block;
use nalgebra::{Vector3, vector};

use super::{
    World,
    data::CHUNK_HEIGHT,
};

#[derive(Clone)]
pub struct WorldView<'a> {
    world: &'a World,
    center: Vector3<i32>,
}

impl<'a> WorldView<'a> {
    pub fn new(world: &'a World, center: Vector3<i32>)
        -> Self
    {
        Self {
            world, center,
        }
    }

    pub fn get(&self, x: i32, y: i32, z: i32) 
        -> &'static Block 
    {
        let coords = self.center + vector!(x, y, z);
        self.world.get_block(&coords)
    }

    pub fn set(&self, x: i32, y: i32, z: i32, 
        block: &'static Block) 
    {
        let coords = self.center + vector!(x, y, z);
        self.world.set_block(&coords, block);
    }

    pub fn displace(&mut self, dir: Vector3<i32>) {
        self.center += dir
    }
}
