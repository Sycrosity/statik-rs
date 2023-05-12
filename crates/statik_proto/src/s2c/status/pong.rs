use statik_derive::Packet;

#[derive(Debug)]
pub struct S2CPong {
    ///Should be the same as sent by the client.
    payload: i64,
}
