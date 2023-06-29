pub mod c2s;
pub mod s2c;

pub mod prelude {

    pub use crate::{
        c2s::{handshake::*, login::*, status::*, C2SPacket},
        s2c::{login::*, status::*, S2CPacket},
    };
}
