use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::Result;
use nalgebra::{Vector3, vector};
use tokio::io::AsyncWriteExt;
use tokio::{fs::File, io::AsyncReadExt};

pub fn get_time_millis() -> u64 {
    let since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_epoch.as_millis() as u64
}

pub async fn read_file(path: impl AsRef<Path>) 
    -> Result<Vec<u8>>
{
    let mut file = File::open(path).await?;
    let mut data = vec![];
    file.read_to_end(&mut data).await?;
    Ok(data)
}

pub async fn write_file(path: impl AsRef<Path>, data: &[u8])
    -> Result<()>
{
    let mut file = File::create(path).await?;
    file.write_all(data).await?;
    Ok(())
}

pub fn cardinal_to_vec(dir: &str) -> Vector3<i32> {
    match dir {
        "south" => vector!(0, 0, 1),
        "west" => vector!(-1, 0, 0),
        "north" => vector!(0, 0, -1),
        _east => vector!(1, 0, 0),
    }
}

const ADJACENT_DELTAS: [Vector3<i32>; 6] = [
    vector!(-1, 0, 0), vector!(0, -1, 0), vector!(0, 0, -1),
    vector!(1, 0, 0), vector!(0, 1, 0), vector!(0, 0, 1),
];

pub fn adjacent_coords(center: &Vector3<i32>) -> Vec<Vector3<i32>> {
    ADJACENT_DELTAS.iter()
        .map(|delta| center + delta)
        .collect()
}
