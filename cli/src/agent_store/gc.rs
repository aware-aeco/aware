//! `aware agent gc` (#629-b, plan §4): remove the store packages nothing needs.
//!
//! The decision is the reference table's ([`super::refs`]); GC only acts on
//! it. A dry run (the default) builds the table under the store lock held
//! SHARED — it never blocks a run — and reports what `--apply` would remove.
//! `--apply`:
//!
//! 1. takes the store lock EXCLUSIVE within `--wait` (default 0), or reports
//!    `deferred: store-busy` and does nothing. Every reader and writer of a
//!    store reference holds it shared (runs from before reading their approval
//!    until their lease exists; compile, install, update, every migrate verb),
//!    so while GC holds it nothing can start relying on a package;
//! 2. stamps the packages of every stale lease (a run that ended without
//!    releasing it) and only then deletes that lease file (plan §9 R2-5);
//! 3. builds the table; if anything could not be read it stops with
//!    `E_AGENT_GC_REFS_INCOMPLETE` and removes nothing;
//! 4. renames each removable package, under its own name's folder, to
//!    `.trash-<uuid>` (one atomic, same-directory, no-replace rename: it
//!    vanishes whole or not at all), and writes a tombstone
//!    `.removed-<receipt-key>.yaml` beside it so a later `app check` can say
//!    what happened. A package Windows will not let go of (an open handle) is
//!    `skipped`, intact;
//! 5. releases the lock, then deletes the trash. A delete that fails is
//!    `pending-delete`, retried by the next GC, which also removes trash and
//!    interrupted snapshots (`.tmp-*` older than an hour) left by dead
//!    processes.
//!
//! GC never touches `agents/`, `apps/`, any lock, candidate, archive, HOLD or
//! evidence, the legacy `agent-store/`, bridges, or names it does not
//! recognise. It never modifies a package: it removes it whole.

use std::path::{Path, PathBuf};
use std::time::Duration;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::RefGuard;
use super::refs::{self, RefTable, Reference, State, TOMBSTONE_PREFIX, TRASH_PREFIX, Window};
use crate::error::AwareError;
use crate::paths::Paths;

/// `aware agent gc --json` schema.
pub const GC_FORMAT: &str = "aware.agent-gc/v1";
/// The tombstone format.
pub const TOMBSTONE_FORMAT: &str = "aware.agent-tombstone/v1";
/// An interrupted snapshot younger than this may belong to a live process
/// that does not hold the store lock (an older CLI): left alone.
const TEMP_MIN_AGE: chrono::Duration = chrono::Duration::hours(1);
/// Trash younger than this may be another GC's, being deleted right now.
const TRASH_MIN_AGE: chrono::Duration = chrono::Duration::minutes(10);

/// What to collect.
#[derive(Debug, Clone)]
pub struct Options {
    pub apply: bool,
    pub window: Window,
    /// Only packages of this agent.
    pub agent: Option<String>,
    /// Only this one package: `(agent, digest)`.
    pub only: Option<(String, String)>,
    /// How long `--apply` waits for the store lock.
    pub wait: Duration,
}

/// Parse `--only <agent>@sha256:<hex>`.
pub fn parse_only(text: &str) -> Result<(String, String), AwareError> {
    let invalid = || {
        AwareError::Validation(format!(
            "[E_AGENT_GC_ONLY_INVALID] --only {text:?} is not <agent>@sha256:<64 hex>"
        ))
    };
    let (agent, digest) = text.split_once('@').ok_or_else(invalid)?;
    if !crate::manifest::loader::is_safe_segment(agent) || super::digest_hex(digest).is_none() {
        return Err(invalid());
    }
    Ok((agent.to_string(), digest.to_string()))
}

/// A package GC removed, or would remove.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Gone {
    pub agent: String,
    pub version: Option<String>,
    pub digest: String,
    pub receipt_key: String,
    pub path: String,
    pub bytes: u64,
    /// `true` for a package that failed verification.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub invalid: bool,
}

/// A package GC keeps, and why.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Keep {
    pub agent: String,
    pub version: Option<String>,
    pub digest: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
    pub references: Vec<Reference>,
}

/// Something GC tried and could not do; nothing was lost.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Skipped {
    pub path: String,
    pub reason: String,
}

