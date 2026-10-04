//! The immutable, content-addressed agent store (#626).
//!
//! `agents/<id>/` is the *current, mutable* working copy — install, update,
//! build and the skill builder all rewrite it. An approved app must not run on
//! whatever happens to sit there after an update, so every lock-bound run
//! dispatches from a **snapshot** instead:
//!
//! ```text
//! <AWARE_HOME>/agent-store/<id>/<tree-hex>/<receipt-key>/
//!     manifest.yaml, …, .aware-install.yaml   # the copied tree, receipt included
//!     .aware-package.yaml                     # { agent, version, digest, receipt-key, snapshotted-at }
//! <AWARE_HOME>/agent-store/<id>/<tree-hex>/.tmp-<random>/   # in-progress; ignored by every reader
//! ```
//!
//! * `<tree-hex>` is the 64-hex body of the bundle's `tree_digest`, which
//!   excludes both the receipt and `.aware-package.yaml`, so a snapshot's digest
//!   equals the working copy it came from.
//! * `<receipt-key>` is the sha256 hex of the receipt bytes, or the literal
//!   `no-receipt`. The same bytes installed from an official registry and from a
//!   local folder are two snapshots with two provenances, never one directory
//!   whose receipt depends on install order.
//! * [`snapshot`] is the only writer. It copies into a temp directory **inside
//!   the digest container**, re-checks the digest and receipt key, and publishes
//!   with one same-directory rename. Nothing in AWARE renames, rewrites or
//!   deletes a published package; repair and removal are GC's job (#629, which
//!   will need #627's run leases).
//!
//! Threat model: the store protects approved bytes from AWARE's *own* writers.
//! A person hand-editing files under `agent-store/` is out of scope, exactly as
//! hand-editing `bridges/` is — and is caught by the next run's digest check.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::error::AwareError;
use crate::paths::Paths;

/// The per-package record, dot-prefixed so it reads as metadata. Excluded from
/// `tree_digest` (see [`crate::install::integrity::is_install_metadata`]).
pub const PACKAGE_FILE: &str = ".aware-package.yaml";

/// The receipt key of a tree that carries no `.aware-install.yaml`.
pub const NO_RECEIPT: &str = "no-receipt";

/// Prefix of an in-progress snapshot directory inside a digest container.
const TEMP_PREFIX: &str = ".tmp-";

/// What `.aware-package.yaml` says about the package it sits in.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct PackageMetadata {
    pub agent: String,
    pub version: String,
    pub digest: String,
    pub receipt_key: String,
    pub snapshotted_at: String,
}

/// A verified store package.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoredPackage {
    pub root: PathBuf,
    pub agent: String,
    pub version: String,
    pub digest: String,
    pub receipt_key: String,
}

/// The 64 lowercase-hex body of a `sha256:<hex>` digest, or `None` when the
/// string is not exactly that shape. Every store join goes through this, so a
/// lock-supplied digest can never form a path segment other than 64 hex.
pub fn digest_hex(digest: &str) -> Option<&str> {
    let hex = digest.strip_prefix("sha256:")?;
    (hex.len() == 64
        && hex
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)))
    .then_some(hex)
}

/// Whether `key` is a receipt-key directory name: 64 lowercase hex or `no-receipt`.
pub fn is_receipt_key(key: &str) -> bool {
    key == NO_RECEIPT
        || (key.len() == 64
            && key
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)))
}

/// The receipt key of a tree: sha256 hex of its `.aware-install.yaml` bytes, or
/// [`NO_RECEIPT`] when it has none.
pub fn receipt_key(root: &Path) -> Result<String, AwareError> {
    let path = root.join(crate::install::provenance::FILE);
    match std::fs::symlink_metadata(&path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(NO_RECEIPT.to_string());
        }
        Err(error) => {
            return Err(
                std::io::Error::new(error.kind(), format!("{}: {error}", path.display())).into(),
            );
        }
        Ok(metadata) if !crate::fs::is_plain_file(&metadata) => {
            return Err(AwareError::Validation(format!(
                "[E_AGENT_STORE_INVALID] install receipt {} is not a plain file",
                path.display()
            )));
        }
        Ok(_) => {}
    }
    let bytes = std::fs::read(&path)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", path.display())))?;
    Ok(format!("{:x}", Sha256::digest(&bytes)))
}

