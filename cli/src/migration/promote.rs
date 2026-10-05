//! Promotion (#628 PR3b, plan §6–§7 revised by §11–§15): replace an app's
//! `<app>.lock` with a prepared candidate, carrying its approval forward — by
//! a person's recorded approval or under a person-approved policy — and append
//! one successor link to the approval record. `revert` is the same act with
//! `kind: reverted`, back to pins that were approved before.
//!
//! What a promotion guarantees:
//!
//! * **The candidate file is evidence, never the authority.** Under the app's
//!   promotion lock the candidate is recompiled from the CURRENT source and the
//!   CURRENT lock and must reach the same plan; the lock it started from must
//!   still be the lock on disk; a person approval must name these exact bytes.
//! * **The record never lies.** A person approval is recorded as a claim
//!   (`attested: false`); every archive the new link cites is in place and
//!   verified by the PR3a reader ([`crate::app_lock::approval::assess`])
//!   BEFORE the lock moves — the writer cannot produce a lock its own reader
//!   refuses or reports incomplete.
//! * **Old or new, never half.** The promotion is a transaction under
//!   `.aware-approvals/.txn/<app>.<uuid>/`: staged + fsynced, archives moved
//!   into place, then ONE atomic replace of `<app>.lock`. A run reads the lock
//!   once at preflight, so it sees the old plan or the whole new one. A crash
//!   is recovered by the next migrate verb: finished if the lock already moved,
//!   rolled back otherwise.
//!
//! Lock order: the store reference lock (`RefGuard`, shared — the new lock is a
//! store reference), then the app's promotion lock ([`lock_app`]), which a
//! person's `aware app compile` of the same app takes too.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use fs2::FileExt;
use serde::{Deserialize, Serialize};

use super::Reason;
use super::files::{self, Evidence};
use super::plan::{self, State};
use super::policy;
use crate::agent_resolution::PinTarget;
use crate::app_lock::approval::{
    self, APPROVAL_FORMAT, ApprovalChain, ApprovalPin, CarriedForwardBy, OriginalApproval,
    SUCCESSOR_SOURCE_PREFIX, Successor, SuccessorKind, records,
};
use crate::app_lock::{CandidatePin, LockFile, lock_digest, plan_digest};
use crate::error::AwareError;
use crate::paths::Paths;

/// The transaction directory inside `.aware-approvals/`.
pub const TXN_DIR: &str = ".txn";
const INTENT_FILE: &str = "intent.json";
const STAGED_LOCK: &str = "new.lock";
const INTENT_FORMAT: u32 = 1;

/// The environment markers of an AI coding session. Their presence makes a
/// missing `--front-door` on a person approval an explicit accident report
/// (§11 R1): an accident guard, NOT an adversary guard — any process can unset
/// them. The trust boundary is the front door (owner decision 1,
/// pawellisowski/floless.app#1985).
pub const AI_SESSION_MARKERS: &[&str] = &["CLAUDECODE", "CODEX_SANDBOX", "AWARE_AI_SESSION"];

/// The AI-session marker set in this process's environment, if any.
pub fn ai_session_marker() -> Option<&'static str> {
    AI_SESSION_MARKERS
        .iter()
        .copied()
        .find(|name| std::env::var_os(name).is_some_and(|v| !v.is_empty()))
}

/// A refusal: a stable code, the error, and machine details (reasons, lists).
#[derive(Debug)]
pub struct Refused {
    pub code: String,
    pub error: Box<AwareError>,
    pub details: serde_json::Value,
}

impl Refused {
    pub fn new(code: &str, message: impl std::fmt::Display) -> Self {
        Self {
            code: code.to_string(),
            error: Box::new(AwareError::Validation(format!("[{code}] {message}"))),
            details: serde_json::Value::Null,
        }
    }

    fn with(mut self, details: serde_json::Value) -> Self {
        self.details = details;
        self
    }
}

impl From<AwareError> for Refused {
    fn from(error: AwareError) -> Self {
        let code = match &error {
            AwareError::NotFound(_) => "E_MIGRATE_APP_NOT_FOUND".to_string(),
            AwareError::Validation(_) => super::reason_from_error(&error).error_code(),
            _ => "E_MIGRATE_FAILED".to_string(),
        };
        let code = if code == "E_CANNOT_EVALUATE" {
            "E_MIGRATE_FAILED".to_string()
        } else {
            code
        };
        Self {
            code,
            error: Box::new(error),
            details: serde_json::Value::Null,
        }
    }
}

type Result<T> = std::result::Result<T, Refused>;

fn io(path: &Path, error: std::io::Error) -> Refused {
    Refused::from(AwareError::from(std::io::Error::new(
        error.kind(),
        format!("{}: {error}", path.display()),
    )))
}

// ── The promotion lock ─────────────────────────────────────────────────────

/// A held promotion lock of one app. Dropping it releases the OS lock.
#[derive(Debug)]
pub struct PromotionLock {
    _file: std::fs::File,
}

/// Take app `app`'s promotion lock (exclusive, blocking). The key is the
/// canonical source directory plus the app id, so every path to the same lock
/// file names the same OS lock. Requires the store reference lock first — the
/// lock order everywhere is store lock, then this one.
pub fn lock_app(
    paths: &Paths,
    guard: &crate::agent_store::RefGuard,
    source_dir: &Path,
    app: &str,
) -> std::result::Result<PromotionLock, AwareError> {
    crate::agent_store::guard::require_home(guard, paths)?;
    let canonical = std::fs::canonicalize(source_dir)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", source_dir.display())))?;
    let key = lock_digest(format!("{}\n{app}", canonical.display()).as_bytes());
    let hex = crate::agent_store::digest_hex(&key)
        .ok_or_else(|| AwareError::Internal(format!("{key} is not a digest")))?;
    let dir = paths.migration_locks_dir();
    std::fs::create_dir_all(&dir)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", dir.display())))?;
    let path = dir.join(format!("{hex}.flock"));
    let file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(&path)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", path.display())))?;
    file.lock_exclusive().map_err(|e| {
        std::io::Error::new(
            e.kind(),
            format!("take the promotion lock {}: {e}", path.display()),
        )
    })?;
    Ok(PromotionLock { _file: file })
}