/// `aware agent gc --json` data.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct GcReport {
    pub format: &'static str,
    pub applied: bool,
    /// Why `--apply` did nothing: `store-busy` (something held the store
    /// lock for longer than `--wait`) or `network-volume`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deferred: Option<&'static str>,
    pub recovery_window: String,
    /// Whether the reference table could be fully read; absent when GC was
    /// deferred and built no table.
    pub complete: Option<bool>,
    /// Removed (`--apply`) or that `--apply` would remove (dry run; empty
    /// while the table is incomplete, because `--apply` would remove nothing).
    pub removed: Vec<Gone>,
    pub kept: Vec<Keep>,
    pub in_window: Vec<Keep>,
    pub skipped: Vec<Skipped>,
    /// Removed from the store but not yet deleted from disk; the next GC
    /// deletes them.
    pub pending_delete: Vec<String>,
    /// Leftovers of dead processes removed (`.trash-*`, old `.tmp-*`).
    pub leftovers_removed: Vec<String>,
    /// Stale lease files stamped and removed.
    pub stale_leases_removed: Vec<String>,
    pub blockers: Vec<refs::Blocker>,
}

/// GC's record of a removed package, beside where it was.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Tombstone {
    pub format: String,
    pub agent: String,
    pub version: Option<String>,
    pub digest: String,
    pub receipt_key: String,
    pub removed_at: String,
    pub removed_by: String,
    pub cli_version: String,
}

