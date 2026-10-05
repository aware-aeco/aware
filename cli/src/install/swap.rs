//! Journaled agent swaps and per-agent swap locks (#627).
//!
//! Every writer of a current working copy `agents/<id>/` — registry and local
//! install, update (including a dotted-id rename's second directory), agent
//! uninstall, and the synthesized agent an `exposes-as-agent` app writes or
//! removes on app install / uninstall / rename / duplicate — changes it through
//! ONE transaction shape, so `agents/<id>` is always a complete old tree or a
//! complete new tree, never a partial one and (to a locked reader) never absent:
//!
//! ```text
//! agents/.aware-swap/<txn>/                 # same volume as agents/<id>, two levels deep
//!     incoming/                             # the new tree, staged + verified + snapshotted
//!     intent.json                           # ids (sorted), new-name, incoming digest, outgoing
//!     journal.log                           # `pending <step>` before each rename, `done <step>` after
//!     outgoing-<id>/                        # the old trees, moved aside whole
//! agents/.aware-swap/locks/<id>.flock       # per-agent swap lock (fs2)
//! ```
//!
//! **Order.** Stage the incoming tree (outside any lock) → take the swap locks
//! of every id the swap touches, exclusive and sorted (store lock first — the
//! API takes `&RefGuard`) → recover any crashed transaction naming those ids →
//! the caller's checks + snapshots of every outgoing copy → write `intent.json`
//! (temp + rename, fsynced) → for each outgoing id: `pending out <id>`, rename
//! `agents/<id>` → `outgoing-<id>` (no-replace, write-through), `done out <id>`
//! → `pending in <name>`, rename `incoming` → `agents/<name>`, `done in <name>`
//! → `commit` → delete the transaction directory.
//!
//! **Recovery** (whoever next takes the locks: a writer, a run's or compile's
//! reader, `aware doctor`) holds the swap locks of the transaction's WHOLE id
//! set, then reconciles every `pending` step by looking at the paths and the
//! incoming digest rather than trusting the journal alone. A recorded `commit`,
//! or a move-in that physically happened (the incoming tree is gone and
//! `agents/<name>` hashes to the intent's digest), or — for an uninstall — every
//! outgoing tree physically moved out, commits forward: the outgoing copies are
//! deleted. Anything else rolls the whole set back: every moved-out tree is
//! renamed back (journaled `pending restore` / `done restore`) and the incoming
//! tree dropped. A state recovery cannot explain (both copies present, an
//! outgoing copy missing) is reported and left exactly as it is.
//!
//! **Readers** of the current copy — a run's preflight hashing / snapshotting
//! it, compile snapshotting it — take its swap lock SHARED for exactly that
//! long ([`read_lock`]). A reader first recovers any crashed transaction naming
//! its ids (taking that transaction's whole id set exclusive), so a reader never
//! needs `aware doctor` or another writer to see the agent again. Readers that
//! take no lock (`agent list`, `app check`, FloLess reading `agents/`) may, for
//! the microseconds between the two renames of a live swap, see the agent
//! absent; they never see a partial tree.

use std::collections::BTreeSet;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

use fs2::FileExt;
use serde::{Deserialize, Serialize};

use crate::agent_store::RefGuard;
use crate::error::AwareError;
use crate::paths::Paths;

/// The swap area's directory name under `agents/`.
pub const SWAP_DIR: &str = ".aware-swap";

/// Whether a name under `agents/` is the swap area — case-insensitively, as
/// a case-insensitive filesystem would resolve it.
pub fn is_swap_area(name: &str) -> bool {
    name.eq_ignore_ascii_case(SWAP_DIR)
}
const LOCKS_DIR: &str = "locks";
const INTENT_FILE: &str = "intent.json";
const JOURNAL_FILE: &str = "journal.log";
const INCOMING_DIR: &str = "incoming";
const OUTGOING_PREFIX: &str = "outgoing-";
const INTENT_FORMAT: &str = "aware.agent-swap/v1";
const COMMIT: &str = "commit";
const ROLLED_BACK: &str = "rolled-back";
/// Prefix of a settled transaction being deleted: never a transaction name.
const SETTLED_PREFIX: &str = ".done-";

/// What a transaction does — informational; recovery decides from the paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Op {
    Install,
    Update,
    Uninstall,
    /// A synthesized (app-backed) agent written over its previous copy.
    Replace,
}

/// A tree a swap moves out of `agents/`, with its digest when it could be hashed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Outgoing {
    pub id: String,
    pub digest: Option<String>,
}

/// `intent.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Intent {
    pub format: String,
    pub txn: String,
    pub op: Op,
    /// Every id whose swap lock the writer holds, sorted. Recovery takes all of them.
    pub ids: Vec<String>,
    pub new_name: Option<String>,
    pub incoming_digest: Option<String>,
    pub outgoing: Vec<Outgoing>,
    pub started_at: String,
    pub pid: u32,
}

