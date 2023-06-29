pub mod response;

use response::*;
use statik_core::prelude::*;
use statik_derive::*;

#[derive(Debug, Packet)]
#[packet(id = 0x00, state = State::Status)]
pub struct S2CStatusResponse {
    ///See [Server List Ping#Response](https://wiki.vg/Server_List_Ping#Response); as with all strings this is prefixed by its length as a VarInt.
    pub json_response: StatusResponse,
}

#[derive(Debug, Packet)]
#[packet(id = 0x01, state = State::Status)]
pub struct S2CPong {
    /// Should be the same as sent by the client.
    ///
    /// This is technically a signed integer, but doesn't matter for this
    /// packet.
    pub payload: u64,
}