/// `agent-store/<id>/<tree-hex>/` — a container of receipt-key packages, never
/// itself a package. Refuses an id that is not a plain segment or a digest that
/// is not `sha256:` + 64 lowercase hex, before any path is formed.
pub fn digest_container(paths: &Paths, id: &str, digest: &str) -> Result<PathBuf, AwareError> {
    if !crate::manifest::loader::is_safe_segment(id) {
        return Err(AwareError::Validation(format!(
            "[E_AGENT_STORE_INVALID] agent id {id:?} is not a plain name"
        )));
    }
    let hex = digest_hex(digest).ok_or_else(|| {
        AwareError::Validation(format!(
            "[E_AGENT_STORE_INVALID] {digest:?} is not a sha256 bundle digest"
        ))
    })?;
    Ok(paths.agent_store_dir().join(id).join(hex))
}

/// Load the manifest of a store package (or a staged tree) by its directory.
pub fn package_manifest(package: &Path) -> Result<crate::manifest::Agent, AwareError> {
    crate::manifest::loader::load_agent(&package.join("manifest.yaml"))
}

/// Verify the package at `dir` against the identity it must have. Returns the
/// reason it fails as a sentence, so callers can report it or skip it.
///
/// Checks, all fresh from disk: the directory is a plain directory; its tree
/// digest is `digest`; its receipt hashes to `key`; its `manifest.yaml` loads and
/// names `id`; its `.aware-package.yaml` agrees with the manifest's agent and
/// version and with `digest` and `key`.
pub fn verify_package(
    dir: &Path,
    id: &str,
    digest: &str,
    key: &str,
) -> Result<StoredPackage, String> {
    let metadata = std::fs::symlink_metadata(dir)
        .map_err(|e| format!("cannot read {}: {e}", dir.display()))?;
    if !crate::fs::is_plain_dir(&metadata) {
        return Err(format!("{} is not a plain directory", dir.display()));
    }
    let actual = crate::install::integrity::tree_digest(dir)
        .map_err(|e| format!("cannot hash {}: {e}", dir.display()))?;
    if actual != digest {
        return Err(format!(
            "its files hash to {actual}, not {digest} — they were changed after the snapshot"
        ));
    }
    let actual_key = receipt_key(dir).map_err(|e| e.to_string())?;
    if actual_key != key {
        return Err(format!(
            "its install receipt hashes to {actual_key}, not {key}"
        ));
    }
    let manifest = crate::manifest::loader::load_agent(&dir.join("manifest.yaml"))
        .map_err(|e| format!("its manifest does not load: {e}"))?;
    if manifest.agent != id {
        return Err(format!(
            "its manifest names agent {:?}, not {id:?}",
            manifest.agent
        ));
    }
    let record_path = dir.join(PACKAGE_FILE);
    let record: PackageMetadata = std::fs::read_to_string(&record_path)
        .map_err(|e| format!("cannot read {}: {e}", record_path.display()))
        .and_then(|text| {
            serde_yaml::from_str(&text)
                .map_err(|e| format!("{} is malformed: {e}", record_path.display()))
        })?;
    if record.agent != manifest.agent
        || record.version != manifest.version
        || record.digest != digest
        || record.receipt_key != key
    {
        return Err(format!(
            "its {PACKAGE_FILE} ({} {} {} {}) disagrees with the package ({} {} {digest} {key})",
            record.agent,
            record.version,
            record.digest,
            record.receipt_key,
            manifest.agent,
            manifest.version
        ));
    }
    Ok(StoredPackage {
        root: dir.to_path_buf(),
        agent: manifest.agent,
        version: manifest.version,
        digest: digest.to_string(),
        receipt_key: key.to_string(),
    })
}

