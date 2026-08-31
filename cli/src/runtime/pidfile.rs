//! Pidfile under `<instance_dir>/pidfile.yaml` for `aware app stop`.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::AwareError;

/// Kernel-backed, crash-released single-run fence used only by app graphs that contain the exact
/// `model-reference-reader` agent. Ordinary one-shot apps never acquire it.
pub struct ExclusiveControl {
    file: std::fs::File,
    instance_dir: std::path::PathBuf,
}

impl ExclusiveControl {
    pub fn acquire(instance_dir: &Path, pidfile: &Pidfile) -> Result<Self, AwareError> {
        use fs2::FileExt;
        use std::io::{Seek, Write};
        std::fs::create_dir_all(instance_dir)?;
        let mut file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(instance_dir.join("model-reader-control.lock"))?;
        file.try_lock_exclusive().map_err(|_| {
            AwareError::Conflict(
                "a model-reference-reader run already owns this app instance".into(),
            )
        })?;
        // A bridge can die before its managed host has finished cancelling and joining the
        // commercial provider tree. The host holds this second lock for its entire lifetime.
        // Pass it exclusively before admitting a replacement run, while retaining the primary
        // instance lock so two replacements cannot race each other.
        let cleanup_path = Self::host_cleanup_fence_path(instance_dir);
        let cleanup = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(cleanup_path)?;
        cleanup.lock_exclusive()?;
        cleanup.unlock()?;
        // Persist the exact reader owner while the kernel lock is held. If the process crashes,
        // `app stop` can distinguish its stale diagnostic pidfile from a later non-reader run.
        file.set_len(0)?;
        file.rewind()?;
        let owner = serde_yaml::to_string(pidfile)
            .map_err(|error| AwareError::Internal(format!("control owner: {error}")))?;
        file.write_all(owner.as_bytes())?;
        file.sync_all()?;
        write(pidfile, instance_dir)?;
        Ok(Self {
            file,
            instance_dir: instance_dir.to_path_buf(),
        })
    }

    pub fn host_cleanup_fence_path(instance_dir: &Path) -> std::path::PathBuf {
        instance_dir.join("model-reader-host-cleanup.lock")
    }

    /// Whether the exact reader-run kernel fence is currently owned. Unlike a pidfile check,
    /// this cannot mistake a recycled PID for the run that created the control record.
    #[cfg(test)]
    pub fn is_held(instance_dir: &Path) -> Result<bool, AwareError> {
        use fs2::FileExt;
        let path = instance_dir.join("model-reader-control.lock");
        if !path.is_file() {
            return Ok(false);
        }
        let file = match std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
        {
            Ok(file) => file,
            Err(error) if lock_is_contended(&error) => return Ok(true),
            Err(error) => return Err(error.into()),
        };
        match file.try_lock_exclusive() {
            Ok(()) => {
                file.unlock()?;
                Ok(false)
            }
            Err(error) if lock_is_contended(&error) => Ok(true),
            Err(error) => Err(error.into()),
        }
    }

    /// Remove a stale reader pidfile while retaining the same kernel lock that proved no reader
    /// owns it. Returning `false` means either a live reader owns the fence or the current pidfile
    /// belongs to a different run and must be handled by the ordinary stop path.
    pub fn reclaim_stale(instance_dir: &Path) -> Result<bool, AwareError> {
        use fs2::FileExt;
        use std::io::{Read, Seek, Write};
        let path = instance_dir.join("model-reader-control.lock");
        if !path.is_file() {
            return Ok(false);
        }
        let mut file = match std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(path)
        {
            Ok(file) => file,
            Err(error) if lock_is_contended(&error) => return Ok(false),
            Err(error) => return Err(error.into()),
        };
        match file.try_lock_exclusive() {
            Ok(()) => {
                // A new reader cannot acquire its fence until this decision is complete. Match the
                // persisted reader owner to the current pidfile before removing it: an unrelated
                // long-running app may legitimately have reused this instance after the reader died.
                file.rewind()?;
                let mut owner_text = String::new();
                file.read_to_string(&mut owner_text)?;
                let owner = serde_yaml::from_str::<Pidfile>(&owner_text).ok();
                let current = read(instance_dir).ok();
                let belongs_to_stale_reader = match (&owner, &current) {
                    (Some(owner), Some(current)) => {
                        owner.app == current.app
                            && owner.instance == current.instance
                            && owner.run_id == current.run_id
                    }
                    (Some(_), None) | (None, None) => true,
                    // Compatibility with control files created before owner metadata existed.
                    // Never let such an unlocked file mask a live run, reader or otherwise.
                    (None, Some(current)) => !process_is_alive(current.pid),
                };
                if belongs_to_stale_reader {
                    remove(instance_dir);
                }
                file.set_len(0)?;
                file.rewind()?;
                file.flush()?;
                file.unlock()?;
                Ok(belongs_to_stale_reader)
            }
            Err(error) if lock_is_contended(&error) => Ok(false),
            Err(error) => Err(error.into()),
        }
    }
}

