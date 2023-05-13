use tokio::{
    signal::{self, unix::SignalKind},
    sync::broadcast,
};

use statik_common::prelude::*;

/// Listens for the server shutdown signal.
///
/// Shutdown is signalled using a `broadcast::Receiver`. Only a single value is
/// ever sent. Once a value has been sent via the broadcast channel, the server
/// should shutdown.
///
/// The `Shutdown` struct listens for the signal and tracks that the signal has
/// been received. Callers may query for whether the shutdown signal has been
/// received or not.
#[derive(Debug)]
pub struct Shutdown {
    /// `true` if the shutdown signal has been received - should be a one way change (you can't 'un-shutdown' a server).
    is_shutdown: bool,

    /// The receive half of the channel used to listen for shutdown.
    recv: broadcast::Receiver<String>,
}

impl Shutdown {
    /// Create a new `Shutdown` backed by the given `broadcast::Receiver`.
    pub(crate) fn new(recv: broadcast::Receiver<String>) -> Shutdown {
        Shutdown {
            is_shutdown: false,
            recv,
        }
    }

    /// Returns `true` if the shutdown signal has been received.
    pub(crate) fn is_shutdown(&self) -> bool {
        self.is_shutdown
    }

    /// Receive the shutdown notice, waiting if necessary.
    pub(crate) async fn recv(&mut self) -> anyhow::Result<String> {
        // Cannot receive a "lag error" as only one value is ever sent.
        // let reason = self.recv.recv().await?;
        let reason = self.recv.recv().await?;

        // Remember that the signal has been received.
        self.is_shutdown = true;

        Ok(reason)
    }
}

pub async fn sigterm() -> tokio::io::Result<()> {
    signal::unix::signal(SignalKind::terminate())?.recv().await;
    Ok(())
}

pub async fn sigquit() -> tokio::io::Result<()> {
    signal::unix::signal(SignalKind::quit())?.recv().await;
    Ok(())
}

pub async fn ctrl_c() -> tokio::io::Result<()> {
    signal::ctrl_c().await?;
    Ok(())
}
