//! Durable ownership record for a report reservation when a launcher loses the
//! normal run-start announcement. This records identity, never report content.

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::AwareError;
use crate::runtime::provenance::validate_artifact_component;

const RESERVATION_ENV: &str = "AWARE_REPORT_RESERVATION_ID";
const SCHEMA: &str = "aware.report-reservation/v1";
const MAX_MARKER_BYTES: u64 = 2048;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ArtifactScope {
    pub app: String,
    pub instance: String,
    pub run_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ReservationOwner {
    pub schema_version: String,
    pub reservation_id: String,
    pub app: String,
    pub instance: String,
    pub run_id: String,
    pub artifact_scope: ArtifactScope,
}

fn marker_dir(logs_dir: &Path) -> PathBuf {
    logs_dir.join(".report-reservations")
}

fn marker_path(logs_dir: &Path, reservation_id: &str) -> Result<PathBuf, AwareError> {
    validate_artifact_component(reservation_id, "reservation id")?;
    Ok(marker_dir(logs_dir).join(format!("{reservation_id}.json")))
}

#[cfg(unix)]
fn sync_directory(path: &Path) -> Result<(), AwareError> {
    File::open(path)?.sync_all()?;
    Ok(())
}

#[cfg(windows)]
fn sync_directory(path: &Path) -> Result<(), AwareError> {
    use std::os::windows::fs::OpenOptionsExt;
    use windows_sys::Win32::Storage::FileSystem::FILE_FLAG_BACKUP_SEMANTICS;

    let directory = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
        .open(path)?;
    directory.sync_all()?;
    Ok(())
}

fn require_plain_directory(path: &Path) -> Result<(), AwareError> {
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(AwareError::Validation(
            "report reservation directory is not a plain directory".into(),
        ));
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        if metadata.file_attributes() & 0x400 != 0 {
            return Err(AwareError::Validation(
                "report reservation directory is a reparse point".into(),
            ));
        }
    }
    Ok(())
}

fn require_plain_file(path: &Path) -> Result<(), AwareError> {
    let metadata = fs::symlink_metadata(path)?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(AwareError::Validation(
            "report reservation marker is not a plain file".into(),
        ));
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        if metadata.file_attributes() & 0x400 != 0 {
            return Err(AwareError::Validation(
                "report reservation marker is a reparse point".into(),
            ));
        }
    }
    if metadata.len() > MAX_MARKER_BYTES {
        return Err(AwareError::Validation(
            "report reservation marker is too large".into(),
        ));
    }
    Ok(())
}

pub fn record_if_reserved(
    logs_dir: &Path,
    app: &str,
    instance: &str,
    run_id: &str,
) -> Result<(), AwareError> {
    let Some(reservation_id) = std::env::var_os(RESERVATION_ENV) else {
        return Ok(());
    };
    let reservation_id = reservation_id
        .to_str()
        .ok_or_else(|| AwareError::Validation("report reservation ID must be plain text".into()))?;
    let path = marker_path(logs_dir, reservation_id)?;
    for (label, value) in [("app", app), ("instance", instance), ("run id", run_id)] {
        validate_artifact_component(value, label)?;
    }
    fs::create_dir_all(logs_dir)?;
    require_plain_directory(logs_dir)?;
    fs::create_dir_all(marker_dir(logs_dir))?;
    require_plain_directory(&marker_dir(logs_dir))?;
    let owner = ReservationOwner {
        schema_version: SCHEMA.into(),
        reservation_id: reservation_id.into(),
        app: app.into(),
        instance: instance.into(),
        run_id: run_id.into(),
        artifact_scope: ArtifactScope {
            app: app.into(),
            instance: instance.into(),
            run_id: run_id.into(),
        },
    };
    let data = serde_json::to_vec(&owner)?;
    if data.len() as u64 > MAX_MARKER_BYTES {
        return Err(AwareError::Validation(
            "report reservation marker is too large".into(),
        ));
    }
    publish(&marker_dir(logs_dir), &path, &data, || {})?;
    sync_directory(&marker_dir(logs_dir))?;
    sync_directory(logs_dir)?;
    if let Some(home) = logs_dir.parent() {
        sync_directory(home)?;
    }
    Ok(())
}

fn publish(
    directory: &Path,
    destination: &Path,
    data: &[u8],
    before_publish: impl FnOnce(),
) -> Result<(), AwareError> {
    let mut candidate = tempfile::NamedTempFile::new_in(directory)?;
    candidate.write_all(data)?;
    candidate.as_file().sync_all()?;
    before_publish();
    // `persist_noclobber` uses a no-replace publication, so inspection sees
    // either this complete synced record or no record at all. A crash before
    // publication leaves only a nameless-to-callers temporary candidate.
    candidate.persist_noclobber(destination).map_err(|error| {
        if error.error.kind() == std::io::ErrorKind::AlreadyExists {
            AwareError::Conflict("report reservation was already used".into())
        } else {
            AwareError::Io(error.error)
        }
    })?;
    Ok(())
}

pub fn inspect(logs_dir: &Path, reservation_id: &str) -> Result<ReservationOwner, AwareError> {
    let path = marker_path(logs_dir, reservation_id)?;
    match fs::symlink_metadata(&path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Err(AwareError::NotFound(format!(
                "report reservation {reservation_id}"
            )));
        }
        Err(error) => return Err(AwareError::Io(error)),
        Ok(_) => {}
    }
    require_plain_directory(logs_dir)?;
    require_plain_directory(&marker_dir(logs_dir))?;
    require_plain_file(&path)?;
    let file = File::open(path)?;
    let mut data = Vec::new();
    file.take(MAX_MARKER_BYTES + 1).read_to_end(&mut data)?;
    if data.len() as u64 > MAX_MARKER_BYTES {
        return Err(AwareError::Validation(
            "report reservation marker is too large".into(),
        ));
    }
    let owner: ReservationOwner = serde_json::from_slice(&data)
        .map_err(|_| AwareError::Validation("report reservation marker is invalid".into()))?;
    if owner.schema_version != SCHEMA
        || owner.reservation_id != reservation_id
        || owner.artifact_scope.app != owner.app
        || owner.artifact_scope.instance != owner.instance
        || owner.artifact_scope.run_id != owner.run_id
    {
        return Err(AwareError::Validation(
            "report reservation marker has mismatched ownership".into(),
        ));
    }
    for (label, value) in [
        ("app", owner.app.as_str()),
        ("instance", owner.instance.as_str()),
        ("run id", owner.run_id.as_str()),
    ] {
        validate_artifact_component(value, label)?;
    }
    Ok(owner)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn directory_sync_is_supported_for_reservation_publication() {
        let temp = tempfile::tempdir().unwrap();
        sync_directory(temp.path()).unwrap();
    }

    #[test]
    fn inspection_never_sees_a_candidate_before_atomic_publication() {
        let temp = tempfile::tempdir().unwrap();
        let logs = temp.path().join("logs");
        let directory = marker_dir(&logs);
        fs::create_dir_all(&directory).unwrap();
        let destination = marker_path(&logs, "reservation-1").unwrap();
        publish(&directory, &destination, b"complete", || {
            let error = inspect(&logs, "reservation-1").unwrap_err();
            assert!(matches!(error, AwareError::NotFound(_)));
        })
        .unwrap();
        assert_eq!(fs::read(destination).unwrap(), b"complete");
    }
}
