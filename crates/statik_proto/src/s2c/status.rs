pub mod pong;
pub mod response;

use pong::*;
use response::*;

#[derive(Debug)]
pub enum S2CStatusPacket {
    StatusResponse(S2CStatusResponse),
    Pong(S2CPong),
}
