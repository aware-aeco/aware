//! The store reference table (#629-a, plan §3): for every package in
//! `agent-store-v2/`, what still needs it — and so whether `aware agent gc`
//! (#629-b) may remove it. Read-only: building the table writes nothing (the
//! one exception is the swap recovery any reader of a working copy performs).
//!
//! A package is **kept** while anything references it without expiry (its
//! agent's working copy, an approved lock, a migration candidate, a promotion
//! in progress, a run lease, a lease left by a run that ended without
//! releasing it, the approval record of an app on HOLD), **in-window** while
//! only expiring references remain (an archived or superseded approval, the
//! package's own recent use), and **removable** once the recovery window has
//! passed with nothing needing it.
//!
//! The table fails closed: anything it could not read that might have held a
//! reference (an unparseable lock, candidate or archive, an approval record of
//! an unknown format, an unreadable app directory, a missing registered root,
//! an unreadable store directory or lease) is a **blocker**, and a table with
//! a blocker is `complete: false` — GC then removes nothing.
//!
//! Where locks are found: every app directory under `AWARE_HOME/apps/`, and
//! every directory a front door registered with `aware agent refs roots add`
//! (`agent-store-control/roots.yaml`), walked the same way. A lock AWARE cannot
//! see is unprotected: after GC its pin is reported not installed, and nothing
//! else is ever run in its place.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use chrono::{DateTime, Utc};
use fs2::FileExt;
use serde::{Deserialize, Serialize};

use super::RefGuard;
use crate::app_lock::LockFile;
use crate::app_lock::approval::{APPROVAL_FORMAT, ApprovalPin};
use crate::error::AwareError;
use crate::paths::Paths;

/// The `aware agent refs --json` schema.
pub const REFS_FORMAT: &str = "aware.agent-refs/v1";
/// The registered-roots file format.
pub const ROOTS_FORMAT: &str = "aware.agent-store-roots/v1";
/// The recovery window when neither `--recovery-window` nor `config.yaml`
/// sets one (plan §7 Q1).
pub const DEFAULT_WINDOW: &str = "30d";
/// How deep below a root the walk looks for app directories.
const MAX_DEPTH: usize = 8;
/// Directory names the walk never descends into.
const SKIPPED_DIRS: [&str; 3] = ["node_modules", "target", "__pycache__"];

// ---------------------------------------------------------------- window

/// A recovery window: `<n>{s,m,h,d}` (`0s` allowed).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Window {
    text: String,
    seconds: i64,
}

impl Window {
    pub fn parse(text: &str) -> Result<Self, AwareError> {
        let invalid = || {
            AwareError::Validation(format!(
                "[E_AGENT_REFS_WINDOW_INVALID] recovery window {text:?} is not <number><s|m|h|d> (e.g. 30d, 12h, 0s)"
            ))
        };
        let text = text.trim();
        let unit = text.chars().last().ok_or_else(invalid)?;
        let per = match unit {
            's' => 1,
            'm' => 60,
            'h' => 3_600,
            'd' => 86_400,
            _ => return Err(invalid()),
        };
        let number = &text[..text.len() - unit.len_utf8()];
        if number.is_empty() || !number.bytes().all(|b| b.is_ascii_digit()) {
            return Err(invalid());
        }
        let n: i64 = number.parse().map_err(|_| invalid())?;
        // A window beyond ~100 years is a typo, and would overflow the clock.
        let seconds = n.checked_mul(per).filter(|s| *s <= 36_500 * 86_400);
        Ok(Self {
            text: text.to_string(),
            seconds: seconds.ok_or_else(invalid)?,
        })
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn duration(&self) -> chrono::Duration {
        chrono::Duration::seconds(self.seconds)
    }
}

/// The recovery window: `--recovery-window` when given, else `config.yaml`'s
/// `agent-store.recovery-window`, else [`DEFAULT_WINDOW`]. A config file that
/// cannot be read or names a malformed window is an error, never a silent
/// fall-back to the default.
pub fn recovery_window(paths: &Paths, flag: Option<&str>) -> Result<Window, AwareError> {
    if let Some(flag) = flag {
        return Window::parse(flag);
    }
    let path = paths.config_path();
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Window::parse(DEFAULT_WINDOW);
        }
        Err(error) => {
            return Err(
                std::io::Error::new(error.kind(), format!("{}: {error}", path.display())).into(),
            );
        }
    };
    let config: serde_yaml::Value = serde_yaml::from_str(&text).map_err(|error| {
        AwareError::Validation(format!(
            "[E_AGENT_REFS_WINDOW_INVALID] {} is not valid YAML: {error}",
            path.display()
        ))
    })?;
    match config
        .get("agent-store")
        .and_then(|store| store.get("recovery-window"))
    {
        None => Window::parse(DEFAULT_WINDOW),
        Some(serde_yaml::Value::String(window)) => Window::parse(window),
        Some(other) => Err(AwareError::Validation(format!(
            "[E_AGENT_REFS_WINDOW_INVALID] agent-store.recovery-window in {} must be a string like \"30d\", not {other:?}",
            path.display()
        ))),
    }
}

