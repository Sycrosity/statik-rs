use std::io::{Read, Write};

use crate::{prelude::*, state};

/// The [`Encode`] + [`Decode`] implementations must read and write a
/// leading [`VarInt`] packet ID before any other data.
///
/// a packet must have these fields internally: \[length, packetId, AllOtherData\]
///
/// with the types: \[`VarInt`, `VarInt`, `[u8]`\]
pub trait Packet: Decode + Encode + Sized + std::fmt::Debug {
    const ID: i32;
    const STATE: state::State;

    /// the VarInt ID of a specified packet (needed to send
    /// any type of any packet)
    fn id(&self) -> VarInt;
    /// which server [`State`] this packet is a part of.
    fn state(&self) -> state::State;
}

pub trait Decode: Sized {
    fn decode(buffer: impl Read) -> Result<Self>;
}

pub trait Encode: Sized {
    /// Writes this object to the provided writer.
    ///
    /// If this type also implements [`Decode`] then successful calls to this
    /// function returning `Ok(())` must always successfully [`Decode::decode`] using
    /// the data that was written to the writer. The exact number of bytes
    /// that were originally written must be consumed during the decoding.
    fn encode(&self, buffer: impl Write) -> Result<()>;
}

pub const MAX_PACKET_SIZE: i32 = 2097152;
