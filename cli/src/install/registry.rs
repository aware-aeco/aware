//! Install an agent by resolving `<id>[@version]` in the registry index,
//! fetching the tarball, extracting the agent's subdirectory, then handing
//! off to `local::install_agent_from_path`.

use std::path::{Path, PathBuf};
use std::time::SystemTime;

use flate2::read::GzDecoder;
use sha2::{Digest, Sha256};
use tar::Archive;

use crate::error::AwareError;
use crate::install::local::{copy_dir_recursive, install_agent_from_path};
use crate::manifest::loader::load_agent;
use crate::paths::Paths;
use crate::registry::Index;
use crate::registry::fetch::CACHE_TTL;
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
    // Cache the downloaded tarball by its URL + the index's snapshot fingerprint,
    // NOT the agent id: every agent in a substrate registry shares ONE tarball (the
    // repo archive), so a per-agent key made each new agent re-download the same
    // (often hundreds-of-MB) archive (#243). With this key all agents in one index
    // snapshot share a single cached download — only the first install pays for it —
    // and any change to the index (an agent added / removed / repinned, or a bumped
    // `updated-at`) rotates the key so the cache can't go stale. The fingerprint
    // replaces keying on `updated-at` ALONE here (#254): that field is hand-maintained
    // and went stale, so a newly-added agent's subdir stayed absent from the cached
    // archive forever — hashing the content too busts the cache regardless.
    //
    // The fingerprint busts the cache the instant `registry-index.json` changes, but a
    // manifest edit made INSIDE the rolling `main.tar.gz` (a `status:` flip, a keyword
    // fix) that leaves the index byte-identical never rotates it — so the cache also
    // carries the same 1h TTL the index/catalog use (`fetch::CACHE_TTL`), bounding that
    // residual staleness to one self-healing re-download per snapshot (#270).
    let cache_file = cache_dir.join(tarball_cache_name(
        &entry.tarball,
        &index.snapshot_fingerprint(),
    ));

    if cache_file.is_file() && cache_is_fresh(&cache_file) {
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

    // Extract ONLY the agent's subtree, not the whole archive: the substrate tarball is
    // the entire monorepo, so unpacking all of it (tens of thousands of files) to reach
    // one agent dominated install time — #243. We still stream through the gzip (it isn't
    // seekable) but write only the matching entries.
    let extract_root = scratch.path().join("extract");
    extract_subdir(&tarball_path, &extract_root, &entry.subdir)?;

    let subdir = extract_root.join(&entry.subdir);
    if !subdir.is_dir() {
        return Err(AwareError::Validation(format!(
            "registry entry {key}@{resolved_version}: subdir {} not in tarball",
            entry.subdir,
        )));
    }
    Ok((scratch, subdir))
}

/// Cache filename for a registry tarball, keyed by its URL + the index's snapshot
/// fingerprint (`Index::snapshot_fingerprint`). Agents sharing one tarball in one index
/// snapshot share one cache file (#243); any change to the index rotates the key (#254).
/// Hashed so any URL maps to a fixed-length, filesystem-safe name (SHA-256 is already a
/// dependency and is stable across runs/platforms).
fn tarball_cache_name(tarball: &str, snapshot: &str) -> String {
    let mut h = Sha256::new();
    h.update(tarball.as_bytes());
    h.update([0]); // domain-separate URL from snapshot so a+b can't collide with a'+b'
    h.update(snapshot.as_bytes());
    format!("tarball-{:x}.tar.gz", h.finalize())
}

