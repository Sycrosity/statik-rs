pub mod pong;
pub mod response;

use pong::*;
use response::*;

#[derive(Debug)]
#[repr(i32)]

pub enum S2CStatusPacket {
    StatusResponse(S2CStatusResponse) = 0x00,
    Pong(S2CPong) = 0x01,
}
