use statik_common::prelude::*;
use statik_derive::Packet;

#[derive(Debug, Packet)]
#[packet_id = 0x03]
pub struct S2CSetCompression {
    pub threshold: VarInt,
}
