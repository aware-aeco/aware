//! The files a migration writes before any promotion (#628 PR2), all beside
//! the app's source:
//!
//! * `.aware-migration/<app>.candidate.lock` — the candidate plan;
//! * `.aware-migration/<app>.evidence.json` — what was checked, and the
//!   candidate header that binds it to the base lock and candidate bytes;
//! * `.aware-approvals/HOLD` — a person's hold: the app is sealed, certified
//!   or frozen and must not be carried forward.
//!
//! None of these is an approval and `aware app run` reads none of them: the
//! approved plan is `<app>.lock` alone. Writes are atomic (same-directory temp
//! file, fsync, rename), so a reader sees an old or a new file, never a torn one.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::app_lock::CandidateHeader;
use crate::error::AwareError;

/// The candidate directory beside a source file's directory.
pub const MIGRATION_DIR: &str = ".aware-migration";
/// The approval-records directory (HOLD now; archives from #628 PR3).
pub const APPROVALS_DIR: &str = ".aware-approvals";
/// The hold file name.
pub const HOLD_FILE: &str = "HOLD";
/// The evidence format tag.
pub const EVIDENCE_FORMAT: &str = "aware.migration-evidence/v1";

pub fn candidate_path(source_dir: &Path, app: &str) -> PathBuf {
    source_dir
        .join(MIGRATION_DIR)
        .join(format!("{app}.candidate.lock"))
}

pub fn evidence_path(source_dir: &Path, app: &str) -> PathBuf {
    source_dir
        .join(MIGRATION_DIR)
        .join(format!("{app}.evidence.json"))
}

pub fn hold_path(source_dir: &Path) -> PathBuf {
    source_dir.join(APPROVALS_DIR).join(HOLD_FILE)
}

/// A person's hold on an app.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct HoldRecord {
    pub format: u32,
    pub held_by: String,
    pub held_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// The hold on the app whose source lives in `source_dir`, if any. A HOLD
/// file that cannot be parsed still HOLDS — it is reported with an unknown
/// holder rather than ignored.
pub fn read_hold(source_dir: &Path) -> Result<Option<HoldRecord>, AwareError> {
    let path = hold_path(source_dir);
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(
                std::io::Error::new(error.kind(), format!("{}: {error}", path.display())).into(),
            );
        }
    };
    Ok(Some(serde_yaml::from_str(&text).unwrap_or_else(|error| {
        HoldRecord {
            format: 1,
            held_by: "(unknown — the HOLD file could not be read)".into(),
            held_at: String::new(),
            reason: Some(format!("{}: {error}", path.display())),
        }
    })))
}

/// Put the app on hold (replacing any earlier hold record).
pub fn write_hold(source_dir: &Path, record: &HoldRecord) -> Result<PathBuf, AwareError> {
    let path = hold_path(source_dir);
    let dir = source_dir.join(APPROVALS_DIR);
    std::fs::create_dir_all(&dir)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", dir.display())))?;
    let yaml = serde_yaml::to_string(record)
        .map_err(|e| AwareError::Internal(format!("serialize hold: {e}")))?;
    write_atomically(&path, yaml.as_bytes())?;
    Ok(path)
}

/// Lift a hold. `Ok(false)` when there was none.
pub fn remove_hold(source_dir: &Path) -> Result<bool, AwareError> {
    remove_if_present(&hold_path(source_dir))
}

/// What is on disk for one app's candidate.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct StoredCandidate {
    pub present: bool,
    /// The candidate bytes' digest, as the evidence header records it.
    pub candidate_digest: Option<String>,
    /// The header, when the evidence could be read and its candidate digest
    /// matches the candidate file's bytes. `None` for a half-written,
    /// tampered or unreadable pair — such a candidate is never fresh.
    #[serde(skip)]
    pub header: Option<CandidateHeader>,
}

/// The evidence file's body.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Evidence {
    pub format: String,
    pub app: String,
    pub prepared_at: String,
    pub cli_version: String,
    pub header: CandidateHeader,
    /// The plan row as computed when the candidate was prepared.
    pub row: serde_json::Value,
}

/// Read the candidate + evidence pair of `app`, verifying they belong together.
pub fn read_candidate(source_dir: &Path, app: &str) -> Result<StoredCandidate, AwareError> {
    let candidate = candidate_path(source_dir, app);
    let bytes = match std::fs::read(&candidate) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(StoredCandidate::default());
        }
        Err(error) => {
            return Err(std::io::Error::new(
                error.kind(),
                format!("{}: {error}", candidate.display()),
            )
            .into());
        }
    };
    let digest = crate::app_lock::lock_digest(&bytes);
    let header = std::fs::read(evidence_path(source_dir, app))
        .ok()
        .and_then(|text| serde_json::from_slice::<Evidence>(&text).ok())
        .map(|evidence| evidence.header)
        .filter(|header| header.candidate_digest == digest && header.app == app);
    Ok(StoredCandidate {
        present: true,
        candidate_digest: Some(digest),
        header,
    })
}

/// Write the candidate and its evidence. Evidence first, then the candidate:
/// a crash between the two leaves a pair whose digests disagree, which
/// [`read_candidate`] reports as present but never fresh.
pub fn write_candidate(
    source_dir: &Path,
    app: &str,
    candidate: &[u8],
    evidence: &[u8],
) -> Result<(PathBuf, PathBuf), AwareError> {
    let dir = source_dir.join(MIGRATION_DIR);
    std::fs::create_dir_all(&dir)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", dir.display())))?;
    let evidence_file = evidence_path(source_dir, app);
    let candidate_file = candidate_path(source_dir, app);
    write_atomically(&evidence_file, evidence)?;
    write_atomically(&candidate_file, candidate)?;
    Ok((candidate_file, evidence_file))
}