// ----------------------------------------------------------------- roots

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct RootsFile {
    format: String,
    #[serde(default)]
    roots: Vec<RegisteredRoot>,
}

/// A directory a front door asked AWARE to search for locks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct RegisteredRoot {
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    pub added_at: String,
}

fn roots_path(paths: &Paths) -> PathBuf {
    paths.agent_store_control_dir().join("roots.yaml")
}

/// The registered roots. A missing file is none; one that cannot be read or
/// parsed is an error (the table turns it into a blocker).
pub fn read_roots(paths: &Paths) -> Result<Vec<RegisteredRoot>, AwareError> {
    let path = roots_path(paths);
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => {
            return Err(
                std::io::Error::new(error.kind(), format!("{}: {error}", path.display())).into(),
            );
        }
    };
    let file: RootsFile = serde_yaml::from_str(&text).map_err(|error| {
        AwareError::Validation(format!(
            "[E_AGENT_REFS_ROOTS_INVALID] {} is malformed: {error}",
            path.display()
        ))
    })?;
    if file.format != ROOTS_FORMAT {
        return Err(AwareError::Validation(format!(
            "[E_AGENT_REFS_ROOTS_INVALID] {} has format {:?}, not {ROOTS_FORMAT}",
            path.display(),
            file.format
        )));
    }
    Ok(file.roots)
}

/// The form a root is recorded and compared in: absolute, as given.
fn normalize_root(dir: &Path) -> Result<String, AwareError> {
    std::path::absolute(dir)
        .map(|p| p.display().to_string())
        .map_err(|e| AwareError::Validation(format!("{}: {e}", dir.display())))
}

/// Run `change` on the roots list under `roots.flock`, then publish it
/// atomically. Held under the store guard (plan §8 R1-7: every reference
/// mutation happens under the shared guard, so GC's exclusive lock never sees
/// half of one).
fn edit_roots<T>(
    paths: &Paths,
    guard: &RefGuard,
    change: impl FnOnce(&mut Vec<RegisteredRoot>) -> T,
) -> Result<T, AwareError> {
    super::guard::require_home(guard, paths)?;
    let control = paths.agent_store_control_dir();
    std::fs::create_dir_all(&control)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", control.display())))?;
    let lock_path = control.join("roots.flock");
    let lock = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(&lock_path)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", lock_path.display())))?;
    lock.lock_exclusive()
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", lock_path.display())))?;
    let mut roots = read_roots(paths)?;
    let out = change(&mut roots);
    let file = RootsFile {
        format: ROOTS_FORMAT.to_string(),
        roots,
    };
    let text = serde_yaml::to_string(&file)
        .map_err(|e| AwareError::Internal(format!("serialize roots: {e}")))?;
    let path = roots_path(paths);
    crate::app_lock::replace_atomically(&path, text.as_bytes())
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", path.display())))?;
    Ok(out)
}

/// Register `dir` as a place locks live. `Ok((root, false))` when it already
/// was (its label is updated when a new one is given).
pub fn add_root(
    paths: &Paths,
    guard: &RefGuard,
    dir: &Path,
    label: Option<&str>,
) -> Result<(RegisteredRoot, bool), AwareError> {
    if !dir.is_dir() {
        return Err(AwareError::Validation(format!(
            "[E_AGENT_REFS_ROOT_INVALID] {} is not a directory",
            dir.display()
        )));
    }
    let path = normalize_root(dir)?;
    edit_roots(paths, guard, |roots| {
        if let Some(existing) = roots.iter_mut().find(|r| r.path == path) {
            if let Some(label) = label {
                existing.label = Some(label.to_string());
            }
            return (existing.clone(), false);
        }
        let root = RegisteredRoot {
            path: path.clone(),
            label: label.map(str::to_string),
            added_at: Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        };
        roots.push(root.clone());
        (root, true)
    })
}

