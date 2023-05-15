
// use std::io::{Write, Read};

// use anyhow::Result;

// use crate::prelude::*;

// #[derive(Debug)]
// pub struct VarIntPrefixedVec<T>(pub Vec<T>);

// impl<T: Encode + Decode + Sized> Decode for VarIntPrefixedVec<T> {
//     fn decode(mut buffer: impl Read) -> Result<Self> {

//         let length = VarInt::decode(&mut buffer)?;
//         let mut vec = Vec::with_capacity(length.into());

//         for _ in 0..length.0 {
//             vec.push(T::decode(&mut buffer)?);
//         }

//         Ok(Self(vec))
//     }
// }

// impl<T: Encode + Decode + Sized> Encode for VarIntPrefixedVec<T> {

//     fn encode(&self, mut buffer: impl Write) -> Result<()> {
//         VarInt::from(self.0.len()).encode(&mut buffer)?;
//         for element in &self.0 {
//             element.encode(&mut buffer)?;
//         }

//         Ok(())
//     }

// }