use nalgebra::Vector3;
use num_integer::Integer;

pub const BUCKET_SIZE: u32 = 16;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct BucketCoords(i32, i32);

impl BucketCoords {
    pub fn from_pos(pos: &Vector3<f64>) -> Self {
        Self((pos.x.floor() as i32).div_floor(&(BUCKET_SIZE as i32)),
             (pos.z.floor() as i32).div_floor(&(BUCKET_SIZE as i32)))
    }

    pub fn get_close(&self, distance: u32) -> Vec<Self> {
        let mut result = vec![];
        let bucket_distance = distance.div_ceil(&BUCKET_SIZE) as i32;
        for delta_x in -bucket_distance..=bucket_distance {
            for delta_z in -bucket_distance..=bucket_distance {
                result.push(Self(self.0 + delta_x, self.1 + delta_z));
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::buckets::coords::BUCKET_SIZE;

    use super::BucketCoords;
    use nalgebra::Vector3;

    #[test]
    fn close_coords_test() {
        let coords = BucketCoords(2, 2);
        let close = coords.get_close(BUCKET_SIZE * 3);
        let expected: Vec<BucketCoords> = vec![
            (-1, -1), (-1, 0), (-1, 1), (-1, 2), (-1, 3), (-1, 4), (-1, 5),
            (0, -1), (0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), 
            (1, -1), (1, 0), (1, 1), (1, 2), (1, 3), (1, 4), (1, 5), 
            (2, -1), (2, 0), (2, 1), (2, 2), (2, 3), (2, 4), (2, 5), 
            (3, -1), (3, 0), (3, 1), (3, 2), (3, 3), (3, 4), (3, 5), 
            (4, -1), (4, 0), (4, 1), (4, 2), (4, 3), (4, 4), (4, 5), 
            (5, -1), (5, 0), (5, 1), (5, 2), (5, 3), (5, 4), (5, 5)]
            .into_iter().map(|(x, z)| BucketCoords(x, z)).collect();
        assert_eq!(close, expected);
    }
}