use super::chunk::CHUNK_WIDTH;
use nalgebra::Vector3;
use num_integer::Integer;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub struct ChunkCoords(pub i32, pub i32);

impl ChunkCoords {
    pub fn from_pos(pos: Vector3<f64>) -> Self {
        let (x, _, z) = block_coords(pos);
        Self::from_block(x, z)
    }
    
    pub fn from_block(x: i32, z: i32) -> Self {
        Self(x.div_floor(&(CHUNK_WIDTH as i32)), z.div_floor(&(CHUNK_WIDTH as i32)))
    }

    pub fn relative(&self, x: i32, y: i32, z: i32)
        -> (usize, usize, usize)
    {
        let x = x - self.0 * 16;
        let z = z - self.1 * 16;
        (x as usize, y as usize, z as usize)
    }

    pub fn global(&self, x: usize, y: usize, z: usize) 
        -> (i32, i32, i32) 
    {
        let x = x as i32 + self.0 * 16;
        let z = z as i32 + self.1 * 16;
        (x, y as i32, z)
    }

    pub fn get_close(&self, chunk_distance: i32) -> Vec<Self> {
        let mut result = vec![];
        for delta_x in -chunk_distance..=chunk_distance {
            for delta_z in -chunk_distance..=chunk_distance {
                result.push(Self(self.0 + delta_x, self.1 + delta_z));
            }
        }
        result
    }
}

pub fn block_coords(pos: Vector3<f64>) -> (i32, i32, i32) {
    (pos.x.floor() as i32, pos.y.floor() as i32, pos.z.floor() as i32)
}