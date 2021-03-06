use nalgebra::Vector3;
use nbt::{Value, to_writer};
use crate::serialization::push_varint;

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

    pub fn add_angle(&mut self, angle: f32) -> &mut Self {
        let angle = ((angle / 360.0) * 256.0).rem_euclid(256.0) as u8;
        self.add_bytes(&[angle]);
        self
    }

    pub fn add_position_delta(&mut self, delta: f64) -> &mut Self {
        let delta = (delta * 4096.0) as i16;
        self.add_bytes(&delta.to_be_bytes());
        self
    }

    pub fn add_block_position(&mut self, pos: &Vector3<i32>)
        -> &mut Self
    {
        let pos = ((pos.x as i64 & 0x3FFFFFF) << 38) | 
            ((pos.z as i64 & 0x3FFFFFF) << 12) | 
            (pos.y as i64 & 0xFFF);
        self.add_bytes(&pos.to_be_bytes());
        self
    }

    pub fn build(&self) -> Vec<u8> {
        let mut packet = Vec::with_capacity(self.bytes.len() + 5);
        push_varint(self.bytes.len() as u32, &mut packet);
        packet.extend(self.bytes.iter());
        packet
    }
}