/// The cached tarball is fresh if it was (re)written within `CACHE_TTL`. The snapshot
/// fingerprint busts the cache the instant `registry-index.json` changes, but a manifest
/// edit made INSIDE the rolling `main.tar.gz` that leaves the index byte-identical never
/// rotates the fingerprint — so without a TTL that change would be served stale forever
/// (#270). This mirrors the index/catalog 1h TTL (`fetch::CACHE_TTL`): a warm cache
/// self-refreshes within an hour, bounded to one re-download per snapshot (not per agent).
/// Any failure to read the mtime falls to `false` (re-download) — prefer fresh over stale.
fn cache_is_fresh(cache_file: &Path) -> bool {
    std::fs::metadata(cache_file)
        .and_then(|m| m.modified())
        .ok()
        .and_then(|modified| SystemTime::now().duration_since(modified).ok())
        .is_some_and(|age| age < CACHE_TTL)
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

    // 4. Guard the suffix fallback. When `id` is not itself a registry key, the
    //    key was inferred from a dotted suffix (`allplan-2024.0` → `allplan-2024`).
    //    That inference only holds if the registry entry actually produces THIS
    //    install id; otherwise `id` is an unrelated agent that merely looks like
    //    `<key>.<suffix>` (e.g. a local `tekla.dev`), and replacing it would
    //    hijack/delete it. Fail non-destructively instead — nothing on disk has
    //    been touched yet (#174).
    if !index.agents.contains_key(id) && new_name != id {
        return Err(AwareError::NotFound(format!("agent {id} not in registry")));
    }

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

/// Extract only the entries under `subdir` (the agent's own subtree) from the tarball
/// into `dest`, preserving their archive-relative paths. The substrate tarball is the
/// whole monorepo, so unpacking everything to reach one agent was the dominant install
/// cost (#243); this writes just the agent's files. The gzip stream is read in full
/// (gzip isn't seekable), but that decompress is cheap next to the avoided disk writes.
fn extract_subdir(tarball: &Path, dest: &Path, subdir: &str) -> Result<(), AwareError> {
    std::fs::create_dir_all(dest)?;
    let file = std::fs::File::open(tarball)?;
    let gz = GzDecoder::new(file);
    let mut archive = Archive::new(gz);
    let want = subdir.trim_end_matches('/');
    let prefix = format!("{want}/");
    // A `fn` (not a closure) so it can be reused across the `map_err` calls below.
    fn map(e: std::io::Error) -> AwareError {
        AwareError::Validation(format!("tarball extract: {e}"))
    }
    for entry in archive.entries().map_err(map)? {
        let mut entry = entry.map_err(map)?;
        let path = entry.path().map_err(map)?.into_owned();
        let raw = path.to_string_lossy();
        // Normalize a leading "./" (some archivers prefix entries with it) before
        // matching; tar paths are '/'-delimited on every platform.
        let p = raw.strip_prefix("./").unwrap_or(&raw);
        // The agent dir itself or anything beneath it.
        if p.trim_end_matches('/') == want || p.starts_with(&prefix) {
            // unpack_in is path-traversal-safe (it refuses to escape `dest`).
            entry.unpack_in(dest).map_err(map)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::registry::{IndexEntry, VersionEntry};
    use std::collections::BTreeMap;
    use std::io::Write;

    #[test]
    fn tarball_cache_name_shares_one_file_per_url_and_snapshot() {
        let url = "https://github.com/aware-aeco/aware/archive/refs/heads/main.tar.gz";
        // Two different agents in the SAME registry snapshot share one cache file (#243).
        assert_eq!(
            tarball_cache_name(url, "2026-06-16T00:00:00Z"),
            tarball_cache_name(url, "2026-06-16T00:00:00Z"),
        );
        // A new index snapshot (different content fingerprint) rotates the key → fresh download.
        assert_ne!(
            tarball_cache_name(url, "fingerprint-a"),
            tarball_cache_name(url, "fingerprint-b"),
        );
        // A different tarball URL → a different cache file.
        assert_ne!(
            tarball_cache_name(url, "snap"),
            tarball_cache_name("file:///tmp/other.tar.gz", "snap"),
        );
        // Domain separation: (url+"\0"+ts) can't collide across a boundary shift.
        assert_ne!(tarball_cache_name("ab", "c"), tarball_cache_name("a", "bc"));
        let name = tarball_cache_name(url, "snap");
        assert!(name.starts_with("tarball-") && name.ends_with(".tar.gz"));
    }

    #[test]
    fn extract_subdir_extracts_only_the_requested_subtree() {
        // Build a tarball with two sibling agent dirs (plus a prefix-lookalike that must
        // NOT match) and confirm extract_subdir writes only the requested one — the whole
        // point of #243's fix (don't unpack the rest of the monorepo).
        let tmp = tempfile::tempdir().unwrap();
        let tarball = tmp.path().join("multi.tar.gz");
        {
            let enc = flate2::write::GzEncoder::new(
                std::fs::File::create(&tarball).unwrap(),
                flate2::Compression::default(),
            );
            let mut tar = tar::Builder::new(enc);
            for path in [
                "root/20-agents/alpha/manifest.yaml",
                "root/20-agents/alpha/skills/a.md",
                "root/20-agents/beta/manifest.yaml",
                "root/20-agents/alpha-extra/manifest.yaml", // prefix lookalike — must NOT match
            ] {
                let body = b"x";
                let mut h = tar::Header::new_gnu();
                h.set_size(body.len() as u64);
                h.set_mode(0o644);
                h.set_cksum();
                tar.append_data(&mut h, path, &body[..]).unwrap();
            }
            tar.into_inner().unwrap().finish().unwrap();
        }

        let dest = tmp.path().join("out");
        extract_subdir(&tarball, &dest, "root/20-agents/alpha").unwrap();

        assert!(dest.join("root/20-agents/alpha/manifest.yaml").is_file());
        assert!(dest.join("root/20-agents/alpha/skills/a.md").is_file());
        // The sibling and the prefix-lookalike must be absent.
        assert!(!dest.join("root/20-agents/beta").exists());
        assert!(!dest.join("root/20-agents/alpha-extra").exists());
    }

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
        agents.insert(
            "tekla".into(),
            IndexEntry {
                versions,
                ..Default::default()
            },
        );
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

    /// Build a `main`-archive-shaped tarball holding the named agents, each a copy of
    /// the real tekla agent renamed and placed at `aware-main/20-agents/<name>` — close
    /// enough to the substrate's monorepo archive to drive a real install + validation.
    fn write_repo_tarball(path: &Path, agent_names: &[&str]) {
        let enc = flate2::write::GzEncoder::new(
            std::fs::File::create(path).unwrap(),
            flate2::Compression::default(),
        );
        let mut tar = tar::Builder::new(enc);

        let repo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let tekla_manifest =
            std::fs::read_to_string(repo.join("20-agents/aeco/engineering/tekla/manifest.yaml"))
                .unwrap();
        let agent: crate::manifest::Agent = serde_yaml::from_str(&tekla_manifest).unwrap();
        let skills_src = repo.join("20-agents/aeco/engineering/tekla/skills");

        let header = |len: usize| {
            let mut h = tar::Header::new_gnu();
            h.set_size(len as u64);
            h.set_mode(0o644);
            h.set_cksum();
            h
        };

        for &name in agent_names {
            // Rename the `agent:` value (line 1) so each subdir is a distinct, valid agent.
            let mut lines: Vec<String> = tekla_manifest.lines().map(str::to_string).collect();
            lines[0] = format!("agent: {name}");
            let manifest = lines.join("\n") + "\n";
            tar.append_data(
                &mut header(manifest.len()),
                format!("aware-main/20-agents/{name}/manifest.yaml"),
                manifest.as_bytes(),
            )
            .unwrap();
            for skill in &agent.skills {
                let body = std::fs::read_to_string(skills_src.join(skill)).unwrap();
                tar.append_data(
                    &mut header(body.len()),
                    format!("aware-main/20-agents/{name}/skills/{skill}"),
                    body.as_bytes(),
                )
                .unwrap();
            }
        }
        let mut file = tar.into_inner().unwrap().finish().unwrap();
        file.flush().unwrap();
    }

    #[test]
    fn update_migrates_an_aliased_rename_to_the_new_id() {
        // #256, end to end through the real update path. An agent installed under its
        // OLD id is renamed in the registry by keeping the old key as an `alias-of` the
        // new id, with its subdir pointed at the new agent's payload. `aware agent
        // update <old>` must then MIGRATE the install to the new id (old folder gone,
        // new folder present) — exactly the option-2 behavior the issue relies on.
        let tmp = tempfile::tempdir().unwrap();
        let aware = tmp.path().join("aware");
        let paths = Paths {
            aware_home: aware.clone(),
        };
        let archive = tmp.path().join("main.tar.gz");
        let url = format!("file://{}", archive.display());

        let entry = |subdir: &str| {
            let mut versions = BTreeMap::new();
            versions.insert(
                "0.1.0".to_string(),
                VersionEntry {
                    tarball: url.clone(),
                    subdir: subdir.to_string(),
                },
            );
            versions
        };

        // 1. Before the rename: install the agent under its OLD id.
        write_repo_tarball(&archive, &["steel-detailer-aisc"]);
        let mut before_agents = BTreeMap::new();
        before_agents.insert(
            "steel-detailer-aisc".to_string(),
            IndexEntry {
                versions: entry("aware-main/20-agents/steel-detailer-aisc"),
                ..Default::default()
            },
        );
        let before = Index {
            version: "1.0".into(),
            updated_at: "2026-06-18T00:00:00Z".into(),
            agents: before_agents,
            bundles: BTreeMap::new(),
        };
        assert_eq!(
            install_agent_from_registry("steel-detailer-aisc", None, &paths, &before).unwrap(),
            "steel-detailer-aisc"
        );
        assert!(
            aware
                .join("agents/steel-detailer-aisc/manifest.yaml")
                .is_file()
        );

        // 2. The rename ships: the archive now carries the NEW agent, and the index
        //    keeps the old key as an alias whose subdir points at the new payload.
        write_repo_tarball(&archive, &["steel-detailer-us"]);
        let mut after_agents = BTreeMap::new();
        after_agents.insert(
            "steel-detailer-us".to_string(),
            IndexEntry {
                versions: entry("aware-main/20-agents/steel-detailer-us"),
                ..Default::default()
            },
        );
        after_agents.insert(
            "steel-detailer-aisc".to_string(),
            IndexEntry {
                versions: entry("aware-main/20-agents/steel-detailer-us"),
                alias_of: Some("steel-detailer-us".to_string()),
                deprecated: false,
            },
        );
        let after = Index {
            version: "1.0".into(),
            updated_at: "2026-06-20T00:00:00Z".into(),
            agents: after_agents,
            bundles: BTreeMap::new(),
        };

        // 3. Updating the OLD install migrates it to the new id.
        let migrated = update_agent_from_registry("steel-detailer-aisc", &paths, &after).unwrap();
        assert_eq!(migrated, "steel-detailer-us");
        assert!(
            aware
                .join("agents/steel-detailer-us/manifest.yaml")
                .is_file(),
            "the install now lives under the new id"
        );
        assert!(
            !aware.join("agents/steel-detailer-aisc").exists(),
            "the old id folder is gone — no duplicate install"
        );
    }

    /// Like `write_repo_tarball` but for a single `alpha` agent whose `display-name`
    /// carries a caller-chosen marker, so a test can tell one archive *state* from
    /// another while the registry index stays byte-identical (same fingerprint).
    fn write_alpha_archive(path: &Path, display_marker: &str) {
        let enc = flate2::write::GzEncoder::new(
            std::fs::File::create(path).unwrap(),
            flate2::Compression::default(),
        );
        let mut tar = tar::Builder::new(enc);

        let repo = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf();
        let tekla_manifest =
            std::fs::read_to_string(repo.join("20-agents/aeco/engineering/tekla/manifest.yaml"))
                .unwrap();
        let agent: crate::manifest::Agent = serde_yaml::from_str(&tekla_manifest).unwrap();
        let skills_src = repo.join("20-agents/aeco/engineering/tekla/skills");

        let header = |len: usize| {
            let mut h = tar::Header::new_gnu();
            h.set_size(len as u64);
            h.set_mode(0o644);
            h.set_cksum();
            h
        };

        // Rename `agent:` → alpha and stamp `display-name:` with the marker (both single lines).
        let manifest = tekla_manifest
            .lines()
            .map(|l| {
                if l.starts_with("agent:") {
                    "agent: alpha".to_string()
                } else if l.starts_with("display-name:") {
                    format!("display-name: {display_marker}")
                } else {
                    l.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
            + "\n";
        tar.append_data(
            &mut header(manifest.len()),
            "aware-main/20-agents/alpha/manifest.yaml",
            manifest.as_bytes(),
        )
        .unwrap();
        for skill in &agent.skills {
            let body = std::fs::read_to_string(skills_src.join(skill)).unwrap();
            tar.append_data(
                &mut header(body.len()),
                format!("aware-main/20-agents/alpha/skills/{skill}"),
                body.as_bytes(),
            )
            .unwrap();
        }
        let mut file = tar.into_inner().unwrap().finish().unwrap();
        file.flush().unwrap();
    }

    #[test]
    fn stale_tarball_cache_self_refreshes_after_ttl_even_when_index_unchanged() {
        // #270: a manifest change made INSIDE the rolling `main.tar.gz` that leaves
        // `registry-index.json` byte-identical does not rotate the snapshot fingerprint,
        // so the fingerprint key alone can never bust the tarball cache. The cache has no
        // per-agent re-download budget (that is the #243 optimization), so without a TTL
        // the stale archive is served forever. This proves the tarball cache self-refreshes
        // once it ages past CACHE_TTL — the same 1h lever the index/catalog already pull.
        use crate::registry::fetch::CACHE_TTL;
        use std::time::{Duration, SystemTime};

        let tmp = tempfile::tempdir().unwrap();
        let aware = tmp.path().join("aware");
        let paths = Paths {
            aware_home: aware.clone(),
        };

        // One archive file = the single mutable `main` archive URL over time.
        let archive = tmp.path().join("main.tar.gz");
        let url = format!("file://{}", archive.display());

        // An index that does NOT change between the two archive states → identical fingerprint.
        let index = {
            let mut versions = BTreeMap::new();
            versions.insert(
                "1".to_string(),
                VersionEntry {
                    tarball: url.clone(),
                    subdir: "aware-main/20-agents/alpha".to_string(),
                },
            );
            let mut agents = BTreeMap::new();
            agents.insert(
                "alpha".to_string(),
                IndexEntry {
                    versions,
                    ..Default::default()
                },
            );
            Index {
                version: "1.0".into(),
                updated_at: "2026-06-25T00:00:00Z".into(),
                agents,
                bundles: BTreeMap::new(),
            }
        };

        let cache_file = paths
            .cache_dir()
            .join("agents")
            .join(tarball_cache_name(&url, &index.snapshot_fingerprint()));

        // v1 of the archive: alpha carries marker DISPLAY-V1. Stage it → caches v1.
        write_alpha_archive(&archive, "DISPLAY-V1");
        let (_g1, sub1) = stage_agent_from_registry("alpha", None, &paths, &index).unwrap();
        assert!(
            std::fs::read_to_string(sub1.join("manifest.yaml"))
                .unwrap()
                .contains("DISPLAY-V1")
        );
        assert!(cache_file.is_file(), "the tarball was cached");

        // The manifest inside the SAME archive flips, but the index is byte-identical.
        write_alpha_archive(&archive, "DISPLAY-V2");

        // Within TTL the warm cache is still served (bounded staleness, by design — the
        // fingerprint key cannot see a change that lives inside the archive).
        let (_g2, sub2) = stage_agent_from_registry("alpha", None, &paths, &index).unwrap();
        assert!(
            std::fs::read_to_string(sub2.join("manifest.yaml"))
                .unwrap()
                .contains("DISPLAY-V1"),
            "a fresh cache is reused — the fingerprint key cannot bust an in-archive change"
        );

        // Age the cache past the TTL → the next stage must re-pull the now-current archive.
        let stale = SystemTime::now()
            .checked_sub(CACHE_TTL + Duration::from_secs(60))
            .unwrap();
        std::fs::File::options()
            .write(true)
            .open(&cache_file)
            .unwrap()
            .set_modified(stale)
            .unwrap();

        let (_g3, sub3) = stage_agent_from_registry("alpha", None, &paths, &index).unwrap();
        assert!(
            std::fs::read_to_string(sub3.join("manifest.yaml"))
                .unwrap()
                .contains("DISPLAY-V2"),
            "#270: once the tarball cache ages past CACHE_TTL it self-refreshes despite an unchanged index"
        );
    }

    #[test]
    fn install_busts_cache_when_index_grows_even_with_frozen_updated_at() {
        // The #254 end-to-end regression. The shared `main` archive is a MUTABLE ref:
        // its content changes when an agent merges, but the URL is constant. The old
        // cache key was sha256(url + index.updated_at); once `updated-at` froze, a
        // newly-added agent's subdir stayed absent from the cached archive forever →
        // `subdir not in tarball`, no self-recovery. Keying on the index's content
        // fingerprint busts the cache the instant the installable set grows.
        let tmp = tempfile::tempdir().unwrap();
        let aware = tmp.path().join("aware");
        let paths = Paths {
            aware_home: aware.clone(),
        };

        // One shared tarball file = the single `main` archive URL over time.
        let archive = tmp.path().join("main.tar.gz");
        let url = format!("file://{}", archive.display());

        let index_with = |names: &[&str], updated_at: &str| {
            let mut agents = BTreeMap::new();
            for n in names {
                let mut versions = BTreeMap::new();
                versions.insert(
                    "1".to_string(),
                    VersionEntry {
                        tarball: url.clone(),
                        subdir: format!("aware-main/20-agents/{n}"),
                    },
                );
                agents.insert(
                    (*n).to_string(),
                    IndexEntry {
                        versions,
                        ..Default::default()
                    },
                );
            }
            Index {
                version: "1.0".into(),
                updated_at: updated_at.into(),
                agents,
                bundles: BTreeMap::new(),
            }
        };

        // v1: only `alpha` exists on main. Install it → caches the v1 archive.
        write_repo_tarball(&archive, &["alpha"]);
        let v1 = index_with(&["alpha"], "2026-06-10T00:00:00Z");
        assert_eq!(
            install_agent_from_registry("alpha", None, &paths, &v1).unwrap(),
            "alpha"
        );

        // main advances: `beta` merges. The archive content changes; the URL does not;
        // and (the bug) `updated-at` is left FROZEN at the v1 value.
        write_repo_tarball(&archive, &["alpha", "beta"]);
        let v2 = index_with(&["alpha", "beta"], "2026-06-10T00:00:00Z");

        // The OLD key (updated_at) would NOT rotate — same value → stale cache reused.
        assert_eq!(
            tarball_cache_name(&url, &v1.updated_at),
            tarball_cache_name(&url, &v2.updated_at),
            "frozen updated-at: the old key could not bust the cache (the bug)"
        );
        // The NEW key (snapshot fingerprint) rotates the moment the set grows.
        assert_ne!(
            tarball_cache_name(&url, &v1.snapshot_fingerprint()),
            tarball_cache_name(&url, &v2.snapshot_fingerprint()),
            "the snapshot fingerprint busts the cache when an agent is added (the fix)"
        );

        // End to end: installing the newly-added agent now succeeds against the fresh
        // archive instead of failing with `subdir not in tarball`.
        assert_eq!(
            install_agent_from_registry("beta", None, &paths, &v2).unwrap(),
            "beta"
        );
        assert!(aware.join("agents/beta/manifest.yaml").is_file());
    }
}
