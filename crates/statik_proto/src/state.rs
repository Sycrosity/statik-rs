// #[derive(Debug, Clone, Copy)]
// pub enum State {
//     Handshake = VarInt(0),
//     Status = VarInt(1),
//     Login = VarInt(2),
//     Play = VarInt(3),
// }

#[derive(Debug, Clone, Copy)]
#[repr(i32)]
pub enum State {
    Handshake = 0x00,
    Status = 0x01,
    Login = 0x02,
    Play = 0x03,
}
