use tokio::signal::{
    unix::SignalKind,
    {self},
};

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