/// Take app `app`'s promotion lock and recover any leftover transaction of it
/// first — what every migrate verb that touches the app's migration files
/// does (plan §12 R1-7: recovery at the start of every migrate verb).
pub fn lock_and_recover(
    paths: &Paths,
    guard: &crate::agent_store::RefGuard,
    source_dir: &Path,
    app: &str,
) -> std::result::Result<(PromotionLock, Vec<Recovered>), AwareError> {
    let lock = lock_app(paths, guard, source_dir, app)?;
    let recovered = recover(source_dir, app, &lock)?;
    Ok((lock, recovered))
}

// ── The transaction ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct StagedArchive {
    digest: String,
    ext: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct Intent {
    format: u32,
    app: String,
    base_lock_digest: String,
    new_lock_digest: String,
    archives: Vec<StagedArchive>,
    /// The archives that were NOT in `.aware-approvals/` before this
    /// transaction: the only ones a rollback removes (review round 3 — a
    /// refused promotion leaves no new records behind).
    #[serde(default)]
    fresh: Vec<StagedArchive>,
}

/// A step after which a test can make the promotion "crash" (fault
/// injection, plan §12 R1-7). Never set outside tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Fault {
    Staged,
    ArchivesPlaced,
    LockReplaced,
}

#[cfg(test)]
thread_local! {
    static FAULT: std::cell::Cell<Option<Fault>> = const { std::cell::Cell::new(None) };
}

#[cfg(test)]
pub(crate) fn inject_fault(fault: Fault) {
    FAULT.with(|f| f.set(Some(fault)));
}

#[cfg(test)]
fn crash_point(at: Fault) -> std::result::Result<(), AwareError> {
    if FAULT.with(|f| f.get()) == Some(at) {
        FAULT.with(|f| f.set(None));
        return Err(AwareError::Internal(format!(
            "injected crash at {at:?} (test)"
        )));
    }
    Ok(())
}

#[cfg(not(test))]
fn crash_point(_: Fault) -> std::result::Result<(), AwareError> {
    Ok(())
}

#[cfg(test)]
thread_local! {
    /// Run once on this thread right after a policy promotion's eligibility
    /// verdict, while it holds the policies lock (§15.1 R3 test).
    static AFTER_POLICY_VERDICT: std::cell::RefCell<Option<Box<dyn FnOnce()>>> =
        std::cell::RefCell::new(None);
}

#[cfg(test)]
pub(crate) fn on_policy_verdict(hook: Box<dyn FnOnce()>) {
    AFTER_POLICY_VERDICT.with(|h| *h.borrow_mut() = Some(hook));
}

#[cfg(test)]
fn after_policy_verdict() {
    if let Some(hook) = AFTER_POLICY_VERDICT.with(|h| h.borrow_mut().take()) {
        hook();
    }
}

#[cfg(not(test))]
fn after_policy_verdict() {}

#[cfg(test)]
thread_local! {
    /// When set, the next promotion on this thread writes a chain whose last
    /// link names the wrong plan — the negative control of the writer's
    /// self-check by the reader (§15).
    static CORRUPT_CHAIN: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(test)]
pub(crate) fn corrupt_next_chain() {
    CORRUPT_CHAIN.with(|c| c.set(true));
}

#[cfg(test)]
fn maybe_corrupt(chain: &mut ApprovalChain) {
    if CORRUPT_CHAIN.with(|c| c.replace(false))
        && let Some(last) = chain.successors.last_mut()
    {
        last.to_plan_digest = format!("sha256:{}", "e".repeat(64));
    }
}

#[cfg(not(test))]
fn maybe_corrupt(_: &mut ApprovalChain) {}

fn approvals_dir(source_dir: &Path) -> PathBuf {
    source_dir.join(approval::ARCHIVE_DIR)
}

fn txn_root(source_dir: &Path) -> PathBuf {
    approvals_dir(source_dir).join(TXN_DIR)
}

fn write_synced(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    file.write_all(bytes)?;
    file.sync_all()
}

/// The txn directories of `app` under `source_dir`: `<app>.<32 hex>`.
fn owned_txns(source_dir: &Path, app: &str) -> std::io::Result<Vec<PathBuf>> {
    let root = txn_root(source_dir);
    let entries = match std::fs::read_dir(&root) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error),
    };
    let mut out = Vec::new();
    for entry in entries {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if let Some((owner, id)) = name.rsplit_once('.')
            && owner == app
            && id.len() == 32
            && id.bytes().all(|b| b.is_ascii_hexdigit())
        {
            out.push(entry.path());
        }
    }
    out.sort();
    Ok(out)
}

/// What [`recover`] did with one leftover transaction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Recovered {
    pub txn: String,
    /// `finished` (the lock had moved) | `rolled-back` (it had not).
    pub outcome: &'static str,
}

/// Finish or roll back every leftover promotion transaction of `app` (plan
/// §12 R1-7). Run under the app's promotion lock. A transaction whose lock
/// already moved is finished (its archives are put in place); any other is
/// rolled back — the lock was never touched.
pub fn recover(
    source_dir: &Path,
    app: &str,
    _lock: &PromotionLock,
) -> std::result::Result<Vec<Recovered>, AwareError> {
    let lock_path = source_dir.join(format!("{app}.lock"));
    let current = std::fs::read(&lock_path).ok().map(|b| lock_digest(&b));
    let mut out = Vec::new();
    for txn in owned_txns(source_dir, app).map_err(|e| {
        std::io::Error::new(e.kind(), format!("{}: {e}", txn_root(source_dir).display()))
    })? {
        let intent: Option<Intent> = std::fs::read(txn.join(INTENT_FILE))
            .ok()
            .and_then(|bytes| serde_json::from_slice(&bytes).ok())
            .filter(|i: &Intent| i.format == INTENT_FORMAT && i.app == app);
        let finished = intent
            .as_ref()
            .is_some_and(|i| current.as_deref() == Some(i.new_lock_digest.as_str()));
        match (finished, &intent) {
            (true, Some(intent)) => place_archives(source_dir, &txn, &intent.archives)?,
            (false, Some(intent)) => remove_fresh(source_dir, &intent.fresh),
            _ => {}
        }
        std::fs::remove_dir_all(&txn)
            .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", txn.display())))?;
        out.push(Recovered {
            txn: txn.display().to_string(),
            outcome: if finished { "finished" } else { "rolled-back" },
        });
    }
    Ok(out)
}

