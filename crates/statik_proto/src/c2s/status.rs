pub mod ping;
pub mod request;

use ping::*;
use request::*;

#[derive(Debug)]
#[repr(i32)]
pub enum C2SStatusPacket {
    StatusRequest(C2SStatusRequest) = 0x00,
    Ping(C2SPing) = 0x01,
}
