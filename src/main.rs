#![allow(dead_code)]

mod quit;

use std::path::PathBuf;

use clap::Parser;
use statik_common::prelude::*;
use statik_server::{config::ServerConfig, server::Server};
use tokio::{
    select,
    sync::{broadcast, mpsc},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let config_path = cli.config.unwrap_or("statik.toml".into());

    let config = match tokio::fs::read_to_string(&config_path).await {
        Ok(s) => match toml::from_str::<ServerConfig>(&s) {
            Ok(res) => res,
            Err(e) => {
                println!(
                    "Incorrectly formatted statik config file: \"{}\", using default values: {e}",
                    &config_path.display()
                );
                ServerConfig::default()
            }
        },
        Err(e) => {
            if config_path == PathBuf::from("statik.toml") {
                //will error if we don't have write permissions.
                if let Err(e) = tokio::fs::write(
                    PathBuf::from("statik.toml"),
                    //this shouldn't be able to error, as ServerConfig can be serialised.
                    toml::to_string_pretty(&ServerConfig::default()).unwrap(),
                )
                .await
                {
                    println!("statik.toml could not be found, but couldn't create one: {e}");
                } else {
                    println!("Created statik.toml as it could not be found.");
                }
            } else {
                println!(
                    "Could not read statik config file: \"{}\", using default values: {e}",
                    &config_path.display()
                );
            }

            ServerConfig::default()
        }
    };

    let config_filter = match config.general.log_level.parse::<tracing::Level>() {
        Ok(res) => res,
        Err(e) => {
            println!("Incorrect value provided for log level: {e}. Using default DEBUG level.");
            tracing::Level::DEBUG
        }
    };

    //make this dynamic in the future!
    tracing_subscriber::fmt()
        .with_max_level(config_filter)
        .with_line_number(true)
        // .with_file(true)
        // .with_thread_names(true)
        .init();

    info!("Statik server is starting.");

    // When the provided `shutdown` future completes, we must send a shutdown
    // message to all active connections. We use a broadcast channel for this
    // purpose. The call below ignores the receiver of the broadcast pair, and when
    // a receiver is needed, the subscribe() method on the sender is used to create
    // one.
    let (notify_shutdown, mut _shutdown_rx) = broadcast::channel::<String>(1);
    let (shutdown_complete_tx, mut _shutdown_complete_rx) = mpsc::channel(1);

    let mut server = Server::new(config, notify_shutdown, shutdown_complete_tx).await?;

    loop {
        select! {

            res = server.run() => {

                // Errors encountered when handling individual connections should not
                // bubble up to this point.
                if let Err(err) = res {
                    error!("Failed to accept connection: {:#}", anyhow!(err));
                }
            }

            _ = quit::ctrl_c() => {

                debug!("SIGINT (ctrl_c) OS signal recieved.");
                server.shutdown(None).await?;
                break;

            },
            _ = quit::sigquit() => {

                debug!("SIGQUIT (quit) OS signal recieved.");
                server.shutdown(None).await?;
                break;
            },
            _ = quit::sigterm() => {

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
