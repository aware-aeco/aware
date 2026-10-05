//! The store reference lock (#627): `AWARE_HOME/agent-store-control/store.flock`.
//!
//! Every operation that creates or relies on a reference into the agent store
//! — a snapshot, an install / update / uninstall, a compile writing a lock, a
//! migrate verb, an app install / rename / duplicate / uninstall, a run from
//! the moment it reads its approval until its agents are resolved — holds this
//! lock **shared** for as long as it does so. Only GC (#629) will ever take it
//! **exclusive**, and only through a bounded try ([`RefGuard::exclusive_within`]),
//! so an exclusive request never blocks a run.
//!
//! **Lock order, everywhere:** the store lock FIRST, then any per-agent swap
//! locks (`install::swap`), sorted by id. The swap-lock API takes a
//! `&RefGuard` and ties every swap lock's lifetime to it, so the order is
//! enforced by the compiler: a swap lock cannot be taken without a guard and
//! cannot outlive it.
//!
//! **No upgrades.** A guard is obtained only through [`crate::agent_store::open`]
//! (shared) or [`RefGuard::exclusive_within`]. No code path holds a shared guard
//! while asking for an exclusive one — that would wait on itself — and a debug
//! assertion enforces it per thread (plan §12 R5-1).
//!
//! The lock is an fs2 OS lock on an open handle: it dies with the process, so a
//! crash or a kill never leaves the store locked. Rust opens files
//! non-inheritable, so no child process (a bridge, a sidecar) holds it on.

use std::cell::Cell;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use fs2::FileExt;

use crate::error::AwareError;
use crate::paths::Paths;

/// The lock file's name inside `agent-store-control/`. `.flock`, never
/// `.lock`: in AWARE a `*.lock` file is an approval (`<app>.lock`, archived
/// approvals), and nothing that looks for approvals may mistake an OS lock
/// file for one. The per-agent swap locks follow suit.
pub const STORE_LOCK_FILE: &str = "store.flock";

thread_local! {
    /// Shared guards currently held by this thread — the debug check that no
    /// thread asks for an exclusive guard while holding a shared one.
    static SHARED_HELD: Cell<u32> = const { Cell::new(0) };
}

/// A held store reference lock. Dropping it releases the lock.
#[derive(Debug)]
pub struct RefGuard {
    _file: std::fs::File,
    home: PathBuf,
    exclusive: bool,
}

impl RefGuard {
    /// Take the store lock shared, blocking until no exclusive holder remains.
    /// Reached only through [`crate::agent_store::open`], so that the one place
    /// a future store import (#627-b) must run before any shared guard exists is
    /// the only door to a guard.
    pub(super) fn shared(paths: &Paths) -> Result<Self, AwareError> {
        let file = open_lock_file(paths)?;
        file.lock_shared().map_err(|error| {
            lock_error(paths, "take the agent store reference lock (shared)", error)
        })?;
        SHARED_HELD.with(|held| held.set(held.get() + 1));
        Ok(Self {
            _file: file,
            home: paths.aware_home.clone(),
            exclusive: false,
        })
    }

    /// Try to take the store lock exclusive for up to `wait`; `Ok(None)` when it
    /// stayed busy. Never blocks unboundedly, so it can never deadlock a run.
    /// Used by GC (#629); never called while this thread holds a shared guard.
    #[cfg_attr(not(test), allow(dead_code))]
    pub fn exclusive_within(paths: &Paths, wait: Duration) -> Result<Option<Self>, AwareError> {
        debug_assert_eq!(
            SHARED_HELD.with(Cell::get),
            0,
            "an exclusive store guard was requested while this thread holds a shared one — \
             that is a lock upgrade, which can wait on itself (#627 plan R5-1)"
        );
        let file = open_lock_file(paths)?;
        let deadline = Instant::now() + wait;
        loop {
            match file.try_lock_exclusive() {
                Ok(()) => {
                    return Ok(Some(Self {
                        _file: file,
                        home: paths.aware_home.clone(),
                        exclusive: true,
                    }));
                }
                Err(error) if lock_is_contended(&error) => {
                    if Instant::now() >= deadline {
                        return Ok(None);
                    }
                    std::thread::sleep(Duration::from_millis(25));
                }
                Err(error) => {
                    return Err(lock_error(
                        paths,
                        "take the agent store reference lock (exclusive)",
                        error,
                    ));
                }
            }
        }
    }

