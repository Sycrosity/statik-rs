use std::{
    io::{self, Cursor, ErrorKind},
    net::SocketAddr,
    sync::Arc,
    time::Duration,
};

use anyhow::bail;
use bytes::{BytesMut, Buf};
use statik_common::prelude::*;

use statik_proto::{
    c2s::{handshaking::C2SHandshakingPacket, status::C2SStatusPacket},
    s2c::status::{
        pong::S2CPong,
        response::{Players, S2CStatusResponse, StatusResponse},
    },
    state::State,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::TcpStream,
    sync::RwLock,
    time::timeout,
};

use crate::ServerConfig;

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
    pub stream: BufWriter<TcpStream>,

    /// The address that the connection comes from.
    pub address: SocketAddr,

    /// The buffer for reading frames.
    pub buffer: BytesMut,

    /// Buffer used for queuing and sending bytes.
    queue: Vec<u8>,

    ///staging
    staging: Vec<u8>,

    /// Current state of the handler: should go from 0 (Handshake) to 1 (status)
    /// or to 2 (login, which then goes to 3 (play))
    pub state: State,
}

impl Connection {
    /// Create a new `Connection`, backed by `socket`. Read and write buffers
    /// are initialized.
    pub async fn new(
        config: Arc<RwLock<ServerConfig>>,
        socket: TcpStream,
        address: SocketAddr,
    ) -> Self {
        let max_packet_size = config.read().await.max_packet_size;

        Self {
            config,
            stream: BufWriter::new(socket),
            address,
            buffer: BytesMut::with_capacity(max_packet_size),
            queue: Vec::with_capacity(max_packet_size - 1),
            staging: Vec::with_capacity(max_packet_size),
            state: State::Handshake,
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
    pub async fn handle_connection(&mut self) -> anyhow::Result<()> {
        loop {
            
            warn!("handling connection");


            let bytes_read = self.stream.read_buf(&mut self.buffer).await?;

            if bytes_read == 0 {
                return Err(io::Error::from(ErrorKind::UnexpectedEof).into());
            }

            trace!("Read {bytes_read} bytes from {}.", self.address);
            // println!("{:?}", &self.buffer);


            self.parse_packet().await?;
        }

        //timeout(Duration::from_secs(10), async {})

        // There is not enough buffered data to read a frame. Attempt to
        // read more data from the socket.
        //
        // On success, the number of bytes is returned. `0` indicates "end
        // of stream".
        // if 0 == self.stream.read_buf(&mut self.buffer).await? {
        //     // The remote closed the connection. For this to be a clean
        //     // shutdown, there should be no data in the read buffer. If
        //     // there is, this means that the peer closed the socket while
        //     // sending a frame.
        //     if self.buffer.is_empty() {
        //         return Ok(None);
        //     } else {
        //         bail!("connection reset by peer");
        //     }
        // }
        // }
    }

    /// Tries to parse a frame from the buffer. If the buffer contains enough
    /// data, the frame is returned and the data removed from the buffer. If not
    /// enough data has been buffered yet, `Ok(None)` is returned. If the
    /// buffered data does not represent a valid frame, `Err` is returned.
    pub async fn parse_packet(&mut self) -> anyhow::Result<()> {

        // Cursor is used to track the "current" location in the
        // buffer. Cursor also implements `Buf` from the `bytes` crate
        // which provides a number of helpful utilities for working
        // with bytes.

        // Ok(Some(C2SLegacyPing { payload: 0x01 }))
        // todo!()

        loop {

            let buffer_len = self.buffer.len();

            println!("{:?}", &self.buffer);

            let mut buf = Cursor::new(&self.buffer[..]);
    
            let packet_len = VarInt::decode(&mut buf)?.0 as usize;
    
            trace!("Packet should be {} bytes long", packet_len + 1);
    
            if buffer_len < packet_len {
                bail!(
                    "Packet wasn't long enough!
                Packet was {} bytes long while the packet stated it should be {} bytes long.",
                    self.buffer.len(),
                    packet_len + 1
                );
            }
    
            match self.state {
                State::Handshake => {
                    warn!("here3");
                    println!("h: {:?}", &buf);
                    self.handle_handshake(C2SHandshakingPacket::decode(&mut buf)?)
                        .await?
                }
                State::Status => {
                    println!("s: {:?}", &buf);
                    self.handle_status(C2SStatusPacket::decode(&mut buf)?).await?
                },
                State::Login => unimplemented!(),
                State::Play => unimplemented!(),
            }   
        }
        // Ok(())
    }

    pub async fn handle_handshake(&mut self, packet: C2SHandshakingPacket) -> anyhow::Result<()> {
        trace!("(↓) Packet recieved: {:?}", &packet);
        match packet {
            C2SHandshakingPacket::Handshake(handshake) => {
                if handshake.protocol_version.0 as usize != PROTOCOL_VERSION {
                    return Err(anyhow::anyhow!("Protocol versions do not match! Client had protocol version: {}, while the server's protocol version is {}.", handshake.protocol_version.0, PROTOCOL_VERSION));
                };

                let next_state = handshake.next_state;

                self.state = next_state;

                Ok(())
            } // C2SHandshakingPacket::LegacyPing(_legacy_ping) => {
              //     unimplemented!()
              // }
        }
    }

    pub async fn handle_status(&mut self, packet: C2SStatusPacket) -> anyhow::Result<()> {
        trace!("(↓) Packet recieved: {:?}", &packet);
        match packet {
            C2SStatusPacket::StatusRequest(_status_request) => {

                warn!("status_req");

                let config = self.config.read().await;

                let status_response = S2CStatusResponse {
                    json_response: StatusResponse::new(
                        Players::new(config.max_players, 0, vec![]),
                        Chat::new(config.motd.clone()),
                        None,
                        false,
                    ),
                };

                drop(config);

                self.write_packet(status_response).await?;

                Ok(())
            }
            C2SStatusPacket::Ping(ping) => {

                warn!("pong");
                
                let pong = S2CPong {
                    payload: ping.payload,
                };

                self.write_packet(pong).await?;

                Ok(())
            }
        }
    }

    ///jank as. fix.
    pub async fn write_packet(&mut self, packet: impl Packet) -> anyhow::Result<()> {
        // let start_len = self.queue.len();
        // let mut buf = Cursor::new(&mut self.queue[..]);

        packet.encode(&mut self.queue)?;

        // let mut buf = [0u8; 5];
        // let mut length_bytes = Cursor::new(&mut buf[..]);

        // error!("{}", self.queue.len());
        // error!("{:?}", &self.queue);
        let packet_len = self.queue.len();
        VarInt(packet_len as i32).encode(&mut self.staging)?;

        self.staging.extend_from_slice(&self.queue);


        trace!("Writing packet to tcp stream.");
        self.stream.write_all(&self.staging).await?;

        self.stream.flush().await?;

        self.queue.clear();

        trace!("(↑) Packet sent: {packet:?}");

        self.staging.clear();

        // let position = length_bytes.position() as usize;

        // self.staging.extend_from_slice(&buf[..position]);
        // self.staging.extend_from_slice(&self.queue);

        // // let data_len = self.queue.len() - start_len;

        // self.stream.write_all(&self.staging).await?;
        // self.stream.flush().await?;
        // self.queue.clear();

        // packet.encode(&mut self.buffer)

        Ok(())
    }
}
