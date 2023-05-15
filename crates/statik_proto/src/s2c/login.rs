pub mod disconnect;
pub mod encryption_request;
pub mod login_plugin_request;
pub mod login_success;
pub mod set_compression;

use disconnect::*;
use encryption_request::*;
use login_plugin_request::*;
use login_success::*;
use set_compression::*;
use statik_derive::PacketGroup;

#[derive(Debug, PacketGroup)]
pub enum S2CLoginPacket {
    Disconnect(S2CDisconnect),
    EncryptionRequest(S2CEncryptionRequest),
    LoginSuccess(S2CLoginSuccess),
    SetCompression(S2CSetCompression),
    LoginPluginRequest(S2CLoginPluginRequest),
}
