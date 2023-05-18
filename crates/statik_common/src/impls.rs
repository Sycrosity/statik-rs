use std::borrow::Cow;
use std::io::{Read, Write};

use anyhow::{ensure, Context, Result};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use uuid::Uuid;

use crate::prelude::*;

// == Primitive impls == \\

// = Encode impls = \\

impl Encode for bool {
    fn encode(&self, mut buffer: impl Write) -> Result<()> {
        Ok(buffer.write_u8(*self as u8)?)
    }

    // fn encode_slice(slice: &[bool], mut buffer: impl Write) -> Result<()> {
    //     // SAFETY: Bools have the same layout as u8.
    //     // Bools are guaranteed to have the correct bit pattern.
    //     let bytes: &[u8] = unsafe { mem::transmute(slice) };
    //     Ok(buffer.write_all(bytes)?)
    // }
}

// unsigned ints \\

impl Encode for u8 {
    fn encode(&self, mut buffer: impl Write) -> Result<()> {
        Ok(buffer.write_u8(*self)?)
    }
}

impl Encode for u16 {
    fn encode(&self, mut buffer: impl Write) -> Result<()> {
        Ok(buffer.write_u16::<BigEndian>(*self)?)
    }
}

impl Encode for u32 {
    fn encode(&self, mut buffer: impl Write) -> Result<()> {
        Ok(buffer.write_u32::<BigEndian>(*self)?)
    }
}

impl Encode for u64 {
    fn encode(&self, mut buffer: impl Write) -> Result<()> {
        Ok(buffer.write_u64::<BigEndian>(*self)?)
    }
}

impl Encode for u128 {
    fn encode(&self, mut buffer: impl Write) -> Result<()> {
        Ok(buffer.write_u128::<BigEndian>(*self)?)
    }
}

// signed ints \\

impl Encode for i8 {
    fn encode(&self, mut buffer: impl Write) -> Result<()> {
        Ok(buffer.write_i8(*self)?)
    }
}

impl Encode for i16 {
    fn encode(&self, mut buffer: impl Write) -> Result<()> {
        Ok(buffer.write_i16::<BigEndian>(*self)?)
    }
}

impl Encode for i32 {
    fn encode(&self, mut buffer: impl Write) -> Result<()> {
        Ok(buffer.write_i32::<BigEndian>(*self)?)
    }
}

impl Encode for i64 {
    fn encode(&self, mut buffer: impl Write) -> Result<()> {
        Ok(buffer.write_i64::<BigEndian>(*self)?)
    }
}

impl Encode for i128 {
    fn encode(&self, mut buffer: impl Write) -> Result<()> {
        Ok(buffer.write_i128::<BigEndian>(*self)?)
    }
}

// = Decode impls = \\

impl Decode for bool {
    fn decode(mut buffer: impl Read) -> Result<Self> {
        let n = buffer.read_u8()?;
        ensure!(n <= 1, "decoded boolean is not 0 or 1 (got {n})");
        Ok(n == 1)
    }
}

// unsigned ints \\

impl Decode for u8 {
    fn decode(mut buffer: impl Read) -> Result<Self> {
        Ok(buffer.read_u8()?)
    }
}

impl Decode for u16 {
    fn decode(mut buffer: impl Read) -> Result<Self> {
        Ok(buffer.read_u16::<BigEndian>()?)
    }
}

impl Decode for u32 {
    fn decode(mut buffer: impl Read) -> Result<Self> {
        Ok(buffer.read_u32::<BigEndian>()?)
    }
}

impl Decode for u64 {
    fn decode(mut buffer: impl Read) -> Result<Self> {
        Ok(buffer.read_u64::<BigEndian>()?)
    }
}

impl Decode for u128 {
    fn decode(mut buffer: impl Read) -> Result<Self> {
        Ok(buffer.read_u128::<BigEndian>()?)
    }
}

// signed ints \\

impl Decode for i8 {
    fn decode(mut buffer: impl Read) -> Result<Self> {
        Ok(buffer.read_i8()?)
    }
}

impl Decode for i16 {
    fn decode(mut buffer: impl Read) -> Result<Self> {
        Ok(buffer.read_i16::<BigEndian>()?)
    }
}

impl Decode for i32 {
    fn decode(mut buffer: impl Read) -> Result<Self> {
        Ok(buffer.read_i32::<BigEndian>()?)
    }
}

impl Decode for i64 {
    fn decode(mut buffer: impl Read) -> Result<Self> {
        Ok(buffer.read_i64::<BigEndian>()?)
    }
}

impl Decode for i128 {
    fn decode(mut buffer: impl Read) -> Result<Self> {
        Ok(buffer.read_i128::<BigEndian>()?)
    }
}

// = String = //

impl Encode for String {
    fn encode(&self, mut buffer: impl Write) -> Result<()> {
        let length = self.len();
        ensure!(
            length <= i32::MAX as usize,
            "byte length of string ({length}) exceeds i32::MAX"
        );

        VarInt::from(length).encode(&mut buffer)?;
        Ok(buffer.write_all(self.as_bytes())?)
    }
}

