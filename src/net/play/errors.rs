use thiserror::Error;

#[derive(Error, Debug)]
#[error("Unknown packet id: {0}")]
pub struct UnknownPacket(pub u8);
