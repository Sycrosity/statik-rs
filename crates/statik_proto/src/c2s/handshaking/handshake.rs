use statik_derive::Packet;

use crate::state::State;

use statik_common::prelude::*;

#[derive(Debug)]
pub struct C2SHandshake {
    ///See [protocol version numbers](https://wiki.vg/Protocol_version_numbers) (currently 762 in Minecraft 1.19.4).
    pub protocol_version: VarInt,
    ///Hostname or IP, e.g. localhost or 127.0.0.1, that was used to connect. The Notchian server does not use this information. Note that SRV records are a simple redirect, e.g. if _minecraft._tcp.example.com points to mc.example.org, users connecting to example.com will provide example.org as server address in addition to connecting to it.
    pub server_address: String,
    ///Default is 25565. The Notchian server does not use this information.
    pub server_port: u16,
    ///1 for Status, 2 for Login.
    pub next_state: State,
}
