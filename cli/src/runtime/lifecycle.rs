//! Lifecycle helpers for the runtime: stop-broadcast channel + Ctrl+C handler.

#![allow(dead_code)]

use tokio::sync::watch;

pub type StopSender = watch::Sender<bool>;
pub type StopReceiver = watch::Receiver<bool>;

pub fn stop_channel() -> (StopSender, StopReceiver) {
    watch::channel(false)
}

/// Register a Ctrl+C handler that flips `stop` to true on signal.
/// Returns a JoinHandle which the caller can ignore (or await for graceful shutdown).
pub fn install_ctrl_c_handler(stop: StopSender) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        if tokio::signal::ctrl_c().await.is_ok() {
            let _ = stop.send(true);
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn stop_channel_round_trips() {
        let (tx, mut rx) = stop_channel();
        assert!(!*rx.borrow());
        tx.send(true).unwrap();
        rx.changed().await.unwrap();
        assert!(*rx.borrow());
    }
}
