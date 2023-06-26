pub mod pong;
pub mod response;

use pong::*;
use response::*;
use statik_derive::PacketGroup;

#[derive(Debug, PacketGroup)]
pub enum S2CStatusPacket {
    StatusResponse(S2CStatusResponse),
    Pong(S2CPong),
}
