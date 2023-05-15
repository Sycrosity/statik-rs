use statik_common::prelude::*;
use statik_derive::Packet;

#[derive(Debug, Packet)]
#[packet_id = 0x04]
pub struct S2CLoginPluginRequest {
    pub message_id: VarInt,
    /// See: https://wiki.vg/Protocol#Identifier
    pub channel: String,
    pub data: Vec<u8>,
}
