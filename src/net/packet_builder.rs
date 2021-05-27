use anyhow::Result;
use tokio::io::{AsyncWrite, AsyncWriteExt};
use nbt::{Value, to_writer};
use crate::util::push_varint;

pub struct PacketBuilder {
    bytes: Vec<u8>,
}

impl PacketBuilder {
    pub fn new(id: u8) -> Self {
        Self {
            bytes: vec![id],
        }
    }

    pub fn add_varint(&mut self, value: u32) -> &mut Self {
        push_varint(value, &mut self.bytes);
        self
    }

    pub fn add_str(&mut self, value: &str) -> &mut Self {
        self.add_varint(value.len() as u32)
            .add_bytes(value.as_bytes())
    }

    pub fn add_bytes(&mut self, value: &[u8]) -> &mut Self {
        self.bytes.extend_from_slice(value);
        self
    }

    pub fn add_nbt(&mut self, value: &Value) -> &mut Self {
        to_writer(&mut self.bytes, value, None).unwrap();
        self
    }

    pub async fn write<W: AsyncWrite>(&self, writer: &mut W) -> Result<()>
        where W: Unpin
    {
        let mut length = vec![];
        push_varint(self.bytes.len() as u32, &mut length);
        writer.write_all(length.as_slice()).await?;
        writer.write_all(self.bytes.as_slice()).await?;
        Ok(())
    }
}