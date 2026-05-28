//! Install an agent by resolving `<id>[@version]` in the registry index,
//! fetching the tarball, extracting the agent's subdirectory, then handing
//! off to `local::install_agent_from_path`.

use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;
use tar::Archive;

use crate::error::AwareError;
use crate::install::local::{copy_dir_recursive, install_agent_from_path};
use crate::manifest::loader::load_agent;
use crate::paths::Paths;
use crate::registry::Index;
use crate::validate::{Severity, has_errors, validate_agent_on_disk};

pub fn install_agent_from_registry(
    id: &str,
    version_pin: Option<&str>,
    paths: &Paths,
    index: &Index,
) -> Result<String, AwareError> {
    let (_scratch, subdir) = stage_agent_from_registry(id, version_pin, paths, index)?;
    install_agent_from_path(&subdir, paths)
}

/// Resolve `<key>[@version]`, fetch the tarball (cache → `file://` → network),
/// extract it and return the agent's source subdir (containing `manifest.yaml`).
///
/// Pure fetch + extract into a scratch dir — it never touches the install
/// directory, so a failure here (network timeout, missing tarball, bad subdir)
/// is side-effect-free. The returned `TempDir` owns the scratch space; callers
/// must keep it alive until they have copied the agent out of `subdir`.
fn stage_agent_from_registry(
    key: &str,
    version_pin: Option<&str>,
    paths: &Paths,
    index: &Index,
) -> Result<(tempfile::TempDir, PathBuf), AwareError> {
    let (resolved_version, entry) = index.resolve(key, version_pin)?;

    let scratch = tempfile::tempdir()?;
    let tarball_path = scratch.path().join("agent.tar.gz");

    let cache_dir = paths.cache_dir().join("agents");
    std::fs::create_dir_all(&cache_dir)?;
    let cache_file = cache_dir.join(format!("{key}-{resolved_version}.tar.gz"));

    if cache_file.is_file() {
        std::fs::copy(&cache_file, &tarball_path)?;
    } else if let Some(path) = entry.tarball.strip_prefix("file://") {
        std::fs::copy(path, &tarball_path)?;
        let _ = std::fs::copy(&tarball_path, &cache_file);
    } else {
        let resp = ureq::get(&entry.tarball)
            .timeout(std::time::Duration::from_secs(60))
            .call()
            .map_err(|e| AwareError::Network(format!("GET {}: {e}", entry.tarball)))?;
        let mut reader = resp.into_reader();
        let mut file = std::fs::File::create(&tarball_path)?;
        std::io::copy(&mut reader, &mut file)?;
        let _ = std::fs::copy(&tarball_path, &cache_file);
    }

    let extract_root = scratch.path().join("extract");
    extract_tarball(&tarball_path, &extract_root)?;

    let subdir = extract_root.join(&entry.subdir);
    if !subdir.is_dir() {
        return Err(AwareError::Validation(format!(
            "registry entry {key}@{resolved_version}: subdir {} not in tarball",
            entry.subdir,
        )));
    }
    Ok((scratch, subdir))
}