/// The receipt-key package directories under `agent-store/<id>/<digest-hex>/`,
/// sorted by receipt key. `.tmp-*` and anything that is not a receipt-key name
/// are ignored. A missing container is an empty list. Nothing is verified here.
pub fn package_candidates(
    paths: &Paths,
    id: &str,
    digest: &str,
) -> Result<Vec<(String, PathBuf)>, AwareError> {
    let container = digest_container(paths, id, digest)?;
    let entries = match std::fs::read_dir(&container) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => {
            return Err(std::io::Error::new(
                error.kind(),
                format!("{}: {error}", container.display()),
            )
            .into());
        }
    };
    let mut out = Vec::new();
    for entry in entries {
        let entry = entry?;
        let Some(name) = entry.file_name().to_str().map(str::to_string) else {
            continue;
        };
        if name.starts_with(TEMP_PREFIX) || !is_receipt_key(&name) {
            continue;
        }
        out.push((name, entry.path()));
    }
    out.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(out)
}

/// The receipt-choice rank of a package (lower wins): an official registry
/// receipt, then any other registry receipt, then a local one, then none (or
/// one that does not parse). Reads the stored receipt only — never the index.
pub fn receipt_rank(package: &Path) -> u8 {
    use crate::install::provenance::InstallSource;
    match crate::install::provenance::read(package) {
        Some(InstallSource::Registry {
            official_source: true,
            ..
        }) => 0,
        Some(InstallSource::Registry { .. }) => 1,
        Some(InstallSource::Local { .. }) => 2,
        None => 3,
    }
}

/// One stored `(version, digest)`, for display in `agent list --json`.
#[derive(Debug, Clone, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct StoredVersion {
    pub version: String,
    pub digest: String,
}

/// Every `(version, digest)` the store holds for `id`, read from the package
/// records without verifying them — display only; decisions belong to the
/// resolver (`aware app check`).
pub fn stored_versions(paths: &Paths, id: &str) -> Vec<StoredVersion> {
    if !crate::manifest::loader::is_safe_segment(id) {
        return Vec::new();
    }
    let mut out = std::collections::BTreeSet::new();
    let Ok(digests) = std::fs::read_dir(paths.agent_store_dir().join(id)) else {
        return Vec::new();
    };
    for digest_dir in digests.flatten() {
        let Ok(packages) = std::fs::read_dir(digest_dir.path()) else {
            continue;
        };
        for package in packages.flatten() {
            let name = package.file_name();
            if name.to_str().is_none_or(|n| !is_receipt_key(n)) {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(package.path().join(PACKAGE_FILE)) else {
                continue;
            };
            let Ok(record) = serde_yaml::from_str::<PackageMetadata>(&text) else {
                continue;
            };
            if record.agent == id && digest_hex(&record.digest).is_some() {
                out.insert(StoredVersion {
                    version: record.version,
                    digest: record.digest,
                });
            }
        }
    }
    out.into_iter().collect()
}

/// The step a test can make [`snapshot`] fail at.
#[cfg_attr(not(test), allow(dead_code))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FaultStep {
    Copy,
    Recheck,
    Rename,
}

#[cfg(test)]
thread_local! {
    /// `(n, step)`: the `n`th snapshot call (0-based) on this thread fails at `step`.
    static FAULT: std::cell::Cell<Option<(u32, FaultStep)>> = const { std::cell::Cell::new(None) };
    static CALLS: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
    /// Runs after the digest is computed and before the copy — where a
    /// concurrent writer would have to strike to be caught by the recheck.
    #[allow(clippy::type_complexity)]
    static BEFORE_COPY: std::cell::RefCell<Option<Box<dyn FnMut()>>> = const { std::cell::RefCell::new(None) };
}

/// Arrange for the `nth` snapshot call on this thread to fail at `step`.
#[cfg(test)]
pub(crate) fn inject_fault(nth: u32, step: FaultStep) {
    CALLS.with(|c| c.set(0));
    FAULT.with(|f| f.set(Some((nth, step))));
}

#[cfg(test)]
pub(crate) fn clear_fault() {
    FAULT.with(|f| f.set(None));
    CALLS.with(|c| c.set(0));
    BEFORE_COPY.with(|h| *h.borrow_mut() = None);
}