/// Remove the candidate and its evidence. `Ok(false)` when neither existed.
pub fn discard_candidate(source_dir: &Path, app: &str) -> Result<bool, AwareError> {
    let candidate = remove_if_present(&candidate_path(source_dir, app))?;
    let evidence = remove_if_present(&evidence_path(source_dir, app))?;
    // Tidy an empty directory; a directory still holding anything stays.
    let _ = std::fs::remove_dir(source_dir.join(MIGRATION_DIR));
    Ok(candidate || evidence)
}

fn remove_if_present(path: &Path) -> Result<bool, AwareError> {
    match std::fs::remove_file(path) {
        Ok(()) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => {
            Err(std::io::Error::new(error.kind(), format!("{}: {error}", path.display())).into())
        }
    }
}

fn write_atomically(path: &Path, bytes: &[u8]) -> Result<(), AwareError> {
    match crate::app_lock::replace_atomically(path, bytes)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", path.display())))?
    {
        crate::fs::Replaced::Durable => {}
        crate::fs::Replaced::NotDurable(error) => eprintln!(
            "\u{26a0} {} was written, but making it durable failed ({error})",
            path.display()
        ),
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn header(digest: &str) -> CandidateHeader {
        CandidateHeader {
            format: "f".into(),
            app: "demo".into(),
            base_lock_digest: "sha256:base".into(),
            base_source_hash: "sha256:src".into(),
            targets: Default::default(),
            candidate_digest: digest.into(),
            plan_digest: "sha256:plan".into(),
        }
    }

    fn evidence(digest: &str) -> Vec<u8> {
        serde_json::to_vec(&Evidence {
            format: EVIDENCE_FORMAT.into(),
            app: "demo".into(),
            prepared_at: "t".into(),
            cli_version: "v".into(),
            header: header(digest),
            row: serde_json::Value::Null,
        })
        .unwrap()
    }

    #[test]
    fn a_candidate_pairs_with_its_evidence_only_by_digest() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path();
        assert!(!read_candidate(dir, "demo").unwrap().present);

        let bytes = b"candidate bytes";
        let digest = crate::app_lock::lock_digest(bytes);
        write_candidate(dir, "demo", bytes, &evidence(&digest)).unwrap();
        let stored = read_candidate(dir, "demo").unwrap();
        assert!(stored.present);
        assert_eq!(stored.header.unwrap().candidate_digest, digest);

        // Evidence naming other bytes (a torn or tampered pair): present, no header.
        write_candidate(dir, "demo", b"other bytes", &evidence(&digest)).unwrap();
        let torn = read_candidate(dir, "demo").unwrap();
        assert!(torn.present && torn.header.is_none());

        assert!(discard_candidate(dir, "demo").unwrap());
        assert!(!dir.join(MIGRATION_DIR).exists(), "empty dir tidied");
        assert!(!discard_candidate(dir, "demo").unwrap());
    }

    /// Whether Rust source text reaches the candidate store.
    fn reaches_candidates(text: &str) -> bool {
        [
            MIGRATION_DIR,
            "MIGRATION_DIR",
            "candidate_path",
            "read_candidate",
            "migration::files",
        ]
        .iter()
        .any(|needle| text.contains(needle))
    }

    /// `aware app run` must never read a candidate (app-spec § candidate
    /// locks): the run path — the runtime, the resolver, the approval gate
    /// and the `app` command module — never names the candidate store.
    #[test]
    fn the_run_path_never_reaches_the_candidate_store() {
        assert!(reaches_candidates(
            "let p = dir.join(\".aware-migration\");"
        ));
        assert!(reaches_candidates(
            "crate::migration::files::read_candidate(d, a)"
        ));
        assert!(!reaches_candidates(
            "let p = dir.join(format!(\"{app}.lock\"));"
        ));

        let src = Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut scanned = Vec::new();
        let mut files: Vec<PathBuf> = vec![
            src.join("app_lock.rs"),
            src.join("agent_resolution.rs"),
            src.join("commands").join("app.rs"),
        ];
        for entry in std::fs::read_dir(src.join("runtime")).unwrap() {
            files.push(entry.unwrap().path());
        }
        for file in files
            .into_iter()
            .filter(|f| f.extension().is_some_and(|e| e == "rs"))
        {
            let text = std::fs::read_to_string(&file).unwrap();
            assert!(
                !reaches_candidates(&text),
                "{} reaches the migration candidate store; a run must read only <app>.lock",
                file.display()
            );
            scanned.push(file);
        }
        assert!(
            scanned.len() > 10,
            "the scan covered too little: {scanned:?}"
        );
    }

    #[test]
    fn a_hold_round_trips_and_an_unreadable_hold_still_holds() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path();
        assert_eq!(read_hold(dir).unwrap(), None);
        let record = HoldRecord {
            format: 1,
            held_by: "pawel".into(),
            held_at: "2026-10-05T00:00:00Z".into(),
            reason: Some("sealed".into()),
        };
        write_hold(dir, &record).unwrap();
        assert_eq!(read_hold(dir).unwrap(), Some(record));

        std::fs::write(hold_path(dir), "{ not yaml").unwrap();
        let garbled = read_hold(dir).unwrap().expect("a garbled HOLD still holds");
        assert!(garbled.held_by.contains("unknown"));

        assert!(remove_hold(dir).unwrap());
        assert!(!remove_hold(dir).unwrap());
    }
}
