use std::sync::Arc;

use base64::prelude::{Engine as _, BASE64_STANDARD};
use statik_common::prelude::*;
use tokio::net::TcpListener;
use tokio::select;
use tokio::sync::{broadcast, mpsc, RwLock};

use crate::config::ServerConfig;
use crate::connection::Connection;
use crate::handler::Handler;
use crate::shutdown::Shutdown;

pub struct Server {
    /// Configuration for how the server should be run.
    pub config: Arc<RwLock<ServerConfig>>,

    /// Minecraft TCP listener that the server will bind and accept minecraft
    /// client connections from. Set by the `config.general.host` and
    /// `config.mc.port` fields.
    pub mc_listener: TcpListener,

    /// API TCP listener that the server will bind and accept api connections
    /// from. Set by the `config.general.host` and `config.api.port` fields.
    pub api_listener: TcpListener,

    /// Able to broadcast a shutdown signal to all active connections.
    ///
    /// The initial `notify_shutdown` trigger is provided by the `run` server
    /// function: the server is then responsible for gracefully shutting down
    /// active connections. When a connection task is spawned, it is passed
    /// a handle to the broadcast receiver. When a graceful shutdown is
    /// initiated, a `String` value is sent via the broadcast::Sender. Each
    /// active connection receives it, parses the template, reaches a safe
    /// termination state, and disconnects the client, completing the tast.
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
        mut config: ServerConfig,
        notify_shutdown: broadcast::Sender<String>,
        shutdown_complete_tx: mpsc::Sender<String>,
    ) -> anyhow::Result<Self> {
        let mc_address = format!("{}:{}", config.general.host, config.mc.port);
        let api_address = format!("{}:{}", config.general.host, config.api.port);

        let mc_listener = TcpListener::bind(&mc_address).await?;
        let api_listener = TcpListener::bind(&api_address).await?;

        config.mc.icon = if let Some(s) = config.mc.icon {
            match tokio::fs::read(&s).await {
                Ok(s) => Some(BASE64_STANDARD.encode(s)),
                Err(e) => {
                    warn!("could not read icon file \"{s}\", defaulting to no icon: {e}");
                    None
                }
            }
        } else {
            None
        };

        let config = Arc::new(RwLock::new(config));

        info!(
            "Statik server is up! Broadcasting the mc server on {mc_address}, and the api server \
             on {api_address}."
        );

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
                            info!("New mc connection from {}.", address);

                            let shutdown_complete_tx = self.shutdown_complete_tx.clone();
                            let shutdown = Shutdown::new(self.notify_shutdown.subscribe());

                            //replace this with shared config struct later
                            let config = self.config.clone();
                            let config2 = self.config.clone();

                            tokio::spawn(async move {

                                if let Err(err) = Handler::new(config, Connection::new(config2, stream, address).await, shutdown, shutdown_complete_tx).await.run().await {
                                    error!("Connection error: {err:#}");
                                }

                                info!("Connection with mc client {} ended.", address);

                            });
                        },
                        Err(err) => error!("Failed to accept mc connection: {:#}", anyhow::anyhow!(err)),
                    }
                }
                res = self.api_listener.accept() => {
                    match res {
                        Ok((_stream, address)) => {
                            info!("New api connection from {}.", address);

                            let _shutdown_complete_tx = self.shutdown_complete_tx.clone();
                            let _shutdown = Shutdown::new(self.notify_shutdown.subscribe());

                            let _config = self.config.clone();

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
    }

    /// Gracefully sends shutdown signals to all clients connected to the
    /// server. Supply `None` to use the default disconnect message, or
    /// `Some(my_disconnecting_reason)` to send clients a custom disconnect
    /// message. the message will be parsed using the [`Tera`] templater.
    pub async fn shutdown(&self, reason: Option<String>) -> anyhow::Result<()> {
        info!("Shutting down the server...");

        let template = match reason {
            Some(template) => template,
            None => self.config.read().await.mc.disconnect_msg.clone(),
        };

        debug!(
            "sending shutdown notice to connected clients, using disconnect message template: \
             \"{template}\""
        );

        self.notify_shutdown.send(template)?;

        Ok(())
    }
}
