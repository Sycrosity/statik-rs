pub mod login;
pub mod status;

use login::*;
use statik_derive::PacketGroup;
use status::*;

#[derive(Debug, PacketGroup)]
pub enum S2CPacket {
    //Status
    StatusResponse(S2CStatusResponse),
    Pong(S2CPong),

    //Login
    Disconnect(S2CDisconnect),
    EncryptionRequest(S2CEncryptionRequest),
    LoginSuccess(S2CLoginSuccess),
    SetCompression(S2CSetCompression),
    LoginPluginRequest(S2CLoginPluginRequest),
}
