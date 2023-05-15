use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use statik_common::prelude::*;
use uuid::Uuid;

use statik_derive::Packet;

#[derive(Debug, Packet)]
#[packet_id = 0x00]
pub struct S2CStatusResponse {
    ///See [Server List Ping#Response](https://wiki.vg/Server_List_Ping#Response); as with all strings this is prefixed by its length as a VarInt.
    pub json_response: StatusResponse,
}

/// # Examples
///
/// A sample status response in json:
/// ```json
/// {
/// "version": {
///     "name": "1.19.4",
///     "protocol": 761
/// },
/// "players": {
///     "max": 100,
///     "online": 5,
///     "sample": [
///         {
///             "name": "thinkofdeath",
///             "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
///         }
///     ]
/// },
/// "description": {
///     "text": "Hello world"
/// },
/// "favicon": "data:image/png;base64,<data>",
/// "enforcesSecureChat": true
/// }
/// ```
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusResponse {
    // {
    //     "version": {
    //         "name": "1.19.4",
    //         "protocol": 761
    //     },
    //     "players": {
    //         "max": 100,
    //         "online": 5,
    //         "sample": [
    //             {
    //                 "name": "thinkofdeath",
    //                 "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
    //             }
    //         ]
    //     },
    //     "description": {
    //         "text": "Hello world"
    //     },
    //     "favicon": "data:image/png;base64,<data>",
    //     "enforcesSecureChat": true
    //     }
    // }
    version: Version,

    players: Players,

    ///doesn't need to be the same level as [Players] or [Version], as [Chat] already has the `text` field as the description field has.
    description: Chat,

    ///byte slice of an image encoded as base64, prefixed by
    #[serde(skip_serializing_if = "Option::is_none")]
    favicon: Option<String>,
    enforces_secure_chat: bool,
}

impl std::fmt::Debug for StatusResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Formatter::debug_struct(f, "StatusResponse")
            .field("version", &self.version)
            .field("players", &self.players)
            .field("description", &self.description)
            .field(
                "favicon",
                match &self.favicon {
                    Some(_) => &"#HIDDEN: ICON AS BASE64 STRING#",
                    None => &None::<String>,
                },
            )
            .field("enforces_secure_chat", &self.enforces_secure_chat)
            .finish()
    }
}

impl StatusResponse {
    pub fn new(
        // version: Version,
        players: Players,
        description: Chat,
        favicon: Option<String>,
        enforces_secure_chat: bool,
    ) -> Self {
        Self {
            version: Version::default(),
            players,
            description,
            favicon: favicon.map(|data| format!("data:image/png;base64,{data}")),
            enforces_secure_chat,
        }
    }
}

impl Encode for StatusResponse {
    fn encode(&self, buffer: impl std::io::Write) -> anyhow::Result<()> {
        serde_json::to_string(self)?.encode(buffer)
    }
}

impl Decode for StatusResponse {
    fn decode(buffer: impl std::io::Read) -> anyhow::Result<Self> {
        Ok(serde_json::from_str(&String::decode(buffer)?)?)
    }
}

/// # Examples
///
/// Sample version field in json:
/// ```json
/// {
///     "name": "1.19.4",
///     "protocol": 761
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    name: Cow<'static, str>,
    protocol: usize,
}

impl Default for Version {
    fn default() -> Self {
        Self {
            name: Cow::Borrowed(MINECRAFT_VERSION),
            protocol: PROTOCOL_VERSION,
        }
    }
}

impl Version {
    pub fn new<S: Into<Cow<'static, str>>>(name: S, protocol: usize) -> Self {
        Self {
            name: name.into(),
            protocol,
        }
    }
}

/// # Examples
///
/// Sample player field in json:
/// ```json
/// {
///     "max": 100,
///     "online": 5,
///     "sample": [
///         {
///             "name": "thinkofdeath",
///             "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
///         }
///     ]
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct Players {
    max: i32,
    online: i32,
    sample: Vec<PlayerSample>,
}

impl Players {
    pub fn new(max: i32, online: i32, sample: Vec<PlayerSample>) -> Self {
        Self {
            max,
            online,
            sample,
        }
    }
}

/// # Examples
///
/// Sample player_sample field in json:
/// ```json
/// {
///     "name": "thinkofdeath",
///     "id": "4566e69f-c907-48ee-8d71-d7ba5aa00d20"
/// }
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerSample {
    name: Cow<'static, str>,
    id: Uuid,
}

impl PlayerSample {
    pub fn new<S: Into<Cow<'static, str>>>(name: S, id: Uuid) -> Self {
        Self {
            name: name.into(),
            id,
        }
    }
}
