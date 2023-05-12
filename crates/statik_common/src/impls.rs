use std::io::{Read, Write};

use anyhow::{ensure, Context, Result};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

use crate::prelude::*;

// == Primitive impls == \\

// = Encode impls = \\

impl Encode for bool {
    fn encode(&self, buffer: &mut dyn Write) -> Result<()> {
        Ok(buffer.write_u8(*self as u8)?)
    }
}

// unsigned ints \\

impl Encode for u8 {
    fn encode(&self, buffer: &mut dyn Write) -> Result<()> {
        Ok(buffer.write_u8(*self)?)
    }
}

impl Encode for u16 {
    fn encode(&self, buffer: &mut dyn Write) -> Result<()> {
        Ok(buffer.write_u16::<BigEndian>(*self)?)
    }
}

impl Encode for u32 {
    fn encode(&self, buffer: &mut dyn Write) -> Result<()> {
        Ok(buffer.write_u32::<BigEndian>(*self)?)
    }
}

impl Encode for u64 {
    fn encode(&self, buffer: &mut dyn Write) -> Result<()> {
        Ok(buffer.write_u64::<BigEndian>(*self)?)
    }
}

impl Encode for u128 {
    fn encode(&self, buffer: &mut dyn Write) -> Result<()> {
        Ok(buffer.write_u128::<BigEndian>(*self)?)
    }
}

// signed ints \\

impl Encode for i8 {
    fn encode(&self, buffer: &mut dyn Write) -> Result<()> {
        Ok(buffer.write_i8(*self)?)
    }
}

impl Encode for i16 {
    fn encode(&self, buffer: &mut dyn Write) -> Result<()> {
        Ok(buffer.write_i16::<BigEndian>(*self)?)
    }
}

impl Encode for i32 {
    fn encode(&self, buffer: &mut dyn Write) -> Result<()> {
        Ok(buffer.write_i32::<BigEndian>(*self)?)
    }
}

impl Encode for i64 {
    fn encode(&self, buffer: &mut dyn Write) -> Result<()> {
        Ok(buffer.write_i64::<BigEndian>(*self)?)
    }
}

impl Encode for i128 {
    fn encode(&self, buffer: &mut dyn Write) -> Result<()> {
        Ok(buffer.write_i128::<BigEndian>(*self)?)
    }
}

// = Decode impls = \\

impl Decode for bool {
    fn decode(buffer: &mut dyn Read) -> Result<Self> {
        let n = buffer.read_u8()?;
        ensure!(n <= 1, "decoded boolean is not 0 or 1 (got {n})");
        Ok(n == 1)
    }
}

// unsigned ints \\

impl Decode for u8 {
    fn decode(buffer: &mut dyn Read) -> Result<Self> {
        Ok(buffer.read_u8()?)
    }
}

impl Decode for u16 {
    fn decode(buffer: &mut dyn Read) -> Result<Self> {
        Ok(buffer.read_u16::<BigEndian>()?)
    }
}

impl Decode for u32 {
    fn decode(buffer: &mut dyn Read) -> Result<Self> {
        Ok(buffer.read_u32::<BigEndian>()?)
    }
}

impl Decode for u64 {
    fn decode(buffer: &mut dyn Read) -> Result<Self> {
        Ok(buffer.read_u64::<BigEndian>()?)
    }
}

impl Decode for u128 {
    fn decode(buffer: &mut dyn Read) -> Result<Self> {
        Ok(buffer.read_u128::<BigEndian>()?)
    }
}

// signed ints \\

impl Decode for i8 {
    fn decode(buffer: &mut dyn Read) -> Result<Self> {
        Ok(buffer.read_i8()?)
    }
}

impl Decode for i16 {
    fn decode(buffer: &mut dyn Read) -> Result<Self> {
        Ok(buffer.read_i16::<BigEndian>()?)
    }
}

impl Decode for i32 {
    fn decode(buffer: &mut dyn Read) -> Result<Self> {
        Ok(buffer.read_i32::<BigEndian>()?)
    }
}

impl Decode for i64 {
    fn decode(buffer: &mut dyn Read) -> Result<Self> {
        Ok(buffer.read_i64::<BigEndian>()?)
    }
}

impl Decode for i128 {
    fn decode(buffer: &mut dyn Read) -> Result<Self> {
        Ok(buffer.read_i128::<BigEndian>()?)
    }
}

// = String = //

impl Encode for String {
    fn encode(&self, buffer: &mut dyn Write) -> Result<()> {
        let len = self.len();
        ensure!(
            len <= i32::MAX as usize,
            "byte length of string ({len}) exceeds i32::MAX"
        );

        VarInt(self.len() as i32).encode(buffer)?;
        Ok(buffer.write_all(self.as_bytes())?)
    }
}

impl Decode for String {
    fn decode(buffer: &mut dyn Read) -> Result<Self> {
        let len = VarInt::decode(buffer)?.0;

        ensure!(len >= 0, "attempt to decode struct with negative length");
        let len = len as usize;

        let mut buf: Vec<u8> = Vec::new();

        for _ in 0..len {
            buf.push(buffer.read_u8().context("not enough data remaining to decode string: buffer length must be {len}, accordign to the starting VarInt.")?);
        }
        // ensure!(buffer.count(); >= len, "not enough data remaining to decode string");
        // buffer.read(&mut buf)?;

        Ok(std::string::String::from_utf8(buf)?)
    }
}

// impl Encode for String {
//     fn encode(&self, buffer: &mut dyn Write) -> Result<()> {
//         self.as_str().encode(buffer)
//     }
// }

// impl Decode for String {
//     fn decode(buffer: &mut dyn Read) -> Result<Self> {
//         Ok(<&str>::decode(buffer)?.into())
//     }
// }
