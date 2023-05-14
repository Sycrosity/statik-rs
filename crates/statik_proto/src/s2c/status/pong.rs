use statik_derive::Packet;

#[derive(Debug, Packet)]
#[packet_id = 0x01]
pub struct S2CPong {
    ///Should be the same as sent by the client.
    pub payload: i64,
}