// ── swap locks ────────────────────────────────────────────────────────────────

/// Held swap locks. Borrow the [`RefGuard`] they were taken under, so a swap
/// lock can neither be taken without the store lock nor outlive it (lock order:
/// store first, then swap locks — enforced by the compiler).
#[derive(Debug)]
pub struct SwapLocks<'g> {
    _files: Vec<std::fs::File>,
    ids: Vec<String>,
    _guard: PhantomData<&'g RefGuard>,
}

impl SwapLocks<'_> {
    pub fn ids(&self) -> &[String] {
        &self.ids
    }
}

/// The ids a swap lock can be taken for: a plain path segment that is not the
/// swap area itself.
fn check_id(id: &str) -> Result<(), AwareError> {
    if crate::manifest::loader::is_safe_segment(id) && !is_swap_area(id) {
        Ok(())
    } else {
        Err(AwareError::NotFound(format!("agent {id} is not installed")))
    }
}

/// The key a swap lock is taken under: the id case-folded. On a
/// case-insensitive filesystem (Windows, macOS by default) `Alpha` and `alpha`
/// name ONE directory, so they must be one lock — two handles on one lock file
/// held by one transaction would wait on each other forever (review #627-a).
/// On a case-sensitive filesystem the two directories share a lock, which only
/// serializes more than necessary.
pub(crate) fn lock_key(id: &str) -> String {
    id.to_lowercase()
}

/// The lock keys of `ids`: validated, case-folded, deduplicated, sorted.
fn sorted_ids<S: AsRef<str>>(ids: &[S]) -> Result<Vec<String>, AwareError> {
    let mut set = BTreeSet::new();
    for id in ids {
        check_id(id.as_ref())?;
        set.insert(lock_key(id.as_ref()));
    }
    Ok(set.into_iter().collect())
}

/// Whether the lock keys `keys` cover agent id `id`.
fn covers(keys: &[String], id: &str) -> bool {
    let key = lock_key(id);
    keys.contains(&key)
}

fn lock_path(paths: &Paths, id: &str) -> PathBuf {
    paths
        .agent_swap_dir()
        .join(LOCKS_DIR)
        .join(format!("{id}.flock"))
}

/// Take the swap locks of `ids` — deduplicated, in sorted order — exclusive or
/// shared, blocking. The only place a swap lock file is ever locked.
fn acquire<'g, S: AsRef<str>>(
    paths: &Paths,
    guard: &'g RefGuard,
    ids: &[S],
    exclusive: bool,
) -> Result<SwapLocks<'g>, AwareError> {
    crate::agent_store::guard::require_home(guard, paths)?;
    let ids = sorted_ids(ids)?;
    if ids.is_empty() {
        return Ok(SwapLocks {
            _files: Vec::new(),
            ids,
            _guard: PhantomData,
        });
    }
    let dir = paths.agent_swap_dir().join(LOCKS_DIR);
    std::fs::create_dir_all(&dir)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", dir.display())))?;
    let mut files = Vec::with_capacity(ids.len());
    for id in &ids {
        let path = lock_path(paths, id);
        let file = std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&path)
            .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", path.display())))?;
        let locked = if exclusive {
            file.lock_exclusive()
        } else {
            file.lock_shared()
        };
        locked.map_err(|e| {
            std::io::Error::new(
                e.kind(),
                format!("cannot lock {} for agent {id}: {e}", path.display()),
            )
        })?;
        files.push(file);
    }
    Ok(SwapLocks {
        _files: files,
        ids,
        _guard: PhantomData,
    })
}

/// For a reader of the current copies of `ids` (a run's preflight, compile's
/// snapshot step): recover any crashed transaction naming them, then hold their
/// swap locks SHARED. While held, no writer can move those copies, so the
/// reader hashes and snapshots a complete tree.
pub fn read_lock<'g, S: AsRef<str>>(
    paths: &Paths,
    guard: &'g RefGuard,
    ids: &[S],
) -> Result<SwapLocks<'g>, AwareError> {
    let ids = sorted_ids(ids)?;
    for _ in 0..4 {
        let shared = acquire(paths, guard, &ids, false)?;
        // Under the shared lock no LIVE writer of these ids can hold an intent
        // (writers write it only while holding the lock exclusive), so a
        // pending transaction naming them now is one that crashed: let go,
        // recover it (exclusive, its whole id set), and look again. Readers
        // never take the lock exclusive otherwise, so concurrent runs of one
        // agent never wait on each other.
        if pending_naming(paths, &ids)?.is_empty() {
            return Ok(shared);
        }
        drop(shared);
        drop(lock_and_recover(paths, guard, &ids)?);
    }
    Err(AwareError::Conflict(format!(
        "[E_AGENT_SWAP_BUSY] agent(s) {} kept being swapped while this command tried to read them; retry",
        ids.join(", ")
    )))
}

