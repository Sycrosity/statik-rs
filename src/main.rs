#![allow(unused)]

use std::{io::Cursor, sync::Arc};

use bytes::BytesMut;
use config::ServerConfig;
use statik_proto::s2c::status::{
    response::{Players, S2CStatusResponse, StatusResponse, Version},
    S2CStatusPacket,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufWriter},
    net::{TcpListener, TcpStream, ToSocketAddrs},
    select,
    signal::{self, unix::SignalKind},
    sync::{broadcast, mpsc, RwLock},
};

use anyhow::anyhow;

use log::{debug, error, info, log, trace, warn};

mod config;
mod connection;
mod shutdown;

use crate::{connection::Handler, shutdown::Shutdown};

use statik_common::prelude::*;

struct Server {
    /// Configuration for how the server should be run.
    config: Arc<RwLock<ServerConfig>>,

    /// Minecraft TCP listener that the server will bind and accept minecraft
    /// client connections from. Set by the `config.host` and `config.port`
    /// fields.
    mc_listener: TcpListener,

    /// API TCP listener that the server will bind and accept api connections
    /// from. Set by the `config.host` and `config.api_port` fields.
    api_listener: TcpListener,

    /// Able to broadcast a shutdown signal to all active connections.
    ///
    /// The initial `notify_shutdown` trigger is provided by the `run` server
    /// function: the server is then responsible for gracefully shutting down active
    /// connections. When a connection task is spawned, it is passed a handle to
    /// the broadcast receiver. When a graceful shutdown is initiated, a `()` value
    /// is sent via the broadcast::Sender. Each active connection receives it,
    /// reaches a safe termination state, and completes the task.
    notify_shutdown: broadcast::Sender<()>,

    /// Used as part of the graceful shutdown process to wait for client
    /// connections to complete processing.
    ///
    /// Tokio channels are closed once all `Sender` handles go out of scope.
    /// When a channel is closed, the receiver receives `None`. This is
    /// leveraged to detect all connection handlers completing. When a
    /// connection handler is initialized, it is assigned a clone of
    /// `shutdown_complete_tx`. When the listener shuts down, it drops the
    /// sender held by this `shutdown_complete_tx` field. Once all handler tasks
    /// complete, all clones of the `Sender` are also dropped. This results in
    /// `shutdown_complete_rx.recv()` completing with `None`. At this point, it
    /// is safe to exit the server process.
    shutdown_complete_tx: mpsc::Sender<()>,
}

impl Server {
    async fn new(
        config: ServerConfig,
        notify_shutdown: broadcast::Sender<()>,
        shutdown_complete_tx: mpsc::Sender<()>,
    ) -> anyhow::Result<Self> {
        let mc_address = format!("{}:{}", config.host, config.port);
        let api_address = format!("{}:{}", config.host, config.api_port);

        let mc_listener = TcpListener::bind(mc_address).await?;
        let api_listener = TcpListener::bind(api_address).await?;

        let icon = match &config.icon {
            Some(s) => Some(tokio::fs::read(s).await?),
            None => None,
        };

        let config = Arc::new(RwLock::new(config));

        Ok(Self {
            config,
            mc_listener,
            api_listener,
            notify_shutdown,
            shutdown_complete_tx,
        })
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        loop {
            select! {

                res = self.mc_listener.accept() => {
                    match res {
                        Ok((stream, address)) => {
                            debug!("New mc connection from {}", address);

                            let shutdown_complete_tx = self.shutdown_complete_tx.clone();
                            let shutdown = Shutdown::new(self.notify_shutdown.subscribe());

                            let config = self.config.clone();

                            tokio::spawn(async move {

                                //handler
                                Handler::new(config, stream, shutdown, shutdown_complete_tx).await.run().await;

                            });
                        },
                        Err(err) => error!("Failed to accept mc connection: {:#}", anyhow!(err)),
                    }
                }
                res = self.api_listener.accept() => {
                    match res {
                        Ok((stream, address)) => {
                            debug!("New api connection from {}", address);

                            let shutdown_complete_tx = self.shutdown_complete_tx.clone();
                            let shutdown = Shutdown::new(self.notify_shutdown.subscribe());

                            let config = self.config.clone();

                            todo!()

                            // tokio::spawn(async move {

                            //     //handler
                            //     Handler::new(config, stream, shutdown, shutdown_complete_tx).await.run().await;

                            // });
                        },
                        Err(err) => error!("Failed to accept api connection: {:#}", anyhow!(err)),
                    }
                }
            }
        }

        Ok(())
    }

