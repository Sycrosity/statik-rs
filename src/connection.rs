use std::{io::Cursor, sync::Arc};

use bytes::BytesMut;
use statik_common::prelude::*;

use statik_proto::{
    c2s::handshaking::{handshake::C2SHandshake, legacy_ping::C2SLegacyPing},
    state::State,
};
use tokio::{
    io::{AsyncReadExt, BufWriter},
    net::TcpStream,
    select,
    sync::{
        broadcast,
        mpsc::{self, Sender, UnboundedSender},
        RwLock,
    },
};

use uuid::Uuid;

use crate::{shutdown::Shutdown, ServerConfig};

/// Checks if a username COULD be a valid minecraft account's username.
///
/// There is a few possible cases where this won't apply, like the handful
/// of single/double character accounts or the accounts with spaces in them,
/// but they are so rare (and not really applicable to this server's use case)
/// thtat it's not worth considering them here.
fn is_valid_username(username: &str) -> bool {
    (3..=16).contains(&username.len())
        && username
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
}

/// Per-connection handler. Reads packets sent from `connection` (a tcp stream from a
/// minecraft client) and sends responses accordingly.
#[derive(Debug)]
pub(crate) struct Handler {
    config: Arc<RwLock<ServerConfig>>,

    /// A uniquely identifying UUID corresponding to the users minecraft account,
    /// which can be used to check against a whitelist/blacklist/banlist, find
    /// their username, download their currently active skin and more. Defaults
    /// to None, until (or if) login sequence starts.
    uuid: Option<Uuid>,

    /// The username of an account as sent in the initial login packet. Defaults
    /// to None, until (or if) login sequence starts.
    username: Option<String>,

    /// Current state of the handler: should go from 0 (Handshake) to 1 (status)
    /// or to 2 (login, which then goes to 3 (play))
    state: State,

    /// The TCP connection implemented using a buffered `TcpStream` for parsing
    /// minecraft packets.
    ///
    /// When the [`Server`] receives an inbound connection, the `TcpStream` is
    /// passed to `Connection::new`, which initializes the associated buffers.
    /// `Connection` allows the handler to operate at the "frame" level and keep
    /// the byte level protocol parsing details encapsulated in `Connection`.
    connection: Connection,

    /// Listen for shutdown notifications.
    ///
    /// A wrapper around the `broadcast::Receiver` paired with the sender in
    /// [`Server`]. The connection handler processes requests from the
    /// connection until the peer disconnects **or** a shutdown notification is
    /// received from `shutdown`. In the latter case, any in-flight work being
    /// processed for the peer is continued until it reaches a safe state, at
    /// which point the connection is terminated.
    shutdown: Shutdown,

    /// Not used directly. Instead, when `Handler` is dropped, this cloned
    /// to the shutdown channel is also dropped - all clones must be dropped
    /// for the server to shutdown, and thus this is a good way of checking
    /// when all connections have finished/been terminated.
    _shutdown_complete: mpsc::Sender<()>,
}

impl Handler {
    pub async fn new(
        config: Arc<RwLock<ServerConfig>>,
        stream: TcpStream,
        shutdown: Shutdown,
        _shutdown_complete: mpsc::Sender<()>,
    ) -> Self {
        let config_clone = config.clone();
        Self {
            config,
            uuid: None,
            username: None,
            state: State::Handshake,
            connection: Connection::new(config_clone, stream).await,
            shutdown,
            _shutdown_complete,
        }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        // As long as the shutdown signal has not been received, try to read a
        // new request frame.

        while !self.shutdown.is_shutdown() {
            // While reading a request frame, also listen for the shutdown
            // signal - otherwise on a long job this could hang!
            // let maybe_packet = tokio::select! {
            //     res = self.connection.read_packet::<C2SLegacyPing>() => res?,
            //     _ = self.shutdown.recv() => {
            //         // If a shutdown signal is received, return from `run`.
            //         // This will result in the task terminating.
            //         return Ok(());
            //     }
            // };

            //If `None` is returned from `read_frame()` then the peer closed
            //the socket. There is no further work to do and the task can be
            //terminated.
            // let packet = match maybe_packet {
            //     Some(packet) => packet,
            //     None => return Ok(()),
            // };

            let packet = self.connection.read_packet().await?;

            todo!()
        }

        println!("hi");

        Ok(())
    }
}
/// Send and receive `Frame` values from a minecraft client.
///
/// When implementing networking protocols, a message on that protocol is
/// often composed of several smaller messages known as frames. The purpose of
/// `Connection` is to read and write frames on the underlying `TcpStream`.
///
/// To read frames, the `Connection` uses an internal buffer, which is filled
/// up until there are enough bytes to create a full frame. Once this happens,
/// the `Connection` creates the frame and returns it to the caller.
///
/// When sending frames, the frame is first encoded into the write buffer.
/// The contents of the write buffer are then written to the socket.
#[derive(Debug)]
pub struct Connection {
    config: Arc<RwLock<ServerConfig>>,

    // The `TcpStream`. It is decorated with a `BufWriter`, which provides write
    // level buffering. The `BufWriter` implementation provided by Tokio is
    // sufficient for our needs.
    stream: BufWriter<TcpStream>,

