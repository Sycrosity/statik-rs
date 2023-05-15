pub mod chat;
pub mod impls;
pub mod packet;
pub mod raw;
pub mod varint;

pub mod prelude {

    pub use crate::chat::*;
    pub use crate::impls::*;
    pub use crate::packet::*;
    pub use crate::raw::*;
    pub use crate::varint::*;

    pub use crate::MINECRAFT_VERSION;
    pub use crate::PROTOCOL_VERSION;

    pub use anyhow::{anyhow, bail};
    pub use log::{debug, error, info, log, trace, warn};
}

pub const MINECRAFT_VERSION: &str = "1.19.4";
pub const PROTOCOL_VERSION: usize = 762;
