use std::sync::Arc;

use anyhow::Result;
use tokio::{
    net::TcpStream,
    sync::{mpsc, RwLock},
};
use uuid::Uuid;

use crate::{config::ServerConfig, connection::Connection, player::Player, shutdown::Shutdown};

use statik_common::prelude::*;

use tera::{Context, Tera};

/// Per-connection handler. Reads packets sent from `connection` (a tcp stream from a
/// minecraft client) and sends responses accordingly.
#[derive(Debug)]
pub struct Handler {
    config: Arc<RwLock<ServerConfig>>,

    /// All the data accociated with the client after they have connected, including
    /// their username, UUID, (in the future) items, ect. Defaults to None, as this data
    /// isn't sent with a status request, only on login.
    player: Option<Player>,

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
    _shutdown_complete: mpsc::Sender<String>,
}

impl Handler {
    pub async fn new(
        config: Arc<RwLock<ServerConfig>>,
        connection: Connection,
        shutdown: Shutdown,
        _shutdown_complete: mpsc::Sender<String>,
    ) -> Self {
        let config_clone = config.clone();
        Self {
            config,
            player: None,
            connection,
            shutdown,
            _shutdown_complete,
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        // As long as the shutdown signal has not been received, try to read a
        // new packet.
        while !self.shutdown.is_shutdown() {
            // While reading a packet, also listen for the shutdown
            // signal - otherwise on a long job this could hang!
            let maybe_packet = tokio::select! {
                res = {tokio::time::sleep(std::time::Duration::from_secs(5)).await; self.connection.read_packet()} => res?,
                // If a shutdown signal is received, return from `run`.
                // This will result in the task terminating.
                reason = self.shutdown.recv() => {

                    let template = reason?;

                    let context = Context::from_serialize(&self.player.clone().unwrap_or_default())?;

                    let disconnect_msg = match Tera::one_off(&template, &context, false) {
                        Ok(s) => s,
                        Err(e) => {
                            warn!("Sending disconnect template as plain text. Could not parse Tera template: {e}");
                            template
                        }
                    };

                    debug!("Client connection from {} disconnected with reason: \"{disconnect_msg}\"", &self.connection.address);

                    //write disconnect packet using disconnect_msg:
                    // self.connection.write_packet().await?;

                    return Ok(());
                }
            };

            //If `None` is returned from `read_packet()` then the peer closed
            //the socket. There is no further work to do and the task can be
            //terminated.
            let packet = match maybe_packet {
                Some(packet) => packet,
                None => return Ok(()),
            };
            trace!("(↓) Packet recieved: {packet:?}");
        }

        Ok(())
    }
}