/// Unregister `dir`. `Ok(false)` when it was not registered. The directory
/// need not exist any more — removing a deleted root is how a person clears
/// its blocker.
pub fn remove_root(paths: &Paths, guard: &RefGuard, dir: &Path) -> Result<bool, AwareError> {
    let path = normalize_root(dir)?;
    edit_roots(paths, guard, |roots| {
        let before = roots.len();
        roots.retain(|r| r.path != path);
        roots.len() != before
    })
}

// ----------------------------------------------------------------- table

/// Why a package is still needed.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum Reference {
    /// The working copy `agents/<id>` has exactly these bytes.
    Current { path: String },
    /// The working copy could not be hashed: every package of the agent is kept.
    #[serde(rename_all = "kebab-case")]
    CurrentUnhashable { path: String, problem: String },
    /// An approved `<app>.lock` pins these bytes.
    ApprovedLock { lock: String },
    /// A migration candidate pins these bytes.
    CandidateLock { candidate: String },
    /// A migration candidate moves a pin away from these bytes.
    CandidateBase { evidence: String },
    /// A promotion that has not finished stages a lock pinning these bytes.
    PromotionInProgress { path: String },
    /// An archived approval pins these bytes; `until` absent while the app is
    /// on HOLD.
    #[serde(rename_all = "kebab-case")]
    ApprovalArchive {
        archive: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        until: Option<String>,
        #[serde(skip_serializing_if = "std::ops::Not::not")]
        held: bool,
    },
    /// The original approval of a carried-forward lock pinned these bytes.
    #[serde(rename_all = "kebab-case")]
    ApprovalOriginal {
        lock: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        until: Option<String>,
        #[serde(skip_serializing_if = "std::ops::Not::not")]
        held: bool,
    },
    /// A successor link of a carried-forward lock moved away from these bytes.
    #[serde(rename_all = "kebab-case")]
    SuccessorFrom {
        lock: String,
        seq: u32,
        #[serde(skip_serializing_if = "Option::is_none")]
        until: Option<String>,
        #[serde(skip_serializing_if = "std::ops::Not::not")]
        held: bool,
    },
    /// A run in progress is using these bytes.
    #[serde(rename_all = "kebab-case")]
    Lease { run_id: String, app: String },
    /// A run ended without releasing its lease: kept until GC records when it
    /// was last needed and removes the lease (plan §9 R2-5).
    #[serde(rename_all = "kebab-case")]
    StaleLeaseUnstamped {
        run_id: String,
        app: String,
        path: String,
    },
    /// Snapshotted or last needed recently.
    Recent { until: String },
}

/// What GC may do with a package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum State {
    Kept,
    InWindow,
    Removable,
}

/// One verified store package.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct PackageRow {
    pub agent: String,
    pub version: String,
    pub digest: String,
    pub receipt_key: String,
    pub path: String,
    pub bytes: u64,
    pub snapshotted_at: Option<String>,
    pub last_needed_at: Option<String>,
    pub state: State,
    /// For `in-window`: when the last expiring reference runs out.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kept_until: Option<String>,
    pub references: Vec<Reference>,
}

/// A store package that fails verification. Referenced: kept as evidence.
/// Unreferenced: removable after the window ("repair is GC's job", #626).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct InvalidPackage {
    pub agent: String,
    pub digest: String,
    pub receipt_key: String,
    pub path: String,
    pub bytes: u64,
    pub reason: String,
    pub state: State,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kept_until: Option<String>,
    pub references: Vec<Reference>,
}

/// Something in the store that is not a package.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Leftover {
    pub path: String,
    /// `temp` (an interrupted snapshot), `trash` (an interrupted GC removal)
    /// or `unrecognized` (never touched; reported so a person can look).
    pub kind: &'static str,
}

/// Something the table could not read that might have held a reference.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Blocker {
    pub path: String,
    pub problem: String,
}

/// One place locks were looked for.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RootRow {
    pub path: String,
    /// `apps` (AWARE's own) or `registered`.
    pub kind: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// `ok`, `missing` or `unreadable`.
    pub status: &'static str,
    /// Locks found below it.
    pub locks: usize,
    /// Links and junctions below it that were not followed: locks behind them
    /// are not protected.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub links_not_followed: Vec<String>,
}

/// The older CLIs' store, which nothing new writes or removes.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct LegacyStore {
    pub path: String,
    pub bytes: u64,
}

/// `aware agent refs --json`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct RefTable {
    pub format: &'static str,
    pub generated_at: String,
    pub recovery_window: String,
    pub complete: bool,
    pub roots: Vec<RootRow>,
    pub blockers: Vec<Blocker>,
    pub packages: Vec<PackageRow>,
    pub invalid_packages: Vec<InvalidPackage>,
    pub leftovers: Vec<Leftover>,
    pub stale_leases: Vec<super::lease::LeaseRow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legacy_store: Option<LegacyStore>,
}

