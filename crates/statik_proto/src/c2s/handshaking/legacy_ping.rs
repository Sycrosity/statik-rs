use statik_common::prelude::*;
// use statik_derive::Packet;
use byteorder::{ReadBytesExt, WriteBytesExt};

#[derive(Debug)]
pub struct C2SLegacyPing {
    ///always 1 (0x01).
    pub payload: u8,
}