    // The buffer for reading frames.
    buffer: BytesMut,
}

impl Connection {
    /// Create a new `Connection`, backed by `socket`. Read and write buffers
    /// are initialized.
    pub async fn new(config: Arc<RwLock<ServerConfig>>, socket: TcpStream) -> Self {
        let max_packet_size = config.read().await.max_packet_size;

        Self {
            config,
            stream: BufWriter::new(socket),
            buffer: BytesMut::with_capacity(max_packet_size),
        }
    }

    /// Read a single `Packet` value from the underlying stream.
    ///
    /// The function waits until it has retrieved enough data to parse a packet.
    /// Any data remaining in the read buffer after the packet has been parsed is
    /// kept there for the next call to `read_packet`.
    ///
    /// # Returns
    ///
    /// On success, the received packet is returned. If the `TcpStream`
    /// is closed in a way that doesn't break a packet in half, it returns
    /// `None`. Otherwise, an error is returned.
    pub async fn read_packet(&mut self) -> anyhow::Result<Option<impl Packet>> {
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;

        self.parse_packet()

        // loop {
        //     // Attempt to parse a frame from the buffered data. If enough data
        //     // has been buffered, the frame is returned.
        //     if let Some(frame) = self.parse_frame()? {
        //         return Ok(Some(frame));
        //     }

        //     // There is not enough buffered data to read a frame. Attempt to
        //     // read more data from the socket.
        //     //
        //     // On success, the number of bytes is returned. `0` indicates "end
        //     // of stream".
        //     if 0 == self.stream.read_buf(&mut self.buffer).await? {
        //         // The remote closed the connection. For this to be a clean
        //         // shutdown, there should be no data in the read buffer. If
        //         // there is, this means that the peer closed the socket while
        //         // sending a frame.
        //         if self.buffer.is_empty() {
        //             return Ok(None);
        //         } else {
        //             return Err("connection reset by peer".into());
        //         }
        //     }
        // }
    }

    /// Tries to parse a frame from the buffer. If the buffer contains enough
    /// data, the frame is returned and the data removed from the buffer. If not
    /// enough data has been buffered yet, `Ok(None)` is returned. If the
    /// buffered data does not represent a valid frame, `Err` is returned.
    pub fn parse_packet(&mut self) -> anyhow::Result<Option<impl Packet>> {
        // Cursor is used to track the "current" location in the
        // buffer. Cursor also implements `Buf` from the `bytes` crate
        // which provides a number of helpful utilities for working
        // with bytes.

        Ok(Some(C2SLegacyPing { payload: 0x01 }))
        // todo!()
        // let mut buf = Cursor::new(&self.buffer[..]);
        // Ok(Some(C2SLegacyPing::decode(&mut buf)?))

        //     // The first step is to check if enough data has been buffered to parse
        //     // a single frame. This step is usually much faster than doing a full
        //     // parse of the frame, and allows us to skip allocating data structures
        //     // to hold the frame data unless we know the full frame has been
        //     // received.
        //     match Frame::check(&mut buf) {
        //         Ok(_) => {
        //             // The `check` function will have advanced the cursor until the
        //             // end of the frame. Since the cursor had position set to zero
        //             // before `Frame::check` was called, we obtain the length of the
        //             // frame by checking the cursor position.
        //             let len = buf.position() as usize;

        //             // Reset the position to zero before passing the cursor to
        //             // `Frame::parse`.
        //             buf.set_position(0);

        //             // Parse the frame from the buffer. This allocates the necessary
        //             // structures to represent the frame and returns the frame
        //             // value.
        //             //
        //             // If the encoded frame representation is invalid, an error is
        //             // returned. This should terminate the **current** connection
        //             // but should not impact any other connected client.
        //             let frame = Frame::parse(&mut buf)?;

        //             // Discard the parsed data from the read buffer.
        //             //
        //             // When `advance` is called on the read buffer, all of the data
        //             // up to `len` is discarded. The details of how this works is
        //             // left to `BytesMut`. This is often done by moving an internal
        //             // cursor, but it may be done by reallocating and copying data.
        //             self.buffer.advance(len);

        //             // Return the parsed frame to the caller.
        //             Ok(Some(frame))
        //         }
        //         // There is not enough data present in the read buffer to parse a
        //         // single frame. We must wait for more data to be received from the
        //         // socket. Reading from the socket will be done in the statement
        //         // after this `match`.
        //         //
        //         // We do not want to return `Err` from here as this "error" is an
        //         // expected runtime condition.
        //         Err(Incomplete) => Ok(None),
        //         // An error was encountered while parsing the frame. The connection
        //         // is now in an invalid state. Returning `Err` from here will result
        //         // in the connection being closed.
        //         Err(e) => Err(e.into()),
        //     }
    }
}

// pub async fn process_connection(
//     stream: TcpStream,
//     _shutdown: UnboundedSender<()>,
//     _done: Sender<()>,
// ) {
//     let writer = BufWriter::new(stream);

//     let buffer = BytesMut::new();

//     // loop {
//     //     select! {}
//     // }
// }

// async fn read_packet() -> Result<C2SPacket, DecodeError> {
//     todo!()
// }
