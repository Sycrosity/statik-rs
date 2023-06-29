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