/// Whether an unfinished (crashed or in-flight) swap names agent `id` — for a
/// reader that takes no lock to decide whether it must recover first.
pub fn has_pending(paths: &Paths, id: &str) -> Result<bool, AwareError> {
    Ok(!pending_naming(paths, &[lock_key(id)])?.is_empty())
}

/// Take `ids` exclusive (plus every id of any pending transaction that names
/// one of them — a transaction is only ever recovered whole), recover those
/// transactions, and return the exclusive locks over the whole set.
fn lock_and_recover<'g>(
    paths: &Paths,
    guard: &'g RefGuard,
    ids: &[String],
) -> Result<SwapLocks<'g>, AwareError> {
    let mut wanted: BTreeSet<String> = ids.iter().cloned().collect();
    for _ in 0..8 {
        let set: Vec<String> = wanted.iter().cloned().collect();
        for txn in naming(paths, &set)? {
            wanted.extend(txn.intent.ids.iter().cloned());
        }
        let set: Vec<String> = wanted.iter().cloned().collect();
        let locks = acquire(paths, guard, &set, true)?;
        let found = naming(paths, &set)?;
        if found.iter().any(|txn| {
            txn.intent
                .ids
                .iter()
                .any(|id| !wanted.contains(&lock_key(id)))
        }) {
            // A transaction reaching beyond what we hold: widen and retake in order.
            continue;
        }
        for txn in found {
            if txn.settled() {
                // Its final delete failed earlier: inert, removed best effort.
                finish(&txn.dir);
            } else {
                recover(paths, &txn)?;
            }
        }
        return Ok(locks);
    }
    Err(AwareError::Conflict(format!(
        "[E_AGENT_SWAP_BUSY] the swaps touching agent(s) {} kept changing while they were being recovered; retry",
        ids.join(", ")
    )))
}

// ── transactions on disk ──────────────────────────────────────────────────────

/// A transaction directory with a readable intent.
#[derive(Debug)]
struct OnDisk {
    dir: PathBuf,
    intent: Intent,
    journal: Vec<String>,
}

impl OnDisk {
    fn has(&self, line: &str) -> bool {
        self.journal.iter().any(|l| l == line)
    }
    fn settled(&self) -> bool {
        self.has(COMMIT) || self.has(ROLLED_BACK)
    }
}

fn is_txn_name(name: &str) -> bool {
    name.len() == 32 && name.bytes().all(|b| b.is_ascii_hexdigit())
}

/// Every transaction directory with a parseable intent. A directory without
/// one is either a live writer still staging (it has renamed nothing yet) or
/// the leftover of one that crashed before its first rename; both are inert.
fn transactions(paths: &Paths) -> Result<Vec<OnDisk>, AwareError> {
    let root = paths.agent_swap_dir();
    let entries = match std::fs::read_dir(&root) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => {
            return Err(
                std::io::Error::new(error.kind(), format!("{}: {error}", root.display())).into(),
            );
        }
    };
    let mut out = Vec::new();
    for entry in entries {
        let entry = entry?;
        let Some(name) = entry.file_name().to_str().map(str::to_string) else {
            continue;
        };
        if !is_txn_name(&name) {
            continue;
        }
        let dir = entry.path();
        // A transaction being deleted concurrently (its writer just finished)
        // reads as access-denied on Windows while its files are delete-pending:
        // retried until it is gone, then skipped.
        let intent = match read_member(&dir, INTENT_FILE)? {
            Some(bytes) => match serde_json::from_slice::<Intent>(&bytes) {
                Ok(intent) if intent.format == INTENT_FORMAT => intent,
                // Written by temp + rename, so a torn intent is corruption,
                // not a crash; `aware doctor` reports it.
                _ => continue,
            },
            None => continue,
        };
        let journal = match read_member(&dir, JOURNAL_FILE)? {
            Some(bytes) => String::from_utf8_lossy(&bytes)
                .lines()
                .map(str::to_string)
                .collect(),
            None if !dir.exists() => continue,
            None => Vec::new(),
        };
        out.push(OnDisk {
            dir,
            intent,
            journal,
        });
    }
    out.sort_by(|a, b| a.intent.txn.cmp(&b.intent.txn));
    Ok(out)
}

