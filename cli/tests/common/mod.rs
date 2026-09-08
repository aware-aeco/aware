//! Shared integration-test fixtures.
//!
//! Each test process gets one TempDir populated from <repo>/20-agents/
//! and <repo>/30-apps/_examples/, mirrored into `<tmp>/agents/<id>/` and
//! `<tmp>/apps/<id>/` (flat — the install layout, not the repo layout).
//!
//! Tests set AWARE_HOME=<that tmp dir> and run the binary read-only.

// `common` is compiled once per test binary; not every binary uses every item.
#![allow(dead_code)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use sha2::{Digest, Sha256};
use tempfile::TempDir;

static FIXTURE: OnceLock<TempDir> = OnceLock::new();

pub fn aware_home() -> &'static Path {
    FIXTURE.get_or_init(populate).path()
}

fn populate() -> TempDir {
    let tmp = tempfile::tempdir().expect("create tempdir");
    let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root")
        .to_path_buf();

    std::fs::create_dir_all(tmp.path().join("agents")).unwrap();
    std::fs::create_dir_all(tmp.path().join("apps")).unwrap();

    // Walk 20-agents/ looking for manifest.yaml; for each, copy the agent
    // folder (manifest + skills + commands subdirs) into <tmp>/agents/<id>/.
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
        let dst_dir = tmp.path().join("agents").join(&agent_id);
        copy_dir_recursive(src_dir, &dst_dir).unwrap();
    }

    // Apps: each app file (.app/.flo) in 30-apps/_examples/ becomes <tmp>/apps/<stem>/<stem>.<ext>
    let apps_src = repo_root.join("30-apps/_examples");
    for entry in std::fs::read_dir(&apps_src).unwrap().flatten() {
        let p = entry.path();
        if p.extension().is_some_and(|e| e == "flo" || e == "app") {
            let stem = p.file_stem().unwrap().to_string_lossy().to_string();
            let dst_dir = tmp.path().join("apps").join(&stem);
            std::fs::create_dir_all(&dst_dir).unwrap();
            std::fs::copy(&p, dst_dir.join(p.file_name().unwrap())).unwrap();
        }
    }

    tmp
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
