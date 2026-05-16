//! Shared integration-test fixtures.
//!
//! Each test process gets one TempDir populated from <repo>/20-agents/
//! and <repo>/30-apps/_examples/, mirrored into `<tmp>/agents/<id>/` and
//! `<tmp>/apps/<id>/` (flat — the install layout, not the repo layout).
//!
//! Tests set AWARE_HOME=<that tmp dir> and run the binary read-only.

// `common` is compiled once per test binary; not every binary uses every item.
#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

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
    for manifest_path in find_manifests(&repo_root.join("20-agents"), "manifest.yaml") {
        let src_dir = manifest_path.parent().unwrap();
        let agent_id = src_dir.file_name().unwrap();
        let dst_dir = tmp.path().join("agents").join(agent_id);
        copy_dir_recursive(src_dir, &dst_dir).unwrap();
    }

    // Apps: each .flo in 30-apps/_examples/ becomes <tmp>/apps/<stem>/<stem>.flo
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