/// What a reference keeps: one package digest of an agent, or every package
/// of the agent.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Scope {
    Digest(String, String),
    Agent(String),
}

/// A reference with its expiry (`None`: no expiry).
struct Found {
    scope: Scope,
    reference: Reference,
    until: Option<DateTime<Utc>>,
}

struct Collector {
    window: chrono::Duration,
    now: DateTime<Utc>,
    found: Vec<Found>,
    blockers: Vec<Blocker>,
    links: Vec<String>,
    locks: usize,
}

fn stamp(time: DateTime<Utc>) -> String {
    time.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

fn parse_time(text: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(text.trim())
        .ok()
        .map(|t| t.with_timezone(&Utc))
}

fn modified(path: &Path) -> Option<DateTime<Utc>> {
    std::fs::symlink_metadata(path)
        .and_then(|m| m.modified())
        .ok()
        .map(DateTime::<Utc>::from)
}

impl Collector {
    fn block(&mut self, path: &Path, problem: impl Into<String>) {
        self.blockers.push(Blocker {
            path: path.display().to_string(),
            problem: problem.into(),
        });
    }

    fn keep(&mut self, id: &str, digest: &str, reference: Reference, until: Option<DateTime<Utc>>) {
        // A pin whose digest is not a sha256 cannot name a store package; the
        // run refuses such a lock, so it keeps nothing.
        if super::digest_hex(digest).is_none() {
            return;
        }
        self.found.push(Found {
            scope: Scope::Digest(id.to_string(), digest.to_string()),
            reference,
            until,
        });
    }

    fn keep_lock_pins(&mut self, lock: &LockFile, reference: impl Fn() -> Reference) {
        for id in lock.agent_pins.keys() {
            if let Some(digest) = crate::app_lock::pinned_digest(lock, id) {
                self.keep(id, digest, reference(), None);
            }
        }
    }

    fn keep_pins(
        &mut self,
        pins: &BTreeMap<String, ApprovalPin>,
        until: Option<DateTime<Utc>>,
        reference: impl Fn(Option<String>) -> Reference,
    ) {
        for (id, pin) in pins {
            if let Some(digest) = pin.effective_digest() {
                self.keep(id, digest, reference(until.map(stamp)), until);
            }
        }
    }

    fn read_lock(&mut self, path: &Path) -> Option<LockFile> {
        let parsed = std::fs::read(path)
            .map_err(|e| format!("cannot read it: {e}"))
            .and_then(|bytes| {
                serde_yaml::from_slice::<LockFile>(&bytes)
                    .map_err(|e| format!("it is not a lock AWARE can read: {e}"))
            });
        match parsed {
            Ok(lock) => Some(lock),
            Err(problem) => {
                self.block(path, problem);
                None
            }
        }
    }

    /// The pins of an approved lock, and of its approval record.
    fn approved_lock(&mut self, dir: &Path, path: &Path, held_apps: &[String]) {
        let Some(lock) = self.read_lock(path) else {
            return;
        };
        self.locks += 1;
        let shown = path.display().to_string();
        self.keep_lock_pins(&lock, || Reference::ApprovedLock {
            lock: shown.clone(),
        });
        let Some(chain) = &lock.approval else {
            return;
        };
        if chain.format != APPROVAL_FORMAT {
            self.block(
                path,
                format!(
                    "its approval record has format {}, which this AWARE cannot read",
                    chain.format
                ),
            );
            return;
        }
        if let Err(problem) = crate::app_lock::approval::check_chain(&lock) {
            self.block(
                path,
                format!("its approval record is inconsistent: {problem}"),
            );
            return;
        }
        let held = held_apps.contains(&lock.app) || hold_exists(dir, &lock.app);
        let mut anchors = Vec::with_capacity(chain.successors.len());
        for successor in &chain.successors {
            match parse_time(&successor.promoted_at) {
                Some(at) => anchors.push(at),
                None => {
                    self.block(
                        path,
                        format!(
                            "successor {} has promoted-at {:?}, which is not a time",
                            successor.seq, successor.promoted_at
                        ),
                    );
                    return;
                }
            }
        }
        let window = self.window;
        let until = |at: DateTime<Utc>| (!held).then(|| at + window);
        // The original approval was replaced by the first successor.
        let original_until = anchors.first().copied().and_then(until);
        let original = crate::app_lock::approval::original_pins(&chain.original);
        self.keep_pins(&original, original_until, |until| {
            Reference::ApprovalOriginal {
                lock: shown.clone(),
                until,
                held,
            }
        });
        for (successor, at) in chain.successors.iter().zip(anchors) {
            let seq = successor.seq;
            self.keep_pins(&successor.from, until(at), |until| {
                Reference::SuccessorFrom {
                    lock: shown.clone(),
                    seq,
                    until,
                    held,
                }
            });
        }
    }

    /// Everything in one directory that may reference a package: its approved
    /// locks (only beside an app source), its migration candidates, its
    /// approval archives and any promotion in progress.
    fn app_dir(&mut self, dir: &Path, files: &[PathBuf]) {
        let has_source = files.iter().any(|f| {
            f.extension()
                .and_then(|e| e.to_str())
                .is_some_and(|e| crate::manifest::loader::APP_MANIFEST_EXTENSIONS.contains(&e))
        });
        let approvals = dir.join(crate::migration::files::APPROVALS_DIR);
        let held_apps = holds_in(&approvals);
        let locks: Vec<PathBuf> = if has_source {
            files
                .iter()
                .filter(|f| f.extension().is_some_and(|e| e == "lock"))
                .cloned()
                .collect()
        } else {
            Vec::new()
        };
        for lock in &locks {
            self.approved_lock(dir, lock, &held_apps);
        }
        self.candidates(&dir.join(crate::migration::files::MIGRATION_DIR));
        self.archives(&approvals, !held_apps.is_empty());
    }

    fn candidates(&mut self, dir: &Path) {
        let Some(entries) = self.list(dir) else {
            return;
        };
        for path in entries {
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if name.ends_with(".candidate.lock") {
                if let Some(lock) = self.read_lock(&path) {
                    self.locks += 1;
                    let shown = path.display().to_string();
                    self.keep_lock_pins(&lock, || Reference::CandidateLock {
                        candidate: shown.clone(),
                    });
                }
            } else if name.ends_with(".evidence.json") {
                // The pins a candidate moves away from. The base lock usually
                // still references them; an evidence file that cannot be read
                // leaves the candidate's own pins, which are what it runs.
                let Some(evidence) = std::fs::read(&path).ok().and_then(|b| {
                    serde_json::from_slice::<crate::migration::files::Evidence>(&b).ok()
                }) else {
                    continue;
                };
                let shown = path.display().to_string();
                for (id, moved) in &evidence.header.targets {
                    self.keep(
                        id,
                        &moved.from.digest,
                        Reference::CandidateBase {
                            evidence: shown.clone(),
                        },
                        None,
                    );
                }
            }
        }
    }

    /// An archived lock keeps its pins for the window from when it was
    /// archived (the promotion that replaced it), or for ever while an app in
    /// its folder is on HOLD. The carried-forward lock's own record keeps the
    /// same pins from the same promotion time (`approval-original`,
    /// `successor-from`); the archive alone still counts after a person's
    /// recompile drops that record.
    fn archives(&mut self, dir: &Path, held: bool) {
        let Some(entries) = self.list(dir) else {
            return;
        };
        for path in entries {
            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };
            if name == ".txn" {
                self.promotions(&path);
                continue;
            }
            if !name.ends_with(".lock") {
                continue; // evidence `.json`, HOLD files
            }
            let Some(lock) = self.read_lock(&path) else {
                continue;
            };
            self.locks += 1;
            let anchor = modified(&path).unwrap_or(self.now);
            let until = (!held).then(|| anchor + self.window);
            let shown = path.display().to_string();
            for id in lock.agent_pins.keys() {
                if let Some(digest) = crate::app_lock::pinned_digest(&lock, id) {
                    self.keep(
                        id,
                        digest,
                        Reference::ApprovalArchive {
                            archive: shown.clone(),
                            until: until.map(stamp),
                            held,
                        },
                        until,
                    );
                }
            }
        }
    }

    fn promotions(&mut self, txn_root: &Path) {
        let Some(txns) = self.list(txn_root) else {
            return;
        };
        for txn in txns {
            if !txn.is_dir() {
                continue;
            }
            let Some(files) = self.list(&txn) else {
                continue;
            };
            for path in files {
                if path.extension().is_some_and(|e| e == "lock")
                    && let Some(lock) = self.read_lock(&path)
                {
                    self.locks += 1;
                    let shown = path.display().to_string();
                    self.keep_lock_pins(&lock, || Reference::PromotionInProgress {
                        path: shown.clone(),
                    });
                }
            }
        }
    }

    /// The entries of `dir`; `None` when it does not exist. Unreadable is a
    /// blocker.
    fn list(&mut self, dir: &Path) -> Option<Vec<PathBuf>> {
        match std::fs::read_dir(dir) {
            Ok(entries) => {
                let mut out = Vec::new();
                for entry in entries {
                    match entry {
                        Ok(entry) => out.push(entry.path()),
                        Err(error) => {
                            self.block(dir, format!("cannot list it: {error}"));
                            return None;
                        }
                    }
                }
                out.sort();
                Some(out)
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
            Err(error) => {
                self.block(dir, format!("cannot list it: {error}"));
                None
            }
        }
    }

    /// Walk `dir` and below for app directories, never through a link.
    fn walk(&mut self, dir: &Path, depth: usize) {
        let Some(entries) = self.list(dir) else {
            return;
        };
        let mut files = Vec::new();
        let mut subdirs = Vec::new();
        for path in entries {
            let metadata = match std::fs::symlink_metadata(&path) {
                Ok(metadata) => metadata,
                Err(error) => {
                    self.block(&path, format!("cannot read it: {error}"));
                    continue;
                }
            };
            if metadata.file_type().is_symlink() || crate::fs::is_reparse_point(&metadata) {
                if std::fs::metadata(&path).is_ok_and(|m| m.is_dir()) {
                    self.links.push(path.display().to_string());
                }
                continue;
            }
            if metadata.is_dir() {
                let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                if !name.starts_with('.') && !SKIPPED_DIRS.contains(&name) {
                    subdirs.push(path);
                }
            } else {
                files.push(path);
            }
        }
        self.app_dir(dir, &files);
        if depth < MAX_DEPTH {
            for sub in subdirs {
                self.walk(&sub, depth + 1);
            }
        }
    }

    /// Scan one root; returns its row.
    fn root(&mut self, path: &Path, kind: &'static str, label: Option<String>) -> RootRow {
        let before_locks = self.locks;
        let before_blockers = self.blockers.len();
        let before_links = self.links.len();
        let status = match std::fs::metadata(path) {
            Ok(m) if m.is_dir() => {
                self.walk(path, 0);
                if self.blockers[before_blockers..]
                    .iter()
                    .any(|b| b.path == path.display().to_string())
                {
                    "unreadable"
                } else {
                    "ok"
                }
            }
            Ok(_) => {
                self.block(path, "it is not a directory");
                "unreadable"
            }
            // AWARE's own apps/ may simply not exist yet; a registered root
            // that has gone may have held locks AWARE can no longer see.
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                if kind == "registered" {
                    self.block(
                        path,
                        "this registered folder is missing; locks in it cannot be seen (remove it with `aware agent refs roots remove` if it is gone for good)",
                    );
                }
                "missing"
            }
            Err(error) => {
                self.block(path, format!("cannot read it: {error}"));
                "unreadable"
            }
        };
        RootRow {
            path: path.display().to_string(),
            kind,
            label,
            status,
            locks: self.locks - before_locks,
            links_not_followed: self.links[before_links..].to_vec(),
        }
    }
}

