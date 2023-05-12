pub mod handshake;
pub mod legacy_ping;

use handshake::*;
use legacy_ping::*;

#[derive(Debug)]
#[repr(i32)]
pub enum C2SHandshakingPacket {
    Handshake(C2SHandshake) = 0x00,
    LegacyPing(C2SLegacyPing) = 0xFE,
}