fn lock_is_contended(error: &std::io::Error) -> bool {
    error.kind() == std::io::ErrorKind::WouldBlock
        // Windows reports ERROR_LOCK_VIOLATION while opening a file whose byte range is locked.
        || (cfg!(windows) && error.raw_os_error() == Some(33))
}

#[cfg(windows)]
fn process_is_alive(pid: u32) -> bool {
    use windows_sys::Win32::Foundation::{CloseHandle, ERROR_INVALID_PARAMETER, GetLastError};
    use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION};

    // SAFETY: `OpenProcess` receives a numeric PID and no inherited handle. Every non-null handle is
    // closed below. An access-denied/unknown failure is treated conservatively as a live process.
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid);
        if handle.is_null() {
            return GetLastError() != ERROR_INVALID_PARAMETER;
        }
        let _ = CloseHandle(handle);
        true
    }
}

#[cfg(unix)]
fn process_is_alive(pid: u32) -> bool {
    if pid > i32::MAX as u32 {
        return false;
    }
    // SAFETY: the invariant is that this declaration matches the platform's real `kill` — a
    // mismatched signature is undefined behaviour at the call site below, not a compile error.
    // POSIX declares `int kill(pid_t, int)`, and both `pid_t` and `int` are `i32` on the two Unix
    // targets this crate ships (`linux-x64` and `osx-arm64` in `release.yml`), so `fn kill(i32,
    // i32) -> i32` is that signature. The `pid > i32::MAX` guard above is what keeps the `as i32`
    // at the call site from wrapping into a negative PID, which `kill` reads as a process *group*.
    // `clippy::undocumented_unsafe_blocks` does not reach an `unsafe extern` block — measured, see
    // `tests/lint_gates.rs` — so this comment is the gate, and
    // `every_unsafe_construct_clippy_misses_is_documented` there is what keeps it present.
    unsafe extern "C" {
        fn kill(pid: i32, signal: i32) -> i32;
    }
    // Signal 0 performs permission/existence checks without delivering a signal. EPERM (1) still
    // proves that the process exists, so only ESRCH and other failures are treated as absent.
    // SAFETY: `kill` with signal 0 performs only an existence/permission probe and does not
    // dereference memory; the PID has been range-checked for the platform `pid_t` representation.
    let result = unsafe { kill(pid as i32, 0) };
    result == 0 || std::io::Error::last_os_error().raw_os_error() == Some(1)
}