fn hold_exists(dir: &Path, app: &str) -> bool {
    crate::migration::files::hold_path(dir, app).exists()
}

/// The apps with a `HOLD.<app>` file in an approvals directory.
fn holds_in(approvals: &Path) -> Vec<String> {
    let prefix = format!("{}.", crate::migration::files::HOLD_FILE);
    std::fs::read_dir(approvals)
        .map(|entries| {
            entries
                .flatten()
                .filter_map(|e| e.file_name().to_str().map(str::to_string))
                .filter_map(|name| name.strip_prefix(&prefix).map(str::to_string))
                .collect()
        })
        .unwrap_or_default()
}

/// Total size of the regular files under `dir`, never through a link.
fn bytes_under(dir: &Path) -> u64 {
    let mut total = 0;
    let mut stack = vec![dir.to_path_buf()];
    while let Some(d) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&d) else {
            continue;
        };
        for entry in entries.flatten() {
            let Ok(metadata) = std::fs::symlink_metadata(entry.path()) else {
                continue;
            };
            if metadata.file_type().is_symlink() || crate::fs::is_reparse_point(&metadata) {
                continue;
            }
            if metadata.is_dir() {
                stack.push(entry.path());
            } else {
                total += metadata.len();
            }
        }
    }
    total
}

/// The references, expiry and state of package `(id, digest)`.
fn judge(
    found: &[Found],
    id: &str,
    digest: &str,
    recent: Option<DateTime<Utc>>,
    now: DateTime<Utc>,
) -> (State, Option<String>, Vec<Reference>) {
    let mut references = Vec::new();
    let mut forever = false;
    let mut until: Option<DateTime<Utc>> = None;
    for f in found {
        let applies = match &f.scope {
            Scope::Digest(fid, fdigest) => fid == id && fdigest == digest,
            Scope::Agent(fid) => fid == id,
        };
        if !applies {
            continue;
        }
        match f.until {
            None => forever = true,
            Some(u) if u > now => until = Some(until.map_or(u, |c| c.max(u))),
            Some(_) => continue, // expired: no longer a reason
        }
        references.push(f.reference.clone());
    }
    if let Some(recent) = recent.filter(|r| *r > now) {
        until = Some(until.map_or(recent, |c| c.max(recent)));
        references.push(Reference::Recent {
            until: stamp(recent),
        });
    }
    references.sort();
    references.dedup();
    if forever {
        (State::Kept, None, references)
    } else if let Some(until) = until {
        (State::InWindow, Some(stamp(until)), references)
    } else {
        (State::Removable, None, references)
    }
}