/// Remove the archives a rolled-back transaction placed that were not there
/// before it (each only when it still holds exactly its named bytes).
fn remove_fresh(source_dir: &Path, fresh: &[StagedArchive]) {
    for archive in fresh {
        let Some(rel) = approval::archive_rel(&archive.digest, &archive.ext) else {
            continue;
        };
        let path = source_dir.join(rel);
        if std::fs::read(&path).is_ok_and(|bytes| lock_digest(&bytes) == archive.digest) {
            let _ = std::fs::remove_file(&path);
        }
    }
}

/// Check every archive destination before anything is written: an archive
/// already in place must hold exactly its named bytes. Returns the archives
/// that are not there yet.
fn preflight_archives(
    source_dir: &Path,
    archives: &[StagedArchive],
) -> std::result::Result<Vec<StagedArchive>, AwareError> {
    let mut fresh = Vec::new();
    for archive in archives {
        let rel = approval::archive_rel(&archive.digest, &archive.ext)
            .ok_or_else(|| AwareError::Internal(format!("{} is not a digest", archive.digest)))?;
        let dest = source_dir.join(&rel);
        match std::fs::read(&dest) {
            Ok(bytes) if lock_digest(&bytes) == archive.digest => {}
            Ok(_) => {
                return Err(AwareError::Validation(format!(
                    "[E_MIGRATE_ARCHIVE_INVALID] {} does not hash to {}: an archived approval record was changed; nothing was promoted",
                    dest.display(),
                    archive.digest
                )));
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                if !fresh
                    .iter()
                    .any(|f: &StagedArchive| f.digest == archive.digest && f.ext == archive.ext)
                {
                    fresh.push(archive.clone());
                }
            }
            Err(error) => {
                return Err(std::io::Error::new(
                    error.kind(),
                    format!("{}: {error}", dest.display()),
                )
                .into());
            }
        }
    }
    Ok(fresh)
}

/// Move each staged archive of `txn` into `.aware-approvals/`. An archive
/// already there must hash to its name (content-addressed: the same bytes);
/// one that does not is a changed record, refused before the lock moves.
fn place_archives(
    source_dir: &Path,
    txn: &Path,
    archives: &[StagedArchive],
) -> std::result::Result<(), AwareError> {
    let dir = approvals_dir(source_dir);
    for archive in archives {
        let rel = approval::archive_rel(&archive.digest, &archive.ext)
            .ok_or_else(|| AwareError::Internal(format!("{} is not a digest", archive.digest)))?;
        let dest = source_dir.join(&rel);
        let name = dest
            .file_name()
            .map(|n| n.to_os_string())
            .unwrap_or_default();
        let staged = txn.join(&name);
        match std::fs::read(&dest) {
            Ok(bytes) if lock_digest(&bytes) == archive.digest => {
                let _ = std::fs::remove_file(&staged);
                continue;
            }
            Ok(_) => {
                return Err(AwareError::Validation(format!(
                    "[E_MIGRATE_ARCHIVE_INVALID] {} does not hash to {}: an archived approval record was changed; nothing was promoted",
                    dest.display(),
                    archive.digest
                )));
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(std::io::Error::new(
                    error.kind(),
                    format!("{}: {error}", dest.display()),
                )
                .into());
            }
        }
        if !staged.is_file() {
            return Err(AwareError::Internal(format!(
                "staged archive {} is missing and {} is not in place",
                staged.display(),
                dest.display()
            )));
        }
        crate::fs::replace_file(&staged, &dest)
            .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", dest.display())))?;
    }
    crate::fs::sync_dir(&dir)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", dir.display())))?;
    Ok(())
}

/// Stage, place archives, verify with the reader, compare-and-swap the lock.
fn commit(
    source_dir: &Path,
    lock_path: &Path,
    intent: &Intent,
    archives: &[(Vec<u8>, &str)],
    new_lock: &LockFile,
    new_bytes: &[u8],
) -> std::result::Result<(), AwareError> {
    // Nothing is written until every destination checks out.
    let mut intent = intent.clone();
    intent.fresh = preflight_archives(source_dir, &intent.archives)?;
    let intent = &intent;
    let root = txn_root(source_dir);
    std::fs::create_dir_all(&root)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", root.display())))?;
    let txn = root.join(format!("{}.{}", intent.app, uuid::Uuid::new_v4().simple()));
    std::fs::create_dir(&txn)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", txn.display())))?;
    let staged = (|| -> std::result::Result<(), AwareError> {
        let intent_bytes = serde_json::to_vec_pretty(intent)
            .map_err(|e| AwareError::Internal(format!("serialize intent: {e}")))?;
        write_synced(&txn.join(INTENT_FILE), &intent_bytes)?;
        for (bytes, ext) in archives {
            let digest = lock_digest(bytes);
            let hex = crate::agent_store::digest_hex(&digest)
                .ok_or_else(|| AwareError::Internal(format!("{digest} is not a digest")))?;
            let path = txn.join(format!("{hex}.{ext}"));
            if !path.exists() {
                write_synced(&path, bytes)?;
            }
        }
        write_synced(&txn.join(STAGED_LOCK), new_bytes)?;
        crate::fs::sync_dir(&txn)?;
        Ok(())
    })();
    let rollback = |error: AwareError| {
        remove_fresh(source_dir, &intent.fresh);
        let _ = std::fs::remove_dir_all(&txn);
        error
    };
    staged.map_err(rollback)?;
    crash_point(Fault::Staged)?;

    place_archives(source_dir, &txn, &intent.archives).map_err(rollback)?;
    crash_point(Fault::ArchivesPlaced)?;

    // The writer never produces a lock its own reader refuses (§15).
    let verified = crate::agent_resolution::check_lock_consistency(new_lock)
        .and_then(|()| approval::assess(new_lock, source_dir))
        .and_then(|summary| {
            if summary.record_complete {
                Ok(())
            } else {
                Err(format!("missing: {}", summary.missing.join("; ")))
            }
        });
    if let Err(reason) = verified {
        return Err(rollback(AwareError::Internal(format!(
            "the promoted lock would not pass its own reader ({reason}); nothing was promoted"
        ))));
    }

    // Compare and swap: the lock must still be the one the candidate started
    // from. (The promotion lock makes this a check, not a race: a compile of
    // this app takes the same lock.)
    let current = std::fs::read(lock_path)
        .map(|b| lock_digest(&b))
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", lock_path.display())));
    match current {
        Ok(digest) if digest == intent.base_lock_digest => {}
        Ok(digest) => {
            return Err(rollback(AwareError::Validation(format!(
                "[E_MIGRATE_CANDIDATE_STALE] {} changed while it was being promoted (it is now {digest}); nothing was promoted — prepare again",
                lock_path.display()
            ))));
        }
        Err(error) => return Err(rollback(error.into())),
    }
    match crate::fs::replace_file(&txn.join(STAGED_LOCK), lock_path) {
        Ok(crate::fs::Replaced::Durable) => {}
        Ok(crate::fs::Replaced::NotDurable(error)) => eprintln!(
            "\u{26a0} {} was replaced and is in effect, but making the change durable failed ({error})",
            lock_path.display()
        ),
        Err(error) => {
            return Err(rollback(
                std::io::Error::new(error.kind(), format!("{}: {error}", lock_path.display()))
                    .into(),
            ));
        }
    }
    crash_point(Fault::LockReplaced)?;
    let _ = std::fs::remove_dir_all(&txn);
    Ok(())
}

