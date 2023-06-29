use std::io::{Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};

use crate::prelude::*;

const SEGMENT_BITS: u8 = 0x7f;
const CONTINUE_BIT: u8 = 0x80;

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct VarInt(pub i32);

impl std::fmt::Display for VarInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Encode for VarInt {
    fn encode(&self, mut buffer: impl Write) -> Result<()> {
        let mut value = self.0 as u32;

        loop {
            let part = value as u8;
            value >>= 7;
            if value == 0 {
                buffer.write_u8(part & 0x7f)?;
                break Ok(());
            } else {
                buffer.write_u8(part | 0x80)?;
            }
        }
    }
}

impl Decode for VarInt {
    fn decode(mut buffer: impl Read) -> Result<Self> {
        let mut value = 0b0;
        let mut pos = 0b0;

        loop {
            let byte = buffer.read_u8()?;

            value |= ((byte & SEGMENT_BITS) as i32) << pos;

            if (byte & CONTINUE_BIT) == 0 {
                return Ok(VarInt(value));
            }

            pos += 7;

            if pos >= 32 {
                return Err(anyhow!(
                    "Cannot decode VarInt! Exceeds maximum capacity of 5 bytes \
                     (2147483647/-2147483648)."
                ));
            }
        }
    }
}

impl From<i32> for VarInt {
    fn from(value: i32) -> Self {
        VarInt(value)
    }
}

impl From<u32> for VarInt {
    fn from(value: u32) -> Self {
        VarInt(value as i32)
    }
}

impl From<usize> for VarInt {
    fn from(value: usize) -> Self {
        VarInt(value as i32)
    }
}

impl From<isize> for VarInt {
    fn from(value: isize) -> Self {
        VarInt(value as i32)
    }
}

impl From<VarInt> for i32 {
    fn from(value: VarInt) -> Self {
        value.0
    }
}

impl From<VarInt> for u32 {
    fn from(value: VarInt) -> Self {
        value.0 as u32
    }
}

impl From<VarInt> for usize {
    fn from(value: VarInt) -> Self {
        value.0 as usize
    }
}

impl From<VarInt> for isize {
    fn from(value: VarInt) -> Self {
        value.0 as isize
    }
}
