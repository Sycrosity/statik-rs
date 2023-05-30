use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]

pub struct ServerConfig {
    pub general: GeneralServerConfig,
    pub mc: McServerConfig,
    pub api: ApiServerConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralServerConfig {
    /// Defaults to "0.0.0.0" which accepts all incoming connections.
    pub host: String,
    pub log_level: String,
}

impl Default for GeneralServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            log_level: "debug".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct McServerConfig {
    /// The port minecraft clients connect to. Defaults to 25565.
    pub port: u16,

    /// How many players can join the server at once. Defaults to 20.
    ///
    /// Note: negative values are valid for setting the max number of
    /// players on a minecraft server.
    pub max_players: i32,

    /// Hides the player count to a querying client. Defaults to false.
    ///
    /// Note: for continuity, this should probably match the setting of
    /// your actual minecraft server.
    pub hide_player_count: bool,

    /// The "Message of the Day" (text displayed when a client checks
    /// the status of a server). Can be formatted with the § symbol
    /// as defined on the [Minecraft wiki page](https://minecraft.fandom.com/wiki/Formatting_codes).
    ///
    /// Defaults to "A Statik server!"
    ///
    /// Note: for continuity, this should probably match the MOTD of
    /// your actual minecraft server.
    pub motd: String,

    /// The maximum size (in bytes) that a packet can be.
    /// Defaults to 4096.
    pub max_packet_size: usize,

    /// The URI (unique reference identifier) corresponding to the ~~website
    /// link or~~ local file containing the server icon.
    ///
    /// Note: must be a 64x64 pixel image, or the minecraft client will not
    /// be able to parse it, and the server will have a blank icon.
    pub icon: Option<String>,

    /// Whether this server appears online or not. Defaults to false.
    ///
    /// Note: this would pretty much make the statik server worthless!
    /// Make sure you are certain this is what you want to enable.
    pub hidden: bool,

    /// What message should be sent to the client by default when disconnecting
    /// them. Defaults to: "Disconnected from the server."
    ///
    /// Note: this can be overridden by disconnect specific packets, this is
    /// merely the default, no reason given fallback message.
    /// Note: can be templated using [Tera](https://tera.netlify.app/), a templating
    /// library inspired by Jinja2 and Django - read their [Documentation](https://tera.netlify.app/docs/)
    /// and [Examples](https://github.com/Keats/tera/tree/master/examples) for possible
    /// templates.
    pub disconnect_msg: String,
}

impl Default for McServerConfig {
    fn default() -> Self {
        Self {
            max_packet_size: 4096,
            port: 25565,
            max_players: 20,
            hide_player_count: false,
            motd: "A Statik server!".to_string(),
            icon: None,
            hidden: false,
            disconnect_msg: "Disconnected from the server.".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiServerConfig {
    /// The port api connections use. Defaults to 8080.
    pub port: usize,
}

impl Default for ApiServerConfig {
    fn default() -> Self {
        Self { port: 8080 }
    }
}