#[cfg(test)]
pub(crate) fn set_before_copy(hook: Box<dyn FnMut()>) {
    BEFORE_COPY.with(|h| *h.borrow_mut() = Some(hook));
}

#[cfg(test)]
fn before_copy() {
    BEFORE_COPY.with(|h| {
        if let Some(hook) = h.borrow_mut().as_mut() {
            hook();
        }
    });
}

#[cfg(not(test))]
fn before_copy() {}

#[cfg(test)]
fn fault_at(call: u32, step: FaultStep) -> Result<(), AwareError> {
    if FAULT.with(|f| f.get()) == Some((call, step)) {
        return Err(AwareError::Internal(format!(
            "injected snapshot fault at {step:?}"
        )));
    }
    Ok(())
}

#[cfg(not(test))]
fn fault_at(_call: u32, _step: FaultStep) -> Result<(), AwareError> {
    Ok(())
}

#[cfg(test)]
fn next_call() -> u32 {
    CALLS.with(|c| {
        let n = c.get();
        c.set(n + 1);
        n
    })
}

#[cfg(not(test))]
fn next_call() -> u32 {
    0
}

/// Snapshot the agent tree at `current_root` into the store and return the
/// verified package. Idempotent: an existing valid package is returned as is.
///
/// The algorithm (plan §Snapshot):
/// 0. the manifest's id must be a plain segment;
/// 1. D = `tree_digest(current_root)`, K = its receipt key;
/// 2. an existing `<id>/<D-hex>/<K>/` is verified and returned — or, if it does
///    not verify, the snapshot **refuses**, naming it (nothing is ever repaired
///    or replaced here);
/// 3. otherwise the tree is copied into `<id>/<D-hex>/.tmp-<random>/` (same
///    directory as the final name), every file fsynced, and the package record
///    written from the *copied* manifest;
/// 4. the copy is re-hashed: its digest must be D and its receipt key K, or the
///    working copy changed during the copy — retry once, then refuse;
/// 5. one rename publishes it; if the name appeared concurrently, that package
///    is verified and used instead, and only our temp dir is removed;
/// 6. durability: Unix fsyncs the package, digest and id directories; Windows
///    renames with `MoveFileExW(MOVEFILE_WRITE_THROUGH)`.
pub fn snapshot(paths: &Paths, current_root: &Path) -> Result<StoredPackage, AwareError> {
    let call = next_call();
    let mut last_change = String::new();
    for _ in 0..2 {
        match snapshot_once(paths, current_root, call)? {
            Attempt::Done(package) => return Ok(package),
            Attempt::Changed(reason) => last_change = reason,
        }
    }
    Err(AwareError::Validation(format!(
        "[E_AGENT_STORE_CHANGED] the agent at {} kept changing while it was being snapshotted ({last_change}); \
         wait for whatever is writing it to finish and try again",
        current_root.display()
    )))
}

enum Attempt {
    Done(StoredPackage),
    Changed(String),
}

