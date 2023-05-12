pub mod ping;
pub mod request;

use ping::*;
use request::*;

#[derive(Debug)]
#[repr(i32)]
pub enum C2SStatusPacket {
    // #[id = 0x00]
    StatusRequest(C2SStatusRequest),
    // #[id = 0x01]
    Ping(C2SPing),
}
