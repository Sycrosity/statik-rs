pub mod handshake;
pub mod legacy_ping;

use handshake::*;

use statik_common::prelude::*;

#[derive(Debug)]
pub enum C2SHandshakingPacket {
    // #[id = 0x00]
    Handshake(C2SHandshake),
    // #[id = 0xFE]
    // LegacyPing(C2SLegacyPing),
}

impl Packet for C2SHandshakingPacket {
    const PACKET_ID: i32 = 0;

    fn id(&self) -> VarInt {
        match self {
            C2SHandshakingPacket::Handshake(_) => VarInt(0),
        }
    }
}

impl Encode for C2SHandshakingPacket {
    fn encode(&self, _buffer: impl std::io::Write) -> anyhow::Result<()> {
        todo!()
    }
}

impl Decode for C2SHandshakingPacket {
    fn decode(mut buffer: impl std::io::Read) -> anyhow::Result<Self> {
        let id = VarInt::decode(&mut buffer)?;

        debug!("Handshaking packet id: {id}");

        Ok(match id.0 {
            0 => Self::Handshake(C2SHandshake::decode(buffer)?),
            _ => unimplemented!(),
        })
    }
}