/// Read `<dir>/<name>`: `None` when it (or the whole transaction directory)
/// is gone, retrying while Windows reports it transiently held.
fn read_member(dir: &Path, name: &str) -> Result<Option<Vec<u8>>, AwareError> {
    let path = dir.join(name);
    match crate::fs::retry_transient(|| std::fs::read(&path)) {
        Ok(bytes) => Ok(Some(bytes)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(_) if !dir.exists() => Ok(None),
        Err(error) => {
            Err(std::io::Error::new(error.kind(), format!("{}: {error}", path.display())).into())
        }
    }
}

/// Every transaction (settled or not) naming any of `ids`.
fn naming(paths: &Paths, ids: &[String]) -> Result<Vec<OnDisk>, AwareError> {
    Ok(transactions(paths)?
        .into_iter()
        .filter(|txn| txn.intent.ids.iter().any(|id| covers(ids, id)))
        .collect())
}

/// Unsettled transactions naming any of `ids`.
fn pending_naming(paths: &Paths, ids: &[String]) -> Result<Vec<OnDisk>, AwareError> {
    Ok(transactions(paths)?
        .into_iter()
        .filter(|txn| !txn.settled() && txn.intent.ids.iter().any(|id| covers(ids, id)))
        .collect())
}

/// Whether `path` exists — `NotFound` is "absent", any other failure to look
/// is an error naming the path.
fn exists(path: &Path) -> Result<bool, AwareError> {
    Ok(crate::agent_store::probe(path)?.is_some())
}

/// What recovery did with one transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Recovered {
    Committed,
    RolledBack,
}

/// Recover one unsettled transaction. The caller holds the swap locks of
/// every id in its intent, exclusive.
fn recover(paths: &Paths, txn: &OnDisk) -> Result<Recovered, AwareError> {
    let agents = paths.agents_dir();
    let intent = &txn.intent;
    let committed = if txn.has(COMMIT) {
        true
    } else if let Some(name) = &intent.new_name {
        // Reconcile the move-in from the paths: it happened iff the incoming
        // tree is gone and `agents/<name>` now holds exactly its bytes.
        let started = txn.has(&format!("pending in {name}"));
        let incoming = exists(&txn.dir.join(INCOMING_DIR))?;
        let target = agents.join(name);
        if started && !incoming && exists(&target)? {
            let expected = intent.incoming_digest.as_deref().unwrap_or_default();
            match crate::install::integrity::tree_digest(&target) {
                Ok(actual) if actual == expected => true,
                Ok(actual) => {
                    return Err(unexplained(
                        txn,
                        &format!(
                            "agents/{name} hashes to {actual}, not the {expected} the swap moved in"
                        ),
                    ));
                }
                Err(error) => {
                    return Err(unexplained(
                        txn,
                        &format!("agents/{name} cannot be hashed ({error})"),
                    ));
                }
            }
        } else {
            false
        }
    } else {
        // An uninstall commits once every outgoing tree is out of `agents/`.
        !intent.outgoing.is_empty()
            && intent.outgoing.iter().all(|o| {
                txn.has(&format!("pending out {}", o.id))
                    && exists(&txn.dir.join(format!("{OUTGOING_PREFIX}{}", o.id))).unwrap_or(false)
                    && !exists(&agents.join(&o.id)).unwrap_or(true)
            })
    };
    let mut journal = Journal::open(&txn.dir)?;
    if committed {
        if !txn.has(COMMIT) {
            journal.line(COMMIT)?;
        }
        journal.close();
        finish(&txn.dir);
        return Ok(Recovered::Committed);
    }
    roll_back(paths, &txn.dir, intent, &mut journal)?;
    Ok(Recovered::RolledBack)
}

fn unexplained(txn: &OnDisk, what: &str) -> AwareError {
    AwareError::Validation(format!(
        "[E_AGENT_SWAP_RECOVERY] the interrupted agent swap {} ({:?} of {}) cannot be recovered automatically: {what}. \
         Nothing was changed; inspect {} and agents/ by hand",
        txn.intent.txn,
        txn.intent.op,
        txn.intent.ids.join(", "),
        txn.dir.display()
    ))
}

/// Put every moved-out tree back (reverse order), drop the incoming tree,
/// record `rolled-back`, delete the transaction.
fn roll_back(
    paths: &Paths,
    dir: &Path,
    intent: &Intent,
    journal: &mut Journal,
) -> Result<(), AwareError> {
    let agents = paths.agents_dir();
    for outgoing in intent.outgoing.iter().rev() {
        let aside = dir.join(format!("{OUTGOING_PREFIX}{}", outgoing.id));
        let current = agents.join(&outgoing.id);
        match (exists(&aside)?, exists(&current)?) {
            (false, true) => {} // never moved
            (true, false) => {
                let step = format!("restore {}", outgoing.id);
                journal.line(&format!("pending {step}"))?;
                crate::fs::rename_dir_no_replace(&aside, &current).map_err(|e| {
                    swap_io(
                        &format!("put {} back at {}", aside.display(), current.display()),
                        e,
                    )
                })?;
                sync_dirs(&[dir, &agents])?;
                journal.line(&format!("done {step}"))?;
            }
            (true, true) => {
                return Err(AwareError::Validation(format!(
                    "[E_AGENT_SWAP_RECOVERY] rolling back agent swap {}: both {} and {} exist; nothing was changed — inspect them by hand",
                    intent.txn,
                    aside.display(),
                    current.display()
                )));
            }
            (false, false) => {
                return Err(AwareError::Validation(format!(
                    "[E_AGENT_SWAP_RECOVERY] rolling back agent swap {}: the moved-aside copy of agent {} ({}) is missing; nothing was changed — reinstall {}",
                    intent.txn,
                    outgoing.id,
                    aside.display(),
                    outgoing.id
                )));
            }
        }
    }
    journal.line(ROLLED_BACK)?;
    journal.close();
    finish(dir);
    Ok(())
}

