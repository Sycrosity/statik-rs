use statik_derive::Packet;

#[derive(Debug, Packet)]
#[packet_id = 0xFE]
pub struct C2SLegacyPing {
    ///always 1 (0x01).
    pub payload: u8,
}
