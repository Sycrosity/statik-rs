// #[derive(Debug, Clone, Copy)]
// pub enum State {
//     Handshake = VarInt(0),
//     Status = VarInt(1),
//     Login = VarInt(2),
//     Play = VarInt(3),
// }

use statik_common::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Handshake = 0,
    Status = 1,
    Login = 2,
    Play = 3,
}

impl Encode for State {
    fn encode(&self, buffer: impl std::io::Write) -> anyhow::Result<()> {
        VarInt(*self as i32).encode(buffer)
    }
}

impl Decode for State {
    fn decode(buffer: impl std::io::Read) -> anyhow::Result<Self> {
        Ok(match VarInt::decode(buffer)?.0 {
            0 => Self::Handshake,
            1 => Self::Status,
            2 => Self::Login,
            3 => Self::Play,
            n => anyhow::bail!(
                "parsed VarInt returned an invalid State: {n}. Only values 0,1,2 and 3 are valid."
            ),
        })
    }
}