// ── The request ────────────────────────────────────────────────────────────

/// Who carries the approval forward.
pub enum Approver<'a> {
    /// A person's approval, recorded by a front door: the record file's exact
    /// bytes, bound to this candidate.
    Person {
        actor: &'a str,
        front_door: &'a str,
        record: Vec<u8>,
    },
    /// A person-approved policy. `official` verifies one new package against a
    /// freshly fetched official registry index (`Err` = the reason it could
    /// not be verified).
    Policy {
        id: &'a str,
        front_door: &'a str,
        official: &'a dyn Fn(&CandidatePin) -> std::result::Result<(), String>,
    },
    /// `revert --automatic --reason run-failed:<run-id>`: a caller's policy
    /// moving a policy-carried link back (§11, §15).
    Automatic {
        reason: &'a str,
        front_door: &'a str,
    },
}

/// What a promotion did.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Promoted {
    pub app: String,
    pub lock: String,
    pub kind: SuccessorKind,
    pub seq: u32,
    pub by_kind: &'static str,
    /// The digest of the lock this replaced, and of the lock now in place.
    pub replaced_lock_digest: String,
    pub lock_digest: String,
    pub plan_digest: String,
    pub resulting_lock_digest: String,
    pub evidence_digest: String,
    pub from: BTreeMap<String, ApprovalPin>,
    pub to: BTreeMap<String, ApprovalPin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    /// Runs already in progress keep the plan they started with.
    pub running_instances: Vec<plan::RunningInstance>,
    /// Leftover transactions recovered before this promotion.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub recovered: Vec<Recovered>,
    pub warnings: Vec<Reason>,
    pub label: String,
}

