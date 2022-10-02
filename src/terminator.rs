use anyhow::{Context, Result};
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::oneshot;
use tracing::*;

async fn wait_for_sigterm(sender: oneshot::Sender<()>) -> Result<()> {
    trace!("finger on the trigger");
    let mut stream = signal(SignalKind::terminate())?;
    let _ = stream.recv().await.context("terminator terminated");
    let _ = sender.send(());
    Ok(())
}

pub fn new() -> oneshot::Receiver<()> {
    debug!("terminator was born");
    let (tx, rx) = oneshot::channel::<()>();
    tokio::spawn(wait_for_sigterm(tx));

    rx
}
