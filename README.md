`statik-rs`
==================

statik ~~is~~ (hopefully will become) a lightweight, containerisable minecraft server with the sole purpose of making a server appear online while it is not, sending a single to start the actual server. This allows a typically high mem and cpu intensive java minecraft server be closed when no-one is playing, yet still have a user be able to view a server as being online.

Statik has a MSRV (Minimum Supported Rust Version) of `1.56.1`.

-------

## Usage

To run statik, you must have rust installed via [rustup](https://rustup.rs).

To run the server on default port 25565, run this command:

```bash
$ cargo run --release
```

While for development just run:

```bash
$ cargo run
```
(This will compile the projects dependencies with release mode, but have the actual binary compiled with the default dev profile and debug assertions)


(UNIMPLIMENTED FEATURE)
Or change one of the various settings (also availible through the `statik.toml` file):
```bash
$ cargo run --release -- -p 25566 -m "The server is turning on: please wait ~30 seconds!"
```

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