fn snapshot_once(paths: &Paths, current_root: &Path, call: u32) -> Result<Attempt, AwareError> {
    let manifest = crate::manifest::loader::load_agent(&current_root.join("manifest.yaml"))?;
    let id = manifest.agent.clone();
    if !crate::manifest::loader::is_safe_segment(&id) {
        return Err(AwareError::Validation(format!(
            "[E_AGENT_STORE_INVALID] agent id {id:?} is not a plain name"
        )));
    }
    let digest = crate::install::integrity::tree_digest(current_root)?;
    let key = receipt_key(current_root)?;
    let container = digest_container(paths, &id, &digest)?;
    let package = container.join(&key);

    if std::fs::symlink_metadata(&package).is_ok() {
        return verify_existing(&package, &id, &digest, &key).map(Attempt::Done);
    }

    std::fs::create_dir_all(&container)?;
    let temp = container.join(format!("{TEMP_PREFIX}{}", uuid::Uuid::new_v4().simple()));
    before_copy();
    let staged = (|| -> Result<Result<PackageMetadata, String>, AwareError> {
        copy_tree(current_root, &temp)?;
        fault_at(call, FaultStep::Copy)?;
        // Step 4, before the record is written: the copy must be exactly the
        // bytes D and receipt K were computed from.
        let copied_digest = crate::install::integrity::tree_digest(&temp)?;
        let copied_key = receipt_key(&temp)?;
        fault_at(call, FaultStep::Recheck)?;
        if copied_digest != digest || copied_key != key {
            return Ok(Err(format!(
                "digest {digest} / receipt {key} became {copied_digest} / {copied_key}"
            )));
        }
        let copied = crate::manifest::loader::load_agent(&temp.join("manifest.yaml"))?;
        if copied.agent != id {
            return Ok(Err(format!(
                "its manifest id changed from {id:?} to {:?}",
                copied.agent
            )));
        }
        let record = PackageMetadata {
            agent: copied.agent,
            version: copied.version,
            digest: digest.clone(),
            receipt_key: key.clone(),
            snapshotted_at: chrono::Utc::now().to_rfc3339(),
        };
        write_synced(
            &temp.join(PACKAGE_FILE),
            serde_yaml::to_string(&record)?.as_bytes(),
        )?;
        Ok(Ok(record))
    })();
    let record = match staged {
        Ok(Ok(record)) => record,
        Ok(Err(reason)) => {
            let _ = std::fs::remove_dir_all(&temp);
            return Ok(Attempt::Changed(reason));
        }
        Err(error) => {
            let _ = std::fs::remove_dir_all(&temp);
            return Err(error);
        }
    };

    let renamed =
        fault_at(call, FaultStep::Rename).and_then(|()| Ok(publish_dir(&temp, &package)?));
    if let Err(error) = renamed {
        let _ = std::fs::remove_dir_all(&temp);
        // Lost a race to an identical snapshot: use theirs, verified.
        if std::fs::symlink_metadata(&package).is_ok() {
            return verify_existing(&package, &id, &digest, &key).map(Attempt::Done);
        }
        return Err(error);
    }
    sync_parents(&package, &container);
    Ok(Attempt::Done(StoredPackage {
        root: package,
        agent: record.agent,
        version: record.version,
        digest,
        receipt_key: key,
    }))
}

fn verify_existing(
    package: &Path,
    id: &str,
    digest: &str,
    key: &str,
) -> Result<StoredPackage, AwareError> {
    verify_package(package, id, digest, key).map_err(|reason| {
        AwareError::Validation(format!(
            "[E_AGENT_STORE_INVALID] the stored package {} does not verify: {reason}. \
             AWARE never repairs or replaces a stored package; move it aside by hand to let a fresh snapshot be taken",
            package.display()
        ))
    })
}

/// Copy every plain file of `src` (minus any `.aware-package.yaml`, which is
/// rewritten for the copy) into a fresh `dst`, fsyncing each file. A symlink or
/// reparse point refuses, exactly as `tree_digest` does.
fn copy_tree(src: &Path, dst: &Path) -> Result<(), AwareError> {
    std::fs::create_dir(dst)?;
    for (relative, from) in crate::fs::plain_files_under(src, "agent bundle")? {
        if relative == PACKAGE_FILE {
            continue;
        }
        let to = relative
            .split('/')
            .fold(dst.to_path_buf(), |path, part| path.join(part));
        if let Some(parent) = to.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::copy(&from, &to)?;
        std::fs::OpenOptions::new()
            .write(true)
            .open(&to)?
            .sync_all()?;
    }
    Ok(())
}

fn write_synced(path: &Path, bytes: &[u8]) -> Result<(), AwareError> {
    use std::io::Write;
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    Ok(())
}

/// Rename a fully written temp directory to its final name in the same
/// directory, failing (never replacing) when the name already exists.
#[cfg(not(windows))]
fn publish_dir(temp: &Path, package: &Path) -> std::io::Result<()> {
    // `rename(2)` replaces an EMPTY destination directory; a published package
    // is never empty (it holds at least the manifest and the record), and the
    // caller checked the name was free, so the remaining race is with another
    // snapshot of the same bytes, which the caller verifies on failure.
    if std::fs::symlink_metadata(package).is_ok() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            format!("{} already exists", package.display()),
        ));
    }
    std::fs::rename(temp, package)
}

