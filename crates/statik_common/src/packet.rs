use std::io::{Read, Write};

use crate::prelude::*;

/// The [`Encode`] + [`Decode`] implementations must read and write a
/// leading [`VarInt`] packet ID before any other data.
///
/// a packet must have these fields: \[length, packetId, Data\]
///
/// with the types: \[[VarInt], [VarInt], \[bytes\]\]
pub trait Packet: Decode + Encode {
    /// the VarInt ID of a specified packet (needed to send
    /// any type of any packet)
    fn id(&self) -> VarInt;
    /// How long this packet is in bytes (needed to send
    /// any type of any packet) - should be derived from
    /// the length of the Packet ID + Data length.
    fn length(&self) -> VarInt;
}

pub trait Decode: Sized {
    fn decode(buffer: &mut dyn Read) -> anyhow::Result<Self>;
}

pub trait Encode: Sized {
    fn encode(&self, buffer: &mut dyn Write) -> anyhow::Result<()>;
}

pub const MAX_PACKET_SIZE: i32 = 2097152;
