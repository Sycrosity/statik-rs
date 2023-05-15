use statik_common::prelude::*;
use statik_derive::Packet;

#[derive(Debug, Packet)]
#[packet_id = 0x00]
pub struct S2CDisconnect {
    /// Why the client was disconnected before login success.
    pub reason: Chat,
}
