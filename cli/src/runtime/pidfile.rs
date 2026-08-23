//! Pidfile under `<instance_dir>/pidfile.yaml` for `aware app stop`.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::AwareError;

/// Kernel-backed, crash-released single-run fence used only by app graphs that contain the exact
/// `model-reference-reader` agent. Ordinary one-shot apps never acquire it.
pub struct ExclusiveControl {
    file: std::fs::File,
}

impl ExclusiveControl {
    pub fn acquire(instance_dir: &Path) -> Result<Self, AwareError> {
        use fs2::FileExt;
        std::fs::create_dir_all(instance_dir)?;
        let file = std::fs::OpenOptions::new()
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
        Ok(Self { file })
    }
}

impl Drop for ExclusiveControl {
    fn drop(&mut self) {
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
        let first = ExclusiveControl::acquire(tmp.path()).unwrap();
        assert!(ExclusiveControl::acquire(tmp.path()).is_err());
        drop(first);
        ExclusiveControl::acquire(tmp.path()).unwrap();
    }
}
