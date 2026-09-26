//! Shared integration-test fixtures.
//!
//! Each test process needs an `AWARE_HOME` populated from `<repo>/20-agents/`
//! and `<repo>/30-apps/_examples/`, mirrored into `<home>/agents/<id>/` and
//! `<home>/apps/<id>/` (flat — the install layout, not the repo layout).
//!
//! # Why this is not a `TempDir` in a `static` (#578)
//!
//! It used to be `static FIXTURE: OnceLock<TempDir>`. `TempDir` deletes its
//! directory in `Drop`, but **Rust never runs destructors for values in a
//! `static`** — so that `TempDir` was never dropped and its copy of the
//! catalogue was never deleted. Every test binary that called [`aware_home`]
//! leaked one, on every platform, on every run: measured at 8 leftovers of
//! ~400 MB each, 3.2 GB per `cargo test`, and reported on one dev machine as
//! 345 leftovers totalling 51 GB.
//!
//! The fixture now lives under `CARGO_TARGET_TMPDIR` (`target/tmp/`) at a path
//! keyed by the state of the source catalogue. So it is:
//!
//! * deleted by `cargo clean`, like everything else under `target/`;
//! * built **once** per catalogue state and shared by every test binary,
//!   instead of copied once per binary;
//! * reused by later runs until a source file changes, which changes the key.
//!
//! # The fixture is READ-ONLY
//!
//! It is shared between test binaries running **in parallel** and reused by
//! **later runs**, so a test that writes into [`aware_home`] would corrupt
//! every test that follows it, including on someone else's next run. A test
//! that needs a home it can mutate builds its own `tempfile::tempdir()` — as
//! the `app_run` and `streamed_report` fixtures do — and may hand that to
//! [`approve_installed_apps`].

// `common` is compiled once per test binary; not every binary uses every item.
#![allow(dead_code)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::{Duration, UNIX_EPOCH};

use sha2::{Digest, Sha256};

/// Bumped when the *layout* [`populate`] writes changes — a new subdirectory, a
/// different flattening rule. The cache key covers the source catalogue, not
/// this file, so without this a run of the new code would be served a directory
/// that was only correct for the old code.
const FIXTURE_FORMAT: &str = "v1";

/// How stale a superseded fixture must be before [`evict_superseded`] removes
/// it. Long enough that a concurrent `cargo test` on an older catalogue state
/// cannot have its fixture deleted mid-run (the suite takes about a minute),
/// short enough that edits do not pile up copies for days.
const EVICTION_GRACE: Duration = Duration::from_secs(24 * 60 * 60);

/// The source trees the fixture mirrors, relative to the repo root. The cache
/// key covers all of both, which is a superset of what [`populate`] copies —
/// erring towards rebuilding a fixture that did not need it, never towards
/// serving a stale one.
const SOURCE_TREES: [&str; 2] = ["20-agents", "30-apps/_examples"];

/// Holds a `PathBuf`, deliberately, and not the `TempDir` that leaked in #578:
/// a value in a `static` is never dropped, so nothing here may own a resource
/// whose cleanup depends on `Drop`. A leaked heap allocation costs nothing and
/// goes away with the process; a leaked directory does not.
static FIXTURE: OnceLock<PathBuf> = OnceLock::new();

pub fn aware_home() -> &'static Path {
    FIXTURE.get_or_init(install_fixture).as_path()
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root")
        .to_path_buf()
}

/// Build the fixture if this catalogue state has no copy yet, and return the
/// path either way.
fn install_fixture() -> PathBuf {
    let repo_root = repo_root();
    // Set by Cargo for integration tests, and emptied by `cargo clean`.
    let cache_root = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    let home = cache_root.join(format!(
        "aware-fixture-{FIXTURE_FORMAT}-{}",
        catalogue_key(&repo_root)
    ));
    if home.is_dir() {
        return home;
    }
    std::fs::create_dir_all(&cache_root).expect("create the fixture cache root");

    // Populate a staging directory and publish it with one rename, rather than
    // copying into the final path. `cargo test` runs test binaries in parallel,
    // so several processes reach this at once on a cold cache; rename is atomic,
    // so a reader either sees no fixture or sees a complete one — never the
    // half-copied catalogue that copying in place would expose. Staging sits in
    // `cache_root` so the rename stays within one filesystem.
    let staging = tempfile::Builder::new()
        .prefix("aware-fixture-staging-")
        .tempdir_in(&cache_root)
        .expect("create the fixture staging directory");
    populate(&repo_root, staging.path());

    match std::fs::rename(staging.path(), &home) {
        Ok(()) => {
            // The directory has moved; disarm the `TempDir` so its `Drop` does
            // not delete the published fixture out from under the suite.
            let _ = staging.keep();
        }
        Err(err) => {
            // Losing the race is the expected way this fails: another test
            // binary published first, and renaming onto a populated directory
            // is refused. Our copy is then redundant and `Drop` removes it.
            // Anything else — a full disk, a permission fault — leaves no
            // fixture at all, and must fail loudly rather than hand the suite
            // a path that does not exist.
            assert!(
                home.is_dir(),
                "could not publish the test fixture to {}: {err}",
                home.display()
            );
        }
    }
    evict_superseded(&cache_root, &home);
    home
}

