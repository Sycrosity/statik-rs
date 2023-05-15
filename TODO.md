* replace `tera` depedency with something smaller - it's the largest crate dependency according to `cargo bloat --crates` - make my own crate for this?
* add the api, accepting requests ( if authenticated ) to disconnect, change config, ect
* create a DockerFile, compiling it with github actions
* make config a shared state rather than awkwardly cloning its handle.
* (maybe) accept play packets for a very minimal lobby area where players can build
* allow downloading the icon from the internet, downscaling it
* send queries to the minecraft api to check uuid's and player names
* add a whitelist of players that can join
* add a player list that updates when players join, updating the player number dynamically
* see if it's possible to scale down the `toml` dependency, or replace it with a json/ron/other smaller file crates