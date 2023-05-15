use std::io::{Read, Write};

use crate::prelude::*;

/// The [`Encode`] + [`Decode`] implementations must read and write a
/// leading [`VarInt`] packet ID before any other data.
///
/// a packet must have these fields: \[length, packetId, Data\]
///
/// with the types: \[[VarInt], [VarInt], \[bytes\]\]
pub trait Packet: Decode + Encode + Sized + std::fmt::Debug {
    const PACKET_ID: i32;

    /// the VarInt ID of a specified packet (needed to send
    /// any type of any packet)
    fn id(&self) -> VarInt;
    // /// How long this packet is in bytes (needed to send
    // /// any type of any packet) - should be derived from
    // /// the length of the Packet ID + Data length.
    // fn length(&self) -> VarInt;
    // ///
    // fn encode_packet()
}

pub trait Decode: Sized {
    fn decode(buffer: impl Read) -> anyhow::Result<Self>;
}

pub trait Encode: Sized {
    /// Writes this object to the provided writer.
    ///
    /// If this type also implements [`Decode`] then successful calls to this
    /// function returning `Ok(())` must always successfully [`decode`] using
    /// the data that was written to the writer. The exact number of bytes
    /// that were originally written must be consumed during the decoding.
    ///
    /// [`decode`]: Decode::decode
    fn encode(&self, buffer: impl Write) -> anyhow::Result<()>;

    // /// Like [`Encode::encode`], except that a whole slice of values is encoded.
    // ///
    // /// This method must be semantically equivalent to encoding every element of
    // /// the slice in sequence with no leading length prefix (which is exactly
    // /// what the default implementation does), but a more efficient
    // /// implementation may be used.
    // ///
    // /// This optimization is very important for some types like `u8` where
    // /// [`write_all`] is used. Because impl specialization is unavailable in
    // /// stable Rust, we must make the slice specialization part of this trait.
    // ///
    // /// [`write_all`]: Write::write_all
    // fn encode_slice(slice: &[Self], mut buffer: impl Write) -> anyhow::Result<()>
    // where
    //     Self: Sized,
    // {
    //     for value in slice {
    //         value.encode(&mut buffer)?;
    //     }

    //     Ok(())
    // }
}

pub const MAX_PACKET_SIZE: i32 = 2097152;