/// Build the reference table. The caller holds the store guard (shared for
/// a report, exclusive for GC) for as long as it relies on the answer.
pub fn table(
    paths: &Paths,
    guard: &RefGuard,
    window: &Window,
    now: DateTime<Utc>,
) -> Result<RefTable, AwareError> {
    super::guard::require_home(guard, paths)?;
    let mut c = Collector {
        window: window.duration(),
        now,
        found: Vec::new(),
        blockers: Vec::new(),
        links: Vec::new(),
        locks: 0,
    };

    // Locks: AWARE's apps/, then every registered root.
    let mut roots = vec![c.root(&paths.apps_dir(), "apps", None)];
    match read_roots(paths) {
        Ok(registered) => {
            for root in registered {
                roots.push(c.root(Path::new(&root.path), "registered", root.label));
            }
        }
        Err(error) => c.block(&roots_path(paths), error.to_string()),
    }

    // Working copies: each one's exact bytes, read under its swap lock.
    current_copies(&mut c, paths, guard);

    // Runs in progress, and runs that ended without letting go.
    let mut stale_leases = Vec::new();
    match super::lease::list(paths) {
        Ok(leases) => {
            for lease in &leases.leases {
                for package in &lease.packages {
                    c.keep(
                        &package.agent,
                        &package.digest,
                        Reference::Lease {
                            run_id: lease.run_id.clone(),
                            app: lease.app.clone(),
                        },
                        None,
                    );
                }
            }
            for lease in &leases.stale {
                for package in &lease.packages {
                    c.keep(
                        &package.agent,
                        &package.digest,
                        Reference::StaleLeaseUnstamped {
                            run_id: lease.run_id.clone(),
                            app: lease.app.clone(),
                            path: lease.path.clone(),
                        },
                        None,
                    );
                }
            }
            for unreadable in &leases.unreadable {
                c.block(
                    Path::new(&unreadable.path),
                    format!(
                        "this run lease cannot be read ({}); the packages it holds are unknown",
                        unreadable.problem
                    ),
                );
            }
            stale_leases = leases.stale;
        }
        Err(error) => c.block(&super::lease::leases_dir(paths), error.to_string()),
    }

    // The store itself.
    let (packages, invalid_packages, leftovers) = store(&mut c, paths);

    let legacy = paths.legacy_agent_store_dir();
    let legacy_store = legacy.is_dir().then(|| LegacyStore {
        bytes: bytes_under(&legacy),
        path: legacy.display().to_string(),
    });

    c.blockers.sort_by(|a, b| a.path.cmp(&b.path));
    c.blockers.dedup();
    Ok(RefTable {
        format: REFS_FORMAT,
        generated_at: stamp(now),
        recovery_window: window.text().to_string(),
        complete: c.blockers.is_empty(),
        roots,
        blockers: c.blockers,
        packages,
        invalid_packages,
        leftovers,
        stale_leases,
        legacy_store,
    })
}

