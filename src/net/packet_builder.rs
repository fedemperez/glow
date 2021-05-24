use anyhow::Result;
use tokio::io::{AsyncWrite, AsyncWriteExt};
use nbt::{Value, to_writer};

pub struct PacketBuilder {
    bytes: Vec<u8>,
}

impl PacketBuilder {
    pub fn new(id: u8) -> Self {
        Self {
            bytes: vec![id],
        }
    }

    pub fn add_varint(mut self, value: u32) -> Self {
        push_varint(value, &mut self.bytes);
        self
    }

    pub fn add_str(mut self, value: &str) -> Self {
        self.add_varint(value.len() as u32)
            .add_bytes(value.as_bytes())
    }

    pub fn add_bytes(mut self, value: &[u8]) -> Self {
        self.bytes.extend_from_slice(value);
        self
    }

    pub fn add_nbt(mut self, value: &Value) -> Self {
        to_writer(&mut self.bytes, value, None).unwrap();
        self
    }

    pub async fn write<W: AsyncWrite>(self, writer: &mut W) -> Result<()>
        where W: Unpin
    {
        let mut length = vec![];
        push_varint(self.bytes.len() as u32, &mut length);
        writer.write_all(length.as_slice()).await?;
        writer.write_all(self.bytes.as_slice()).await?;
        Ok(())
    }
}

fn push_varint(mut value: u32, buffer: &mut Vec<u8>) {
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