/// Delete fixtures and staging directories left by earlier catalogue states.
///
/// Without this the cache is append-only: every edit under `20-agents/`
/// publishes another ~400 MB copy and keeps the previous one, which on a
/// machine that edits the catalogue often reproduces the disk exhaustion #578
/// was filed about — just inside `target/` instead of `TEMP`. `cargo clean`
/// clears it either way; nobody should have to remember to.
///
/// Two rules keep this from deleting a directory somebody is reading. It never
/// touches `keep`, the fixture this process is about to hand out — so the
/// fixture for the CURRENT catalogue state is never a candidate, however old
/// it is. And it skips anything modified within [`EVICTION_GRACE`]: a second
/// `cargo test` may be running against an older catalogue state while this one
/// builds, and its fixture was published minutes ago, not a day.
///
/// One case those rules do not cover, stated rather than papered over: a
/// concurrent run reusing a fixture that is BOTH superseded and older than the
/// grace — which needs two test runs on different catalogue states sharing one
/// `target/`, the older of them reusing a fixture from a previous day. It
/// fails loudly (missing fixture files), not silently, and a rerun fixes it.
///
/// Best-effort by design. A directory that cannot be removed — held open on
/// Windows, a permission fault — costs disk and nothing else, so failing the
/// suite over it would turn a housekeeping problem into a red build. The next
/// publish tries again.
fn evict_superseded(cache_root: &Path, keep: &Path) {
    let Ok(entries) = std::fs::read_dir(cache_root) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path == keep {
            continue;
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if !name.starts_with("aware-fixture-") {
            continue;
        }
        let superseded = entry
            .metadata()
            .ok()
            .filter(|meta| meta.is_dir())
            .and_then(|meta| meta.modified().ok())
            .and_then(|modified| modified.elapsed().ok())
            .is_some_and(|age| age > EVICTION_GRACE);
        if superseded {
            let _ = std::fs::remove_dir_all(&path);
        }
    }
}

/// A digest of the catalogue's current state, used as the fixture's cache key.
///
/// Content is deliberately not hashed: reading ~400 MB on every test binary
/// would cost more than the copy this cache exists to avoid. Each file
/// contributes its path, length and mtime — which is what Cargo itself treats
/// as evidence that a file changed — and each directory contributes its path,
/// so adding or removing even an empty one produces a different key.
fn catalogue_key(repo_root: &Path) -> String {
    let mut entries: Vec<(String, bool, u64, i128)> = Vec::new();
    for tree in SOURCE_TREES {
        let root = repo_root.join(tree);
        for path in walk(&root) {
            let relative = path
                .strip_prefix(repo_root)
                .unwrap_or(&path)
                .to_string_lossy()
                .into_owned();
            let meta = std::fs::symlink_metadata(&path)
                .unwrap_or_else(|e| panic!("stat {} for the fixture key: {e}", path.display()));
            if meta.is_dir() {
                entries.push((relative, true, 0, 0));
                continue;
            }
            let modified = meta
                .modified()
                .unwrap_or_else(|e| panic!("mtime of {} for the fixture key: {e}", path.display()));
            // Signed, so a pre-epoch mtime stays distinct from an epoch one
            // instead of collapsing to the same key as an edited file.
            let nanos = match modified.duration_since(UNIX_EPOCH) {
                Ok(since) => i128::try_from(since.as_nanos()).unwrap_or(i128::MAX),
                Err(before) => -i128::try_from(before.duration().as_nanos()).unwrap_or(i128::MAX),
            };
            entries.push((relative, false, meta.len(), nanos));
        }
    }
    // The walk order depends on the filesystem; the key must not.
    entries.sort();

    let mut hasher = Sha256::new();
    hasher.update(FIXTURE_FORMAT.as_bytes());
    for (path, is_dir, len, mtime) in entries {
        hasher.update(path.as_bytes());
        hasher.update([u8::from(is_dir)]);
        hasher.update(len.to_le_bytes());
        hasher.update(mtime.to_le_bytes());
        hasher.update([0]);
    }
    let digest = format!("{:x}", hasher.finalize());
    digest[..16].to_string()
}

/// Every path under `root`, directories included, `root` itself excluded.
fn walk(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(read) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in read.flatten() {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path.clone());
            }
            out.push(path);
        }
    }
    out
}

