use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    /// A uniquely identifying UUID corresponding to the users minecraft
    /// account, which can be used to check against a whitelist/blacklist/
    /// banlist, find their username, download their currently active skin
    /// and more. Defaults to an empty (all zeros) Uuid. Set when (or if)
    /// the login sequence starts.
    pub uuid: Uuid,

    /// The username of an account as sent in the initial login packet.
    /// Defaults to "\<NULL\>". Set when (or if) login sequence starts.
    pub username: String,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            uuid: Default::default(),
            username: String::from("<NULL>"),
        }
    }
}

impl Player {
    pub fn new(uuid: Uuid, username: String) -> Self {
        Self { uuid, username }
    }
}
