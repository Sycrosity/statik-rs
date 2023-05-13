use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chat {
    text: Cow<'static, str>,
}

impl Chat {
    pub fn new<S: Into<Cow<'static, str>>>(text: S) -> Self {
        Self { text: text.into() }
    }
}

impl Encode for Chat {
    fn encode(&self, buffer: impl std::io::Write) -> anyhow::Result<()> {
        serde_json::to_string(self)?.encode(buffer)
    }
}

impl Decode for Chat {
    fn decode(buffer: impl std::io::Read) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(&String::decode(buffer)?)?)
    }
}
