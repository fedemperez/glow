use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::Result;
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
