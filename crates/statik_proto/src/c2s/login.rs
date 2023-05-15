pub mod encryption_response;
pub mod login_plugin_response;
pub mod login_start;

use encryption_response::*;
use login_plugin_response::*;
use login_start::*;

#[derive(Debug)]
pub enum C2SLoginPacket {
    LoginStart(C2SLoginStart),
    EncryptionResponse(C2SEncryptionResponse),
    LoginPluginResponse(C2SLoginPluginResponse),
}
