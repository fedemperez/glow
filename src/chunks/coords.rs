use super::data::CHUNK_WIDTH;
use nalgebra::{Vector3, vector};
use num_integer::Integer;
use num_traits::Pow;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub struct ChunkCoords(pub i32, pub i32);

impl ChunkCoords {
    pub fn from_pos(pos: &Vector3<f64>) -> Self {
        Self::from_block(&block_coords(pos))
    }
    
    pub fn from_block(pos: &Vector3<i32>) -> Self {
        Self(pos.x.div_floor(&(CHUNK_WIDTH as i32)), pos.z.div_floor(&(CHUNK_WIDTH as i32)))
    }

    pub fn near(pos: &Vector3<f64>, chunk_distance: i32) -> Vec<Self> {
        let center = Self::from_pos(pos);
        let mut pos = pos / (CHUNK_WIDTH as f64);
        pos.y = 0.0;
        let mut result = vec![];
        let max_sq_distance = (chunk_distance as f64).pow(2.0);
        for delta_x in -chunk_distance..=chunk_distance {
            for delta_z in -chunk_distance..=chunk_distance {
                let chunk = vector!(
                    (center.0 + delta_x) as f64, 
                    0.0, 
                    (center.1 + delta_z) as f64);
                let distance: Vector3<f64> = chunk - pos;
                if distance.magnitude_squared() < max_sq_distance {
                    result.push(Self(center.0 + delta_x, center.1 + delta_z));
                }
            }
        }
        result
    }

    pub fn relative(&self, delta: &Vector3<i32>)
        -> (usize, usize, usize)
    {
        let x = delta.x - self.0 * 16;
        let z = delta.z - self.1 * 16;
        (x as usize, delta.y as usize, z as usize)
    }

    pub fn global(&self, x: usize, y: usize, z: usize) 
        -> Vector3<i32>
    {
        let x = x as i32 + self.0 * 16;
        let z = z as i32 + self.1 * 16;
        vector!(x, y as i32, z)
    }
}

pub fn block_coords(pos: &Vector3<f64>) -> Vector3<i32> {
    vector!(pos.x.floor() as i32, pos.y.floor() as i32, pos.z.floor() as i32)
}