/// Delete a settled transaction directory (incoming leftovers, outgoing
/// copies). Best effort — a settled transaction is inert.
///
/// First the transaction is made unrecognisable in ONE step — renamed to
/// `.done-<txn>`, a name no reader treats as a transaction — and only then
/// deleted. Deleting in place could be killed part-way (`remove_dir_all` takes
/// many steps) and leave an intent whose journal is gone: recovery would then
/// roll a half-deleted old copy back into `agents/`, or refuse for ever
/// (review #627-a). If the rename cannot be made, the intent is deleted first
/// instead, which is the same single step. Leftover `.done-*` directories are
/// removed by `aware doctor`.
fn finish(dir: &Path) {
    let name = dir
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let settled = dir.with_file_name(format!("{SETTLED_PREFIX}{name}"));
    let target = match crate::fs::rename_dir_no_replace(dir, &settled) {
        Ok(()) => settled,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return,
        Err(_) => match std::fs::remove_file(dir.join(INTENT_FILE)) {
            Ok(()) => dir.to_path_buf(),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => dir.to_path_buf(),
            Err(error) => {
                eprintln!(
                    "\u{26a0} could not retire the finished agent swap {} ({error}); it is inert and will be retried",
                    dir.display()
                );
                return;
            }
        },
    };
    if hooks::finish_interrupted() {
        // Test: stop part-way, as a kill during the delete would.
        let _ = std::fs::remove_file(target.join(JOURNAL_FILE));
        if let Ok(entries) = std::fs::read_dir(&target) {
            for e in entries.flatten() {
                if e.file_name().to_string_lossy().starts_with(OUTGOING_PREFIX) {
                    let _ = std::fs::remove_file(e.path().join("manifest.yaml"));
                }
            }
        }
        return;
    }
    if let Err(error) = std::fs::remove_dir_all(&target)
        && error.kind() != std::io::ErrorKind::NotFound
    {
        eprintln!(
            "\u{26a0} could not remove the finished agent swap {} ({error}); it is inert and `aware doctor` removes it",
            target.display()
        );
    }
}

fn sync_dirs(dirs: &[&Path]) -> Result<(), AwareError> {
    for dir in dirs {
        crate::fs::sync_dir(dir)?;
    }
    Ok(())
}

fn swap_io(what: &str, error: std::io::Error) -> AwareError {
    std::io::Error::new(error.kind(), format!("cannot {what}: {error}")).into()
}

/// `journal.log`: one line per step, each fsynced before the next rename.
/// Closed before the transaction is retired: Windows cannot rename a
/// directory while a file inside it is open.
struct Journal {
    file: Option<std::fs::File>,
    dir: PathBuf,
}

impl Journal {
    fn open(dir: &Path) -> Result<Self, AwareError> {
        let path = dir.join(JOURNAL_FILE);
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", path.display())))?;
        Ok(Self {
            file: Some(file),
            dir: dir.to_path_buf(),
        })
    }

    fn line(&mut self, line: &str) -> Result<(), AwareError> {
        use std::io::Write;
        let file = self
            .file
            .as_mut()
            .ok_or_else(|| AwareError::Internal("the swap journal is already closed".into()))?;
        file.write_all(format!("{line}\n").as_bytes())?;
        file.sync_all()?;
        crate::fs::sync_dir(&self.dir)?;
        hooks::after_line(line)
    }

    fn close(&mut self) {
        self.file = None;
    }
}

// ── writers ───────────────────────────────────────────────────────────────────

/// A transaction directory being staged: `incoming/` is filled by the caller
/// outside any lock. Dropped without being executed, it is removed.
#[derive(Debug)]
pub struct Staged {
    dir: PathBuf,
    armed: bool,
}

impl Staged {
    /// Create `agents/.aware-swap/<txn>/` for a swap that will move a new tree in.
    pub fn new(paths: &Paths) -> Result<Self, AwareError> {
        let root = paths.agent_swap_dir();
        std::fs::create_dir_all(&root)
            .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", root.display())))?;
        let dir = root.join(uuid::Uuid::new_v4().simple().to_string());
        std::fs::create_dir(&dir)
            .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", dir.display())))?;
        Ok(Self { dir, armed: true })
    }

    /// Where the caller stages the new tree (does not exist yet).
    pub fn incoming(&self) -> PathBuf {
        self.dir.join(INCOMING_DIR)
    }
}

