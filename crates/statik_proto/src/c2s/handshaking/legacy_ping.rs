use statik_common::prelude::*;
// use statik_derive::Packet;
use byteorder::{ReadBytesExt, WriteBytesExt};

#[derive(Debug)]
pub struct C2SLegacyPing {
    ///always 1 (0x01).
    pub payload: u8,
}

impl Packet for C2SLegacyPing {
    fn id(&self) -> VarInt {
        VarInt(0x0F)
    }

    fn length(&self) -> VarInt {
        VarInt(2)
    }
}

impl Encode for C2SLegacyPing {
    fn encode(&self, buffer: &mut dyn std::io::Write) -> anyhow::Result<()> {
        self.payload.encode(buffer)
    }
}

impl Decode for C2SLegacyPing {
    fn decode(buffer: &mut dyn std::io::Read) -> anyhow::Result<Self> {
        Ok(Self {
            payload: buffer.read_u8()?,
        })
    }
}