/// Promote (or revert, `kind: Reverted`) app `source`'s prepared candidate
/// `candidate_digest`, carried forward by `approver`.
pub fn promote(
    paths: &Paths,
    guard: &crate::agent_store::RefGuard,
    source: &Path,
    candidate_digest: &str,
    approver: Approver<'_>,
    kind: SuccessorKind,
) -> Result<Promoted> {
    let (app, _) = crate::app_lock::read_app_source(source)?;
    let id = app.app.clone();
    let dir = crate::fs::containing_dir(source).to_path_buf();
    let lock_path = dir.join(format!("{id}.lock"));
    let promotion_lock = lock_app(paths, guard, &dir, &id)?;
    let recovered = recover(&dir, &id, &promotion_lock)?;

    if let Some(hold) = files::read_hold(&dir, &id)? {
        return Err(Refused::new(
            "E_MIGRATE_HELD",
            format!(
                "{id} is on hold (held by {}); nothing is carried forward until a person lifts the hold",
                hold.held_by
            ),
        )
        .with(serde_json::json!({ "hold": hold })));
    }

    // The prepared candidate and its evidence: what the approval names.
    let candidate_file = files::candidate_path(&dir, &id);
    let candidate_bytes = match std::fs::read(&candidate_file) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Err(Refused::new(
                "E_MIGRATE_NO_CANDIDATE",
                format!(
                    "{id} has no prepared candidate; run `aware app migrate prepare {id}` first"
                ),
            ));
        }
        Err(error) => return Err(io(&candidate_file, error)),
    };
    let on_disk = lock_digest(&candidate_bytes);
    if on_disk != candidate_digest {
        return Err(Refused::new(
            "E_MIGRATE_CANDIDATE_STALE",
            format!(
                "the prepared candidate of {id} is {on_disk}, not the {candidate_digest} that was approved; nothing was promoted — show the person the current candidate"
            ),
        ));
    }
    let evidence_file = files::evidence_path(&dir, &id);
    let evidence_bytes = std::fs::read(&evidence_file).map_err(|e| io(&evidence_file, e))?;
    let evidence: Evidence = serde_json::from_slice(&evidence_bytes).map_err(|e| {
        Refused::new(
            "E_MIGRATE_CANDIDATE_TAMPERED",
            format!("{} is not migration evidence: {e}", evidence_file.display()),
        )
    })?;
    let header = evidence.header.clone();
    if evidence.format != files::EVIDENCE_FORMAT
        || header.format != crate::app_lock::candidate::CANDIDATE_FORMAT
        || evidence.app != id
        || header.app != id
        || header.candidate_digest != on_disk
    {
        return Err(Refused::new(
            "E_MIGRATE_CANDIDATE_TAMPERED",
            format!(
                "the evidence of {id}'s candidate does not describe it (it names app {} and candidate {}); prepare again",
                header.app, header.candidate_digest
            ),
        ));
    }
    // The archived evidence's row is what the approval labels are worded
    // from, so it must state what is true of this candidate — the facts
    // recomputed below (review round 4).
    let recorded_facts = evidence_facts(&evidence.row);
    if header.targets.is_empty() {
        return Err(Refused::new(
            "E_MIGRATE_NOTHING_TO_PROMOTE",
            format!("{id}'s candidate moves no tool pin; there is nothing to carry forward"),
        ));
    }

    // Revalidate from the CURRENT source and lock (§12 R1-4).
    let targets: BTreeMap<String, PinTarget> = header
        .targets
        .iter()
        .map(|(agent, mv)| (agent.clone(), PinTarget::Digest(mv.to.digest.clone())))
        .collect();
    let eval = plan::evaluate(paths, source, Some(&targets), guard)?;
    let row = eval.row;
    let base_bytes = eval.base_bytes.ok_or_else(|| {
        Refused::new(
            "E_MIGRATE_CANDIDATE_STALE",
            format!("{id} has no approved lock any more; nothing was promoted"),
        )
    })?;
    let base_digest = lock_digest(&base_bytes);
    if base_digest != header.base_lock_digest {
        return Err(Refused::new(
            "E_MIGRATE_CANDIDATE_STALE",
            format!(
                "{id}'s approval changed since the candidate was prepared (the candidate starts from {}, the lock is now {base_digest}); nothing was promoted — prepare again",
                header.base_lock_digest
            ),
        ));
    }
    let reasons = serde_json::json!({ "reasons": row.reasons });
    if row.reasons.iter().any(|r| r.code == "approval-invalid") {
        return Err(Refused::new(
            "E_MIGRATE_ARCHIVE_INVALID",
            format!(
                "{id}'s current approval record is inconsistent with its archives; compile it again"
            ),
        )
        .with(reasons));
    }
    if row.backing_app_moved {
        return Err(Refused::new(
            "E_MIGRATE_BACKING_APP",
            format!(
                "{id}: a workflow that other workflows run as a tool, or a move of one, is never carried forward; compile it and its callers again"
            ),
        )
        .with(reasons));
    }
    if row.state == State::UpToDate {
        return Err(Refused::new(
            "E_MIGRATE_NOTHING_TO_PROMOTE",
            format!(
                "{id}'s candidate moves no tool pin away from its current approval; there is nothing to carry forward"
            ),
        ));
    }
    if row.state == State::Blocked {
        return Err(Refused::new(
            "E_MIGRATE_BLOCKED",
            format!("{id}'s candidate cannot be carried forward as it stands (see the reasons)"),
        )
        .with(reasons));
    }
    let Some(recompiled) = eval.candidate else {
        return Err(Refused::new(
            "E_MIGRATE_BLOCKED",
            format!("{id}'s candidate could not be recompiled"),
        )
        .with(reasons));
    };
    if recompiled.header.plan_digest != header.plan_digest
        || recompiled.header.targets != header.targets
    {
        return Err(Refused::new(
            "E_MIGRATE_CANDIDATE_STALE",
            format!(
                "{id}'s candidate no longer compiles to the plan that was prepared ({} now, {} then); nothing was promoted — prepare again",
                recompiled.header.plan_digest, header.plan_digest
            ),
        ));
    }

    let recomputed = serde_json::to_value(&row)
        .map_err(|e| AwareError::Internal(format!("serialize plan row: {e}")))?;
    if recorded_facts != evidence_facts(&recomputed) {
        return Err(Refused::new(
            "E_MIGRATE_CANDIDATE_TAMPERED",
            format!(
                "the evidence of {id}'s candidate records other facts (effect, contract, comparison or moves) than a check of it finds now; prepare again"
            ),
        )
        .with(serde_json::json!({
            "recorded": recorded_facts,
            "found": evidence_facts(&recomputed),
        })));
    }

    // The current record must be whole: a promotion appends to it (§12 R1-5).
    let base: LockFile = serde_yaml::from_slice(&base_bytes).map_err(|e| {
        Refused::new(
            "E_MIGRATE_ARCHIVE_INVALID",
            format!("{}: {e}", lock_path.display()),
        )
    })?;
    let summary = approval::assess(&base, &dir)
        .map_err(|reason| Refused::new("E_MIGRATE_ARCHIVE_INVALID", reason))?;
    if !summary.record_complete {
        return Err(Refused::new(
            "E_MIGRATE_ARCHIVE_INVALID",
            format!(
                "part of {id}'s approval record is missing ({}); a promotion would build on a record that cannot be shown — compile it again",
                summary.missing.join("; ")
            ),
        )
        .with(serde_json::json!({ "missing": summary.missing })));
    }

    let candidate_lock: LockFile = serde_yaml::from_slice(&candidate_bytes).map_err(|e| {
        Refused::new(
            "E_MIGRATE_CANDIDATE_TAMPERED",
            format!("{} is not a lock: {e}", candidate_file.display()),
        )
    })?;
    if plan_digest(&candidate_lock)? != header.plan_digest {
        return Err(Refused::new(
            "E_MIGRATE_CANDIDATE_TAMPERED",
            format!(
                "{}'s plan is not the one its evidence records",
                candidate_file.display()
            ),
        ));
    }
    let from = approval::pins_of(&base);
    let to = approval::pins_of(&candidate_lock);
    let chain_before = base.approval.clone();
    if kind == SuccessorKind::Reverted {
        check_revert_target(&id, chain_before.as_ref(), &to)?;
    }

    // Who carries it forward.
    let mut archives: Vec<(Vec<u8>, &str)> = Vec::new();
    let mut evidence_out = evidence_bytes.clone();
    let mut policy_id = None;
    let mut policy_guard = None;
    let (by, front_door) = match &approver {
        Approver::Person {
            actor,
            front_door,
            record,
        } => {
            check_person_record(record, actor, front_door, candidate_digest, &header)?;
            archives.push((record.clone(), "json"));
            (
                CarriedForwardBy::Person {
                    actor: actor.to_string(),
                    approval_ref: person_ref(record)?,
                    front_door: front_door.to_string(),
                    attested: false,
                    approval_record_digest: lock_digest(record),
                },
                front_door.to_string(),
            )
        }
        Approver::Policy {
            id: pid,
            front_door,
            official,
        } => {
            if kind == SuccessorKind::Reverted {
                return Err(Refused::new(
                    "E_MIGRATE_NOT_ELIGIBLE",
                    "a revert is never carried out under a policy's no-click rule; use --person, or --automatic for a run failure",
                ));
            }
            // Held shared until the lock has moved: a revocation (exclusive)
            // cannot land between this check and the promotion (§15.1 R3).
            policy_guard = Some(policy::lock_policies(paths, false)?);
            let loaded = policy::load(paths, pid)?;
            if let Some(revoked) = &loaded.revoked {
                return Err(Refused::new(
                    "E_MIGRATE_POLICY_REVOKED",
                    format!("policy {pid} was revoked by {}", revoked.revoked_by),
                ));
            }
            let mut why = policy::ineligibility(&loaded, &row);
            for backing in plan::moving_backing_apps(paths, &recompiled.agents, guard) {
                why.push(Reason::new(
                    "backing-app-moved",
                    format!(
                        "This workflow runs {backing}, which has a pending tool update; a workflow that runs a backing workflow with a pending update is never carried forward under a policy — a person can approve it, or compile {backing} and this workflow again."
                    ),
                ));
            }
            let moved: Vec<&CandidatePin> = recompiled
                .pins
                .iter()
                .filter(|pin| header.targets.contains_key(&pin.agent))
                .collect();
            for pin in moved {
                if let Err(reason) = official(pin) {
                    why.push(Reason::new(
                        "official-unverified",
                        format!(
                            "{} {} could not be verified against the official registry just now ({reason}), so no policy carries it forward; a person can approve it.",
                            pin.agent, pin.version
                        ),
                    ));
                }
            }
            if !why.is_empty() {
                return Err(Refused::new(
                    "E_MIGRATE_NOT_ELIGIBLE",
                    format!(
                        "policy {pid} does not cover carrying {id} forward: {}",
                        why.iter()
                            .map(|r| r.text.as_str())
                            .collect::<Vec<_>>()
                            .join(" ")
                    ),
                )
                .with(serde_json::json!({ "reasons": why })));
            }
            after_policy_verdict();
            policy_id = Some(pid.to_string());
            (
                CarriedForwardBy::Policy {
                    policy_id: pid.to_string(),
                    policy_digest: loaded.digest.clone(),
                    policy_approved_by: loaded.policy.approved_by.actor.clone(),
                },
                front_door.to_string(),
            )
        }
        Approver::Automatic { reason, front_door } => {
            if kind != SuccessorKind::Reverted {
                return Err(Refused::new(
                    "E_MIGRATE_NO_APPROVER",
                    "--automatic is only for reverting after a failed run",
                ));
            }
            let by = automatic_authority(
                &id,
                &dir,
                chain_before.as_ref(),
                &to,
                &header.plan_digest,
                reason,
            )?;
            let last = chain_before.as_ref().and_then(|c| c.successors.last());
            let seq = chain_before
                .as_ref()
                .map(|c| c.successors.len())
                .unwrap_or(0);
            let last_evidence = last.map(|l| l.evidence_digest.as_str()).unwrap_or("");
            let failed = recorded_failure(paths, &id, reason, seq, last_evidence)?;
            if let CarriedForwardBy::Policy { policy_id: p, .. } = &by {
                policy_id = Some(p.clone());
            }
            let mut value: Evidence = evidence.clone();
            value.row["revert"] = serde_json::json!({
                "automatic": true,
                "reason": reason,
                "failed-run": failed,
            });
            evidence_out = serde_json::to_vec_pretty(&value)
                .map_err(|e| AwareError::Internal(format!("serialize evidence: {e}")))?;
            (by, front_door.to_string())
        }
    };
    if front_door.trim().is_empty() {
        return Err(Refused::new(
            "E_MIGRATE_NO_FRONT_DOOR",
            "--front-door must name the front door recording this promotion",
        ));
    }

    // The new lock: the candidate's plan at the top level, the chain appended.
    let original = || OriginalApproval {
        lock_digest: base_digest.clone(),
        archive: approval::archive_rel(&base_digest, "lock").unwrap_or_default(),
        compiled_at: base.compiled_at.clone(),
        compiler_version: base.compiler_version.clone(),
        front_door: base.front_door.clone(),
        agent_pins: base.agent_pins.clone(),
        agent_digests: base.agent_digests.clone(),
        agent_bundle_pins: base.agent_bundle_pins.clone(),
    };
    let mut chain = chain_before.clone().unwrap_or_else(|| ApprovalChain {
        format: APPROVAL_FORMAT,
        original: original(),
        successors: Vec::new(),
    });
    let evidence_digest = lock_digest(&evidence_out);
    let seq = u32::try_from(chain.successors.len() + 1)
        .map_err(|_| AwareError::Internal("too many successors".into()))?;
    let link = Successor {
        seq,
        kind,
        from_lock_digest: base_digest.clone(),
        to_plan_digest: header.plan_digest.clone(),
        resulting_lock_digest: on_disk.clone(),
        from: from.clone(),
        to: to.clone(),
        carried_forward_by: by,
        evidence: approval::archive_rel(&evidence_digest, "json").unwrap_or_default(),
        evidence_digest: evidence_digest.clone(),
        front_door: front_door.clone(),
        cli_version: env!("CARGO_PKG_VERSION").into(),
        promoted_at: chrono::Utc::now().to_rfc3339(),
    };
    if let Approver::Person { record, .. } = &approver {
        records::validate_person_record(record, &link)
            .map_err(|c| Refused::new("E_MIGRATE_APPROVAL_MISMATCH", c.to_string()))?;
    }
    chain.successors.push(link);
    let mut new_lock = candidate_lock;
    let hex = approval::approved_source_hash(&new_lock)
        .strip_prefix("sha256:")
        .map(str::to_string)
        .ok_or_else(|| {
            Refused::new(
                "E_MIGRATE_CANDIDATE_TAMPERED",
                format!(
                    "{}'s source hash is not sha256:<hex>",
                    candidate_file.display()
                ),
            )
        })?;
    new_lock.source_hash = format!("{SUCCESSOR_SOURCE_PREFIX}{hex}");
    maybe_corrupt(&mut chain);
    new_lock.approval = Some(chain);
    let header_text = format!(
        "# {id}.lock — approval carried forward (see approval:); the original approval is archived in {}/\n# DO NOT EDIT — written by `aware app migrate`.\n\n",
        approval::ARCHIVE_DIR
    );
    let new_bytes = crate::app_lock::render_lock(&new_lock, &header_text)?;

    archives.push((base_bytes.clone(), "lock"));
    archives.push((candidate_bytes.clone(), "lock"));
    archives.push((evidence_out.clone(), "json"));
    let intent = Intent {
        format: INTENT_FORMAT,
        app: id.clone(),
        base_lock_digest: base_digest.clone(),
        new_lock_digest: lock_digest(&new_bytes),
        archives: archives
            .iter()
            .map(|(bytes, ext)| StagedArchive {
                digest: lock_digest(bytes),
                ext: ext.to_string(),
            })
            .collect(),
        // Filled in by `commit`'s preflight.
        fresh: Vec::new(),
    };
    // #627-b: the lock being replaced needed its packages until now; a failed
    // stamp fails the promotion before anything changes.
    crate::agent_store::stamps::stamp_lock(paths, &base)?;
    commit(&dir, &lock_path, &intent, &archives, &new_lock, &new_bytes)?;
    drop(policy_guard);

    // Still under the promotion lock (which `prepare` takes too), and only
    // the candidate that was promoted (review round 2).
    let mut warnings = Vec::new();
    let still_ours = std::fs::read(&candidate_file)
        .map(|bytes| lock_digest(&bytes) == on_disk)
        .unwrap_or(false);
    if still_ours && let Err(error) = files::discard_candidate(&dir, &id) {
        warnings.push(Reason::new(
            "candidate-not-removed",
            format!("The promoted candidate could not be removed ({error}); it is no longer fresh and will never be promoted again."),
        ));
    }
    drop(promotion_lock);
    let label = approval::assess(&new_lock, &dir)
        .map(|s| s.label)
        .unwrap_or_default();
    let (moved_from, moved_to) = moved(&from, &to);
    Ok(Promoted {
        app: id,
        lock: lock_path.display().to_string(),
        kind,
        seq,
        by_kind: new_lock
            .approval
            .as_ref()
            .and_then(|c| c.successors.last())
            .map(|l| l.carried_forward_by.kind())
            .unwrap_or("person"),
        replaced_lock_digest: base_digest,
        lock_digest: intent.new_lock_digest,
        plan_digest: header.plan_digest,
        resulting_lock_digest: on_disk,
        evidence_digest,
        from: moved_from,
        to: moved_to,
        policy: policy_id,
        running_instances: row.running_instances,
        recovered,
        warnings,
        label,
    })
}

