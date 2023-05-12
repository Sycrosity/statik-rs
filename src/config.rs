#[derive(Debug)]
pub struct ServerConfig {
    /// Defaults to "0.0.0.0" which accepts all incoming connections.
    pub host: String,
    /// The port minecraft clients connect to. Defaults to 25565.
    pub port: u16,

    /// The port api connections use. Defaults to 8080.
    pub api_port: usize,

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
    /// Note: must be a 64x64 pixel image, or the minecraft client will not be able
    /// to parse it.
    pub icon: Option<String>,

    /// Whether this server appears online or not. Defaults to false.
    pub hidden: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            max_packet_size: 4096,
            host: String::from("0.0.0.0"),
            port: 25565,
            api_port: 8080,
            max_players: 20,
            hide_player_count: false,
            motd: String::from("A Statik server!"),
            icon: None,
            hidden: false,
        }
    }
}