impl Drop for Staged {
    fn drop(&mut self) {
        if self.armed {
            let _ = std::fs::remove_dir_all(&self.dir);
        }
    }
}

/// A swap holding its locks, ready for the caller's checks and then
/// [`Transaction::execute`]. Dropped unexecuted, it changes nothing.
#[derive(Debug)]
pub struct Transaction<'g> {
    paths: Paths,
    locks: SwapLocks<'g>,
    staged: Option<Staged>,
}

/// Take the swap locks of `ids` exclusive (after recovering anything crashed
/// that names them) for a swap that moves `staged`'s incoming tree in (or, with
/// `None`, only moves trees out).
pub fn begin<'g, S: AsRef<str>>(
    paths: &Paths,
    guard: &'g RefGuard,
    ids: &[S],
    staged: Option<Staged>,
) -> Result<Transaction<'g>, AwareError> {
    let ids = sorted_ids(ids)?;
    let locks = lock_and_recover(paths, guard, &ids)?;
    Ok(Transaction {
        paths: paths.clone(),
        locks,
        staged,
    })
}

impl Transaction<'_> {
    /// Run the swap: move every `outgoing` tree out of `agents/` and, when
    /// `new_name` is given, the staged incoming tree in as `agents/<new_name>`.
    /// Every id named must be one this transaction locked. On a failed rename
    /// everything already moved is put back before the error is returned.
    pub fn execute(
        mut self,
        op: Op,
        new_name: Option<&str>,
        incoming_digest: Option<String>,
        outgoing: Vec<Outgoing>,
    ) -> Result<(), AwareError> {
        let locked = self.locks.ids().to_vec();
        for id in outgoing.iter().map(|o| o.id.as_str()).chain(new_name) {
            if !covers(&locked, id) {
                return Err(AwareError::Internal(format!(
                    "agent swap names {id}, whose swap lock it does not hold"
                )));
            }
        }
        if new_name.is_some() != incoming_digest.is_some() {
            return Err(AwareError::Internal(
                "an agent swap moving a tree in needs its digest (and only then)".into(),
            ));
        }
        let staged = match (new_name, self.staged.take()) {
            (Some(_), Some(staged)) => staged,
            (Some(_), None) => {
                return Err(AwareError::Internal(
                    "an agent swap moving a tree in has nothing staged".into(),
                ));
            }
            (None, Some(staged)) => staged,
            (None, None) => Staged::new(&self.paths)?,
        };
        let agents = self.paths.agents_dir();
        std::fs::create_dir_all(&agents)?;
        let intent = Intent {
            format: INTENT_FORMAT.into(),
            txn: staged
                .dir
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_string(),
            op,
            ids: locked,
            new_name: new_name.map(str::to_string),
            incoming_digest,
            outgoing,
            started_at: chrono::Utc::now().to_rfc3339(),
            pid: std::process::id(),
        };
        write_intent(&staged.dir, &intent)?;
        // From here on the directory is the journal of a real swap: it is
        // removed only by commit, rollback or recovery — never by Drop.
        let mut staged = staged;
        staged.armed = false;
        let dir = staged.dir.clone();
        hooks::after_line("intent")?;
        let mut journal = Journal::open(&dir)?;

        for out in &intent.outgoing {
            let step = format!("out {}", out.id);
            journal.line(&format!("pending {step}"))?;
            let from = agents.join(&out.id);
            let to = dir.join(format!("{OUTGOING_PREFIX}{}", out.id));
            if let Err(error) = hooks::rename(&from, &to, &step) {
                return Err(live_rollback(
                    &self.paths,
                    &dir,
                    &intent,
                    &mut journal,
                    swap_io(
                        &format!("move {} aside to {}", from.display(), to.display()),
                        error,
                    ),
                ));
            }
            sync_dirs(&[&agents, &dir])?;
            journal.line(&format!("done {step}"))?;
        }
        if let Some(name) = new_name {
            let step = format!("in {name}");
            journal.line(&format!("pending {step}"))?;
            let from = dir.join(INCOMING_DIR);
            let to = agents.join(name);
            if let Err(error) = hooks::rename(&from, &to, &step) {
                return Err(live_rollback(
                    &self.paths,
                    &dir,
                    &intent,
                    &mut journal,
                    swap_io(&format!("move the new copy into {}", to.display()), error),
                ));
            }
            sync_dirs(&[&agents, &dir])?;
            journal.line(&format!("done {step}"))?;
        }
        journal.line(COMMIT)?;
        journal.close();
        finish(&dir);
        Ok(())
    }
}

