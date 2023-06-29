use statik_derive::Packet;

#[derive(Debug, Packet)]
#[packet(id = 0x01)]
pub struct C2SPing {
    /// May be any number. Notchian clients use a system-dependent time value
    /// which is counted in milliseconds.
    ///
    /// This is technically a signed integer, but doesn't matter for this
    /// packet.
    pub payload: u64,
}