#[cfg(windows)]
fn publish_dir(temp: &Path, package: &Path) -> std::io::Result<()> {
    use std::os::windows::ffi::OsStrExt;
    use windows_sys::Win32::Storage::FileSystem::{MOVEFILE_WRITE_THROUGH, MoveFileExW};
    // Same verbatim-prefix requirement as `provider_store::atomic_rename` (#593).
    let source = crate::fs::win32_verbatim(temp)?;
    let destination = crate::fs::win32_verbatim(package)?;
    let source_wide = source
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    let destination_wide = destination
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<_>>();
    // No MOVEFILE_REPLACE_EXISTING: an existing package must never be replaced.
    // SAFETY: both pointers name live, NUL-terminated UTF-16 buffers for the duration of the call.
    let moved = unsafe {
        MoveFileExW(
            source_wide.as_ptr(),
            destination_wide.as_ptr(),
            MOVEFILE_WRITE_THROUGH,
        )
    };
    if moved == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

/// Make the rename durable before anything is promoted on top of it. Directory
/// handles cannot be flushed portably on Windows, where the write-through move
/// above carries this instead.
#[cfg(unix)]
fn sync_parents(package: &Path, container: &Path) {
    for dir in [Some(package), Some(container), container.parent()]
        .into_iter()
        .flatten()
    {
        if let Ok(handle) = std::fs::File::open(dir) {
            let _ = handle.sync_all();
        }
    }
}

#[cfg(not(unix))]
fn sync_parents(_package: &Path, _container: &Path) {}

#[cfg(test)]
mod tests {
    use super::*;

    fn home() -> (tempfile::TempDir, Paths) {
        let tmp = tempfile::tempdir().unwrap();
        let paths = Paths {
            aware_home: tmp.path().to_path_buf(),
        };
        (tmp, paths)
    }

    pub(crate) fn write_agent(dir: &Path, id: &str, version: &str) {
        std::fs::create_dir_all(dir.join("skills")).unwrap();
        std::fs::write(
            dir.join("manifest.yaml"),
            format!(
                "agent: {id}\nversion: {version}\ndescription: x\nstateful: false\nlicense: MIT\n\
                 transport:\n  cli:\n    binary: aware-{id}\ncommands:\n  go:\n    lifecycle: single\n    description: x\n"
            ),
        )
        .unwrap();
        std::fs::write(dir.join("skills").join("a.md"), format!("skill {version}")).unwrap();
    }

    #[test]
    fn digest_and_key_validation_refuse_path_shaped_input() {
        assert!(digest_hex(&format!("sha256:{}", "a".repeat(64))).is_some());
        assert!(digest_hex(&format!("sha256:{}", "A".repeat(64))).is_none());
        assert!(digest_hex(&format!("sha256:{}", "a".repeat(63))).is_none());
        assert!(digest_hex("sha256:../../etc").is_none());
        assert!(digest_hex(&"a".repeat(64)).is_none());
        assert!(is_receipt_key(NO_RECEIPT));
        assert!(is_receipt_key(&"0".repeat(64)));
        assert!(!is_receipt_key(".tmp-x"));
        assert!(!is_receipt_key(".."));

        let (_tmp, paths) = home();
        let good = format!("sha256:{}", "b".repeat(64));
        assert!(digest_container(&paths, "../evil", &good).is_err());
        assert!(digest_container(&paths, "ok", "sha256:../../x").is_err());
        assert!(digest_container(&paths, "ok", &good).is_ok());
    }

    #[test]
    fn snapshot_creates_a_verified_package_with_its_record_and_is_idempotent() {
        let (_tmp, paths) = home();
        let current = paths.agents_dir().join("alpha");
        write_agent(&current, "alpha", "1.0.0");
        let digest = crate::install::integrity::tree_digest(&current).unwrap();

        let first = snapshot(&paths, &current).unwrap();
        assert_eq!(first.digest, digest);
        assert_eq!(first.receipt_key, NO_RECEIPT);
        assert_eq!(first.version, "1.0.0");
        assert_eq!(
            first.root,
            paths
                .agent_store_dir()
                .join("alpha")
                .join(digest_hex(&digest).unwrap())
                .join(NO_RECEIPT)
        );
        let record: PackageMetadata =
            serde_yaml::from_str(&std::fs::read_to_string(first.root.join(PACKAGE_FILE)).unwrap())
                .unwrap();
        assert_eq!(record.agent, "alpha");
        assert_eq!(record.digest, digest);
        // The snapshot's own digest equals the working copy's.
        assert_eq!(
            crate::install::integrity::tree_digest(&first.root).unwrap(),
            digest
        );
        let second = snapshot(&paths, &current).unwrap();
        assert_eq!(first, second);
        // No temp directory left behind.
        let container = first.root.parent().unwrap();
        assert_eq!(std::fs::read_dir(container).unwrap().count(), 1);
    }

    #[test]
    fn receipt_identity_keys_separate_packages_for_the_same_bytes() {
        let (_tmp, paths) = home();
        let current = paths.agents_dir().join("alpha");
        write_agent(&current, "alpha", "1.0.0");
        let local = snapshot(&paths, &current).unwrap();
        crate::install::provenance::write_required(
            &current,
            &crate::install::provenance::InstallSource::Local { path: "x".into() },
        )
        .unwrap();
        let receipted = snapshot(&paths, &current).unwrap();
        assert_eq!(local.digest, receipted.digest);
        assert_ne!(local.root, receipted.root);
        assert_ne!(receipted.receipt_key, NO_RECEIPT);
        assert_eq!(receipt_rank(&receipted.root), 2);
        assert_eq!(receipt_rank(&local.root), 3);
    }

    #[test]
    fn a_corrupt_existing_package_makes_the_snapshot_refuse_and_is_left_untouched() {
        let (_tmp, paths) = home();
        let current = paths.agents_dir().join("alpha");
        write_agent(&current, "alpha", "1.0.0");
        let package = snapshot(&paths, &current).unwrap();
        std::fs::write(package.root.join("skills").join("a.md"), "tampered").unwrap();

        let error = snapshot(&paths, &current).unwrap_err().to_string();
        assert!(error.contains("E_AGENT_STORE_INVALID"), "{error}");
        assert!(error.contains("does not verify"), "{error}");
        assert_eq!(
            std::fs::read_to_string(package.root.join("skills").join("a.md")).unwrap(),
            "tampered",
            "the store package must never be repaired or replaced"
        );
    }

    #[test]
    fn a_working_copy_that_changes_mid_copy_is_detected_by_the_recheck() {
        let (_tmp, paths) = home();
        let current = paths.agents_dir().join("alpha");
        write_agent(&current, "alpha", "1.0.0");
        let before = crate::install::integrity::tree_digest(&current).unwrap();

        // Once: the first attempt copies bytes that no longer hash to the digest
        // it computed, so it must discard that copy and retry on the new bytes.
        let skill = current.join("skills").join("a.md");
        let mut fired = false;
        let target = skill.clone();
        set_before_copy(Box::new(move || {
            if !fired {
                fired = true;
                std::fs::write(&target, "changed mid-copy").unwrap();
            }
        }));
        let package = snapshot(&paths, &current).unwrap();
        clear_fault();
        let after = crate::install::integrity::tree_digest(&current).unwrap();
        assert_ne!(before, after);
        assert_eq!(
            package.digest, after,
            "the retry must snapshot the new bytes"
        );
        assert_eq!(
            std::fs::read_to_string(package.root.join("skills").join("a.md")).unwrap(),
            "changed mid-copy"
        );
        // The discarded attempt left nothing under the old digest.
        assert!(
            package_candidates(&paths, "alpha", &before)
                .unwrap()
                .is_empty()
        );
        let old_container = digest_container(&paths, "alpha", &before).unwrap();
        assert_eq!(std::fs::read_dir(&old_container).unwrap().count(), 0);

        // Always: the snapshot gives up with a named error rather than looping.
        // (Fresh bytes first, so no existing package short-circuits the copy.)
        std::fs::write(&skill, "fresh bytes").unwrap();
        let target = skill.clone();
        let mut n = 0u32;
        set_before_copy(Box::new(move || {
            n += 1;
            std::fs::write(&target, format!("churn {n}")).unwrap();
        }));
        let error = snapshot(&paths, &current).unwrap_err().to_string();
        clear_fault();
        assert!(error.contains("E_AGENT_STORE_CHANGED"), "{error}");
    }

    #[test]
    fn a_leftover_temp_dir_is_ignored_by_readers_and_by_the_next_snapshot() {
        let (_tmp, paths) = home();
        let current = paths.agents_dir().join("alpha");
        write_agent(&current, "alpha", "1.0.0");
        let digest = crate::install::integrity::tree_digest(&current).unwrap();
        let container = digest_container(&paths, "alpha", &digest).unwrap();
        std::fs::create_dir_all(container.join(".tmp-crashed")).unwrap();
        std::fs::write(container.join(".tmp-crashed").join("manifest.yaml"), "junk").unwrap();
        assert!(
            package_candidates(&paths, "alpha", &digest)
                .unwrap()
                .is_empty()
        );
        let package = snapshot(&paths, &current).unwrap();
        assert_eq!(
            package_candidates(&paths, "alpha", &digest).unwrap(),
            vec![(NO_RECEIPT.to_string(), package.root.clone())]
        );
    }

    #[test]
    fn an_injected_failure_at_any_step_leaves_no_package_and_no_temp_dir() {
        for step in [FaultStep::Copy, FaultStep::Recheck, FaultStep::Rename] {
            let (_tmp, paths) = home();
            let current = paths.agents_dir().join("alpha");
            write_agent(&current, "alpha", "1.0.0");
            let digest = crate::install::integrity::tree_digest(&current).unwrap();
            inject_fault(0, step);
            let error = snapshot(&paths, &current).unwrap_err().to_string();
            clear_fault();
            assert!(error.contains("injected"), "{step:?}: {error}");
            let container = digest_container(&paths, "alpha", &digest).unwrap();
            assert_eq!(
                std::fs::read_dir(&container).unwrap().count(),
                0,
                "{step:?} left something behind"
            );
            // And the next attempt succeeds.
            snapshot(&paths, &current).unwrap();
        }
    }

    #[test]
    fn verify_package_checks_identity_and_record_not_only_bytes() {
        let (_tmp, paths) = home();
        let current = paths.agents_dir().join("alpha");
        write_agent(&current, "alpha", "1.0.0");
        let package = snapshot(&paths, &current).unwrap();
        assert!(verify_package(&package.root, "beta", &package.digest, NO_RECEIPT).is_err());
        assert!(verify_package(&package.root, "alpha", &package.digest, &"0".repeat(64)).is_err());
        // A record that lies about the version refuses even though the bytes hash right.
        let record_path = package.root.join(PACKAGE_FILE);
        let text = std::fs::read_to_string(&record_path).unwrap();
        std::fs::write(
            &record_path,
            text.replace("version: 1.0.0", "version: 9.9.9"),
        )
        .unwrap();
        let reason =
            verify_package(&package.root, "alpha", &package.digest, NO_RECEIPT).unwrap_err();
        assert!(reason.contains("disagrees"), "{reason}");
    }

    #[test]
    fn stored_versions_lists_every_package_once() {
        let (_tmp, paths) = home();
        let current = paths.agents_dir().join("alpha");
        write_agent(&current, "alpha", "1.0.0");
        let one = snapshot(&paths, &current).unwrap();
        crate::install::provenance::write(
            &current,
            &crate::install::provenance::InstallSource::Local { path: "x".into() },
        );
        snapshot(&paths, &current).unwrap();
        write_agent(&current, "alpha", "1.1.0");
        let two = snapshot(&paths, &current).unwrap();
        let listed = stored_versions(&paths, "alpha");
        assert_eq!(
            listed,
            vec![
                StoredVersion {
                    version: "1.0.0".into(),
                    digest: one.digest
                },
                StoredVersion {
                    version: "1.1.0".into(),
                    digest: two.digest
                },
            ]
        );
        assert!(stored_versions(&paths, "../alpha").is_empty());
    }
}
