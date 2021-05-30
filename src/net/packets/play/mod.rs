mod writer;
mod reader;
mod clientbound;
mod serverbound;

pub use clientbound::{ClientboundPacket, PlayerInfo, PlayerInfoProperty};
pub use serverbound::ServerboundPacket;