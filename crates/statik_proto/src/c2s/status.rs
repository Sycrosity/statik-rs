pub mod ping;
pub mod request;

use ping::*;
use request::*;
use statik_derive::{Packet, PrintTokenStream};

#[derive(Debug)]
pub enum C2SStatusPacket {
    // #[id = 0x00]
    StatusRequest(C2SStatusRequest),
    // #[id = 0x01]
    Ping(C2SPing),
}