/// A rename failed mid-swap: put back what moved and return `cause` — or, if
/// even that fails, an error saying exactly where things stand (the
/// transaction stays on disk for the next recovery).
fn live_rollback(
    paths: &Paths,
    dir: &Path,
    intent: &Intent,
    journal: &mut Journal,
    cause: AwareError,
) -> AwareError {
    match roll_back(paths, dir, intent, journal) {
        Ok(()) => cause,
        Err(rollback) => AwareError::Validation(format!(
            "[E_AGENT_SWAP_RECOVERY] the swap failed ({cause}) and putting the previous copy back also failed ({rollback}); \
             the next install, update, run or `aware doctor` retries the recovery"
        )),
    }
}

fn write_intent(dir: &Path, intent: &Intent) -> Result<(), AwareError> {
    use std::io::Write;
    let temp = dir.join(format!("{INTENT_FILE}.tmp"));
    let bytes = serde_json::to_vec_pretty(intent)?;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temp)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", temp.display())))?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    drop(file);
    std::fs::rename(&temp, dir.join(INTENT_FILE))?;
    crate::fs::sync_dir(dir)?;
    Ok(())
}

/// The directories under `agents/` that the ids `ids` name, as they are
/// actually spelled on disk, each once — the outgoing set of a swap.
///
/// An id names the entry spelled exactly like it, else the one entry that
/// differs from it only by case (a case-insensitive filesystem opens that one
/// for it too). Two ids of one swap that differ only by case thus name ONE
/// directory where the filesystem folds case; where it does not and both exist
/// as two directories, the swap refuses — it would otherwise move two agents
/// under one lock key that the person asked to treat as one — and nothing is
/// changed.
pub fn existing_dirs<S: AsRef<str>>(paths: &Paths, ids: &[S]) -> Result<Vec<String>, AwareError> {
    let agents = paths.agents_dir();
    let listed: Vec<String> = match std::fs::read_dir(&agents) {
        Ok(entries) => entries
            .filter_map(|e| e.ok())
            .filter_map(|e| e.file_name().to_str().map(str::to_string))
            .filter(|name| !is_swap_area(name))
            .collect(),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(error) => {
            return Err(std::io::Error::new(
                error.kind(),
                format!("{}: {error}", agents.display()),
            )
            .into());
        }
    };
    let mut out: Vec<String> = Vec::new();
    for id in ids {
        let id = id.as_ref();
        check_id(id)?;
        let entry = if listed.iter().any(|name| name == id) {
            Some(id.to_string())
        } else {
            let folded: Vec<&String> = listed
                .iter()
                .filter(|name| lock_key(name) == lock_key(id))
                .collect();
            match folded.as_slice() {
                [one] => Some((*one).clone()),
                _ => None,
            }
        };
        let Some(entry) = entry else { continue };
        if out.contains(&entry) {
            continue;
        }
        if let Some(other) = out.iter().find(|o| lock_key(o) == lock_key(&entry)) {
            return Err(AwareError::Conflict(format!(
                "agents/{other} and agents/{entry} are two installed agents whose ids differ only by case;                  one update cannot replace both safely. Remove the one you no longer want                  (`aware agent uninstall <id>`) and update again"
            )));
        }
        out.push(entry);
    }
    Ok(out)
}

// ── doctor ────────────────────────────────────────────────────────────────────

/// One transaction `aware doctor` looked at.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct DoctorFinding {
    pub txn: String,
    pub ids: Vec<String>,
    pub outcome: String,
    pub detail: Option<String>,
}

/// Recover every unsettled transaction (each under its whole id set,
/// exclusive), remove settled leftovers, and remove staging directories with
/// no intent that are older than an hour (a live writer stages for seconds).
pub fn recover_all(paths: &Paths, guard: &RefGuard) -> Result<Vec<DoctorFinding>, AwareError> {
    let mut findings = Vec::new();
    for txn in transactions(paths)? {
        let ids = txn.intent.ids.clone();
        let was_settled = txn.settled();
        let outcome = (|| -> Result<(), AwareError> {
            drop(lock_and_recover(paths, guard, &ids)?);
            Ok(())
        })();
        let (outcome, detail) = match outcome {
            Ok(()) if was_settled => ("cleaned".to_string(), None),
            Ok(()) => ("recovered".to_string(), None),
            Err(error) => ("needs-you".to_string(), Some(error.to_string())),
        };
        findings.push(DoctorFinding {
            txn: txn.intent.txn.clone(),
            ids,
            outcome,
            detail,
        });
    }
    // Directories with no readable intent.
    let root = paths.agent_swap_dir();
    if let Ok(entries) = std::fs::read_dir(&root) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with(SETTLED_PREFIX) {
                // A finished swap whose delete was interrupted: inert.
                if std::fs::remove_dir_all(entry.path()).is_ok() {
                    findings.push(DoctorFinding {
                        txn: name,
                        ids: Vec::new(),
                        outcome: "cleaned".into(),
                        detail: Some("a finished swap's leftover files".into()),
                    });
                }
                continue;
            }
            if !is_txn_name(&name) {
                continue;
            }
            let dir = entry.path();
            if dir.join(INTENT_FILE).exists() {
                if serde_json::from_slice::<Intent>(
                    &std::fs::read(dir.join(INTENT_FILE)).unwrap_or_default(),
                )
                .is_err()
                {
                    findings.push(DoctorFinding {
                        txn: name,
                        ids: Vec::new(),
                        outcome: "needs-you".into(),
                        detail: Some(format!(
                            "{} is unreadable; inspect it and agents/ by hand",
                            dir.join(INTENT_FILE).display()
                        )),
                    });
                }
                continue;
            }
            let old = entry
                .metadata()
                .and_then(|m| m.modified())
                .ok()
                .and_then(|t| t.elapsed().ok())
                .is_some_and(|age| age > std::time::Duration::from_secs(3600));
            if old && std::fs::remove_dir_all(&dir).is_ok() {
                findings.push(DoctorFinding {
                    txn: name,
                    ids: Vec::new(),
                    outcome: "cleaned".into(),
                    detail: Some("an abandoned staging directory (nothing was swapped)".into()),
                });
            }
        }
    }
    Ok(findings)
}