/// The facts of a plan row that an approval label is worded from, plus the
/// moves they are about.
fn evidence_facts(row: &serde_json::Value) -> serde_json::Value {
    let targets: Vec<serde_json::Value> = row["targets"]
        .as_array()
        .map(|targets| {
            targets
                .iter()
                .map(|t| {
                    serde_json::json!({
                        "agent": t["agent"],
                        "from": t["from"],
                        "to": t["to"],
                    })
                })
                .collect()
        })
        .unwrap_or_default();
    serde_json::json!({
        "effect": row["effect"],
        "contract-unchanged": row["contract"]["unchanged"],
        "comparison-status": row["comparison"]["status"],
        "comparison-method": row["comparison"]["method"],
        "comparison-runs": row["comparison"]["runs"],
        "targets": targets,
    })
}

/// The agents whose pin differs, at both ends.
fn moved(
    from: &BTreeMap<String, ApprovalPin>,
    to: &BTreeMap<String, ApprovalPin>,
) -> (BTreeMap<String, ApprovalPin>, BTreeMap<String, ApprovalPin>) {
    let mut a = BTreeMap::new();
    let mut b = BTreeMap::new();
    for id in from.keys().chain(to.keys()) {
        if from.get(id) != to.get(id) {
            if let Some(pin) = from.get(id) {
                a.insert(id.clone(), pin.clone());
            }
            if let Some(pin) = to.get(id) {
                b.insert(id.clone(), pin.clone());
            }
        }
    }
    (a, b)
}

