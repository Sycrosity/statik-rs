use std::sync::Arc;

use statik_common::prelude::*;
use tokio::{
    net::TcpListener,
    select,
    sync::{broadcast, mpsc, RwLock},
};

use crate::{config::ServerConfig, connection::Connection, handler::Handler, shutdown::Shutdown};

pub struct Server {
    /// Configuration for how the server should be run.
    pub config: Arc<RwLock<ServerConfig>>,

    /// Minecraft TCP listener that the server will bind and accept minecraft
    /// client connections from. Set by the `config.host` and `config.port`
    /// fields.
    pub mc_listener: TcpListener,

    /// API TCP listener that the server will bind and accept api connections
    /// from. Set by the `config.host` and `config.api_port` fields.
    pub api_listener: TcpListener,

    /// Able to broadcast a shutdown signal to all active connections.
    ///
    /// The initial `notify_shutdown` trigger is provided by the `run` server
    /// function: the server is then responsible for gracefully shutting down active
    /// connections. When a connection task is spawned, it is passed a handle to
    /// the broadcast receiver. When a graceful shutdown is initiated, a `String`
    /// value is sent via the broadcast::Sender. Each active connection receives it,
    /// parses the template, reaches a safe termination state, and disconnects the
    /// client, completing the tast.
    pub notify_shutdown: broadcast::Sender<String>,

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
    pub shutdown_complete_tx: mpsc::Sender<String>,
}

impl Server {
    pub async fn new(
        config: ServerConfig,
        notify_shutdown: broadcast::Sender<String>,
        shutdown_complete_tx: mpsc::Sender<String>,
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
        {
            let config = self.config.read().await;

            let address = format!("{}:{}", config.host, config.port);

            info!("Statik server is up! Broadcasting on {address}.");
        }

        loop {
            select! {

                res = self.mc_listener.accept() => {
                    match res {
                        Ok((stream, address)) => {
                            debug!("New mc connection from {}", address);

                            let shutdown_complete_tx = self.shutdown_complete_tx.clone();
                            let shutdown = Shutdown::new(self.notify_shutdown.subscribe());

                            //replace this with shared config struct later
                            let config = self.config.clone();
                            let config2 = self.config.clone();

                            tokio::spawn(async move {

                                //handler
                                Handler::new(config, Connection::new(config2, stream, address).await, shutdown, shutdown_complete_tx).await.run().await;

                            });
                        },
                        Err(err) => error!("Failed to accept mc connection: {:#}", anyhow::anyhow!(err)),
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
                        Err(err) => error!("Failed to accept api connection: {:#}", anyhow::anyhow!(err)),
                    }
                }
            }
        }

        Ok(())
    }

    /// Gracefully sends shutdown signals to all clients connected to the
    /// server. Supply `None` to use the default disconnect message, or
    /// `Some(my_disconnecting_reason)` to send clients a custom disconnect
    /// message. the message will be parsed using the [`Tera`] templater.
    pub async fn shutdown(&self, reason: Option<String>) -> anyhow::Result<()> {
        info!("Shutting down the server...");

        let template = match reason {
            Some(template) => template,
            None => self.config.read().await.disconnect_msg.clone(),
        };

        debug!("sending shutdown notice to connected clients, using disconnect message template: \"{template}\"");

        self.notify_shutdown.send(template)?;

        Ok(())
    }
}