    /// The AWARE_HOME this guard locks — the swap-lock API refuses a guard of
    /// another home, so holding *a* guard is never mistaken for holding *the*
    /// guard.
    pub fn home(&self) -> &Path {
        &self.home
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn is_exclusive(&self) -> bool {
        self.exclusive
    }
}

impl Drop for RefGuard {
    fn drop(&mut self) {
        if !self.exclusive {
            SHARED_HELD.with(|held| held.set(held.get().saturating_sub(1)));
        }
        // Closing the handle (the field drop) releases the OS lock.
    }
}

/// Refuse a guard that locks a different AWARE_HOME than the one `paths`
/// names: holding *a* guard must never pass for holding *the* guard.
pub(crate) fn require_home(guard: &RefGuard, paths: &Paths) -> Result<(), AwareError> {
    if guard.home() == paths.aware_home.as_path() {
        Ok(())
    } else {
        Err(AwareError::Internal(format!(
            "the agent store guard held is for {}, not {}",
            guard.home().display(),
            paths.aware_home.display()
        )))
    }
}

/// How many shared guards this thread holds.
#[cfg(test)]
pub(crate) fn shared_held_on_this_thread() -> u32 {
    SHARED_HELD.with(Cell::get)
}

fn open_lock_file(paths: &Paths) -> Result<std::fs::File, AwareError> {
    let dir = paths.agent_store_control_dir();
    std::fs::create_dir_all(&dir)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", dir.display())))?;
    let path = dir.join(STORE_LOCK_FILE);
    std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&path)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", path.display())).into())
}

fn lock_error(paths: &Paths, what: &str, error: std::io::Error) -> AwareError {
    std::io::Error::new(
        error.kind(),
        format!(
            "cannot {what} at {}: {error}",
            paths
                .agent_store_control_dir()
                .join(STORE_LOCK_FILE)
                .display()
        ),
    )
    .into()
}

/// Whether a lock call failed only because someone else holds the lock.
/// Windows reports `ERROR_LOCK_VIOLATION` (33) for a contended byte range.
pub(crate) fn lock_is_contended(error: &std::io::Error) -> bool {
    error.kind() == std::io::ErrorKind::WouldBlock
        || (cfg!(windows) && error.raw_os_error() == Some(33))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn home() -> (tempfile::TempDir, Paths) {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: tmp.path().to_path_buf(),
        };
        (tmp, paths)
    }

    #[test]
    fn shared_guards_coexist_and_block_an_exclusive_try() {
        let (_tmp, paths) = home();
        let one = crate::agent_store::open(&paths).unwrap();
        let two = crate::agent_store::open(&paths).unwrap();
        assert!(!one.is_exclusive());
        assert_eq!(shared_held_on_this_thread(), 2);
        // Another thread (holding nothing) cannot take it exclusive while shared
        // guards exist, and gives up within its bound instead of blocking.
        let other = paths.clone();
        let started = Instant::now();
        let got = std::thread::spawn(move || {
            RefGuard::exclusive_within(&other, Duration::from_millis(150))
                .unwrap()
                .is_some()
        })
        .join()
        .unwrap();
        assert!(!got, "exclusive must fail while shared guards are held");
        assert!(started.elapsed() < Duration::from_secs(5));
        drop(one);
        drop(two);
        assert_eq!(shared_held_on_this_thread(), 0);
        let exclusive = RefGuard::exclusive_within(&paths, Duration::from_millis(500))
            .unwrap()
            .expect("free once every shared guard is dropped");
        assert!(exclusive.is_exclusive());
        // And while it is held, a shared request from another thread waits.
        let other = paths.clone();
        let waiter = std::thread::spawn(move || {
            let started = Instant::now();
            let _guard = crate::agent_store::open(&other).unwrap();
            started.elapsed()
        });
        std::thread::sleep(Duration::from_millis(200));
        drop(exclusive);
        assert!(waiter.join().unwrap() >= Duration::from_millis(150));
    }

    /// R5-1: no path may hold a shared guard while asking for an exclusive one.
    #[test]
    #[cfg(debug_assertions)]
    #[should_panic(expected = "lock upgrade")]
    fn asking_for_exclusive_while_holding_shared_is_a_debug_assertion() {
        let (_tmp, paths) = home();
        let _shared = crate::agent_store::open(&paths).unwrap();
        let _ = RefGuard::exclusive_within(&paths, Duration::from_millis(10));
    }
}
