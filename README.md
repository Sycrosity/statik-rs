`statik-rs`
==================

statik ~~is~~ (hopefully will become) a lightweight, containerisable minecraft server with the sole purpose of making a server appear online while it is not, sending a signal to start the actual server. This allows the typically high mem and cpu intensive java minecraft server be closed when no-one is playing, yet still have clients see the server as being online.

Statik's MSRV (Minimum Supported Rust Version) is whatever the most recent version of Rust is, as this project may contain recent rust features that don't exist in previous versions.

-------

## Usage

To run statik, you must have rust installed via [rustup](https://rustup.rs).

To run the server on default port 25565, run this command:

```bash
$ cargo run --release
```

While in development just run:

```bash
$ cargo run
```
(This will compile the projects dependencies with release mode, but have the actual binary compiled with the default dev profile and debug assertions)

various settings can be configured through the `statik.toml` file - this can be provided in whatever directory the run command was executed in, or will be automatically generated if it doesn't yet exist.

Or change the path of the config file:
```bash
$ cargo run -- --config=path/to/my_config_file.toml
```

And when everything is finalised, run the server in release mode:
```bash
$ cargo run --release
```

You can also install the server binary from <https://crates.io/crates/statik>:

```bash
$ cargo install statik --locked
```
Note: this may not be up to date, and installing from this repo's main branch may be better:
```bash
$ cargo install --git https://github.com/Sycrosity/statik-rs --locked
```

-------

## Contributing

Any and all contributions are welcome! Pull requests are checked for `cargo test`, `cargo clippy` and `cargo +nightly fmt`. Note this project uses unstable cargo fmt settings, and requires installing and running cargo fmt on the nighlty edition.


-------

## Credits

This project takes **heavy** inspiration and couldn't be created without the hard work put into the following projects:
* [limbo](https://github.com/chrrs/limbo) - the main inspiration for statik, designed for a very similar use case but left undeveloped since May 2022
* [bamboo](https://gitlab.com/macmv/bamboo) - an attempt at re-writing the minecraft server java code from scratch in rust
* [feather](https://github.com/feather-rs/feather) - another minecraft server re-write that hasn't had a commit since June 2022
* tokio's [mini-redis](https://github.com/tokio-rs/mini-redis) tutorial - a well implimented example tokio server that accepts and processes TCP connections, helping me layout and build this server.

-------

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
