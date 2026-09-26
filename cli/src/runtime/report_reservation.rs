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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub writer_lease: Option<LeaseEvidence>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LeaseEvidence {
    pub file_identity: String,
    pub artifact_directory_identity: String,
    pub writer_class: crate::runtime::artifact_retention::WriterClass,
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
    writer_lease: Option<&LeaseEvidence>,
) -> Result<(), AwareError> {
    let Some(reservation_id) = std::env::var_os(RESERVATION_ENV) else {
        return Ok(());
    };
    let reservation_id = reservation_id
        .to_str()
        .ok_or_else(|| AwareError::Validation("report reservation ID must be plain text".into()))?;
    let writer_lease = writer_lease.ok_or_else(|| {
        AwareError::Validation("report reservation requires a writer lease".into())
    })?;
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
        writer_lease: Some(writer_lease.clone()),
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
    if owner.writer_lease.as_ref().is_some_and(|lease| {
        lease.file_identity.is_empty() || lease.artifact_directory_identity.is_empty()
    }) {
        return Err(AwareError::Validation(
            "report writer lease identity is empty".into(),
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
    use crate::runtime::artifact_retention::WriterClass;
    use crate::test_env::EnvVarGuard;

    /// A marker exactly as a correct run publishes it, written as literal JSON
    /// rather than by serializing `ReservationOwner` — so these tests pin the
    /// on-disk wire format instead of agreeing with whatever the struct
    /// currently emits. Every rejection test that attacks the marker *body*
    /// changes exactly one logical identity value, and the control test proves
    /// the rest of it inspects cleanly, so those rejections can only come from
    /// the value under test. One value is serialized twice — `app` and
    /// `artifactScope.app` — and the path-safety test changes both, because
    /// changing either alone would be rejected by the scope-agreement guard
    /// first and the test would pass on the wrong rung. The size, symlink and
    /// traversal tests attack the file or the path rather than the body.
    fn well_formed(reservation_id: &str) -> serde_json::Value {
        serde_json::json!({
            // The literal persisted value, deliberately NOT the `SCHEMA`
            // constant: using the constant would make this fixture track
            // production, so renaming the schema would keep every test here
            // green while every already-written v1 marker became unreadable.
            "schemaVersion": "aware.report-reservation/v1",
            "reservationId": reservation_id,
            "app": "report-app",
            "instance": "default",
            "runId": "run-1",
            "artifactScope": {
                "app": "report-app",
                "instance": "default",
                "runId": "run-1",
            },
            "writerLease": {
                "fileIdentity": "unix:1:2",
                "artifactDirectoryIdentity": "unix:1:3",
                "writerClass": "in-process",
            },
        })
    }

    fn logs_with_marker(reservation_id: &str, body: &str) -> (tempfile::TempDir, PathBuf) {
        let temp = tempfile::tempdir().unwrap();
        let logs = temp.path().join("logs");
        fs::create_dir_all(marker_dir(&logs)).unwrap();
        fs::write(
            marker_dir(&logs).join(format!("{reservation_id}.json")),
            body,
        )
        .unwrap();
        (temp, logs)
    }

    fn expect_rejected(reservation_id: &str, body: &str, because: &str) {
        let (_temp, logs) = logs_with_marker(reservation_id, body);
        let error = inspect(&logs, reservation_id).unwrap_err();
        assert!(
            matches!(error, AwareError::Validation(_)),
            "{because}: expected a validation refusal, got {error:?}"
        );
    }

    /// The control for every rejection below. Without it a negative test could
    /// pass because the fixture is malformed in some unrelated way.
    #[test]
    fn a_well_formed_marker_inspects_to_its_recorded_owner() {
        let (_temp, logs) = logs_with_marker("res-1", &well_formed("res-1").to_string());
        let owner = inspect(&logs, "res-1").unwrap();
        assert_eq!(owner.app, "report-app");
        assert_eq!(owner.run_id, "run-1");
        assert_eq!(
            owner.writer_lease.unwrap().writer_class,
            WriterClass::InProcess
        );
    }

    /// `prune` never reads `artifact_scope`: it re-checks the caller's run
    /// against `owner.{app,instance,run_id}` and binds to the lease's inode
    /// identities. The scope's consumer is the launcher —
    /// `aware app artifact-reservation` prints this whole record, and
    /// `10-core/cli-spec.md` has that caller act on the scope to locate the
    /// run. So a marker whose scope disagrees with its own identity misdirects
    /// that caller, which is why the two must agree.
    #[test]
    fn inspect_rejects_a_marker_whose_scope_disagrees_with_its_own_identity() {
        for field in ["app", "instance", "runId"] {
            let mut marker = well_formed("res-1");
            marker["artifactScope"][field] = serde_json::json!("elsewhere");
            expect_rejected(
                "res-1",
                &marker.to_string(),
                &format!("artifactScope.{field} disagreeing with the owner"),
            );
        }
    }

    /// The reservation id is the filename *and* a field. Accepting a marker
    /// whose body names a different id would let a marker be copied to a second
    /// filename and replayed to claim a run it never owned.
    #[test]
    fn inspect_rejects_a_marker_replayed_under_another_reservation_id() {
        expect_rejected(
            "res-2",
            &well_formed("res-1").to_string(),
            "a body naming a different reservation id than its filename",
        );
    }

    #[test]
    fn inspect_rejects_a_marker_written_against_a_foreign_schema() {
        let mut marker = well_formed("res-1");
        marker["schemaVersion"] = serde_json::json!("aware.report-reservation/v99");
        expect_rejected("res-1", &marker.to_string(), "an unknown schema version");
    }

    /// The marker is read into memory before it is parsed, so a size cap is
    /// what stops a planted file from being read without bound. The cap sits at
    /// two rungs — a stat pre-check and a bounded read — and a file this size
    /// trips the first, so this pins that a cap exists rather than which rung
    /// applied it.
    #[test]
    fn inspect_rejects_a_marker_larger_than_the_cap() {
        let mut body = well_formed("res-1").to_string();
        body.push_str(&" ".repeat(usize::try_from(MAX_MARKER_BYTES).unwrap() + 1 - body.len()));
        assert!(body.len() as u64 > MAX_MARKER_BYTES);
        expect_rejected("res-1", &body, "a marker past the size cap");
    }

    /// `prune` compares live filesystem identities against these strings. An
    /// empty one compares equal to nothing real, but recording it would mean a
    /// marker that claims a lease it cannot evidence.
    #[test]
    fn inspect_rejects_a_writer_lease_with_an_empty_identity() {
        for field in ["fileIdentity", "artifactDirectoryIdentity"] {
            let mut marker = well_formed("res-1");
            marker["writerLease"][field] = serde_json::json!("");
            expect_rejected(
                "res-1",
                &marker.to_string(),
                &format!("an empty writerLease.{field}"),
            );
        }
    }

    #[test]
    fn inspect_rejects_a_marker_holding_a_path_unsafe_component() {
        let mut marker = well_formed("res-1");
        marker["app"] = serde_json::json!("../escape");
        marker["artifactScope"]["app"] = serde_json::json!("../escape");
        expect_rejected(
            "res-1",
            &marker.to_string(),
            "an app name that is not one path-safe identifier",
        );
    }

    /// Reading through a symlink would let anything that can write inside the
    /// reservation directory point a marker at a file elsewhere on disk.
    #[cfg(unix)]
    #[test]
    fn inspect_refuses_to_follow_a_symlinked_marker() {
        let temp = tempfile::tempdir().unwrap();
        let logs = temp.path().join("logs");
        fs::create_dir_all(marker_dir(&logs)).unwrap();
        let real = temp.path().join("planted.json");
        fs::write(&real, well_formed("res-1").to_string()).unwrap();
        std::os::unix::fs::symlink(&real, marker_dir(&logs).join("res-1.json")).unwrap();
        let error = inspect(&logs, "res-1").unwrap_err();
        assert!(
            matches!(error, AwareError::Validation(_)),
            "a symlinked marker must be refused, got {error:?}"
        );
    }

    /// The id reaches the filesystem as a filename, so it is validated before
    /// any path is built rather than after a lookup misses.
    #[test]
    fn a_traversing_reservation_id_is_refused_before_any_path_is_built() {
        let temp = tempfile::tempdir().unwrap();
        let logs = temp.path().join("logs");
        fs::create_dir_all(marker_dir(&logs)).unwrap();
        let error = inspect(&logs, "../escape").unwrap_err();
        assert!(
            matches!(error, AwareError::Validation(_)),
            "a traversing reservation id must be a validation refusal, not a              lookup miss, got {error:?}"
        );
    }

    fn lease() -> LeaseEvidence {
        LeaseEvidence {
            file_identity: "unix:1:2".into(),
            artifact_directory_identity: "unix:1:3".into(),
            writer_class: WriterClass::InProcess,
        }
    }

    /// One reservation authorizes retiring one run's artifacts. A second claim
    /// on the same id would not gain the first run's files — `prune`'s
    /// owner-equality check still refuses that — it would silently *transfer*
    /// the reservation: the launcher holding this id is handed the second run's
    /// identity, and the first run's artifacts are left with no reservation
    /// that can ever retire them.
    #[test]
    fn a_reservation_id_cannot_be_claimed_twice() {
        let temp = tempfile::tempdir().unwrap();
        let logs = temp.path().join("logs");
        let _env = EnvVarGuard::set(RESERVATION_ENV, "res-1");
        record_if_reserved(&logs, "report-app", "default", "run-1", Some(&lease())).unwrap();
        let error = record_if_reserved(&logs, "report-app", "default", "run-2", Some(&lease()))
            .unwrap_err();
        assert!(
            matches!(error, AwareError::Conflict(_)),
            "a second claim on one reservation id must conflict, got {error:?}"
        );
        assert_eq!(
            inspect(&logs, "res-1").unwrap().run_id,
            "run-1",
            "the first claim must remain the owner of record"
        );
    }

    /// A marker without lease evidence is one `prune` can never act on, and
    /// writing it would burn the reservation id for nothing.
    #[test]
    fn record_if_reserved_refuses_to_publish_without_writer_evidence() {
        let temp = tempfile::tempdir().unwrap();
        let logs = temp.path().join("logs");
        let _env = EnvVarGuard::set(RESERVATION_ENV, "res-1");
        let error = record_if_reserved(&logs, "report-app", "default", "run-1", None).unwrap_err();
        assert!(matches!(error, AwareError::Validation(_)), "got {error:?}");
        assert!(
            !marker_dir(&logs).join("res-1.json").exists(),
            "a refused reservation must not leave a marker behind"
        );
    }

    /// Unreserved runs are the common case; they must not create reservation
    /// state that a later `prune` could find.
    #[test]
    fn record_if_reserved_writes_nothing_without_a_reservation_in_the_environment() {
        let temp = tempfile::tempdir().unwrap();
        let logs = temp.path().join("logs");
        let _env = EnvVarGuard::scope(&[(RESERVATION_ENV, None)]);
        record_if_reserved(&logs, "report-app", "default", "run-1", Some(&lease())).unwrap();
        assert!(
            !marker_dir(&logs).exists(),
            "an unreserved run must not create the reservation directory"
        );
    }

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
