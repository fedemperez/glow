use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;
use anyhow::Result;
use nbt::Value;
use tokio::io::AsyncWriteExt;
use tokio::{fs::File, io::AsyncReadExt};

pub fn compacted_long(values: Vec<u16>, bits_per_value: u32) -> Vec<i64> {
    let mut result = Vec::with_capacity(values.len() / (64 / bits_per_value as usize));
    let mut current_long = 0;
    let mut inserted_bits = 0;
    let mask = 2u16.pow(bits_per_value) - 1;
    for value in values {
        if inserted_bits + bits_per_value > 64 {
            result.push(current_long as i64);
            current_long = 0;
            inserted_bits = 0;
        }
        let mut value = (value & mask) as u64;
        value <<= inserted_bits;
        current_long |= value;
        inserted_bits += bits_per_value;
    }
    if inserted_bits > 0 {
        result.push(current_long as i64);
    }
    result
}

pub fn push_varint(mut value: u32, buffer: &mut Vec<u8>) {
    loop {
        let mut byte = value as u8 & 0b01111111;
        value >>= 7;
        if value != 0 {
            byte |= 0b10000000;
        }
        buffer.push(byte);
        if value == 0 {
            break
        }
    }
}

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

#[cfg(test)]
mod tests {
    use super::compacted_long;
    #[test]
    fn compacted_long_test() {
        let input = vec![1, 2, 2, 3, 4, 4, 5, 6, 6, 4, 8, 0, 7, 4, 3, 13, 15, 16, 9, 14, 10, 12, 0, 2];
        let result = compacted_long(input, 5);
        let expected = vec![0x0020863148418841, 0x01018A7260F68C87];
        assert_eq!(result, expected);
    }
}