//! Lifecycle helpers for the runtime: stop-broadcast channel + Ctrl+C handler.

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

// ── The process-wide Ctrl-C watcher (#692 review) ──
//
// A Blender call waits on Ctrl-C itself, because on Unix Blender leads its own process group and a
// terminal's SIGINT no longer reaches it. But tokio's Ctrl-C handler, once installed, stays
// installed for the life of the process and replaces the default "Ctrl-C ends the program": a
// one-shot run that had finished its Blender call would then swallow every later Ctrl-C. So there
// is ONE watcher, installed lazily by the first Blender call, and each Ctrl-C it sees is routed by
// [`ctrl_c_action`]: interrupt the Blender calls in flight; otherwise defer to a mode that owns
// shutdown itself (long-running `aware app run`); otherwise end the process the way the default
// SIGINT would have.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Once, OnceLock};

/// Blender calls currently waiting on Ctrl-C (live [`InterruptWatch`]es).
static IN_FLIGHT: AtomicUsize = AtomicUsize::new(0);
/// Live [`CtrlCOwner`]s: modes that handle Ctrl-C themselves and must not be exited under.
static OWNERS: AtomicUsize = AtomicUsize::new(0);
/// Bumped once per Ctrl-C delivered to in-flight calls; each call watches it.
static GENERATION: OnceLock<watch::Sender<u64>> = OnceLock::new();
static WATCHER: Once = Once::new();

/// The exit code for a Ctrl-C that ends the process: 128 + SIGINT, what a shell reports.
pub const CTRL_C_EXIT_CODE: i32 = 130;

fn generation() -> &'static watch::Sender<u64> {
    GENERATION.get_or_init(|| watch::channel(0).0)
}

/// What one Ctrl-C does, given the process's state at that moment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CtrlCAction {
    /// A Blender call is in flight: it kills its process tree and fails as interrupted.
    InterruptCalls,
    /// Nothing in flight, but a mode that owns shutdown (long-running run) is handling it.
    DeferToOwner,
    /// Nothing in flight and nobody else listening: end the process, as the default SIGINT would.
    ExitProcess,
}

/// The routing rule, pure so every branch is testable without sending a signal.
pub fn ctrl_c_action(in_flight: usize, owners: usize) -> CtrlCAction {
    if in_flight > 0 {
        CtrlCAction::InterruptCalls
    } else if owners > 0 {
        CtrlCAction::DeferToOwner
    } else {
        CtrlCAction::ExitProcess
    }
}

/// Apply one Ctrl-C to the live process state and return what was decided. `ExitProcess` is
/// returned rather than performed (the watcher performs it), so a test can observe the decision.
pub fn deliver_ctrl_c() -> CtrlCAction {
    let action = ctrl_c_action(
        IN_FLIGHT.load(Ordering::SeqCst),
        OWNERS.load(Ordering::SeqCst),
    );
    if action == CtrlCAction::InterruptCalls {
        generation().send_modify(|g| *g = g.wrapping_add(1));
    }
    action
}

/// Install the one process-wide watcher, if it is not running yet. It lives on its own thread and
/// runtime, so it neither depends on nor dies with whichever runtime made the first Blender call.
/// If the thread or its runtime cannot start, nothing is installed and Ctrl-C keeps its default.
pub fn ensure_ctrl_c_watcher() {
    WATCHER.call_once(|| {
        let _ = std::thread::Builder::new()
            .name("aware-ctrl-c".into())
            .spawn(|| {
                let Ok(rt) = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                else {
                    return;
                };
                rt.block_on(async {
                    while tokio::signal::ctrl_c().await.is_ok() {
                        if deliver_ctrl_c() == CtrlCAction::ExitProcess {
                            std::process::exit(CTRL_C_EXIT_CODE);
                        }
                    }
                });
            });
    });
}

/// One in-flight call's view of Ctrl-C. Counted while alive; dropping it ends the count.
pub struct InterruptWatch {
    rx: watch::Receiver<u64>,
}

/// Register a call as in flight. Subscribes BEFORE counting, so a Ctrl-C that sees the count is
/// always one this call will observe.
pub fn track_call() -> InterruptWatch {
    let rx = generation().subscribe();
    IN_FLIGHT.fetch_add(1, Ordering::SeqCst);
    InterruptWatch { rx }
}

impl InterruptWatch {
    /// Resolves at the next Ctrl-C delivered to in-flight calls.
    pub async fn interrupted(&mut self) {
        if self.rx.changed().await.is_err() {
            std::future::pending::<()>().await;
        }
    }
}

impl Drop for InterruptWatch {
    fn drop(&mut self) {
        IN_FLIGHT.fetch_sub(1, Ordering::SeqCst);
    }
}

/// Held by a mode that handles Ctrl-C itself (long-running `aware app run`), so the watcher never
/// exits the process out from under that mode's own graceful shutdown.
pub struct CtrlCOwner(());

impl CtrlCOwner {
    pub fn claim() -> Self {
        OWNERS.fetch_add(1, Ordering::SeqCst);
        CtrlCOwner(())
    }
}

impl Drop for CtrlCOwner {
    fn drop(&mut self) {
        OWNERS.fetch_sub(1, Ordering::SeqCst);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// #692 review: after a Blender call has finished, Ctrl-C must still end a one-shot run (the
    /// default SIGINT outcome); a call in flight is interrupted instead; and a mode that owns
    /// shutdown is deferred to rather than exited under.
    #[test]
    fn ctrl_c_interrupts_calls_in_flight_and_otherwise_ends_the_process_unless_owned() {
        assert_eq!(ctrl_c_action(0, 0), CtrlCAction::ExitProcess);
        assert_eq!(ctrl_c_action(1, 0), CtrlCAction::InterruptCalls);
        assert_eq!(ctrl_c_action(2, 0), CtrlCAction::InterruptCalls);
        assert_eq!(ctrl_c_action(0, 1), CtrlCAction::DeferToOwner);
        // In long-running mode a Blender call in flight is still interrupted (its owner stops too).
        assert_eq!(ctrl_c_action(1, 1), CtrlCAction::InterruptCalls);
        assert_ne!(CTRL_C_EXIT_CODE, 0);
    }

    /// The live wiring: a tracked call is woken by a delivered Ctrl-C; once it is dropped the same
    /// Ctrl-C would end the process; a claimed owner turns that into a deferral.
    #[tokio::test]
    async fn a_delivered_ctrl_c_wakes_the_tracked_call_then_falls_back_to_exit() {
        let mut call = track_call();
        assert_eq!(deliver_ctrl_c(), CtrlCAction::InterruptCalls);
        tokio::time::timeout(std::time::Duration::from_secs(5), call.interrupted())
            .await
            .expect("the in-flight call must be woken by the Ctrl-C");
        drop(call);

        assert_eq!(deliver_ctrl_c(), CtrlCAction::ExitProcess);
        let owner = CtrlCOwner::claim();
        assert_eq!(deliver_ctrl_c(), CtrlCAction::DeferToOwner);
        drop(owner);
        assert_eq!(deliver_ctrl_c(), CtrlCAction::ExitProcess);
    }

    #[tokio::test]
    async fn stop_channel_round_trips() {
        let (tx, mut rx) = stop_channel();
        assert!(!*rx.borrow());
        tx.send(true).unwrap();
        rx.changed().await.unwrap();
        assert!(*rx.borrow());
    }
}
