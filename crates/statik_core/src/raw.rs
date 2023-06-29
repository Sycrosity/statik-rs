use std::{
    borrow::Cow,
    io::{Read, Write},
};

use crate::prelude::*;

#[derive(Debug, Default)]
pub struct RawBytes(pub Cow<'static, [u8]>);

impl RawBytes {
    pub fn new<S: Into<Cow<'static, [u8]>>>(data: S) -> RawBytes {
        RawBytes(data.into())
    }
}
