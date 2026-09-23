//! Run-scoped, quota-bounded artifacts for opt-in streaming transports.
//! No descriptor contains a filesystem path and no descriptor may escape its run.

use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::AwareError;
use crate::runtime::provenance::validate_artifact_component;

const BLOCK_BYTES: usize = 64 * 1024;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtifactRef {
    pub schema_version: String,
    pub app: String,
    pub instance: String,
    pub run_id: String,
    pub id: String,
    pub bytes: u64,
    pub sha256: String,
    pub content_type: String,
}

#[derive(Debug, Clone)]
pub struct RunArtifactScope {
    dir: PathBuf,
    app: String,
    instance: String,
    run_id: String,
}

/// One source spool and one bundle may consume each trusted run reservation.
/// A successful or crashed claim remains; ordinary errors release it.
pub struct RunClaim {
    path: PathBuf,
    committed: bool,
}

impl RunClaim {
    pub fn commit(mut self) {
        self.committed = true;
    }
}

impl Drop for RunClaim {
    fn drop(&mut self) {
        if !self.committed {
            let _ = fs::remove_file(&self.path);
        }
    }
}

fn reject_reparse(path: &Path) -> Result<(), AwareError> {
    let meta = fs::symlink_metadata(path)?;
    if meta.file_type().is_symlink() {
        return Err(AwareError::Validation(
            "report artifact path must not be a link".into(),
        ));
    }
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        if meta.file_attributes() & 0x400 != 0 {
            return Err(AwareError::Validation(
                "report artifact path must not be a reparse point".into(),
            ));
        }
    }
    Ok(())
}

impl RunArtifactScope {
    pub fn dir(&self) -> &Path {
        &self.dir
    }

    pub fn claim(&self, kind: &str) -> Result<RunClaim, AwareError> {
        if !matches!(kind, "source" | "render") {
            return Err(AwareError::Validation(
                "invalid report reservation kind".into(),
            ));
        }
        let path = self.dir.join(format!(".report-{kind}.claim"));
        let mut file = fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&path)
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::AlreadyExists {
                    AwareError::Validation(format!(
                        "report {kind} reservation was already used in this run"
                    ))
                } else {
                    AwareError::Io(e)
                }
            })?;
        let claim = RunClaim {
            path,
            committed: false,
        };
        file.write_all(b"claimed")?;
        file.sync_all()?;
        Ok(claim)
    }

    pub fn from_dir(dir: &Path) -> Result<Self, AwareError> {
        reject_reparse(dir)?;
        let name = dir
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| AwareError::Validation("invalid report artifact directory".into()))?;
        let run_id = name.strip_suffix(".artifacts").ok_or_else(|| {
            AwareError::Validation("report artifact directory is not run-owned".into())
        })?;
        let instance_dir = dir
            .parent()
            .ok_or_else(|| AwareError::Validation("missing report instance".into()))?;
        let app_dir = instance_dir
            .parent()
            .ok_or_else(|| AwareError::Validation("missing report app".into()))?;
        reject_reparse(instance_dir)?;
        reject_reparse(app_dir)?;
        let instance = instance_dir
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| AwareError::Validation("invalid report instance".into()))?;
        let app = app_dir
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| AwareError::Validation("invalid report app".into()))?;
        for (label, value) in [("app", app), ("instance", instance), ("run id", run_id)] {
            validate_artifact_component(value, label)?;
        }
        Ok(Self {
            dir: fs::canonicalize(dir)?,
            app: app.into(),
            instance: instance.into(),
            run_id: run_id.into(),
        })
    }

    pub fn write<R: Read>(
        &self,
        source: &mut R,
        budget: u64,
        content_type: &str,
    ) -> Result<ArtifactRef, AwareError> {
        if budget == 0 {
            return Err(AwareError::Validation(
                "report artifact budget must be positive".into(),
            ));
        }
        let mut candidate = tempfile::NamedTempFile::new_in(&self.dir)?;
        let mut hasher = Sha256::new();
        let mut bytes = 0u64;
        let mut block = [0u8; BLOCK_BYTES];
        loop {
            let n = source
                .read(&mut block)
                .map_err(|e| AwareError::Network(format!("report stream read failed: {e}")))?;
            if n == 0 {
                break;
            }
            bytes = bytes
                .checked_add(n as u64)
                .ok_or_else(|| AwareError::Validation("report stream size overflow".into()))?;
            if bytes > budget {
                return Err(AwareError::Validation(
                    "not enough space reserved to complete this report".into(),
                ));
            }
            candidate.write_all(&block[..n])?;
            hasher.update(&block[..n]);
        }
        self.publish(
            candidate,
            bytes,
            format!("{:x}", hasher.finalize()),
            content_type,
        )
    }

    pub fn publish(
        &self,
        candidate: tempfile::NamedTempFile,
        bytes: u64,
        sha256: String,
        content_type: &str,
    ) -> Result<ArtifactRef, AwareError> {
        candidate.as_file().sync_all()?;
        let id = format!("report-{}", uuid::Uuid::new_v4());
        validate_artifact_component(&id, "artifact id")?;
        candidate
            .persist_noclobber(self.dir.join(&id))
            .map_err(|e| AwareError::Io(e.error))?;
        Ok(ArtifactRef {
            schema_version: "aware.artifact-ref/v1".into(),
            app: self.app.clone(),
            instance: self.instance.clone(),
            run_id: self.run_id.clone(),
            id,
            bytes,
            sha256,
            content_type: content_type.into(),
        })
    }

    pub fn open_verified(
        &self,
        reference: &ArtifactRef,
        expected_type: &str,
    ) -> Result<File, AwareError> {
        if reference.schema_version != "aware.artifact-ref/v1"
            || reference.app != self.app
            || reference.instance != self.instance
            || reference.run_id != self.run_id
            || reference.content_type != expected_type
            || reference.sha256.len() != 64
            || !reference.sha256.bytes().all(|b| b.is_ascii_hexdigit())
        {
            return Err(AwareError::Validation(
                "report artifact does not belong to this run".into(),
            ));
        }
        validate_artifact_component(&reference.id, "artifact id")?;
        let path = self.dir.join(&reference.id);
        reject_reparse(&path)?;
        let mut file = File::open(path)?;
        if file.metadata()?.len() != reference.bytes {
            return Err(AwareError::Validation(
                "report artifact size changed".into(),
            ));
        }
        let mut hash = Sha256::new();
        let mut block = [0u8; BLOCK_BYTES];
        loop {
            let n = file.read(&mut block)?;
            if n == 0 {
                break;
            }
            hash.update(&block[..n]);
        }
        if format!("{:x}", hash.finalize()) != reference.sha256.to_ascii_lowercase() {
            return Err(AwareError::Validation(
                "report artifact digest changed".into(),
            ));
        }
        file.seek(SeekFrom::Start(0))?;
        Ok(file)
    }
}