impl Decode for String {
    fn decode(mut buffer: impl Read) -> Result<Self> {
        let length = VarInt::decode(&mut buffer)?.0;

        ensure!(length >= 0, "attempt to decode struct with negative length");

        let mut buf = Vec::with_capacity(length as usize);

        for _ in 0..length {
            buf.push(buffer.read_u8().context(
                "Not enough data remaining to decode string: buffer length must be {length}, \
                 according to the starting VarInt",
            )?);
        }

        Ok(std::string::String::from_utf8(buf)?)
    }
}

// = Vec's = \\

impl<T: Encode> Encode for Vec<T> {
    fn encode(&self, mut buffer: impl Write) -> Result<()> {
        let length = self.len();
        ensure!(
            length <= i32::MAX as usize,
            "byte length of Vec ({length}) exceeds i32::MAX"
        );

        VarInt::from(length).encode(&mut buffer)?;
        for element in self {
            element.encode(&mut buffer)?;
        }

        Ok(())
    }
}

impl<T: Decode> Decode for Vec<T> {
    fn decode(mut buffer: impl Read) -> Result<Self> {
        let length = VarInt::decode(&mut buffer)?.0;

        ensure!(
            length >= 0,
            "Attempted to decode struct with negative length"
        );

        // Don't allocate more memory than what would roughly fit in a single packet in
        // case we get a malicious array length.
        let cap = (MAX_PACKET_SIZE as usize / std::mem::size_of::<T>().max(1)).min(length as usize);
        let mut vec = Self::with_capacity(cap);

        for _ in 0..length {
            vec.push(T::decode(&mut buffer)?);
        }

        Ok(vec)
    }
}

// = Option's = \\

impl<T: Encode> Encode for Option<T> {
    fn encode(&self, mut buffer: impl Write) -> Result<()> {
        match self {
            Some(t) => {
                true.encode(&mut buffer)?;
                t.encode(buffer)
            }
            None => false.encode(&mut buffer),
        }
    }
}

impl<T: Decode> Decode for Option<T> {
    fn decode(mut buffer: impl Read) -> Result<Self> {
        Ok(match bool::decode(&mut buffer)? {
            true => Some(T::decode(&mut buffer)?),
            false => None,
        })
    }
}

// = Cow (copy on write) = \\

impl<'a, B> Encode for Cow<'a, B>
where
    B: ToOwned + Encode + ?Sized,
{
    fn encode(&self, buffer: impl Write) -> Result<()> {
        self.as_ref().encode(buffer)
    }
}

impl<'a, B> Decode for Cow<'a, B>
where
    B: ToOwned + ?Sized,
    B::Owned: Decode,
{
    fn decode(buffer: impl Read) -> Result<Self> {
        B::Owned::decode(buffer).map(Cow::Owned)
    }
}

impl Encode for Uuid {
    fn encode(&self, buffer: impl Write) -> Result<()> {
        self.as_u128().encode(buffer)
    }
}

impl Decode for Uuid {
    fn decode(buffer: impl Read) -> Result<Self> {
        u128::decode(buffer).map(Uuid::from_u128)
    }
}

// // == Sequences == \\

// /// Like tuples, fixed-length arrays are encoded and decoded without a VarInt
// /// length prefix.
// impl<const N: usize, T: Encode> Encode for [T; N] {
//     fn encode(&self, buffer: impl Write) -> Result<()> {
//         T::encode_slice(self, buffer)
//     }
// }

// impl<const N: usize, T: Decode> Decode for [T; N] {
//     fn decode(mut buffer: impl Read) -> Result<Self> {

//         // TODO: rewrite using std::array::try_from_fn when stabilized?
//         // TODO: specialization for [f64; 3] improved performance.
//         let mut data: [MaybeUninit<T>; N] = unsafe {
// MaybeUninit::uninit().assume_init() };

//         for (i, elem) in data.iter_mut().enumerate() {
//             match T::decode(&mut buffer) {
//                 Ok(val) => {
//                     elem.write(val);
//                 }
//                 Err(e) => {
//                     // Call destructors for values decoded so far.
//                     for elem in &mut data[..i] {
//                         unsafe { elem.assume_init_drop() };
//                     }
//                     return Err(e);
//                 }
//             }
//         }

//         // All values in `data` are initialized.
//         unsafe { Ok(mem::transmute_copy(&data)) }
//     }
// }

// /// References to fixed-length arrays are not length prefixed.
// impl<const N: usize> Decode for &[u8; N] {
//     fn decode(mut buffer: impl Read) -> Result<Self> {
//         let length = buffer.len();
//         ensure!(
//             length >= N,
//             "not enough data to decode u8 array of length {N}"
//         );

//         let mut buf: Vec<u8> = vec![0; length];

//         buffer.read_exact(&mut buf)?;

//         // let (res, remaining) = buffer.split_at(N);
//         // let arr = <&[u8; N]>::try_from(res).unwrap();
//         // *r = remaining;

//         Ok(<&[u8; N]>::try_from(buf.as_ref()).unwrap())
//     }
// }

// impl<T: Encode> Encode for [T] {
//     fn encode(&self, mut buffer: impl Write) -> Result<()> {
//         let length = self.len();
//         ensure!(
//             length <= i32::MAX as usize,
//             "length of slice ({length}) exceeds i32::MAX"
//         );

//         VarInt::from(length).encode(&mut buffer)?;

//         T::encode_slice(self, buffer)
//     }
// }