fn current_copies(c: &mut Collector, paths: &Paths, guard: &RefGuard) {
    let agents = paths.agents_dir();
    let Some(entries) = c.list(&agents) else {
        return;
    };
    for path in entries {
        let Some(id) = path
            .file_name()
            .and_then(|n| n.to_str())
            .map(str::to_string)
        else {
            continue;
        };
        if id.starts_with('.')
            || crate::install::swap::is_swap_area(&id)
            || !crate::manifest::loader::is_safe_segment(&id)
            || !path.is_dir()
        {
            continue;
        }
        let shown = path.display().to_string();
        let digest =
            crate::install::swap::read_lock(paths, guard, &[id.as_str()]).and_then(|held| {
                let digest = crate::install::integrity::tree_digest(&path);
                drop(held);
                digest
            });
        match digest {
            Ok(digest) => c.keep(&id, &digest, Reference::Current { path: shown }, None),
            Err(error) => c.found.push(Found {
                scope: Scope::Agent(id),
                reference: Reference::CurrentUnhashable {
                    path: shown,
                    problem: error.to_string(),
                },
                until: None,
            }),
        }
    }
}

#[allow(clippy::type_complexity)]
fn store(
    c: &mut Collector,
    paths: &Paths,
) -> (Vec<PackageRow>, Vec<InvalidPackage>, Vec<Leftover>) {
    let mut packages = Vec::new();
    let mut invalid = Vec::new();
    let mut leftovers = Vec::new();
    let root = paths.agent_store_dir();
    let unrecognized = |path: &Path| Leftover {
        path: path.display().to_string(),
        kind: "unrecognized",
    };
    let Some(ids) = c.list(&root) else {
        return (packages, invalid, leftovers);
    };
    for id_dir in ids {
        let id = id_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        if !crate::manifest::loader::is_safe_segment(&id) || !id_dir.is_dir() {
            leftovers.push(unrecognized(&id_dir));
            continue;
        }
        let Some(containers) = c.list(&id_dir) else {
            continue;
        };
        for container in containers {
            let hex = container
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
            let digest = format!("sha256:{hex}");
            if super::digest_hex(&digest).is_none() || !container.is_dir() {
                leftovers.push(unrecognized(&container));
                continue;
            }
            // Formed by the one function that refuses a link below the store.
            if let Err(error) = super::digest_container(paths, &id, &digest) {
                c.block(&container, error.to_string());
                continue;
            }
            let Some(entries) = c.list(&container) else {
                continue;
            };
            for entry in entries {
                let name = entry
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();
                if name.starts_with(super::TEMP_PREFIX) {
                    leftovers.push(Leftover {
                        path: entry.display().to_string(),
                        kind: "temp",
                    });
                    continue;
                }
                if name.starts_with(TRASH_PREFIX) {
                    leftovers.push(Leftover {
                        path: entry.display().to_string(),
                        kind: "trash",
                    });
                    continue;
                }
                if name.starts_with(TOMBSTONE_PREFIX) {
                    continue; // a record of a removal (#629-b)
                }
                if !super::is_receipt_key(&name) {
                    leftovers.push(unrecognized(&entry));
                    continue;
                }
                let record = std::fs::read_to_string(entry.join(super::PACKAGE_FILE))
                    .ok()
                    .and_then(|t| serde_yaml::from_str::<super::PackageMetadata>(&t).ok());
                let snapshotted = record.as_ref().and_then(|r| parse_time(&r.snapshotted_at));
                let last_needed = super::stamps::last_needed(paths, &id, &digest);
                let recent = [
                    snapshotted.or_else(|| modified(&entry)),
                    last_needed.as_deref().and_then(parse_time),
                ]
                .into_iter()
                .flatten()
                .max()
                .map(|at| at + c.window);
                let (state, kept_until, references) = judge(&c.found, &id, &digest, recent, c.now);
                let bytes = bytes_under(&entry);
                match super::verify_package(&entry, &id, &digest, &name) {
                    Ok(package) => packages.push(PackageRow {
                        agent: package.agent,
                        version: package.version,
                        digest: digest.clone(),
                        receipt_key: name,
                        path: entry.display().to_string(),
                        bytes,
                        snapshotted_at: record.map(|r| r.snapshotted_at),
                        last_needed_at: last_needed,
                        state,
                        kept_until,
                        references,
                    }),
                    Err(reason) => invalid.push(InvalidPackage {
                        agent: id.clone(),
                        digest: digest.clone(),
                        receipt_key: name,
                        path: entry.display().to_string(),
                        bytes,
                        reason,
                        state,
                        kept_until,
                        references,
                    }),
                }
            }
        }
    }
    packages.sort_by(|a, b| {
        (&a.agent, &a.version, &a.digest, &a.receipt_key).cmp(&(
            &b.agent,
            &b.version,
            &b.digest,
            &b.receipt_key,
        ))
    });
    (packages, invalid, leftovers)
}

/// Prefix of a package GC has renamed away and not yet deleted (#629-b).
pub const TRASH_PREFIX: &str = ".trash-";
/// Prefix of GC's record of a removed package (#629-b).
pub const TOMBSTONE_PREFIX: &str = ".removed-";

#[cfg(test)]
mod tests;