pub fn budget_from_env(name: &str) -> Result<u64, AwareError> {
    let value = std::env::var(name)
        .map_err(|_| AwareError::Validation(format!("report reservation is missing {name}")))?;
    if value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit()) {
        return Err(AwareError::Validation(format!(
            "report reservation has invalid {name}"
        )));
    }
    value
        .parse::<u64>()
        .ok()
        .filter(|n| *n > 0)
        .ok_or_else(|| AwareError::Validation(format!("report reservation has invalid {name}")))
}

pub fn require_reservation() -> Result<(), AwareError> {
    let value = std::env::var("AWARE_REPORT_RESERVATION_ID")
        .map_err(|_| AwareError::Validation("report reservation is missing".into()))?;
    validate_artifact_component(&value, "reservation id")
}

/// The opt-in REST success path consumes bounded `Read` chunks rather than
/// materializing a model-sized JSON value.
pub fn spool_rest_response(
    response: ureq::Response,
    scope: &RunArtifactScope,
    source_budget: u64,
) -> Result<serde_json::Value, AwareError> {
    let content_type = response
        .header("content-type")
        .unwrap_or("")
        .to_ascii_lowercase();
    if !content_type.starts_with("application/x-ndjson") {
        return Err(AwareError::Validation(
            "report source did not return the expected record stream".into(),
        ));
    }
    let claim = scope.claim("source")?;
    let mut reader = response.into_reader();
    let reference = scope.write(&mut reader, source_budget, "application/x-ndjson")?;
    claim.commit();
    Ok(serde_json::json!({
        "status": 200,
        "headers": {"content-type": "application/x-ndjson"},
        "body": {"artifact": reference},
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Repeating {
        remaining: usize,
    }
    impl Read for Repeating {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            let n = buf.len().min(self.remaining);
            buf[..n].fill(b'x');
            self.remaining -= n;
            Ok(n)
        }
    }

    #[test]
    fn rest_body_spools_without_inline_payload_and_honors_budget() {
        let temp = tempfile::tempdir().expect("temp");
        let dir = temp
            .path()
            .join("app")
            .join("instance")
            .join("run-1.artifacts");
        fs::create_dir_all(&dir).expect("directory");
        let scope = RunArtifactScope::from_dir(&dir).expect("scope");
        let server = tiny_http::Server::http("127.0.0.1:0").expect("server");
        let url = format!("http://{}/report", server.server_addr());
        let responder = std::thread::spawn(move || {
            let request = server.recv().expect("request");
            let header = tiny_http::Header::from_bytes(b"content-type", b"application/x-ndjson")
                .expect("header");
            let response = tiny_http::Response::new(
                tiny_http::StatusCode(200),
                vec![header],
                Repeating {
                    remaining: 4_000_000,
                },
                Some(4_000_000),
                None,
            );
            request.respond(response).expect("respond");
        });
        let response = ureq::get(&url).call().expect("GET");
        let envelope = spool_rest_response(response, &scope, 4_000_000).expect("spool");
        responder.join().expect("server join");
        assert_eq!(envelope["status"], 200);
        assert_eq!(envelope["body"]["artifact"]["bytes"], 4_000_000);
        assert!(envelope.to_string().len() < 600);
        let reference: ArtifactRef =
            serde_json::from_value(envelope["body"]["artifact"].clone()).expect("ref");
        assert_eq!(
            scope
                .open_verified(&reference, "application/x-ndjson")
                .expect("file")
                .metadata()
                .expect("meta")
                .len(),
            4_000_000
        );
        assert!(
            scope
                .write(
                    &mut Repeating { remaining: 65_537 },
                    65_536,
                    "application/x-ndjson"
                )
                .is_err()
        );
        assert_eq!(fs::read_dir(scope.dir()).expect("entries").count(), 2);
        assert!(
            scope.claim("source").is_err(),
            "second source cannot reuse one run budget"
        );
        let render_claim = scope.claim("render").expect("independent render partition");
        drop(render_claim);
        assert!(
            scope.claim("render").is_ok(),
            "failed render releases its claim"
        );
    }
}
