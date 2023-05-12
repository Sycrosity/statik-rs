pub mod chat;
pub mod error;
pub mod impls;
pub mod packet;
pub mod varint;

pub mod prelude {

    pub use crate::chat::*;
    pub use crate::error::*;
    pub use crate::impls::*;
    pub use crate::packet::*;
    pub use crate::varint::*;

    pub use crate::MINECRAFT_VERSION;
    pub use crate::PROTOCOL_VERSION;
}

pub const MINECRAFT_VERSION: &str = "1.19.4";
pub const PROTOCOL_VERSION: usize = 762;
