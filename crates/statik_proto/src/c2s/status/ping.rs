use statik_derive::Packet;

#[derive(Debug)]
pub struct C2SPing {
    ///May be any number. Notchian clients use a system-dependent time value which is counted in milliseconds.
    payload: i64,
}
