use statik_derive::Packet;

#[derive(Debug, Packet)]
#[packet_id = 0x01]
pub struct C2SPing {
    ///May be any number. Notchian clients use a system-dependent time value which is counted in milliseconds.
    pub payload: i64,
}
