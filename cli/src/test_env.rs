//! Test-only serialized access to process-global environment variables.
//!
//! This crate's unit tests all link into a single test binary (there is no
//! `lib.rs`; `main.rs` is the only root), and libtest runs `#[test]` functions
//! on a thread pool by default. Six tests — four in `builder::stubs`, two in
//! `sidecar` — set and cleared `AWARE_SIDECAR`, each under a
//! `// SAFETY: single-threaded test` comment. Nothing made those tests
//! single-threaded or mutually exclusive, so the comment asserted an invariant
//! that did not hold.
//!
//! The concrete failure: `sidecar::discover_respects_env_var_when_file_exists`
//! points `AWARE_SIDECAR` at a file that *does* exist and asserts `discover()`
//! finds it. Any of the four `stubs` tests calling `remove_var` in the window
//! between that write and the read fails it. Widening that window to 30ms on the
//! pre-guard code reproduced it in 5 of 40 runs; the same widening with the
//! guard in place failed 0 of 40.
//!
//! ## What this does and does not discharge
//!
//! Rust 2024 made `set_var` / `remove_var` `unsafe` because they mutate a
//! process-global table, and std's contract requires that no other thread is
//! concurrently reading or writing the environment "through functions or global
//! variables other than the ones in this module" — naming, for example, `getenv`
//! reached via `ToSocketAddrs`.
//!
//! [`EnvVarGuard`] discharges the part of that which is in this crate's reach:
//! it orders every guarded write against every other guarded access. It cannot
//! exclude a `getenv` issued from linked C code — this binary links libsecret and
//! libdbus — or from a DNS lookup on some other thread. No in-process lock can;
//! confining every environment mutation to this one type is what keeps that
//! residue as small as it can be made.
//!
//! Worth stating precisely, because the old comments erred in this direction:
//! on unix, std wraps its own `getenv`/`setenv` in an internal lock, so
//! std-reader-versus-std-writer was already ordered. The defect actually
//! repaired here is the flaky assertion above, not observed UB.
//!
//! The guard also restores the previous value on drop, including during unwind.
//! The two `registry::fetch` tests previously cleared their variable only at the
//! end of the test body, so an earlier failed assertion leaked the override into
//! whatever ran next. (The other six removed theirs before asserting, so they did
//! not leak — they raced.)

use std::ffi::OsStr;
use std::marker::PhantomData;
use std::sync::{Mutex, MutexGuard, OnceLock, TryLockError};
use std::time::{Duration, Instant};

/// Serializes every environment mutation in the test binary. One lock for all
/// variables, not one per key: the hazard is a write racing *any* concurrent
/// read, not just one on the same key.
static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

/// How long to wait before treating a blocked acquisition as a deadlock rather
/// than contention. Generous — the longest guarded test body does file I/O — but
/// finite, so a misuse fails with a named error instead of hanging the binary
/// until CI's job timeout kills it with no diagnostic.
const LOCK_TIMEOUT: Duration = Duration::from_secs(30);

fn env_lock() -> MutexGuard<'static, ()> {
    let mutex = ENV_LOCK.get_or_init(|| Mutex::new(()));
    let deadline = Instant::now() + LOCK_TIMEOUT;
    loop {
        match mutex.try_lock() {
            Ok(guard) => return guard,
            Err(TryLockError::Poisoned(poisoned)) => {
                // A test panicked while holding the guard. libtest already
                // reports that failure; re-panicking here would bury it under a
                // cascade from every later test that touches the environment.
                // `clear_poison` is required to actually reset the flag —
                // `into_inner()` alone recovers the data but leaves the mutex
                // poisoned for the rest of the process, destroying poison as a
                // signal.
                mutex.clear_poison();
                return poisoned.into_inner();
            }
            Err(TryLockError::WouldBlock) => {
                assert!(
                    Instant::now() < deadline,
                    "EnvVarGuard: no environment lock after {LOCK_TIMEOUT:?}. \
                     The usual cause is a second `EnvVarGuard::set` on a thread \
                     that already holds a guard — the lock is not reentrant; use \
                     `replace` to vary one variable within a test."
                );
                std::thread::sleep(Duration::from_millis(2));
            }
        }
    }
}

/// An RAII override of one environment variable, restored on drop.
///
/// Holds the process-wide environment lock for its whole lifetime, so writes
/// through this type are totally ordered.
///
/// The lock is not reentrant, so a second [`set`](EnvVarGuard::set) on a thread
/// that already holds a guard cannot succeed; it fails after [`LOCK_TIMEOUT`]
/// with a message naming the cause. To vary one variable within a test use
/// [`replace`](EnvVarGuard::replace); to override two at once, extend this type.
pub(crate) struct EnvVarGuard {
    key: &'static str,
    previous: Option<std::ffi::OsString>,
    _lock: MutexGuard<'static, ()>,
    /// Makes the guard `!Sync`. Without it `EnvVarGuard` would be `Sync` (a
    /// `MutexGuard` is), letting two threads share one `&EnvVarGuard` and drive
    /// concurrent writes through a single held lock — exactly the race this type
    /// exists to prevent, reachable from safe code. `replace` taking `&mut self`
    /// already blocks that; this makes it structural rather than incidental.
    _not_sync: PhantomData<*const ()>,
}