fn now_text(now: DateTime<Utc>) -> String {
    now.to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

/// The latest tombstone for `id`'s package `digest`, if GC removed it.
pub fn tombstone(paths: &Paths, id: &str, digest: &str) -> Option<Tombstone> {
    let container = super::digest_container(paths, id, digest).ok()?;
    std::fs::read_dir(container)
        .ok()?
        .flatten()
        .filter(|e| {
            e.file_name()
                .to_str()
                .is_some_and(|n| n.starts_with(TOMBSTONE_PREFIX))
        })
        .filter_map(|e| std::fs::read_to_string(e.path()).ok())
        .filter_map(|text| serde_yaml::from_str::<Tombstone>(&text).ok())
        .filter(|t| t.format == TOMBSTONE_FORMAT && t.agent == id && t.digest == digest)
        .max_by(|a, b| a.removed_at.cmp(&b.removed_at))
}

/// Whether AWARE_HOME is on a network volume, where OS file locks — the
/// whole interlock between GC and runs — are not dependable (plan §7 K3).
#[cfg(windows)]
fn on_network_volume(path: &Path) -> bool {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{GetDriveTypeW, GetVolumePathNameW};
    const DRIVE_REMOTE: u32 = 4;
    let wide: Vec<u16> = path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();
    let mut volume = vec![0u16; 1024];
    // SAFETY: both buffers are NUL-terminated / sized as passed.
    let ok = unsafe { GetVolumePathNameW(wide.as_ptr(), volume.as_mut_ptr(), volume.len() as u32) };
    if ok == 0 {
        return false;
    }
    // SAFETY: `volume` was NUL-terminated by the call above.
    unsafe { GetDriveTypeW(volume.as_ptr()) == DRIVE_REMOTE }
}

/// Not detected off Windows (documented): a network home is the person's
/// own risk there.
#[cfg(not(windows))]
fn on_network_volume(_path: &Path) -> bool {
    false
}

fn report(table: &RefTable, apply: bool) -> GcReport {
    let mut kept = Vec::new();
    let mut in_window = Vec::new();
    for p in &table.packages {
        let row = Keep {
            agent: p.agent.clone(),
            version: Some(p.version.clone()),
            digest: p.digest.clone(),
            until: p.kept_until.clone(),
            references: p.references.clone(),
        };
        match p.state {
            State::Kept => kept.push(row),
            State::InWindow => in_window.push(row),
            State::Removable => {}
        }
    }
    for p in &table.invalid_packages {
        let row = Keep {
            agent: p.agent.clone(),
            version: None,
            digest: p.digest.clone(),
            until: p.kept_until.clone(),
            references: p.references.clone(),
        };
        match p.state {
            State::Kept => kept.push(row),
            State::InWindow => in_window.push(row),
            State::Removable => {}
        }
    }
    GcReport {
        format: GC_FORMAT,
        applied: apply,
        deferred: None,
        recovery_window: table.recovery_window.clone(),
        complete: Some(table.complete),
        removed: Vec::new(),
        kept,
        in_window,
        skipped: Vec::new(),
        pending_delete: Vec::new(),
        leftovers_removed: Vec::new(),
        stale_leases_removed: Vec::new(),
        blockers: table.blockers.clone(),
    }
}

/// The removable packages the options select, in a stable order.
fn removable(table: &RefTable, options: &Options) -> Vec<Gone> {
    let selected = |agent: &str, digest: &str| {
        options.agent.as_deref().is_none_or(|a| a == agent)
            && options
                .only
                .as_ref()
                .is_none_or(|(a, d)| a == agent && d == digest)
    };
    let mut out: Vec<Gone> = table
        .packages
        .iter()
        .filter(|p| p.state == State::Removable && selected(&p.agent, &p.digest))
        .map(|p| Gone {
            agent: p.agent.clone(),
            version: Some(p.version.clone()),
            digest: p.digest.clone(),
            receipt_key: p.receipt_key.clone(),
            path: p.path.clone(),
            bytes: p.bytes,
            invalid: false,
        })
        .chain(
            table
                .invalid_packages
                .iter()
                .filter(|p| p.state == State::Removable && selected(&p.agent, &p.digest))
                .map(|p| Gone {
                    agent: p.agent.clone(),
                    version: None,
                    digest: p.digest.clone(),
                    receipt_key: p.receipt_key.clone(),
                    path: p.path.clone(),
                    bytes: p.bytes,
                    invalid: true,
                }),
        )
        .collect();
    out.sort_by(|a, b| a.path.cmp(&b.path));
    out
}

/// `--only` names a package something still needs: refuse, naming why
/// (the #627 acceptance "removal of a referenced package is refused").
fn refuse_referenced(table: &RefTable, agent: &str, digest: &str) -> Result<(), AwareError> {
    let rows = table
        .packages
        .iter()
        .filter(|p| p.agent == agent && p.digest == digest)
        .map(|p| (p.state, p.kept_until.clone(), p.references.clone()))
        .chain(
            table
                .invalid_packages
                .iter()
                .filter(|p| p.agent == agent && p.digest == digest)
                .map(|p| (p.state, p.kept_until.clone(), p.references.clone())),
        )
        .collect::<Vec<_>>();
    if rows.is_empty() {
        return Err(AwareError::NotFound(format!(
            "[E_AGENT_GC_NOT_STORED] no stored package of {agent} {digest}"
        )));
    }
    if let Some((_, until, references)) = rows.iter().find(|(s, ..)| *s != State::Removable) {
        let why = references
            .iter()
            .map(describe)
            .collect::<Vec<_>>()
            .join("; ");
        let until = until
            .as_deref()
            .map(|u| format!(" (kept until {u})"))
            .unwrap_or_default();
        // Kept only by a recovery window: say how to remove it now anyway.
        let only_windows = references.iter().all(|r| {
            matches!(r, Reference::Recent { .. })
                || matches!(
                    r,
                    Reference::ApprovalArchive { held: false, .. }
                        | Reference::ApprovalOriginal { held: false, .. }
                        | Reference::SuccessorFrom { held: false, .. }
                )
        });
        let choice = if only_windows {
            "; only its recovery window keeps it - to remove it now anyway, run gc again with --recovery-window 0s"
        } else {
            ""
        };
        return Err(AwareError::Conflict(format!(
            "[E_AGENT_GC_REFERENCED] {agent} {digest} is still needed{until}: {why}{choice}"
        )));
    }
    Ok(())
}

/// One reason a package is kept, as a sentence fragment.
fn describe(reference: &Reference) -> String {
    let until = |u: &Option<String>, held: bool| {
        if held {
            " (its app is on hold)".to_string()
        } else {
            u.as_deref()
                .map(|u| format!(" until {u}"))
                .unwrap_or_default()
        }
    };
    match reference {
        Reference::Current { path } => format!("it is the installed copy ({path})"),
        Reference::CurrentUnhashable { path, .. } => {
            format!("the installed copy at {path} cannot be read, so every stored version is kept")
        }
        Reference::ApprovedLock { lock } => format!("the approved workflow {lock} uses it"),
        Reference::CandidateLock { candidate } => {
            format!("the prepared update {candidate} uses it")
        }
        Reference::CandidateBase { evidence } => {
            format!("the prepared update {evidence} moves away from it")
        }
        Reference::PromotionInProgress { path } => {
            format!("an approval being carried forward uses it ({path})")
        }
        Reference::ApprovalArchive {
            archive,
            until: u,
            held,
        } => {
            format!("the earlier approval {archive} used it{}", until(u, *held))
        }
        Reference::ApprovalOriginal {
            lock,
            until: u,
            held,
        }
        | Reference::SuccessorFrom {
            lock,
            until: u,
            held,
            ..
        } => {
            format!("an earlier approval of {lock} used it{}", until(u, *held))
        }
        Reference::Lease { run_id, app } => format!("run {run_id} of {app} is using it"),
        Reference::StaleLeaseUnstamped { run_id, app, .. } => {
            format!("run {run_id} of {app} ended without letting go of it")
        }
        Reference::Recent { until } => format!("it was used recently (kept until {until})"),
    }
}

fn incomplete(table: &RefTable) -> AwareError {
    let blockers = table
        .blockers
        .iter()
        .map(|b| format!("{}: {}", b.path, b.problem))
        .collect::<Vec<_>>()
        .join("; ");
    AwareError::Conflict(format!(
        "[E_AGENT_GC_REFS_INCOMPLETE] nothing was removed: what still needs each stored version could not be fully read ({blockers}); fix or remove these, then run gc again (`aware agent refs` lists them)"
    ))
}

/// Run GC. A dry run never writes; `--apply` writes only as described above.
pub fn collect(paths: &Paths, options: &Options) -> Result<GcReport, AwareError> {
    collect_at(paths, options, Utc::now())
}

/// [`collect`] at a given time (tests move the clock).
pub fn collect_at(
    paths: &Paths,
    options: &Options,
    now: DateTime<Utc>,
) -> Result<GcReport, AwareError> {
    if !options.apply {
        let guard = super::open(paths)?;
        let table = refs::table(paths, &guard, &options.window, now)?;
        drop(guard);
        if let Some((agent, digest)) = &options.only {
            refuse_referenced(&table, agent, digest)?;
        }
        let mut out = report(&table, false);
        // While the table is incomplete `--apply` removes nothing, so nothing
        // is listed as what it would remove (review round 1).
        if table.complete {
            out.removed = removable(&table, options);
        }
        return Ok(out);
    }

    // The store must exist and be distinct from the legacy one. Any legacy
    // import runs under GC's own exclusive lock below, bounded by `--wait`
    // like everything else GC waits for (review round 1), never under
    // `open`'s blocking one.
    super::prepare(paths)?;
    let deferred = |reason| {
        Ok(GcReport {
            format: GC_FORMAT,
            applied: false,
            deferred: Some(reason),
            recovery_window: options.window.text().to_string(),
            complete: None,
            removed: Vec::new(),
            kept: Vec::new(),
            in_window: Vec::new(),
            skipped: Vec::new(),
            pending_delete: Vec::new(),
            leftovers_removed: Vec::new(),
            stale_leases_removed: Vec::new(),
            blockers: Vec::new(),
        })
    };
    if on_network_volume(&paths.aware_home) {
        return deferred("network-volume");
    }
    let Some(guard) = RefGuard::exclusive_within(paths, options.wait)? else {
        return deferred("store-busy");
    };
    super::warn_import(super::import_if_needed(paths));

    // Decide before changing anything: a refusal leaves everything as it was
    // (review round 1). A stale lease still counts as a reference here; its
    // package gets its window from the stamp written below.
    let table = refs::table(paths, &guard, &options.window, now)?;
    if !table.complete {
        drop(guard);
        return Err(incomplete(&table));
    }
    if let Some((agent, digest)) = &options.only {
        refuse_referenced(&table, agent, digest)?;
    }
    let mut out = report(&table, true);
    // `--only` removes exactly one package and touches nothing else.
    let housekeeping = options.only.is_none();

    // Stale leases: stamp first, then delete; a failure keeps the lease.
    if housekeeping {
        for lease in &table.stale_leases {
            let path = PathBuf::from(&lease.path);
            if super::lease::is_live(&path).unwrap_or(true) {
                continue; // it came back to life, or cannot be probed
            }
            let stamped = lease
                .packages
                .iter()
                .try_for_each(|p| super::stamps::stamp(paths, &p.agent, &p.digest, Some(now)));
            match stamped.and_then(|()| std::fs::remove_file(&path).map_err(Into::into)) {
                Ok(()) => out.stale_leases_removed.push(lease.path.clone()),
                Err(error) => out.skipped.push(Skipped {
                    path: lease.path.clone(),
                    reason: format!("the stale lease could not be released: {error}"),
                }),
            }
        }
    }

    // Report the state GC leaves: a released lease's packages are now in
    // their window from the stamp, not "kept by a stale lease". What to
    // remove is still decided from the table above (those packages were kept
    // there, and are not removable in this pass).
    if !out.stale_leases_removed.is_empty() {
        let after = report(&refs::table(paths, &guard, &options.window, now)?, true);
        out.kept = after.kept;
        out.in_window = after.in_window;
    }

    // Leftovers of dead processes. Every writer holds the store lock shared,
    // so under the exclusive lock nothing live owns one — but another GC that
    // has just released the lock may still be deleting its own trash, and a
    // `.tmp-*` could be an older CLI's: only old ones go (review round 1).
    let mut leftovers: Vec<PathBuf> = Vec::new();
    if housekeeping {
        for leftover in &table.leftovers {
            let path = PathBuf::from(&leftover.path);
            let age = match leftover.kind {
                "trash" => TRASH_MIN_AGE,
                "temp" => TEMP_MIN_AGE,
                _ => continue, // unrecognized: never removed
            };
            let old_enough = std::fs::symlink_metadata(&path)
                .and_then(|m| m.modified())
                .map(|t| DateTime::<Utc>::from(t) + age <= now)
                .unwrap_or(false);
            if old_enough {
                leftovers.push(path);
            }
        }
    }

    let mut trash: Vec<PathBuf> = Vec::new();
    for gone in removable(&table, options) {
        let path = PathBuf::from(&gone.path);
        let container = path.parent().map(Path::to_path_buf).unwrap_or_default();
        let to = container.join(format!("{TRASH_PREFIX}{}", uuid::Uuid::new_v4().simple()));
        // The record first: a crash after the rename must not leave a package
        // gone with no word of why (review round 1). A tombstone is only read
        // once its package is absent, so one left by a failed rename is inert.
        let record = container.join(format!("{TOMBSTONE_PREFIX}{}.yaml", gone.receipt_key));
        let tombstone = Tombstone {
            format: TOMBSTONE_FORMAT.to_string(),
            agent: gone.agent.clone(),
            version: gone.version.clone(),
            digest: gone.digest.clone(),
            receipt_key: gone.receipt_key.clone(),
            removed_at: now_text(now),
            removed_by: "aware agent gc".to_string(),
            cli_version: env!("CARGO_PKG_VERSION").to_string(),
        };
        let written = serde_yaml::to_string(&tombstone)
            .map_err(|e| std::io::Error::other(e.to_string()))
            .and_then(|text| {
                crate::app_lock::replace_atomically(&record, text.as_bytes()).map(|_| ())
            });
        if let Err(error) = written {
            out.skipped.push(Skipped {
                path: gone.path,
                reason: format!("its removal record could not be written, so it was kept: {error}"),
            });
            continue;
        }
        match crate::fs::rename_dir_no_replace(&path, &to) {
            Ok(()) => {
                let _ = crate::fs::sync_dir(&container);
                trash.push(to);
                out.removed.push(gone);
            }
            Err(error) => {
                let _ = std::fs::remove_file(&record);
                out.skipped.push(Skipped {
                    path: gone.path,
                    reason: if error.kind() == std::io::ErrorKind::PermissionDenied {
                        format!("in use: {error}")
                    } else {
                        error.to_string()
                    },
                });
            }
        }
    }
    drop(guard);

    // Delete outside the lock: renamed away, nothing can reach them.
    for path in trash {
        match std::fs::remove_dir_all(&path) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(_) => out.pending_delete.push(path.display().to_string()),
        }
    }
    for path in leftovers {
        match std::fs::remove_dir_all(&path) {
            Ok(()) => out.leftovers_removed.push(path.display().to_string()),
            // Another process got there first: gone either way.
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => out.skipped.push(Skipped {
                path: path.display().to_string(),
                reason: format!("a leftover could not be deleted yet: {error}"),
            }),
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests;