fn person_ref(record: &[u8]) -> Result<String> {
    serde_json::from_slice::<records::PersonApprovalRecord>(record)
        .map(|r| r.approval_ref)
        .map_err(|e| {
            Refused::new(
                "E_MIGRATE_APPROVAL_MISMATCH",
                format!("the approval record is not a person approval record: {e}"),
            )
        })
}

/// A person approval record names this exact promotion (§12): the actor and
/// front door given on the command line, this candidate, the lock it starts
/// from and the plan it approves.
fn check_person_record(
    bytes: &[u8],
    actor: &str,
    front_door: &str,
    candidate_digest: &str,
    header: &crate::app_lock::CandidateHeader,
) -> Result<()> {
    let mismatch = |what: String| Refused::new("E_MIGRATE_APPROVAL_MISMATCH", what);
    let record: records::PersonApprovalRecord = serde_json::from_slice(bytes).map_err(|e| {
        mismatch(format!(
            "the approval record is not a person approval record: {e}"
        ))
    })?;
    if record.format != records::PERSON_RECORD_FORMAT {
        return Err(mismatch(format!(
            "approval record format {} is not one this CLI reads (it reads {})",
            record.format,
            records::PERSON_RECORD_FORMAT
        )));
    }
    let checks: [(&str, &str, &str); 6] = [
        ("kind", record.kind.as_str(), "person"),
        ("actor", record.actor.as_str(), actor),
        ("front-door", record.front_door.as_str(), front_door),
        (
            "candidate-digest",
            record.candidate_digest.as_str(),
            candidate_digest,
        ),
        (
            "base-lock-digest",
            record.base_lock_digest.as_str(),
            header.base_lock_digest.as_str(),
        ),
        (
            "plan-digest",
            record.plan_digest.as_str(),
            header.plan_digest.as_str(),
        ),
    ];
    for (field, found, want) in checks {
        if found != want {
            return Err(mismatch(format!(
                "the approval record's {field} is {found:?}, but this promotion needs {want:?} — the person approved something else; nothing was promoted"
            )));
        }
    }
    if record.approval_ref.trim().is_empty() || record.at.trim().is_empty() {
        return Err(mismatch(
            "the approval record's approval-ref and at must not be empty".into(),
        ));
    }
    if crate::agent_store::digest_hex(&record.statement_sha256).is_none() {
        return Err(mismatch(format!(
            "the approval record's statement-sha256 {:?} is not a sha256 digest",
            record.statement_sha256
        )));
    }
    Ok(())
}

/// Every pin state approved before now: the original's and each successor's `to`.
fn approved_states(chain: &ApprovalChain) -> Vec<BTreeMap<String, ApprovalPin>> {
    let original = &chain.original;
    let [p, d, b] = [
        &original.agent_pins,
        &original.agent_digests,
        &original.agent_bundle_pins,
    ];
    let original_pins: BTreeMap<String, ApprovalPin> = p
        .iter()
        .map(|(id, version)| {
            (
                id.clone(),
                ApprovalPin {
                    version: version.clone(),
                    digest: d.get(id).cloned(),
                    bundle_pin: b.get(id).cloned(),
                },
            )
        })
        .collect();
    std::iter::once(original_pins)
        .chain(chain.successors.iter().map(|l| l.to.clone()))
        .collect()
}

/// A revert moves only to a state approved before (§6; the reader enforces the
/// same rule, `check_chain`).
fn check_revert_target(
    id: &str,
    chain: Option<&ApprovalChain>,
    to: &BTreeMap<String, ApprovalPin>,
) -> Result<()> {
    let Some(chain) = chain else {
        return Err(Refused::new(
            "E_MIGRATE_REVERT_UNAPPROVED",
            format!("{id} runs its original approval; there is nothing earlier to revert to"),
        ));
    };
    if !approved_states(chain).contains(to) {
        return Err(Refused::new(
            "E_MIGRATE_REVERT_UNAPPROVED",
            format!(
                "{id}'s candidate moves to pins that were never approved before; a revert goes back only to the original approval's pins or an earlier successor's — prepare the candidate with --to <agent>@sha256:<earlier digest>"
            ),
        ));
    }
    Ok(())
}

