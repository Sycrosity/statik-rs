pub mod status;

use statik_common::prelude::*;
use status::*;

use crate::state::State;

// pub enum S2CPacket {
//     Status(S2CStatusPacket),
// }

// impl Packet for S2CPacket {
//     fn id(&self) -> VarInt {
//         todo!()
//     }

//     fn length(&self) -> VarInt {
//         todo!()
//     }
// }

// impl Decode for S2CPacket {
//     fn decode(state: State, buffer: &mut dyn std::io::Read) -> Result<Self, DecodeError> {
//         match state {
//             State::Handshake => todo!(),
//             State::Status => todo!(),
//             State::Login => unimplemented!(),
//             State::Play => unimplemented!(),
//         }
//     }
// }

// impl Encode for S2CPacket {
//     fn encode(&self, buffer: &mut dyn std::io::Write) -> Result<(), EncodeError> {
//         match self {
//             S2CPacket::Status(p) => p.encode(buffer),
//         }
//     }
// }
