use statik_derive::{Decode, Encode};

#[derive(Debug, Encode, Decode)]
pub struct C2SPing {
    ///May be any number. Notchian clients use a system-dependent time value which is counted in milliseconds.
    pub payload: i64,
}
