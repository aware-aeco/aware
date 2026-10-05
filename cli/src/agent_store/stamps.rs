//! Last-needed stamps (#627-b, plan §2 and §8 R1-9): when a store package was
//! last needed by something that has since let go of it — a run that finished,
//! a lock that was replaced, an app that was uninstalled, a candidate that was
//! discarded, a working copy that was updated away.
//!
//! `agent-store-control/refs/<id>/<tree-hex>.last-needed` holds one RFC 3339
//! time, only ever moved forward. GC (#629) keeps an unreferenced package for
//! its recovery window counted from `max(snapshotted-at, last-needed)`. A stamp
//! is availability, never correctness: a run still verifies every byte it
//! dispatches.
//!
//! Updates of one digest are serialized by `<tree-hex>.flock` (exclusive):
//! read, keep the later time, write atomically, release.

use std::path::PathBuf;

use fs2::FileExt;

use crate::app_lock::LockFile;
use crate::error::AwareError;
use crate::paths::Paths;

const STAMP_EXT: &str = "last-needed";

fn refs_dir(paths: &Paths, id: &str) -> Result<PathBuf, AwareError> {
    if !crate::manifest::loader::is_safe_segment(id) {
        return Err(AwareError::Validation(format!(
            "[E_AGENT_STORE_INVALID] agent id {id:?} is not a plain name"
        )));
    }
    Ok(paths.agent_store_control_dir().join("refs").join(id))
}

fn hex_of(digest: &str) -> Result<&str, AwareError> {
    super::digest_hex(digest).ok_or_else(|| {
        AwareError::Validation(format!(
            "[E_AGENT_STORE_INVALID] {digest:?} is not a sha256 bundle digest"
        ))
    })
}

/// The last-needed time of `id`'s package `digest`, if stamped. Read by GC's
/// recovery window (#629); until then only by tests.
#[cfg_attr(not(test), allow(dead_code))]
pub fn last_needed(paths: &Paths, id: &str, digest: &str) -> Option<String> {
    let dir = refs_dir(paths, id).ok()?;
    let hex = hex_of(digest).ok()?;
    std::fs::read_to_string(dir.join(format!("{hex}.{STAMP_EXT}")))
        .ok()
        .map(|t| t.trim().to_string())
        .filter(|t| chrono::DateTime::parse_from_rfc3339(t).is_ok())
}

/// Stamp `id`'s package `digest` as needed until `at` (now when `None`). The
/// stored time only moves forward.
pub fn stamp(
    paths: &Paths,
    id: &str,
    digest: &str,
    at: Option<chrono::DateTime<chrono::Utc>>,
) -> Result<(), AwareError> {
    let dir = refs_dir(paths, id)?;
    let hex = hex_of(digest)?.to_string();
    std::fs::create_dir_all(&dir)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", dir.display())))?;
    let lock_path = dir.join(format!("{hex}.flock"));
    let lock = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(&lock_path)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", lock_path.display())))?;
    lock.lock_exclusive()
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", lock_path.display())))?;
    let at = at.unwrap_or_else(chrono::Utc::now);
    let path = dir.join(format!("{hex}.{STAMP_EXT}"));
    let current = std::fs::read_to_string(&path).ok().and_then(|t| {
        chrono::DateTime::parse_from_rfc3339(t.trim())
            .ok()
            .map(|d| d.with_timezone(&chrono::Utc))
    });
    if current.is_some_and(|c| c >= at) {
        return Ok(());
    }
    let text = at.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    match crate::app_lock::replace_atomically(&path, text.as_bytes())
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", path.display())))?
    {
        crate::fs::Replaced::Durable | crate::fs::Replaced::NotDurable(_) => Ok(()),
    }
}

