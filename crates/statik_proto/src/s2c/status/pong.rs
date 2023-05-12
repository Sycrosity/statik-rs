use statik_derive::{Decode, Encode};

#[derive(Debug, Decode, Encode)]
pub struct S2CPong {
    ///Should be the same as sent by the client.
    payload: i64,
}