impl EnvVarGuard {
    /// Set `key` to `value` until the returned guard drops.
    pub(crate) fn set(key: &'static str, value: impl AsRef<OsStr>) -> Self {
        let lock = env_lock();
        let previous = std::env::var_os(key);
        // SAFETY: `lock` is held across this write and then moves into the
        // returned guard, so this is ordered against every other access that
        // goes through `EnvVarGuard` — the only sanctioned way to mutate the
        // environment in this binary. It does not exclude a `getenv` from linked
        // C code or a DNS lookup on another thread; see the module docs for why
        // no in-process lock can, and why that residue is irreducible here.
        unsafe { std::env::set_var(key, value) };
        Self {
            key,
            previous,
            _lock: lock,
            _not_sync: PhantomData,
        }
    }

    /// Replace the value while keeping the same guard — and therefore the same
    /// lock and the same saved original.
    ///
    /// For tests exercising several values of one variable in sequence. Dropping
    /// and re-acquiring a guard between them would briefly restore the original
    /// and hand the lock to another test mid-scenario.
    ///
    /// Takes `&mut self` so that a shared `&EnvVarGuard` cannot be used to write.
    pub(crate) fn replace(&mut self, value: impl AsRef<OsStr>) {
        // SAFETY: `self._lock` is held for as long as `self` is alive, and
        // `&mut self` proves no other reference to this guard is usable
        // concurrently, so this write carries the same ordering as `set`.
        unsafe { std::env::set_var(self.key, value) };
    }
}

impl Drop for EnvVarGuard {
    fn drop(&mut self) {
        // The lock is still held: a `Drop` body runs before any of the type's
        // fields are dropped, so `_lock` cannot have been released yet whatever
        // its declaration position. The restore is therefore ordered exactly as
        // the writes in `set` are.
        match self.previous.take() {
            // SAFETY: as in `set`, under the still-held environment lock.
            Some(previous) => unsafe { std::env::set_var(self.key, previous) },
            // SAFETY: as above; the variable was unset before this guard ran, so
            // restoring it means removing it.
            None => unsafe { std::env::remove_var(self.key) },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Each test uses its own key. The guard orders writers, but these assertions
    // read the environment outside the lock, so sharing a key between two
    // concurrently-running tests would make them race on the assertion itself —
    // the very bug this type exists to remove.

    #[test]
    fn restores_absent_variable_on_drop() {
        const KEY: &str = "AWARE_TEST_ENV_GUARD_ABSENT";
        assert!(std::env::var_os(KEY).is_none(), "precondition: unset");
        {
            let _guard = EnvVarGuard::set(KEY, "value");
            assert_eq!(std::env::var(KEY).as_deref(), Ok("value"));
        }
        assert!(
            std::env::var_os(KEY).is_none(),
            "guard must remove a variable that was previously unset"
        );
    }

    #[test]
    fn replace_swaps_the_value_without_disturbing_the_saved_original() {
        const KEY: &str = "AWARE_TEST_ENV_GUARD_REPLACE";
        let mut guard = EnvVarGuard::set(KEY, "first");
        assert_eq!(std::env::var(KEY).as_deref(), Ok("first"));
        guard.replace("second");
        assert_eq!(std::env::var(KEY).as_deref(), Ok("second"));
        drop(guard);
        assert!(
            std::env::var_os(KEY).is_none(),
            "replace must not overwrite the original value the guard restores"
        );
    }

    #[test]
    fn restores_after_a_panic_inside_the_scope() {
        const KEY: &str = "AWARE_TEST_ENV_GUARD_PANIC";
        // No panic-hook swap here: libtest captures per-test output and discards
        // it when the test passes, so the backtrace never reaches the log anyway.
        // Replacing the process-global hook to hide it would blank out the
        // diagnostics of any *other* test that happened to fail in the window.
        let result = std::panic::catch_unwind(|| {
            let _guard = EnvVarGuard::set(KEY, "value");
            panic!("boom");
        });
        assert!(result.is_err(), "the panic must propagate");
        assert!(
            std::env::var_os(KEY).is_none(),
            "a panic must not leak the override into the next test"
        );
        // The unwinding guard poisoned the lock; the next acquisition must still
        // succeed, or one failing test would cascade into every later one.
        let _after = EnvVarGuard::set(KEY, "reacquired");
        assert_eq!(std::env::var(KEY).as_deref(), Ok("reacquired"));
    }

    /// The negative control for the module's whole reason to exist. The
    /// save/restore tests above still pass if the mutex is defeated (each
    /// acquisition handed a fresh lock); this one does not.
    #[test]
    fn a_second_guard_cannot_be_acquired_while_the_first_is_alive() {
        const OUTER: &str = "AWARE_TEST_ENV_GUARD_EXCL_OUTER";
        const INNER: &str = "AWARE_TEST_ENV_GUARD_EXCL_INNER";
        let (tx, rx) = std::sync::mpsc::channel();

        let outer = EnvVarGuard::set(OUTER, "held");
        let contender = std::thread::spawn(move || {
            let _inner = EnvVarGuard::set(INNER, "acquired");
            // Only reachable once `outer` has released the lock.
            let _ = tx.send(());
        });

        assert!(
            rx.recv_timeout(Duration::from_millis(250)).is_err(),
            "a second guard must not be acquirable while the first is alive — \
             the environment lock is not providing mutual exclusion"
        );

        drop(outer);
        rx.recv_timeout(LOCK_TIMEOUT)
            .expect("the second guard must be acquirable once the first drops");
        contender.join().expect("contender thread must not panic");
    }
}
