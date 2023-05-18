use std::borrow::Cow;
use std::io::{Read, Write};

use crate::prelude::*;

#[derive(Debug, Default)]
pub struct RawBytes(pub Cow<'static, [u8]>);

impl RawBytes {
    pub fn new<S: Into<Cow<'static, [u8]>>>(data: S) -> RawBytes {
        RawBytes(data.into())
    }
}

impl Encode for RawBytes {
    fn encode(&self, mut buffer: impl Write) -> anyhow::Result<()> {
        Ok(buffer.write_all(&self.0)?)
    }
}

impl Decode for RawBytes {
    fn decode(mut buffer: impl Read) -> anyhow::Result<Self> {
        let mut vec = Vec::new();
        buffer.read_to_end(&mut vec)?;
        Ok(Self::new(vec))
    }
}
