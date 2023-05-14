use statik_derive::Packet;

#[derive(Debug, Packet)]
#[packet_id = 0x00]
///_no fields._
pub struct C2SStatusRequest {}
