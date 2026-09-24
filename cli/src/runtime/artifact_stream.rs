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
    use crate::test_env::EnvVarGuard;

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

    const NDJSON: &str = "application/x-ndjson";

    /// One named way to tamper with an otherwise-valid descriptor.
    type Tamper = (&'static str, fn(&mut ArtifactRef));

    /// Build `<root>/<app>/<instance>/<run>.artifacts` and scope it.
    ///
    /// The three-deep layout is not decoration: `from_dir` reads the app and
    /// the instance out of the two parent directory names, so a flatter
    /// fixture would exercise a different function than production does.
    fn scope_at(root: &Path, app: &str, instance: &str, run: &str) -> RunArtifactScope {
        let dir = root
            .join(app)
            .join(instance)
            .join(format!("{run}.artifacts"));
        fs::create_dir_all(&dir).expect("fixture directory");
        RunArtifactScope::from_dir(&dir).expect("scope")
    }

    fn publish_payload(scope: &RunArtifactScope, payload: &[u8]) -> ArtifactRef {
        scope
            .write(&mut std::io::Cursor::new(payload.to_vec()), 1 << 20, NDJSON)
            .expect("write")
    }

    fn sha256_hex(payload: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(payload);
        format!("{:x}", hasher.finalize())
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

    /// The module's headline invariant — "no descriptor may escape its run".
    ///
    /// The bytes are made to line up on purpose: same content, same digest,
    /// same id, present in *both* runs' directories. That is what leaves the
    /// identity carried in the descriptor as the only thing able to refuse the
    /// open. Without the copy the test would pass on `File::open` returning
    /// NotFound and would stay green with every identity comparison deleted.
    #[test]
    fn a_descriptor_cannot_be_replayed_against_another_run() {
        let temp = tempfile::tempdir().expect("temp");
        let issuing = scope_at(temp.path(), "app", "instance", "run-a");
        let other = scope_at(temp.path(), "app", "instance", "run-b");
        let reference = publish_payload(&issuing, b"{\"row\":1}\n");

        fs::copy(
            issuing.dir().join(&reference.id),
            other.dir().join(&reference.id),
        )
        .expect("the same bytes under the same id in the other run");

        assert!(
            issuing.open_verified(&reference, NDJSON).is_ok(),
            "control: the descriptor opens in the run that issued it"
        );
        assert!(
            other.open_verified(&reference, NDJSON).is_err(),
            "a descriptor stamped run-a must not open against run-b even when \
             the bytes it names are sitting right there"
        );
    }

    /// Each clause of `open_verified`'s guard, one at a time, against a
    /// descriptor that is otherwise valid and whose file is really on disk.
    ///
    /// The control assertion is load-bearing: without it, a guard that refused
    /// *everything* would satisfy every case below.
    ///
    /// So is asserting on which refusal came back rather than merely that one
    /// did. The two digest-shape clauses are a syntactic pre-filter in front of
    /// a hash comparison that would reject the same descriptors anyway, so
    /// `is_err()` alone stays green with either of them deleted — it passes on
    /// "digest changed" instead, having read the whole file to find out. Naming
    /// the expected refusal is what distinguishes the guard from the fallback.
    #[test]
    fn every_identity_field_of_a_descriptor_is_checked() {
        let temp = tempfile::tempdir().expect("temp");
        let scope = scope_at(temp.path(), "app", "instance", "run-1");
        let valid = publish_payload(&scope, b"{\"row\":1}\n");

        assert!(
            scope.open_verified(&valid, NDJSON).is_ok(),
            "control: an untouched descriptor must open, or every case below \
             passes for the wrong reason"
        );

        let mutations: [Tamper; 7] = [
            ("schema version", |r| {
                r.schema_version = "aware.artifact-ref/v2".into()
            }),
            ("app", |r| r.app = "other-app".into()),
            ("instance", |r| r.instance = "other-instance".into()),
            ("run id", |r| r.run_id = "other-run".into()),
            ("content type", |r| {
                r.content_type = "application/json".into()
            }),
            ("digest length", |r| r.sha256.truncate(63)),
            // Still 64 characters, so only the alphabet sweep refuses it.
            ("digest alphabet", |r| r.sha256.replace_range(63.., "z")),
        ];

        for (label, mutate) in mutations {
            let mut tampered = valid.clone();
            mutate(&mut tampered);
            let refusal = scope
                .open_verified(&tampered, NDJSON)
                .expect_err(&format!("a changed {label} must be refused"))
                .to_string();
            assert!(
                refusal.contains("report artifact does not belong to this run"),
                "a descriptor with a changed {label} must be refused by the \
                 identity guard, not by a later check: {refusal}"
            );
        }

        let mut resized = valid.clone();
        resized.bytes += 1;
        let refusal = scope
            .open_verified(&resized, NDJSON)
            .expect_err("a byte count that disagrees with the file must be refused")
            .to_string();
        assert!(
            refusal.contains("report artifact size changed"),
            "a descriptor whose byte count disagrees with the file must be \
             refused before its contents are hashed: {refusal}"
        );
    }

    /// The digest comparison lowercases the descriptor's side, so an uppercase
    /// hex digest is a valid spelling of the same hash. Dropping that
    /// normalisation turns every uppercase descriptor into a false tamper
    /// alarm; dropping the comparison lets a well-formed wrong hash through.
    #[test]
    fn the_digest_is_compared_case_insensitively_but_not_loosely() {
        let temp = tempfile::tempdir().expect("temp");
        let scope = scope_at(temp.path(), "app", "instance", "run-1");
        let valid = publish_payload(&scope, b"{\"row\":1}\n");

        let mut upper = valid.clone();
        upper.sha256 = valid.sha256.to_ascii_uppercase();
        assert_ne!(
            upper.sha256, valid.sha256,
            "the fixture must differ in case"
        );
        assert!(
            scope.open_verified(&upper, NDJSON).is_ok(),
            "an uppercase hex digest names the same hash"
        );

        let mut wrong = valid.clone();
        let swap = if valid.sha256.starts_with('a') {
            "b"
        } else {
            "a"
        };
        wrong.sha256.replace_range(..1, swap);
        assert_ne!(wrong.sha256, valid.sha256);
        assert!(
            scope.open_verified(&wrong, NDJSON).is_err(),
            "a well-formed digest that is not this file's must be refused"
        );
    }

    /// `open_verified` joins the descriptor's id onto the run directory, so the
    /// id has to be a leaf name rather than a path.
    ///
    /// The escape target is planted with matching size *and* matching digest,
    /// which is what makes this go red if the `validate_artifact_component`
    /// call is removed: the join would then resolve, the file would open, and
    /// both integrity checks would agree with the descriptor.
    #[test]
    fn an_artifact_id_cannot_climb_out_of_the_run_directory() {
        let temp = tempfile::tempdir().expect("temp");
        let scope = scope_at(temp.path(), "app", "instance", "run-1");
        let payload = b"another run's rows\n";
        let outside = scope
            .dir()
            .parent()
            .expect("instance directory")
            .join("planted");
        fs::write(&outside, payload).expect("plant");

        let escaping = ArtifactRef {
            schema_version: "aware.artifact-ref/v1".into(),
            app: "app".into(),
            instance: "instance".into(),
            run_id: "run-1".into(),
            id: "../planted".into(),
            bytes: payload.len() as u64,
            sha256: sha256_hex(payload),
            content_type: NDJSON.into(),
        };

        assert!(
            File::open(scope.dir().join(&escaping.id)).is_ok(),
            "control: the escape target is reachable by a plain join, so the \
             refusal below is the id check rather than a missing file"
        );
        assert!(
            scope.open_verified(&escaping, NDJSON).is_err(),
            "an artifact id holding a path separator must be refused"
        );
    }

    /// Publication is not the end of the trust chain: the file sits on a
    /// filesystem the rest of the machine can reach, so `open_verified`
    /// re-reads it every time. Both halves matter — a swap that keeps the
    /// length is invisible to the size check, and a truncation is caught
    /// before the hash is ever computed.
    #[test]
    fn content_edited_after_publication_is_detected() {
        let temp = tempfile::tempdir().expect("temp");
        let scope = scope_at(temp.path(), "app", "instance", "run-1");
        let reference = publish_payload(&scope, b"{\"row\":1}\n");
        let path = scope.dir().join(&reference.id);

        fs::write(&path, b"{\"row\":2}\n").expect("same-length edit");
        assert_eq!(
            fs::metadata(&path).expect("meta").len(),
            reference.bytes,
            "the edit must keep the size, or this tests the size check instead"
        );
        assert!(
            scope.open_verified(&reference, NDJSON).is_err(),
            "a same-length content swap must be caught by the digest"
        );

        fs::write(&path, b"{\"row\":1}").expect("truncating edit");
        assert!(
            scope.open_verified(&reference, NDJSON).is_err(),
            "a file whose length no longer matches the descriptor must be refused"
        );
    }

    /// `bytes > budget` makes the reservation an inclusive ceiling: a stream of
    /// exactly the reserved size is the largest legal one. The over-budget case
    /// passes under `>=` as well, so only the exact-fit half pins the
    /// comparison down.
    #[test]
    fn the_budget_admits_exactly_its_own_size_and_nothing_more() {
        let temp = tempfile::tempdir().expect("temp");
        let scope = scope_at(temp.path(), "app", "instance", "run-1");

        let exact = scope
            .write(&mut Repeating { remaining: 4096 }, 4096, NDJSON)
            .expect("a stream of exactly the reserved size fits");
        assert_eq!(exact.bytes, 4096);

        let published = fs::read_dir(scope.dir()).expect("entries").count();
        assert!(
            scope
                .write(&mut Repeating { remaining: 4097 }, 4096, NDJSON)
                .is_err(),
            "one byte past the reservation must be refused"
        );
        assert_eq!(
            fs::read_dir(scope.dir()).expect("entries").count(),
            published,
            "an over-budget stream must not leave its partial spool behind"
        );

        assert!(
            scope
                .write(&mut Repeating { remaining: 0 }, 0, NDJSON)
                .is_err(),
            "a zero reservation is not a reservation, even for an empty stream"
        );
    }

    /// `budget_from_env` decides how much a run may spool, so "`parse` accepts
    /// it" is not the bar. Two cases carry most of the weight: `+12` parses as
    /// 12 under `u64::from_str` and is refused only by the explicit digit
    /// sweep, and a value one past `u64::MAX` is all digits and is refused only
    /// by the parse that follows it.
    #[test]
    fn a_budget_read_from_the_environment_must_be_a_plain_positive_integer() {
        const KEY: &str = "AWARE_TEST_ARTIFACT_STREAM_BUDGET";

        {
            let _absent = EnvVarGuard::scope(&[(KEY, None)]);
            assert!(
                budget_from_env(KEY).is_err(),
                "a missing reservation is not a budget"
            );
        }

        let mut guard = EnvVarGuard::set(KEY, "1");
        assert_eq!(budget_from_env(KEY).expect("one byte"), 1);
        guard.replace("18446744073709551615");
        assert_eq!(budget_from_env(KEY).expect("u64::MAX"), u64::MAX);

        for rejected in [
            "",                     // present but empty
            "0",                    // a reservation of nothing
            "-1",                   // negative
            "+12",                  // parses as 12; only the digit sweep refuses it
            " 12",                  // leading space
            "12 ",                  // trailing space
            "12\n",                 // the trailing newline a shell `echo` leaves
            "0x10",                 // hex
            "1e6",                  // scientific
            "18446744073709551616", // all digits, one past u64::MAX
        ] {
            guard.replace(rejected);
            assert!(
                budget_from_env(KEY).is_err(),
                "{rejected:?} must not be accepted as a byte budget"
            );
        }
    }

    /// The reservation id reaches `validate_artifact_component`, because it is
    /// joined into paths elsewhere in the run. So the check is on its shape,
    /// not merely on its presence.
    #[test]
    fn a_reservation_id_must_be_present_and_path_safe() {
        const KEY: &str = "AWARE_REPORT_RESERVATION_ID";

        {
            let _absent = EnvVarGuard::scope(&[(KEY, None)]);
            assert!(
                require_reservation().is_err(),
                "an unset variable is not a reservation"
            );
        }

        let mut guard = EnvVarGuard::set(KEY, "run-1.reservation");
        assert!(
            require_reservation().is_ok(),
            "an ordinary reservation id is accepted"
        );

        for rejected in ["", ".", "..", "a/b", "a\\b", "/etc", "run 1"] {
            guard.replace(rejected);
            assert!(
                require_reservation().is_err(),
                "{rejected:?} must not be accepted as a reservation id"
            );
        }
    }

    /// The kind is a whitelist because it becomes a filename. An unknown kind
    /// must be refused *without* leaving a reservation file, or a typo would
    /// quietly mint a partition that nothing ever releases.
    #[test]
    fn a_reservation_kind_outside_the_whitelist_creates_nothing() {
        let temp = tempfile::tempdir().expect("temp");
        let scope = scope_at(temp.path(), "app", "instance", "run-1");
        for kind in ["bundle", "", "source/render", "../escape", "SOURCE"] {
            assert!(
                scope.claim(kind).is_err(),
                "{kind:?} is not a reservation kind"
            );
        }
        assert_eq!(
            fs::read_dir(scope.dir()).expect("entries").count(),
            0,
            "a refused claim must not leave a reservation file behind"
        );
    }

    /// The content-type gate runs before the reservation is claimed. Were those
    /// two reordered, a source that answered with the wrong media type would
    /// burn the run's one source partition, and the retry would then fail with
    /// an unrelated message about a reservation already being used.
    #[test]
    fn a_wrong_media_type_is_refused_without_spending_the_reservation() {
        let temp = tempfile::tempdir().expect("temp");
        let scope = scope_at(temp.path(), "app", "instance", "run-1");
        let server = tiny_http::Server::http("127.0.0.1:0").expect("server");
        let url = format!("http://{}/report", server.server_addr());
        let responder = std::thread::spawn(move || {
            let request = server.recv().expect("request");
            let header = tiny_http::Header::from_bytes(b"content-type", b"application/json")
                .expect("header");
            let body = b"{\"rows\":[]}".to_vec();
            let length = body.len();
            let response = tiny_http::Response::new(
                tiny_http::StatusCode(200),
                vec![header],
                std::io::Cursor::new(body),
                Some(length),
                None,
            );
            request.respond(response).expect("respond");
        });
        let response = ureq::get(&url).call().expect("GET");
        assert!(
            spool_rest_response(response, &scope, 4096).is_err(),
            "a JSON body is not the record stream this path spools"
        );
        responder.join().expect("server join");
        assert!(
            scope.claim("source").is_ok(),
            "the run's source reservation must still be available after a \
             refused response"
        );
    }

    /// `from_dir` derives the whole descriptor identity from three directory
    /// names, so each of them is validated before it can be stamped into an
    /// `ArtifactRef` and joined into a path later on.
    #[test]
    fn a_run_directory_must_name_a_path_safe_run_app_and_instance() {
        let temp = tempfile::tempdir().expect("temp");

        for (label, dir) in [
            (
                "a directory not ending in `.artifacts` is not run-owned",
                temp.path().join("app").join("instance").join("run-1"),
            ),
            (
                "an app directory that is not one path-safe identifier",
                temp.path()
                    .join("app name")
                    .join("instance")
                    .join("run-1.artifacts"),
            ),
            (
                "an instance directory that is not one path-safe identifier",
                temp.path()
                    .join("app")
                    .join("instance%name")
                    .join("run-1.artifacts"),
            ),
            (
                "a run id that is not one path-safe identifier",
                temp.path()
                    .join("app")
                    .join("instance")
                    .join("run 1.artifacts"),
            ),
        ] {
            fs::create_dir_all(&dir).expect("fixture directory");
            assert!(RunArtifactScope::from_dir(&dir).is_err(), "{label}");
        }
    }

    /// A symlinked run directory would let a descriptor's `join` land wherever
    /// the link points, outside anything this run owns, so the scope refuses to
    /// be built on one at all.
    #[cfg(unix)]
    #[test]
    fn a_symlinked_run_directory_is_refused() {
        let temp = tempfile::tempdir().expect("temp");
        let elsewhere = temp.path().join("elsewhere");
        fs::create_dir_all(&elsewhere).expect("fixture");
        let instance = temp.path().join("app").join("instance");
        fs::create_dir_all(&instance).expect("fixture");
        let link = instance.join("run-1.artifacts");
        std::os::unix::fs::symlink(&elsewhere, &link).expect("symlink");

        assert!(
            RunArtifactScope::from_dir(&link).is_err(),
            "a run directory reached through a link is not run-owned"
        );
    }
}