fn populate(repo_root: &Path, dest: &Path) {
    std::fs::create_dir_all(dest.join("agents")).unwrap();
    std::fs::create_dir_all(dest.join("apps")).unwrap();

    // Walk 20-agents/ looking for manifest.yaml; for each, copy the agent
    // folder (manifest + skills + commands subdirs) into <dest>/agents/<id>/.
    //
    // The substrate currently has a handful of agent directories with colliding final
    // segments (e.g. `aeco/architecture/navisworks-2026` and `aeco/construction/navisworks-2026`
    // both declare `agent: navisworks-2026`). Pre-v0.30 the directory copy MERGED them — the
    // second copy added files but didn't remove orphans from the first — which surfaced as
    // spurious `W_SKILL_ORPHAN` warnings in `aware doctor`. To keep the fixture deterministic
    // and the integrity check meaningful, we now skip the duplicate: first source under the
    // walk wins, subsequent ones are ignored. This mirrors the install precedence the runtime
    // would apply on `aware agent install` collisions.
    let mut installed = std::collections::HashSet::<std::ffi::OsString>::new();
    for manifest_path in find_manifests(&repo_root.join("20-agents"), "manifest.yaml") {
        let src_dir = manifest_path.parent().unwrap();
        let agent_id = src_dir.file_name().unwrap().to_owned();
        if !installed.insert(agent_id.clone()) {
            continue;
        }
        let dst_dir = dest.join("agents").join(&agent_id);
        copy_dir_recursive(src_dir, &dst_dir).unwrap();
    }

    // Apps: each app file (.app/.flo) in 30-apps/_examples/ becomes <dest>/apps/<stem>/<stem>.<ext>
    let apps_src = repo_root.join("30-apps/_examples");
    for entry in std::fs::read_dir(&apps_src).unwrap().flatten() {
        let p = entry.path();
        if p.extension().is_some_and(|e| e == "flo" || e == "app") {
            let stem = p.file_stem().unwrap().to_string_lossy().to_string();
            let dst_dir = dest.join("apps").join(&stem);
            std::fs::create_dir_all(&dst_dir).unwrap();
            std::fs::copy(&p, dst_dir.join(p.file_name().unwrap())).unwrap();
        }
    }
}

fn find_manifests(root: &Path, name: &str) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(d) = stack.pop() {
        let read = match std::fs::read_dir(&d) {
            Ok(r) => r,
            Err(_) => continue,
        };
        for entry in read.flatten() {
            let p = entry.path();
            if p.is_dir() {
                stack.push(p);
            } else if p.file_name().is_some_and(|n| n == name) {
                out.push(p);
            }
        }
    }
    out
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)?.flatten() {
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir_recursive(&from, &to)?;
        } else {
            std::fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

/// Write the smallest structurally valid compiled approval needed by runtime
/// integration fixtures. Tests that exercise compilation itself use the real
/// `aware app compile` command instead.
pub fn approve_app_source(source: &Path) {
    let bytes = std::fs::read(source).expect("read app source for approval");
    let parsed: serde_yaml::Value =
        serde_yaml::from_slice(&bytes).expect("parse app source for approval");
    let app = parsed["app"].as_str().expect("app id in fixture");
    let version = parsed["version"].as_str().expect("app version in fixture");
    let hash = format!("sha256:{:x}", Sha256::digest(&bytes));
    let home = source
        .parent()
        .and_then(Path::parent)
        .and_then(Path::parent);
    let mut pins = BTreeMap::new();
    if let Some(agents_dir) = home.map(|path| path.join("agents"))
        && let Ok(entries) = std::fs::read_dir(agents_dir)
    {
        for entry in entries.flatten() {
            let manifest = entry.path().join("manifest.yaml");
            let Some(value) = std::fs::read(&manifest)
                .ok()
                .and_then(|body| serde_yaml::from_slice::<serde_yaml::Value>(&body).ok())
            else {
                continue;
            };
            if let (Some(id), Some(agent_version)) =
                (value["agent"].as_str(), value["version"].as_str())
            {
                pins.insert(id.to_string(), agent_version.to_string());
            }
        }
    }
    let pins_yaml = if pins.is_empty() {
        "{}".to_string()
    } else {
        let yaml = serde_yaml::to_string(&pins).expect("serialize fixture agent pins");
        format!(
            "\n{}",
            yaml.lines()
                .map(|line| format!("  {line}"))
                .collect::<Vec<_>>()
                .join("\n")
        )
    };
    let lock = format!(
        "source-hash: {hash}\ncompiled-at: test\ncompiler-version: test\napp: {app}\nversion: {version}\nagent-pins: {pins_yaml}\nnodes: []\n"
    );
    std::fs::write(source.with_file_name(format!("{app}.lock")), lock)
        .expect("write app approval fixture");
}

/// Approve every installed app source under `<AWARE_HOME>/apps/`.
pub fn approve_installed_apps(home: &Path) {
    let apps = home.join("apps");
    let Ok(entries) = std::fs::read_dir(apps) else {
        return;
    };
    for entry in entries.flatten() {
        let dir = entry.path();
        let Ok(files) = std::fs::read_dir(&dir) else {
            continue;
        };
        for file in files.flatten() {
            let source = file.path();
            if source
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| matches!(ext, "flo" | "app" | "flow" | "aware"))
            {
                // Some negative-path tests deliberately make a nested source
                // unreadable or malformed. Leave those untouched so the
                // production command reports the intended fault; approve only
                // ordinary fixtures that can actually represent an app.
                let approvable = std::fs::read(&source)
                    .ok()
                    .and_then(|bytes| serde_yaml::from_slice::<serde_yaml::Value>(&bytes).ok())
                    .is_some_and(|value| {
                        value["app"].as_str().is_some() && value["version"].as_str().is_some()
                    });
                if approvable {
                    approve_app_source(&source);
                }
            }
        }
    }
}
