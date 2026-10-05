//! Run leases (#627-b, plan §2, §8 R1-9): which store packages a run in
//! progress is using, so `aware agent gc` (#629) never removes them under it.
//!
//! A lease is `agent-store-control/leases/<run-id>.lease`, a JSON record of
//! every package the run resolved (directly, and inside every backing app it
//! runs), held under an **OS lock** (`fs2`, shared) for as long as the run's
//! `aware` process holds the [`RunLease`]. An OS lock dies with the process —
//! a kill, a crash, a power loss — so there is no heartbeat, no expiry clock
//! and no PID-reuse guess: a lease file whose lock can be taken is stale.
//!
//! Acquire happens while the store reference lock is held shared (the run
//! takes it before reading its approval), so GC — which takes it exclusive —
//! never sees a resolved package without its lease, nor a fresh lease file
//! before its lock is held.
//!
//! On release the run's packages are stamped last-needed (`super::stamps`)
//! and only THEN is the lease file deleted; if stamping fails the stale file
//! stays, and GC treats it as a reference until it can stamp it itself.

use std::path::{Path, PathBuf};

use fs2::FileExt;
use serde::{Deserialize, Serialize};

use crate::error::AwareError;
use crate::paths::Paths;

/// The lease record format.
pub const LEASE_FORMAT: &str = "aware.agent-lease/v1";
const LEASE_EXT: &str = "lease";

/// One package a run uses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LeasePackage {
    pub agent: String,
    pub version: String,
    pub digest: String,
    pub receipt_key: String,
    pub root: String,
    /// The app-backed agent this package is reached through, when nested.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub via: Option<String>,
}

/// A lease file's body.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct LeaseRecord {
    pub format: String,
    pub run_id: String,
    pub app: String,
    pub instance: String,
    pub pid: u32,
    pub started_at: String,
    pub cli_version: String,
    pub packages: Vec<LeasePackage>,
}

impl LeaseRecord {
    pub fn new(run_id: &str, app: &str, instance: &str, packages: Vec<LeasePackage>) -> Self {
        Self {
            format: LEASE_FORMAT.into(),
            run_id: run_id.into(),
            app: app.into(),
            instance: instance.into(),
            pid: std::process::id(),
            started_at: chrono::Utc::now().to_rfc3339(),
            cli_version: env!("CARGO_PKG_VERSION").into(),
            packages,
        }
    }
}

#[cfg(test)]
type LockHook = Box<dyn FnOnce(&Path)>;

#[cfg(test)]
thread_local! {
    /// Run once on this thread between writing a lease and locking it — the
    /// window a lister's liveness probe can fall into (review round 1).
    static BEFORE_LOCK: std::cell::RefCell<Option<LockHook>> = std::cell::RefCell::new(None);
}

#[cfg(test)]
pub(crate) fn on_before_lock(hook: LockHook) {
    BEFORE_LOCK.with(|h| *h.borrow_mut() = Some(hook));
}

#[cfg(test)]
fn before_lock(path: &Path) {
    if let Some(hook) = BEFORE_LOCK.with(|h| h.borrow_mut().take()) {
        hook(path);
    }
}

#[cfg(not(test))]
fn before_lock(_: &Path) {}

/// The leases directory.
pub fn leases_dir(paths: &Paths) -> PathBuf {
    paths.agent_store_control_dir().join("leases")
}

/// A held run lease. Dropping it stamps the packages and removes the file.
#[derive(Debug)]
pub struct RunLease {
    file: Option<std::fs::File>,
    path: PathBuf,
    paths: Paths,
    packages: Vec<LeasePackage>,
}

impl RunLease {
    /// Write and lock the lease of `record`. Requires the store reference lock
    /// (`guard`), held since before the run read its approval.
    pub fn acquire(
        paths: &Paths,
        guard: &super::RefGuard,
        record: LeaseRecord,
    ) -> Result<Self, AwareError> {
        super::guard::require_home(guard, paths)?;
        if !crate::manifest::loader::is_safe_segment(&record.run_id) {
            return Err(AwareError::Internal(format!(
                "run id {:?} is not a plain name",
                record.run_id
            )));
        }
        let dir = leases_dir(paths);
        std::fs::create_dir_all(&dir)
            .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", dir.display())))?;
        let path = dir.join(format!("{}.{LEASE_EXT}", record.run_id));
        let bytes = serde_json::to_vec_pretty(&record)
            .map_err(|e| AwareError::Internal(format!("serialize lease: {e}")))?;
        let io = |e: std::io::Error| -> AwareError {
            std::io::Error::new(e.kind(), format!("{}: {e}", path.display())).into()
        };
        let file = (|| -> std::io::Result<std::fs::File> {
            use std::io::Write;
            let mut file = std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .create_new(true)
                .open(&path)?;
            file.write_all(&bytes)?;
            file.sync_all()?;
            before_lock(&path);
            // Blocking, never a single try: a lister probing liveness holds the
            // lock exclusive for an instant, and must not abort a run that is
            // starting (review round 1).
            file.lock_shared()?;
            Ok(file)
        })()
        .map_err(|e| {
            let _ = std::fs::remove_file(&path);
            io(e)
        })?;
        // A run that starts also needs its packages now: the window of a
        // package a long run finishes with counts from its end (drop).
        for package in &record.packages {
            if let Err(error) = super::stamps::stamp(paths, &package.agent, &package.digest, None) {
                eprintln!(
                    "\u{26a0} could not record that {} {} is in use ({error}); the run is unaffected",
                    package.agent, package.version
                );
            }
        }
        Ok(Self {
            file: Some(file),
            path,
            paths: paths.clone(),
            packages: record.packages,
        })
    }

