use statik_core::prelude::*;
use statik_derive::*;

#[derive(Debug, Packet)]
#[packet(id = 0x00, state = State::Status)]
///_no fields._
pub struct C2SStatusRequest {}

#[derive(Debug, Packet)]
#[packet(id = 0x01, state = State::Status)]
pub struct C2SPing {
    /// May be any number. Notchian clients use a system-dependent time value
    /// which is counted in milliseconds.
    ///
    /// This is technically a signed integer, but doesn't matter for this
    /// packet.
    pub payload: u64,
}
