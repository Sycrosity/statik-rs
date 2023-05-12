pub mod pong;
pub mod response;

use pong::*;
use response::*;

#[derive(Debug)]
pub enum S2CStatusPacket {
    // #[id = 0x00]
    StatusResponse(S2CStatusResponse),
    // #[id = 0x01]
    Pong(S2CPong),
}
