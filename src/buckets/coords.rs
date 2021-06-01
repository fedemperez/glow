use nalgebra::Vector3;
use num_integer::Integer;

pub const BUCKET_SIZE: u32 = 16;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct BucketCoords(i32, i32);

impl BucketCoords {
    pub fn from_pos(pos: &Vector3<f32>) -> Self {
        Self((pos.x.floor() as i32).div_floor(&(BUCKET_SIZE as i32)),
             (pos.z.floor() as i32).div_floor(&(BUCKET_SIZE as i32)))
    }

    pub fn get_close(&self, distance: u32) -> Vec<Self> {
        let mut result = vec![];
        let bucket_distance = distance.div_ceil(&BUCKET_SIZE) as i32;
        for delta_x in -bucket_distance..bucket_distance {
            for delta_z in -bucket_distance..bucket_distance {
                result.push(Self(self.0 + delta_x, self.1 + delta_z));
            }
        }
        result
    }
}