impl Drop for ExclusiveControl {
    fn drop(&mut self) {
        use std::io::{Seek, Write};
        remove(&self.instance_dir);
        let _ = self.file.set_len(0);
        let _ = self.file.rewind();
        let _ = self.file.flush();
        let _ = fs2::FileExt::unlock(&self.file);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Pidfile {
    pub app: String,
    pub instance: String,
    pub pid: u32,
    #[serde(rename = "started-at")]
    pub started_at: String,
    #[serde(rename = "run-id")]
    pub run_id: String,
}

pub fn write(pid: &Pidfile, instance_dir: &Path) -> Result<(), AwareError> {
    std::fs::create_dir_all(instance_dir)?;
    let body =
        serde_yaml::to_string(pid).map_err(|e| AwareError::Internal(format!("pidfile: {e}")))?;
    std::fs::write(instance_dir.join("pidfile.yaml"), body)?;
    Ok(())
}

pub fn read(instance_dir: &Path) -> Result<Pidfile, AwareError> {
    let body = std::fs::read_to_string(instance_dir.join("pidfile.yaml"))?;
    serde_yaml::from_str(&body).map_err(|e| AwareError::Validation(format!("pidfile parse: {e}")))
}

pub fn remove(instance_dir: &Path) {
    let _ = std::fs::remove_file(instance_dir.join("pidfile.yaml"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_and_reads_round_trip() {
        let tmp = tempfile::tempdir().unwrap();
        let p = Pidfile {
            app: "welded-to-tc".into(),
            instance: "default".into(),
            pid: 12345,
            started_at: "2026-05-16T14:23:00Z".into(),
            run_id: "r_abc".into(),
        };
        write(&p, tmp.path()).unwrap();
        let back = read(tmp.path()).unwrap();
        assert_eq!(back.app, "welded-to-tc");
        assert_eq!(back.pid, 12345);
    }

    #[test]
    fn remove_is_idempotent() {
        let tmp = tempfile::tempdir().unwrap();
        remove(tmp.path()); // no file — should not panic
        let p = Pidfile {
            app: "x".into(),
            instance: "y".into(),
            pid: 1,
            started_at: "z".into(),
            run_id: "r".into(),
        };
        write(&p, tmp.path()).unwrap();
        remove(tmp.path());
        assert!(!tmp.path().join("pidfile.yaml").exists());
    }

    #[test]
    fn exclusive_control_refuses_a_second_owner_and_releases_on_drop() {
        let tmp = tempfile::tempdir().unwrap();
        let pidfile = Pidfile {
            app: "reader".into(),
            instance: "default".into(),
            pid: 12345,
            started_at: "2026-08-27T00:00:00Z".into(),
            run_id: "r_reader".into(),
        };
        let first = ExclusiveControl::acquire(tmp.path(), &pidfile).unwrap();
        assert!(tmp.path().join("pidfile.yaml").is_file());
        assert!(ExclusiveControl::is_held(tmp.path()).unwrap());
        assert!(ExclusiveControl::acquire(tmp.path(), &pidfile).is_err());
        drop(first);
        assert!(!tmp.path().join("pidfile.yaml").exists());
        assert!(!ExclusiveControl::is_held(tmp.path()).unwrap());
        ExclusiveControl::acquire(tmp.path(), &pidfile).unwrap();
    }

    #[test]
    fn replacement_reader_waits_for_orphan_host_cleanup_fence() {
        use fs2::FileExt;
        use std::sync::mpsc;
        use std::time::Duration;

        let tmp = tempfile::tempdir().unwrap();
        let cleanup = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(ExclusiveControl::host_cleanup_fence_path(tmp.path()))
            .unwrap();
        cleanup.lock_exclusive().unwrap();
        let directory = tmp.path().to_path_buf();
        let (started_tx, started_rx) = mpsc::channel();
        let (finished_tx, finished_rx) = mpsc::channel();
        let join = std::thread::spawn(move || {
            let pidfile = Pidfile {
                app: "reader".into(),
                instance: "default".into(),
                pid: 12345,
                started_at: "2026-08-27T00:00:00Z".into(),
                run_id: "r_replacement".into(),
            };
            started_tx.send(()).unwrap();
            let acquired = ExclusiveControl::acquire(&directory, &pidfile).is_ok();
            finished_tx.send(acquired).unwrap();
        });
        started_rx.recv().unwrap();
        assert!(
            finished_rx
                .recv_timeout(Duration::from_millis(100))
                .is_err()
        );
        cleanup.unlock().unwrap();
        assert!(finished_rx.recv_timeout(Duration::from_secs(2)).unwrap());
        join.join().unwrap();
    }

    #[test]
    fn stale_reclamation_holds_the_control_fence_through_pidfile_removal() {
        let tmp = tempfile::tempdir().unwrap();
        let pidfile = Pidfile {
            app: "reader".into(),
            instance: "default".into(),
            pid: 12345,
            started_at: "2026-08-27T00:00:00Z".into(),
            run_id: "r_reader".into(),
        };
        let live = ExclusiveControl::acquire(tmp.path(), &pidfile).unwrap();
        assert!(!ExclusiveControl::reclaim_stale(tmp.path()).unwrap());
        assert!(tmp.path().join("pidfile.yaml").is_file());
        drop(live);

        // Recreate crash debris: the kernel lock is now unowned but its persisted owner and
        // diagnostic pidfile remain.
        std::fs::write(
            tmp.path().join("model-reader-control.lock"),
            serde_yaml::to_string(&pidfile).unwrap(),
        )
        .unwrap();
        write(&pidfile, tmp.path()).unwrap();
        assert!(ExclusiveControl::reclaim_stale(tmp.path()).unwrap());
        assert!(!tmp.path().join("pidfile.yaml").exists());
    }

    #[test]
    fn stale_reader_control_does_not_remove_a_later_run_pidfile() {
        let tmp = tempfile::tempdir().unwrap();
        let reader = Pidfile {
            app: "reader".into(),
            instance: "default".into(),
            pid: 12345,
            started_at: "2026-08-27T00:00:00Z".into(),
            run_id: "r_reader".into(),
        };
        let later_run = Pidfile {
            app: "reader".into(),
            instance: "default".into(),
            pid: 54321,
            started_at: "2026-08-27T00:01:00Z".into(),
            run_id: "r_later".into(),
        };
        std::fs::write(
            tmp.path().join("model-reader-control.lock"),
            serde_yaml::to_string(&reader).unwrap(),
        )
        .unwrap();
        write(&later_run, tmp.path()).unwrap();

        assert!(!ExclusiveControl::reclaim_stale(tmp.path()).unwrap());
        assert_eq!(read(tmp.path()).unwrap().run_id, "r_later");
    }

    #[test]
    fn legacy_empty_control_never_reclaims_a_live_pidfile() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("model-reader-control.lock"), []).unwrap();
        let live = Pidfile {
            app: "reader".into(),
            instance: "default".into(),
            pid: std::process::id(),
            started_at: "2026-08-27T00:01:00Z".into(),
            run_id: "r_live".into(),
        };
        write(&live, tmp.path()).unwrap();

        assert!(!ExclusiveControl::reclaim_stale(tmp.path()).unwrap());
        assert_eq!(read(tmp.path()).unwrap().run_id, "r_live");
    }
}