/// Atomically update an installed agent to the latest registry version.
///
/// `id` is the agent as it is installed (its `manifest.agent` / folder name) or
/// any spec that resolves to the same registry entry. Fixes #174:
///
/// 1. **Base-name resolution** — the registry key is resolved from `id` first
///    (installed `allplan-2024.0` → key `allplan-2024`), so `update` no longer
///    looks the installed id up verbatim and fails with "not in registry".
/// 2. **Atomic / non-destructive** — the new copy is fetched, extracted and
///    validated in a scratch dir; the on-disk install is only replaced once all
///    of that fallible work has succeeded. A failed re-pull (network timeout,
///    agent dropped from the registry, invalid payload) leaves the existing
///    install untouched instead of deleting it.
/// 3. **Replace in place** — the previous folder and any stale folder already
///    sitting at the new name are removed before the new copy is moved in, so an
///    agent is never left installed under two names.
pub fn update_agent_from_registry(
    id: &str,
    paths: &Paths,
    index: &Index,
) -> Result<String, AwareError> {
    // 1. Resolve the registry key BEFORE touching disk. A spec that maps to no
    //    registry entry fails here, leaving the install intact.
    let key = index
        .resolve_key(id)
        .ok_or_else(|| AwareError::NotFound(format!("agent {id} not in registry")))?
        .to_string();

    // 2. Fetch + extract into scratch (the network-fallible work).
    let (_scratch, subdir) = stage_agent_from_registry(&key, None, paths, index)?;

    // 3. Validate the freshly fetched copy before it can replace a good install.
    let manifest_path = subdir.join("manifest.yaml");
    if !manifest_path.is_file() {
        return Err(AwareError::Validation(format!(
            "registry entry {key}: no manifest.yaml in payload"
        )));
    }
    let agent = load_agent(&manifest_path)?;
    let issues = validate_agent_on_disk(&agent, &subdir);
    if has_errors(&issues) {
        let summary = issues
            .iter()
            .filter(|i| i.severity == Severity::Error)
            .map(|i| format!("[{}] {}", i.code, i.message))
            .collect::<Vec<_>>()
            .join("; ");
        return Err(AwareError::Validation(summary));
    }
    let new_name = agent.agent.clone();

    // ── past this point only local fs work remains ──────────────────────────
    let agents_dir = paths.agents_dir();
    std::fs::create_dir_all(&agents_dir)?;

    // Stage the new copy on the same filesystem as the install but under the
    // cache dir (NOT agents_dir) so a crash mid-update can never surface a
    // half-written agent in `agent list`. The final rename is then atomic.
    let staging = paths.cache_dir().join("update-staging").join(&new_name);
    if let Some(parent) = staging.parent() {
        std::fs::create_dir_all(parent)?;
    }
    if staging.exists() {
        std::fs::remove_dir_all(&staging)?;
    }
    if let Err(e) = copy_dir_recursive(&subdir, &staging) {
        let _ = std::fs::remove_dir_all(&staging);
        return Err(e.into());
    }

    // Remove the prior install (the folder we updated from) and any stale folder
    // already at the new name — collapses the duplicate-folder bug.
    let prev_dir = agents_dir.join(id);
    if prev_dir.exists() {
        std::fs::remove_dir_all(&prev_dir)?;
    }
    let final_dir = agents_dir.join(&new_name);
    if final_dir.exists() {
        std::fs::remove_dir_all(&final_dir)?;
    }

    // Atomic move into place (same filesystem).
    std::fs::rename(&staging, &final_dir)?;
    Ok(new_name)
}

fn extract_tarball(tarball: &Path, dest: &Path) -> Result<(), AwareError> {
    std::fs::create_dir_all(dest)?;
    let file = std::fs::File::open(tarball)?;
    let gz = GzDecoder::new(file);
    let mut archive = Archive::new(gz);
    archive
        .unpack(dest)
        .map_err(|e| AwareError::Validation(format!("tarball extract: {e}")))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::{IndexEntry, VersionEntry};
    use std::collections::BTreeMap;
    use std::io::Write;

    fn make_test_tarball(path: &Path) {
        let tar_gz = std::fs::File::create(path).unwrap();
        let enc = flate2::write::GzEncoder::new(tar_gz, flate2::Compression::default());
        let mut tar = tar::Builder::new(enc);

        let repo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let manifest_text =
            std::fs::read_to_string(repo.join("20-agents/aeco/engineering/tekla/manifest.yaml"))
                .unwrap();

        let mut h = tar::Header::new_gnu();
        h.set_size(manifest_text.len() as u64);
        h.set_mode(0o644);
        h.set_cksum();
        tar.append_data(
            &mut h,
            "aware-main/20-agents/tekla/manifest.yaml",
            manifest_text.as_bytes(),
        )
        .unwrap();

        let agent: crate::manifest::Agent = serde_yaml::from_str(&manifest_text).unwrap();
        let skills_src = repo.join("20-agents/aeco/engineering/tekla/skills");
        for skill in &agent.skills {
            let body = std::fs::read_to_string(skills_src.join(skill)).unwrap();
            let mut h = tar::Header::new_gnu();
            h.set_size(body.len() as u64);
            h.set_mode(0o644);
            h.set_cksum();
            tar.append_data(
                &mut h,
                format!("aware-main/20-agents/tekla/skills/{skill}"),
                body.as_bytes(),
            )
            .unwrap();
        }

        let gz_writer = tar.into_inner().unwrap();
        let mut file = gz_writer.finish().unwrap();
        file.flush().unwrap();
    }

    #[test]
    fn installs_from_local_tarball() {
        let tmp = tempfile::tempdir().unwrap();
        let tarball = tmp.path().join("tekla.tar.gz");
        make_test_tarball(&tarball);

        let mut versions = BTreeMap::new();
        versions.insert(
            "2025.0.1".into(),
            VersionEntry {
                tarball: format!("file://{}", tarball.display()),
                subdir: "aware-main/20-agents/tekla".into(),
            },
        );
        let mut agents = BTreeMap::new();
        agents.insert("tekla".into(), IndexEntry { versions });
        let index = Index {
            version: "1.0".into(),
            updated_at: "x".into(),
            agents,
            bundles: BTreeMap::new(),
        };

        let aware = tmp.path().join("aware");
        let paths = Paths {
            aware_home: aware.clone(),
        };
        let installed = install_agent_from_registry("tekla", None, &paths, &index).unwrap();
        assert_eq!(installed, "tekla");
        assert!(aware.join("agents/tekla/manifest.yaml").is_file());
    }
}
