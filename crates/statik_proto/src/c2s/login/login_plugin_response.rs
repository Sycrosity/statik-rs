use statik_common::prelude::*;
use statik_derive::Packet;

#[derive(Debug, Packet)]
#[packet_id = 0x02]
pub struct C2SLoginPluginResponse {
    /// Should match message ID from the server.
    pub message_id: VarInt,
    /// Any data, depending on the channel. The length of this array must be
    /// inferred from the packet length. Only sent if the previous byte is
    /// true (denoted bythe Option enum).
    pub data: Option<RawBytes>,
}
