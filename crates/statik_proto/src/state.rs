use statik_common::prelude::*;

#[derive(Debug, Clone, Copy)]
pub enum State {
    Handshake = 0,
    Status = 1,
    Login = 2,
    Play = 3,
}

impl Encode for State {
    fn encode(&self, buffer: impl std::io::Write) -> Result<()> {
        VarInt(*self as i32).encode(buffer)
    }
}

impl Decode for State {
    fn decode(buffer: impl std::io::Read) -> Result<Self> {
        Ok(match VarInt::decode(buffer)?.0 {
            0 => Self::Handshake,
            1 => Self::Status,
            2 => Self::Login,
            3 => Self::Play,
            n => bail!(
                "parsed VarInt returned an invalid State: {n}. Only values 0,1,2 and 3 are valid."
            ),
        })
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string().trim_start_matches("State::"))
    }
}