    async fn shutdown(&self) -> anyhow::Result<()> {
        info!("Shutting down the server...");

        self.notify_shutdown.send(())?;

        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = ServerConfig {
        host: String::from("127.0.0.1"),
        max_players: 64,
        ..Default::default()
    };

    // When the provided `shutdown` future completes, we must send a shutdown
    // message to all active connections. We use a broadcast channel for this
    // purpose. The call below ignores the receiver of the broadcast pair, and when
    // a receiver is needed, the subscribe() method on the sender is used to create
    // one.
    let (notify_shutdown, mut _shutdown_rx) = broadcast::channel(1);
    let (shutdown_complete_tx, mut _shutdown_complete_rx) = mpsc::channel(1);

    info!(
        "Statik server is starting, and will broadcast on {}:{}",
        &config.host, &config.port
    );

    let mut server = Server::new(config, notify_shutdown, shutdown_complete_tx).await?;

    loop {
        select! {

            res = server.run() => {

                // If an error is received here, accepting connections from the TCP
                // listener failed multiple times and the server is giving up and
                // shutting down.
                //
                // Errors encountered when handling individual connections do not
                // bubble up to this point.
                if let Err(err) = res {
                    error!("Failed to accept connection: {:#}", anyhow!(err));
                }
            }

            _ = signal::ctrl_c() => {

                debug!("SIGINT (ctrl_c) OS signal recieved.");
                server.shutdown().await?;
                break;

            }
            _ = shutdown::sigquit() => {

                debug!("SIGQUIT (quit) OS signal recieved.");
                server.shutdown().await?;
                break;
            },
            _ = shutdown::sigterm() => {

                debug!("SIGTERM (terminate) OS signal recieved.");
                server.shutdown().await?;
                break;
            },
            //shutdowns sent from the server itself (e.g. not an external OS signal)
            _ = _shutdown_rx.recv() => {

                debug!("internal shutdown signal recieved.");
                server.shutdown().await;
                break;
            }
        }
    }

    let Server {
        shutdown_complete_tx,
        notify_shutdown,
        ..
    } = server;

    // When `shutdown` is dropped, all tasks which have `subscribe`d will
    // receive the shutdown signal and can exit
    drop(notify_shutdown);

    // Drop final `Sender` so the `Receiver` below can complete
    drop(shutdown_complete_tx);

    // Wait for all active connections to finish processing. As the `Sender`
    // handle held by the listener has been dropped above, the only remaining
    // `Sender` instances are held by connection handler tasks. When those drop,
    // the `mpsc` channel will close and `recv()` will return `None`.
    let _ = _shutdown_complete_rx.recv().await;
    let _ = _shutdown_rx.recv().await;

    info!("The server has been shut down.");

    Ok(())
}

// #[tokio::main]
// async fn main() -> anyhow::Result<()> {
//     tracing_subscriber::fmt()
//         .with_max_level(tracing::Level::DEBUG)
//         .init();

//     let addr = "127.0.0.1:25565";

//     //change to accept cli commands
//     let listener = TcpListener::bind(addr).await.unwrap();

//     let (shutdown_send, mut shutdown_recv) = mpsc::unbounded_channel::<()>();
//     let (done_send, mut done_recv) = mpsc::channel::<()>(1);

//     info!("statik server is now up, broadcasting on {addr}");

//     // S2CPacket::Status(S2CStatusPacket::StatusResponse(S2CStatusResponse { json_response:
//     let b = StatusResponse::new(
//         Version::new(MINECRAFT_VERSION, PROTOCOL_VERSION),
//         Players::new(69, 0, vec![]),
//         Chat::new("The HermitCrab Server!"),
//         None,
//         false,
//     );
//     // }}));

//     // shutdown_send.send(());

//     let c = serde_json::to_string_pretty(&b).unwrap();

//     println!("{c}");

//     loop {
//         select! {
//             res = listener.accept() => {
//                 match res {
//                     Ok((stream, address)) => {
//                         debug!("new connection from {}", address);

//                         let done = done_send.clone();
//                         let shutdown = shutdown_send.clone();

//                         tokio::spawn(async move {

//                             process_connection(stream,shutdown, done).await;

//                         });
//                     },
//                     Err(err) => error!("failed to accept connection: {:#}", anyhow!(err)),
//                 }
//             }
//             _ = signal::ctrl_c() => { debug!("ctrl_c signal recieved."); info!("shutting down the server..."); shutdown_send.send(()); break; },
//             _ = shutdown_recv.recv() => { debug!("shutdown signal recieved."); info!("shutting down the server...");  break; },
//         }
//     }

//     drop(shutdown_send);
//     drop(done_send);

//     let _ = done_recv.recv().await;

//     info!("the server has been shut down successfully.");

//     Ok(())
// }

// async fn process(mut socket: TcpStream) {
//     let mut stream = BufWriter::new(socket);

//     tokio::spawn(async move {
//         // In a loop, read data from the socket and write the data back.
//         loop {
//             let mut buf = BytesMut::new();

//             // select! {

//             // }

//             let n = match stream.read(&mut buf).await {
//                 // socket closed
//                 Ok(n) if n == 0 => return,
//                 Ok(n) => n,
//                 Err(e) => {
//                     eprintln!("failed to read from socket; err = {e:#}");
//                     return;
//                 }
//             };

//             println!("{n:#b}");

//             //Write the data back
//             if let Err(e) = stream.write_all(&buf[0..n]).await {
//                 eprintln!("failed to write to socket; err = {:?}", e);
//                 return;
//             }
//         }
//     });
// }