// ── test / E2E hooks ──────────────────────────────────────────────────────────

mod hooks {
    use std::path::Path;

    use crate::error::AwareError;

    /// Where a test makes the swap fail.
    #[cfg_attr(not(test), allow(dead_code))]
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub(crate) enum Fault {
        /// Stop dead right after writing this journal line (`"intent"` for the
        /// intent itself), as a killed process would: no rollback, no cleanup.
        CrashAfter(String),
        /// Make the rename of this step (`"out <id>"` / `"in <name>"`) fail as
        /// an OS error would; the live rollback then runs.
        FailRename(String),
        /// Stop the final delete of a settled transaction part-way, as a
        /// kill during `remove_dir_all` would.
        InterruptFinish,
    }

    #[cfg(test)]
    thread_local! {
        static FAULT: std::cell::RefCell<Option<Fault>> = const { std::cell::RefCell::new(None) };
    }

    #[cfg(test)]
    pub(crate) fn inject(fault: Fault) {
        FAULT.with(|f| *f.borrow_mut() = Some(fault));
    }

    #[cfg(test)]
    pub(crate) fn clear() {
        FAULT.with(|f| *f.borrow_mut() = None);
    }

    /// The sentinel error a simulated crash returns.
    #[cfg(test)]
    pub(crate) const CRASHED: &str = "injected crash";

    pub(super) fn after_line(line: &str) -> Result<(), AwareError> {
        #[cfg(test)]
        if FAULT.with(|f| f.borrow().as_ref() == Some(&Fault::CrashAfter(line.to_string()))) {
            return Err(AwareError::Internal(format!("{CRASHED} after {line:?}")));
        }
        // E2E only, never in a release build: a debug build pauses here so a
        // test can kill the process mid-swap (`AWARE_TEST_SWAP_PAUSE_AFTER` =
        // a journal line, e.g. `done out tekla`).
        #[cfg(debug_assertions)]
        if std::env::var("AWARE_TEST_SWAP_PAUSE_AFTER").is_ok_and(|want| want == line) {
            eprintln!("aware: test pause after swap step {line:?}");
            std::thread::sleep(std::time::Duration::from_secs(120));
        }
        let _ = line;
        Ok(())
    }

    /// Whether the final delete of a transaction stops part-way (test only).
    pub(super) fn finish_interrupted() -> bool {
        #[cfg(test)]
        if FAULT.with(|f| f.borrow().as_ref() == Some(&Fault::InterruptFinish)) {
            return true;
        }
        false
    }

    pub(super) fn rename(from: &Path, to: &Path, step: &str) -> std::io::Result<()> {
        #[cfg(test)]
        if FAULT.with(|f| f.borrow().as_ref() == Some(&Fault::FailRename(step.to_string()))) {
            return Err(std::io::Error::other(format!(
                "injected rename failure at {step}"
            )));
        }
        let _ = step;
        crate::fs::rename_dir_no_replace(from, to)
    }
}

/// Every transaction directory (settled or not, with or without an intent)
/// under the swap area — for tests asserting a failed operation left no
/// staging behind.
#[cfg(test)]
pub(crate) fn leftover_txn_dirs(paths: &Paths) -> Vec<PathBuf> {
    std::fs::read_dir(paths.agent_swap_dir())
        .map(|entries| {
            entries
                .flatten()
                .filter(|e| {
                    let name = e.file_name().to_string_lossy().into_owned();
                    is_txn_name(&name) || name.starts_with(SETTLED_PREFIX)
                })
                .map(|e| e.path())
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
pub(crate) use hooks::{CRASHED, Fault, clear as clear_fault, inject as inject_fault};

#[cfg(test)]
#[path = "swap/tests.rs"]
mod tests;
