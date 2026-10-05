//! The legacy store import (#627-b, plan §11–§12).
//!
//! From this release every store reader and writer uses
//! `AWARE_HOME/agent-store-v2/`. AWARE 0.149–0.151 have `agent-store/`
//! hard-coded and take no run leases, so nothing they do can depend on a
//! package in `agent-store-v2/` — which is what makes it safe for GC (#629)
//! to remove packages there, and only there. The barrier is structural, not
//! inferred.
//!
//! The legacy `agent-store/` is **never written** by this CLI: it stays the
//! private store of any older CLI still used on the home. Its verified packages
//! are copied into `agent-store-v2/` by [`ensure`], which [`super::open`] runs
//! before any shared guard exists:
//!
//! * the legacy tree is listed by name only (`<id>/<hex>/<key>`, three levels)
//!   and compared with the import record
//!   (`agent-store-control/legacy-import.json`); nothing to do when they agree —
//!   so an older CLI that snapshots a new package later is picked up by the next
//!   command;
//! * otherwise, under the store lock held **exclusive** (taken while this
//!   thread holds no guard — never an upgrade), each new package is verified in
//!   place (digest, receipt key, manifest, record), copied into a same-container
//!   temp in v2 with its original `.aware-package.yaml` bytes (so
//!   `snapshotted-at` is kept), verified again and published with one
//!   no-replace rename. A package that does not verify is not imported, and is
//!   recorded with its reason;
//! * hard links are never used: an older CLI writing through one would alias
//!   v2 bytes.
//!
//! Before importing, both stores must be physically distinct directories and
//! neither may be a link or junction (`E_AGENT_STORE_ALIASED` otherwise): a v2
//! that resolves into the legacy store would let this CLI write the legacy store
//! and let GC delete an older CLI's packages.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use super::{PACKAGE_FILE, TEMP_PREFIX, digest_container, digest_hex, is_receipt_key, probe};
use crate::error::AwareError;
use crate::paths::Paths;

/// The import record's name in `agent-store-control/`.
pub const RECORD_FILE: &str = "legacy-import.json";
const RECORD_FORMAT: u32 = 1;

/// What the import has seen of the legacy store.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ImportRecord {
    pub format: u32,
    /// Every legacy package path (`<id>/<hex>/<key>`) seen at the last import.
    pub seen: BTreeSet<String>,
    /// Those now present in `agent-store-v2/` (copied, or already there).
    pub imported: BTreeSet<String>,
    /// Those that did not verify, and why. Re-tried only if the legacy
    /// listing changes.
    pub skipped: BTreeMap<String, String>,
}

/// What one import pass did.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ImportReport {
    pub copied: Vec<String>,
    pub already_present: Vec<String>,
    pub skipped: BTreeMap<String, String>,
}

fn record_path(paths: &Paths) -> PathBuf {
    paths.agent_store_control_dir().join(RECORD_FILE)
}

/// Read the import record; a missing or unreadable one is empty (the next
/// pass re-verifies everything — idempotent: a package already in v2 is only
/// verified, never copied twice).
pub fn read_record(paths: &Paths) -> ImportRecord {
    std::fs::read(record_path(paths))
        .ok()
        .and_then(|bytes| serde_json::from_slice::<ImportRecord>(&bytes).ok())
        .filter(|r| r.format == RECORD_FORMAT)
        .unwrap_or_default()
}

