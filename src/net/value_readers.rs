use tokio::io::{AsyncRead, AsyncReadExt};
use anyhow::{Result, anyhow};

pub async fn read_varint<R: AsyncRead>(reader: &mut R) -> Result<u32>
    where R: Unpin
{
    let mut byte_counter = 0;
    let mut result: u32 = 0;
    loop {
        let byte = reader.read_u8().await?;
        let value = (byte & 0b01111111) as u32;
        result |= value << (7 * byte_counter);
        byte_counter += 1;
        if byte_counter > 5 {
            break Err(anyhow!("Varint is too long"));
        }
        if byte & 0b10000000 == 0 {
            break Ok(result);
        }
    }
}

pub async fn read_str<R: AsyncRead>(reader: &mut R) -> Result<String>
    where R: Unpin
{
    let length = read_varint(reader).await? as usize;
    let mut buffer = vec![0; length];
    reader.read_exact(buffer.as_mut_slice()).await?;
    String::from_utf8(buffer).map_err(|e| anyhow::Error::new(e))
}