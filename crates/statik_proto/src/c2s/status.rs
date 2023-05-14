pub mod ping;
pub mod request;

use ping::*;
use request::*;
use statik_derive::PacketGroup;

#[derive(Debug, PacketGroup)]
pub enum C2SStatusPacket {
    StatusRequest(C2SStatusRequest),
    Ping(C2SPing),
}
