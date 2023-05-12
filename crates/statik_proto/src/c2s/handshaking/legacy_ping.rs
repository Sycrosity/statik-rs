use statik_derive::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
pub struct C2SLegacyPing {
    ///always 1 (0x01).
    pub payload: u8,
}