    #[cfg_attr(not(test), allow(dead_code))]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for RunLease {
    fn drop(&mut self) {
        let stamped = self.packages.iter().all(|package| {
            super::stamps::stamp(&self.paths, &package.agent, &package.digest, None).is_ok()
        });
        // Release the OS lock (closing the handle), then — only when every
        // package is stamped — remove the file. An unstamped stale lease stays
        // as evidence; GC stamps and removes it under its exclusive lock.
        drop(self.file.take());
        if stamped {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

/// One lease, as `aware agent leases` reports it.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct LeaseRow {
    pub run_id: String,
    pub app: String,
    pub instance: String,
    pub pid: u32,
    pub started_at: String,
    pub cli_version: String,
    pub live: bool,
    pub path: String,
    pub packages: Vec<LeasePackage>,
}

/// A lease file that cannot be read as a lease.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct UnreadableLease {
    pub path: String,
    pub live: Option<bool>,
    pub problem: String,
}

/// Every lease file: live (its lock is held) or stale.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct Leases {
    pub leases: Vec<LeaseRow>,
    pub stale: Vec<LeaseRow>,
    pub unreadable: Vec<UnreadableLease>,
}

/// Whether the lease at `path` is held by a live process. `Err` when the lock
/// cannot even be probed.
pub fn is_live(path: &Path) -> std::io::Result<bool> {
    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)?;
    match file.try_lock_exclusive() {
        Ok(()) => {
            let _ = fs2::FileExt::unlock(&file);
            Ok(false)
        }
        Err(error) if super::guard::lock_is_contended(&error) => Ok(true),
        Err(error) => Err(error),
    }
}

/// Read every lease file. Read-only (probing a lock takes and releases it).
pub fn list(paths: &Paths) -> Result<Leases, AwareError> {
    let dir = leases_dir(paths);
    let entries = match std::fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(Leases::default());
        }
        Err(error) => {
            return Err(
                std::io::Error::new(error.kind(), format!("{}: {error}", dir.display())).into(),
            );
        }
    };
    let mut files: Vec<PathBuf> = entries
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == LEASE_EXT))
        .collect();
    files.sort();
    let mut out = Leases::default();
    for path in files {
        let live = is_live(&path);
        let record = std::fs::read(&path)
            .map_err(|e| e.to_string())
            .and_then(|bytes| {
                serde_json::from_slice::<LeaseRecord>(&bytes).map_err(|e| e.to_string())
            })
            .and_then(|r| {
                if r.format == LEASE_FORMAT {
                    Ok(r)
                } else {
                    Err(format!("format {:?} is not {LEASE_FORMAT}", r.format))
                }
            });
        match (record, live) {
            (Ok(record), Ok(live)) => {
                let row = LeaseRow {
                    run_id: record.run_id,
                    app: record.app,
                    instance: record.instance,
                    pid: record.pid,
                    started_at: record.started_at,
                    cli_version: record.cli_version,
                    live,
                    path: path.display().to_string(),
                    packages: record.packages,
                };
                if live {
                    out.leases.push(row);
                } else {
                    out.stale.push(row);
                }
            }
            (Err(problem), live) => out.unreadable.push(UnreadableLease {
                path: path.display().to_string(),
                live: live.ok(),
                problem,
            }),
            (Ok(_), Err(error)) => out.unreadable.push(UnreadableLease {
                path: path.display().to_string(),
                live: None,
                problem: format!("its lock cannot be probed: {error}"),
            }),
        }
    }
    Ok(out)
}

/// The packages of `catalogue` (and of every backing app it runs, recursively)
/// as lease entries.
pub fn packages_of(catalogue: &crate::agent_resolution::ResolvedCatalogue) -> Vec<LeasePackage> {
    let mut out = Vec::new();
    collect(catalogue, None, &mut out);
    out.sort_by(|a, b| (&a.agent, &a.digest, &a.via).cmp(&(&b.agent, &b.digest, &b.via)));
    out.dedup();
    out
}

fn collect(
    catalogue: &crate::agent_resolution::ResolvedCatalogue,
    via: Option<&str>,
    out: &mut Vec<LeasePackage>,
) {
    for agent in catalogue.agents() {
        let id = &agent.manifest.agent;
        let Some(info) = catalogue.info(id) else {
            continue;
        };
        out.push(LeasePackage {
            agent: id.clone(),
            version: info.version.clone(),
            digest: info.digest.clone(),
            receipt_key: agent
                .root
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default()
                .to_string(),
            root: agent.root.display().to_string(),
            via: via.map(str::to_string),
        });
    }
    for (wrapper, nested) in catalogue.nested_apps() {
        let path = match via {
            Some(outer) => format!("{outer}>{wrapper}"),
            None => wrapper.clone(),
        };
        collect(&nested.catalogue, Some(&path), out);
    }
}

#[cfg(test)]
mod tests;