/// The run an automatic revert names must be a recorded run of this app,
/// under the CURRENT carried-forward approval (successor `seq`), that failed:
/// its trace ends `run-end` with a status other than `ok`/`interrupted`, or
/// records a node error. A claim the CLI cannot find is not a failure (Codex
/// review of #628 PR3b, round 1). Returns what was found, for the evidence.
fn recorded_failure(
    paths: &Paths,
    app: &str,
    reason: &str,
    seq: usize,
    evidence_digest: &str,
) -> Result<serde_json::Value> {
    let not = |why: String| Refused::new("E_MIGRATE_NOT_ELIGIBLE", why);
    let run = reason.strip_prefix("run-failed:").unwrap_or("").trim();
    if !crate::manifest::loader::is_safe_segment(run)
        || !crate::manifest::loader::is_safe_segment(app)
    {
        return Err(not(format!(
            "--reason {reason:?} must name a run id: run-failed:<run-id>"
        )));
    }
    let app_logs = paths.logs_dir().join(app);
    let mut trace = None;
    if let Ok(instances) = std::fs::read_dir(&app_logs) {
        for instance in instances.flatten() {
            let candidate = instance.path().join(format!("{run}.jsonl"));
            if candidate.is_file() {
                trace = Some(candidate);
                break;
            }
        }
    }
    let Some(trace) = trace else {
        return Err(not(format!(
            "no run {run} of {app} is recorded, so there is no failed run to revert after; a person can revert it"
        )));
    };
    let text = std::fs::read_to_string(&trace).map_err(|e| io(&trace, e))?;
    use crate::runtime::provenance::RunEvent;
    let events: Vec<RunEvent> = text
        .lines()
        .filter_map(|line| serde_json::from_str(line).ok())
        .collect();
    let ran_under = events.iter().find_map(|e| match e {
        RunEvent::RunStart { config, .. } => Some(config["approval"].clone()),
        _ => None,
    });
    let under_current = ran_under.as_ref().is_some_and(|a| {
        // seq alone is not unique (a recompile restarts it at 1); the link's
        // evidence digest is (review round 2).
        a["origin"].as_str() == Some("successor")
            && a["seq"].as_u64() == u64::try_from(seq).ok()
            && a["evidence-digest"].as_str() == Some(evidence_digest)
    });
    if !under_current {
        return Err(not(format!(
            "run {run} of {app} did not run under the approval being reverted (successor {seq}), so its failure says nothing about it; a person can revert it"
        )));
    }
    let status = events.iter().rev().find_map(|e| match e {
        RunEvent::RunEnd { status, .. } => Some(status.clone()),
        _ => None,
    });
    let node_error = events
        .iter()
        .any(|e| matches!(e, RunEvent::NodeError { .. }));
    let failed = node_error
        || status
            .as_deref()
            .is_some_and(|s| s != "ok" && s != "interrupted");
    if !failed {
        return Err(not(format!(
            "run {run} of {app} is not recorded as failed (it ended {}); an automatic revert follows only a failed run — a person can revert it",
            status.as_deref().unwrap_or("without a run-end record")
        )));
    }
    Ok(serde_json::json!({
        "run-id": run,
        "trace": trace.display().to_string(),
        "status": status,
        "node-error": node_error,
    }))
}

/// The authority of an automatic revert (§11, §15): only the policy that
/// carried the current approval forward can move it back, only after a failed
/// run, and only to the original's pins or a person-carried successor's.
fn automatic_authority(
    id: &str,
    source_dir: &Path,
    chain: Option<&ApprovalChain>,
    to: &BTreeMap<String, ApprovalPin>,
    to_plan: &str,
    reason: &str,
) -> Result<CarriedForwardBy> {
    let run = reason.strip_prefix("run-failed:").unwrap_or("");
    if run.trim().is_empty() {
        return Err(Refused::new(
            "E_MIGRATE_NOT_ELIGIBLE",
            "--automatic needs --reason run-failed:<run-id>, naming the run that failed",
        ));
    }
    let Some(chain) = chain else {
        return Err(Refused::new(
            "E_MIGRATE_REVERT_UNAPPROVED",
            format!("{id} runs its original approval; there is nothing to revert"),
        ));
    };
    let Some(last) = chain.successors.last() else {
        return Err(Refused::new(
            "E_MIGRATE_REVERT_UNAPPROVED",
            format!("{id} has no successor to revert"),
        ));
    };
    let CarriedForwardBy::Policy { .. } = &last.carried_forward_by else {
        return Err(Refused::new(
            "E_MIGRATE_NOT_ELIGIBLE",
            format!(
                "{id}'s current approval was carried forward by a person; only a person can revert it (use --person)"
            ),
        ));
    };
    // The PLAN a person approved at those pins — not the pins alone: the same
    // source and bytes recompiled by a newer compiler can be a different plan
    // nobody approved (§15.1 R2).
    let mut approved_plans = Vec::new();
    let original = approved_states(chain)
        .into_iter()
        .next()
        .unwrap_or_default();
    if &original == to
        && let Some(rel) = approval::archive_rel(&chain.original.lock_digest, "lock")
        && let Ok(bytes) = std::fs::read(source_dir.join(rel))
        && lock_digest(&bytes) == chain.original.lock_digest
        && let Ok(lock) = serde_yaml::from_slice::<LockFile>(&bytes)
        && let Ok(plan) = plan_digest(&lock)
    {
        approved_plans.push(plan);
    }
    for link in &chain.successors {
        if matches!(link.carried_forward_by, CarriedForwardBy::Person { .. }) && &link.to == to {
            approved_plans.push(link.to_plan_digest.clone());
        }
    }
    if !approved_plans.iter().any(|plan| plan == to_plan) {
        return Err(Refused::new(
            "E_MIGRATE_NOT_ELIGIBLE",
            format!(
                "an automatic revert of {id} goes back only to a plan a person approved — its original approval's, or one a person carried forward — and this candidate is not one of them; a person can approve it"
            ),
        ));
    }
    Ok(last.carried_forward_by.clone())
}

#[cfg(test)]
mod tests;
