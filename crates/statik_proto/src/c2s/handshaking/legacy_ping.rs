use statik_derive::Packet;

#[derive(Debug)]
pub struct C2SLegacyPing {
    ///always 1 (0x01).
    payload: u8,
}
