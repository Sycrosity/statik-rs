#![allow(dead_code)]

use config::ServerConfig;

use tokio::{
    select,
    sync::{broadcast, mpsc},
};

use anyhow::anyhow;

use log::{debug, error, info};

mod config;
mod connection;
mod handler;
mod player;
mod server;
mod shutdown;

use crate::server::Server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = ServerConfig {
        host: String::from("127.0.0.1"),
        max_players: 64,
        disconnect_msg: String::from("{{ username }}, the server closed."),
        ..Default::default()
    };

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_line_number(true)
        // .with_file(true)
        // .with_thread_names(true)
        .init();

    info!("Statik server is starting.");

    let address = format!("{}:{}", &config.host, &config.port);

    // When the provided `shutdown` future completes, we must send a shutdown
    // message to all active connections. We use a broadcast channel for this
    // purpose. The call below ignores the receiver of the broadcast pair, and when
    // a receiver is needed, the subscribe() method on the sender is used to create
    // one.
    let (notify_shutdown, mut _shutdown_rx) = broadcast::channel::<String>(1);
    let (shutdown_complete_tx, mut _shutdown_complete_rx) = mpsc::channel(1);

    let mut server = Server::new(config, notify_shutdown, shutdown_complete_tx).await?;

    info!("Statik server is up! Broadcasting on {address}.");

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

            _ = shutdown::ctrl_c() => {

                debug!("SIGINT (ctrl_c) OS signal recieved.");
                server.shutdown(None).await?;
                break;

            },
            _ = shutdown::sigquit() => {

                debug!("SIGQUIT (quit) OS signal recieved.");
                server.shutdown(None).await?;
                break;
            },
            _ = shutdown::sigterm() => {

                debug!("SIGTERM (terminate) OS signal recieved.");
                server.shutdown(None).await?;
                break;
            },
            //shutdowns sent from the server itself (e.g. not an external OS signal)
            reason = _shutdown_rx.recv() => {

                debug!("internal shutdown signal recieved.");
                server.shutdown(Some(reason?)).await?;
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
