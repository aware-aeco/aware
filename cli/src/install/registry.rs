//! Install an agent by resolving `<id>[@version]` in the registry index,
//! fetching the tarball, extracting the agent's subdirectory, then handing
//! off to `local::install_agent_from_path`.

#![allow(dead_code)]

use std::path::Path;

use flate2::read::GzDecoder;
use tar::Archive;

use crate::error::AwareError;
use crate::install::local::install_agent_from_path;
use crate::paths::Paths;
use crate::registry::Index;

pub fn install_agent_from_registry(
    id: &str,
    version_pin: Option<&str>,
    paths: &Paths,
    index: &Index,
) -> Result<String, AwareError> {
    let (resolved_version, entry) = index.resolve(id, version_pin)?;

    let scratch = tempfile::tempdir()?;
    let tarball_path = scratch.path().join("agent.tar.gz");

    let cache_dir = paths.cache_dir().join("agents");
    std::fs::create_dir_all(&cache_dir)?;
    let cache_file = cache_dir.join(format!("{id}-{resolved_version}.tar.gz"));

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
            "registry entry {id}@{resolved_version}: subdir {} not in tarball",
            entry.subdir,
        )));
    }
    install_agent_from_path(&subdir, paths)
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