/// Stamp every digest a lock pins (the run's resolution rule: `agent-digests`,
/// else `agent-bundle-pins`; a version-only pin references nothing stored).
pub fn stamp_lock(paths: &Paths, lock: &LockFile) -> Result<(), AwareError> {
    for id in lock.agent_pins.keys() {
        if let Some(digest) = crate::app_lock::pinned_digest(lock, id) {
            stamp(paths, id, digest, None)?;
        }
    }
    Ok(())
}

/// [`stamp_lock`] of the lock file at `path`, if there is one and it parses.
/// `Err` only when stamping fails; an absent or unreadable lock stamps nothing.
pub fn stamp_lock_file(paths: &Paths, path: &std::path::Path) -> Result<(), AwareError> {
    let Ok(bytes) = std::fs::read(path) else {
        return Ok(());
    };
    match serde_yaml::from_slice::<LockFile>(&bytes) {
        Ok(lock) => stamp_lock(paths, &lock),
        Err(_) => Ok(()),
    }
}

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

    #[test]
    fn a_stamp_only_moves_forward() {
        let (_tmp, paths) = home();
        let digest = format!("sha256:{}", "a".repeat(64));
        assert_eq!(last_needed(&paths, "tekla", &digest), None);
        let later = chrono::DateTime::parse_from_rfc3339("2026-10-05T12:00:00Z")
            .unwrap()
            .with_timezone(&chrono::Utc);
        let earlier = later - chrono::Duration::hours(1);
        stamp(&paths, "tekla", &digest, Some(later)).unwrap();
        stamp(&paths, "tekla", &digest, Some(earlier)).unwrap();
        assert_eq!(
            last_needed(&paths, "tekla", &digest).as_deref(),
            Some("2026-10-05T12:00:00.000Z")
        );
        let latest = later + chrono::Duration::hours(1);
        stamp(&paths, "tekla", &digest, Some(latest)).unwrap();
        assert_eq!(
            last_needed(&paths, "tekla", &digest).as_deref(),
            Some("2026-10-05T13:00:00.000Z")
        );
    }

    /// Whether Rust source text calls one of the stamp writers.
    fn stamps_something(text: &str) -> bool {
        [
            "stamps::stamp(",
            "stamps::stamp_lock(",
            "stamps::stamp_lock_file(",
        ]
        .iter()
        .any(|call| text.contains(call))
    }

    /// Plan §2 / §8 R1-9 (risk K5): every place that lets go of a store
    /// package stamps it. If a site loses its call, GC would count that
    /// package's recovery window from its snapshot time instead.
    #[test]
    fn every_place_that_lets_go_of_a_package_stamps_it() {
        // Negative controls: the scan rejects text without a call...
        assert!(!stamps_something(
            "let lock = read(path)?; write_lockfile(&lock, s)?;"
        ));
        assert!(!stamps_something("// stamps are written elsewhere"));
        // ...and accepts each call form.
        assert!(stamps_something(
            "crate::agent_store::stamps::stamp_lock(paths, &base)?;"
        ));
        assert!(stamps_something("super::stamps::stamp(paths, id, d, None)"));
        assert!(stamps_something("stamps::stamp_lock_file(paths, &p)?"));

        let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        for site in [
            "agent_store/lease.rs",    // a run acquires / releases
            "install/swap.rs",         // a working copy is updated away
            "app_lock.rs",             // a compile replaces the lock
            "install/uninstall.rs",    // an app is uninstalled
            "commands/app_migrate.rs", // a candidate is discarded
            "migration/promote.rs",    // a promotion replaces the lock
        ] {
            let text = std::fs::read_to_string(src.join(site)).unwrap();
            assert!(
                stamps_something(&text),
                "{site} lets go of store packages but no longer stamps them"
            );
        }
    }

    #[test]
    fn a_stamp_refuses_a_path_it_could_not_name() {
        let (_tmp, paths) = home();
        let digest = format!("sha256:{}", "a".repeat(64));
        assert!(stamp(&paths, "../x", &digest, None).is_err());
        assert!(stamp(&paths, "tekla", "sha256:nope", None).is_err());
    }
}