fn is_hex64(name: &str) -> bool {
    name.len() == 64
        && name
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

/// The legacy store's package paths, by name only. Anything that is not a
/// package name (a `.tmp-*`, a stray file, an unsafe id) is not listed.
pub fn legacy_listing(paths: &Paths) -> Result<BTreeSet<String>, AwareError> {
    let root = paths.legacy_agent_store_dir();
    let mut out = BTreeSet::new();
    let ids = match std::fs::read_dir(&root) {
        Ok(ids) => ids,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(out),
        Err(error) => {
            return Err(
                std::io::Error::new(error.kind(), format!("{}: {error}", root.display())).into(),
            );
        }
    };
    let names = |dir: &Path| -> Result<Vec<(String, PathBuf)>, AwareError> {
        let mut out = Vec::new();
        for entry in std::fs::read_dir(dir)
            .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", dir.display())))?
        {
            let entry = entry
                .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", dir.display())))?;
            if let Some(name) = entry.file_name().to_str() {
                out.push((name.to_string(), entry.path()));
            }
        }
        Ok(out)
    };
    for id in ids {
        let id =
            id.map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", root.display())))?;
        let Some(id_name) = id.file_name().to_str().map(str::to_string) else {
            continue;
        };
        if !crate::manifest::loader::is_safe_segment(&id_name) || !id.path().is_dir() {
            continue;
        }
        for (hex, hex_path) in names(&id.path())? {
            if !is_hex64(&hex) || !hex_path.is_dir() {
                continue;
            }
            for (key, key_path) in names(&hex_path)? {
                if key.starts_with(TEMP_PREFIX) || !is_receipt_key(&key) || !key_path.is_dir() {
                    continue;
                }
                out.insert(format!("{id_name}/{hex}/{key}"));
            }
        }
    }
    Ok(out)
}

/// Refuse a home whose two stores are not physically distinct plain
/// directories (plan §12 R5-3).
pub fn check_distinct(paths: &Paths) -> Result<(), AwareError> {
    check_pair(&paths.agent_store_dir(), &paths.legacy_agent_store_dir())
}

fn check_pair(v2: &Path, legacy: &Path) -> Result<(), AwareError> {
    let mut identities = Vec::new();
    for dir in [v2, legacy] {
        match std::fs::symlink_metadata(dir) {
            Ok(meta) => {
                if crate::fs::is_reparse_point(&meta) || meta.file_type().is_symlink() {
                    return Err(aliased(format!(
                        "{} is a link or junction; the agent stores must be plain directories",
                        dir.display()
                    )));
                }
                identities.push(crate::fs::entry_identity(dir).map_err(|e| {
                    std::io::Error::new(e.kind(), format!("{}: {e}", dir.display()))
                })?);
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(std::io::Error::new(
                    error.kind(),
                    format!("{}: {error}", dir.display()),
                )
                .into());
            }
        }
    }
    if identities.len() == 2 && identities[0] == identities[1] {
        return Err(aliased(format!(
            "{} and {} are the same directory",
            v2.display(),
            legacy.display()
        )));
    }
    Ok(())
}

fn aliased(why: String) -> AwareError {
    AwareError::Validation(format!(
        "[E_AGENT_STORE_ALIASED] {why}. AWARE keeps the agent store of older CLIs (agent-store/) and its own (agent-store-v2/) apart so it never writes or removes an older CLI's packages; make both plain, separate directories"
    ))
}

/// Whether the legacy listing differs from what the record has seen.
pub fn needed(paths: &Paths) -> Result<bool, AwareError> {
    Ok(legacy_listing(paths)? != read_record(paths).seen)
}

/// Import every legacy package not yet in v2. The caller holds the store lock
/// EXCLUSIVE (see [`super::open`]). Writes nothing under `agent-store/`.
pub fn import(paths: &Paths) -> Result<ImportReport, AwareError> {
    check_distinct(paths)?;
    let listing = legacy_listing(paths)?;
    let mut record = read_record(paths);
    let mut report = ImportReport::default();
    let legacy_root = paths.legacy_agent_store_dir();
    for rel in &listing {
        if record.imported.contains(rel) && present_in_v2(paths, rel)? {
            continue;
        }
        let mut parts = rel.split('/');
        let (Some(id), Some(hex), Some(key)) = (parts.next(), parts.next(), parts.next()) else {
            continue;
        };
        let source = legacy_root.join(id).join(hex).join(key);
        match import_one(paths, &source, id, hex, key)? {
            Outcome::Copied => {
                record.skipped.remove(rel);
                record.imported.insert(rel.clone());
                report.copied.push(rel.clone());
            }
            Outcome::AlreadyPresent => {
                record.skipped.remove(rel);
                record.imported.insert(rel.clone());
                report.already_present.push(rel.clone());
            }
            Outcome::Skipped(reason) => {
                record.imported.remove(rel);
                record.skipped.insert(rel.clone(), reason.clone());
                report.skipped.insert(rel.clone(), reason);
            }
        }
    }
    record.format = RECORD_FORMAT;
    record.seen = listing;
    let dir = paths.agent_store_control_dir();
    std::fs::create_dir_all(&dir)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", dir.display())))?;
    let bytes = serde_json::to_vec_pretty(&record)
        .map_err(|e| AwareError::Internal(format!("serialize import record: {e}")))?;
    let path = record_path(paths);
    crate::app_lock::replace_atomically(&path, &bytes)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", path.display())))?;
    for (rel, reason) in &report.skipped {
        eprintln!(
            "\u{26a0} the older agent store's package {rel} was not carried over: it does not verify ({reason}); it is left untouched in agent-store/"
        );
    }
    Ok(report)
}

fn present_in_v2(paths: &Paths, rel: &str) -> Result<bool, AwareError> {
    let mut parts = rel.split('/');
    let (Some(id), Some(hex), Some(key)) = (parts.next(), parts.next(), parts.next()) else {
        return Ok(false);
    };
    let container = digest_container(paths, id, &format!("sha256:{hex}"))?;
    Ok(probe(&container.join(key))?.is_some())
}

enum Outcome {
    Copied,
    AlreadyPresent,
    Skipped(String),
}

fn import_one(
    paths: &Paths,
    source: &Path,
    id: &str,
    hex: &str,
    key: &str,
) -> Result<Outcome, AwareError> {
    let digest = format!("sha256:{hex}");
    if digest_hex(&digest).is_none() {
        return Ok(Outcome::Skipped("not a store digest".into()));
    }
    if let Err(reason) = super::verify_package(source, id, &digest, key) {
        return Ok(Outcome::Skipped(reason));
    }
    let container = digest_container(paths, id, &digest)?;
    let dest = container.join(key);
    if probe(&dest)?.is_some() {
        return match super::verify_package(&dest, id, &digest, key) {
            Ok(_) => Ok(Outcome::AlreadyPresent),
            Err(reason) => Err(AwareError::Validation(format!(
                "[E_AGENT_STORE_INVALID] the stored package {} does not verify: {reason}. AWARE never repairs or replaces a stored package; move it aside by hand",
                dest.display()
            ))),
        };
    }
    std::fs::create_dir_all(&container)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", container.display())))?;
    let temp = container.join(format!("{TEMP_PREFIX}{}", uuid::Uuid::new_v4().simple()));
    let staged = (|| -> Result<Result<(), String>, AwareError> {
        super::copy_tree(source, &temp)?;
        // The original record, byte for byte: its `snapshotted-at` is when
        // the bytes were first approved into a store.
        let record = std::fs::read(source.join(PACKAGE_FILE))?;
        super::write_synced(&temp.join(PACKAGE_FILE), &record)?;
        Ok(super::verify_package(&temp, id, &digest, key).map(|_| ()))
    })();
    match staged {
        Ok(Ok(())) => {}
        Ok(Err(reason)) => {
            let _ = std::fs::remove_dir_all(&temp);
            return Ok(Outcome::Skipped(format!(
                "it changed while it was being copied: {reason}"
            )));
        }
        Err(error) => {
            let _ = std::fs::remove_dir_all(&temp);
            return Err(error);
        }
    }
    if let Err(error) = crate::fs::rename_dir_no_replace(&temp, &dest) {
        let _ = std::fs::remove_dir_all(&temp);
        if probe(&dest)?.is_some() {
            return Ok(Outcome::AlreadyPresent);
        }
        return Err(
            std::io::Error::new(error.kind(), format!("{}: {error}", dest.display())).into(),
        );
    }
    super::sync_parents(&dest, &container)?;
    Ok(Outcome::Copied)
}

#[cfg(test)]
mod tests;
