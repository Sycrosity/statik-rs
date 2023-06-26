pub mod handshake;
// pub mod legacy_ping;

use handshake::*;
// use legacy_ping::*;
use statik_derive::PacketGroup;

#[derive(Debug, PacketGroup)]
pub enum C2SHandshakePacket {
    Handshake(C2SHandshake),
    // LegacyPing(C2SLegacyPing),
}
