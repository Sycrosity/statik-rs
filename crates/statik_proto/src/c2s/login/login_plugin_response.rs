use statik_common::prelude::*;
use statik_derive::Packet;

#[derive(Debug, Packet)]
#[packet_id = 0x02]
pub struct C2SLoginPluginResponse {
    pub message_id: VarInt,
    // pub successful: bool,
    ///
    pub data: Option<RawBytes>,
}
