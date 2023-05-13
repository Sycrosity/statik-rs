pub mod handshake;
pub mod legacy_ping;

use handshake::*;
use legacy_ping::*;
use statik_common::prelude::*;

#[derive(Debug)]
pub enum C2SHandshakingPacket {
    // #[id = 0x00]
    Handshake(C2SHandshake),
    // #[id = 0xFE]
    // LegacyPing(C2SLegacyPing),
}

impl Packet for C2SHandshakingPacket {
    fn id(&self) -> VarInt {
        match self {
            C2SHandshakingPacket::Handshake(_) => VarInt(0),
        }
    }
}

impl Encode for C2SHandshakingPacket {
    fn encode(&self, buffer: &mut dyn std::io::Write) -> anyhow::Result<()> {
        todo!()
    }
}

impl Decode for C2SHandshakingPacket {
    fn decode(buffer: &mut dyn std::io::Read) -> anyhow::Result<Self> {
        let id = VarInt::decode(buffer)?;

        debug!("Handshaking packet id: {id}");

        Ok(match id.0 {
            0 => Self::Handshake(C2SHandshake::decode(buffer)?),
            _ => unimplemented!(),
        })
    }
}
