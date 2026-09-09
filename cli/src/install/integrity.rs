//! Deterministic installed-agent bundle hashing.

use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::error::AwareError;

const DOMAIN: &[u8] = b"aware-agent-bundle-tree-v1\0";

pub fn tree_digest(root: &Path) -> Result<String, AwareError> {
    let root = root.canonicalize()?;
    let mut files = Vec::new();
    collect(&root, &root, &mut files)?;
    digest_files(files)
}

/// Hash the files a repository archive will contain, including untracked
/// publish candidates but excluding ignored build outputs.
pub fn checkout_tree_digest(repo_root: &Path, root: &Path) -> Result<String, AwareError> {
    let relative_root = root.strip_prefix(repo_root).map_err(|_| {
        AwareError::Validation(format!("{} is outside registry checkout", root.display()))
    })?;
    let output = std::process::Command::new("git")
        .current_dir(repo_root)
        .args([
            "ls-files",
            "-z",
            "--cached",
            "--others",
            "--exclude-standard",
            "--",
        ])
        .arg(relative_root)
        .output();
    let output = match output {
        Ok(output) if output.status.success() => output,
        // External/custom registry authoring fixtures need not be Git checkouts.
        // In that case their complete directory is the publish payload.
        Ok(_) | Err(_) => return tree_digest(root),
    };
    let mut files = Vec::new();
    for raw in output.stdout.split(|b| *b == 0).filter(|p| !p.is_empty()) {
        let repo_relative = std::str::from_utf8(raw)
            .map_err(|_| AwareError::Validation("bundle path is not UTF-8".into()))?;
        let path = repo_root.join(repo_relative);
        let metadata = std::fs::symlink_metadata(&path)?;
        if !metadata.is_file() || metadata.file_type().is_symlink() || is_reparse_point(&metadata) {
            return Err(AwareError::Validation(format!(
                "bundle contains non-regular or indirect entry: {}",
                path.display()
            )));
        }
        let relative = path
            .strip_prefix(root)
            .map_err(|_| AwareError::Validation("git returned a path outside bundle".into()))?;
        let normalized = relative
            .components()
            .map(|c| c.as_os_str().to_str())
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| AwareError::Validation("bundle path is not UTF-8".into()))?
            .join("/");
        if normalized != super::provenance::FILE {
            files.push((normalized, path));
        }
    }
    digest_files(files)
}

/// Batch form used by `reindex`: one `git ls-files` snapshot for every release
/// avoids spawning Git once per agent while keeping all digests on one view.
pub fn checkout_tree_digests(
    repo_root: &Path,
    roots: &[PathBuf],
) -> Result<std::collections::BTreeMap<PathBuf, String>, AwareError> {
    let mut unique = roots.to_vec();
    unique.sort();
    unique.dedup();
    let mut command = std::process::Command::new("git");
    command.current_dir(repo_root).args([
        "ls-files",
        "-z",
        "--cached",
        "--others",
        "--exclude-standard",
        "--",
    ]);
    for root in &unique {
        command.arg(root.strip_prefix(repo_root).map_err(|_| {
            AwareError::Validation(format!("{} is outside registry checkout", root.display()))
        })?);
    }
    let output = command.output();
    let output = match output {
        Ok(output) if output.status.success() => output,
        Ok(_) | Err(_) => {
            return unique
                .into_iter()
                .map(|root| {
                    let digest = tree_digest(&root)?;
                    Ok((root, digest))
                })
                .collect();
        }
    };
    let mut grouped: std::collections::BTreeMap<PathBuf, Vec<(String, PathBuf)>> = unique
        .iter()
        .cloned()
        .map(|root| (root, Vec::new()))
        .collect();
    for raw in output.stdout.split(|b| *b == 0).filter(|p| !p.is_empty()) {
        let repo_relative = std::str::from_utf8(raw)
            .map_err(|_| AwareError::Validation("bundle path is not UTF-8".into()))?;
        let path = repo_root.join(repo_relative);
        let root = unique
            .iter()
            .filter(|root| path.starts_with(root))
            .max_by_key(|root| root.components().count())
            .ok_or_else(|| {
                AwareError::Validation("git returned a path outside requested bundles".into())
            })?;
        let metadata = std::fs::symlink_metadata(&path)?;
        if !metadata.is_file() || metadata.file_type().is_symlink() || is_reparse_point(&metadata) {
            return Err(AwareError::Validation(format!(
                "bundle contains non-regular or indirect entry: {}",
                path.display()
            )));
        }
        let relative = path
            .strip_prefix(root)
            .map_err(|_| AwareError::Validation("bundle path escaped root".into()))?;
        let normalized = relative
            .components()
            .map(|c| c.as_os_str().to_str())
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| AwareError::Validation("bundle path is not UTF-8".into()))?
            .join("/");
        if normalized != super::provenance::FILE
            && let Some(files) = grouped.get_mut(root)
        {
            files.push((normalized, path));
        }
    }
    grouped
        .into_iter()
        .map(|(root, files)| Ok((root, digest_files(files)?)))
        .collect()
}

fn digest_files(mut files: Vec<(String, PathBuf)>) -> Result<String, AwareError> {
    files.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
    let mut h = Sha256::new();
    h.update(DOMAIN);
    for (relative, path) in files {
        let name = relative.as_bytes();
        let bytes = std::fs::read(&path).map_err(|e| {
            AwareError::Validation(format!("cannot read bundle file {}: {e}", path.display()))
        })?;
        h.update((name.len() as u64).to_be_bytes());
        h.update(name);
        h.update((bytes.len() as u64).to_be_bytes());
        h.update(&bytes);
    }
    Ok(format!("sha256:{:x}", h.finalize()))
}

fn collect(root: &Path, dir: &Path, out: &mut Vec<(String, PathBuf)>) -> Result<(), AwareError> {
    let mut entries = std::fs::read_dir(dir)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(std::fs::DirEntry::file_name);
    for entry in entries {
        let path = entry.path();
        let metadata = std::fs::symlink_metadata(&path)?;
        if metadata.file_type().is_symlink() || is_reparse_point(&metadata) {
            return Err(AwareError::Validation(format!(
                "agent bundle contains symlink/reparse indirection: {}",
                path.display()
            )));
        }
        if metadata.is_dir() {
            collect(root, &path, out)?;
        } else if metadata.is_file() {
            let relative = path.strip_prefix(root).map_err(|_| {
                AwareError::Validation(format!("bundle path escaped root: {}", path.display()))
            })?;
            let normalized = relative
                .components()
                .map(|c| c.as_os_str().to_str())
                .collect::<Option<Vec<_>>>()
                .ok_or_else(|| AwareError::Validation("bundle path is not UTF-8".into()))?
                .join("/");
            if normalized != super::provenance::FILE {
                out.push((normalized, path));
            }
        } else {
            return Err(AwareError::Validation(format!(
                "agent bundle contains non-regular entry: {}",
                path.display()
            )));
        }
    }
    Ok(())
}

#[cfg(windows)]
fn is_reparse_point(metadata: &std::fs::Metadata) -> bool {
    use std::os::windows::fs::MetadataExt;
    metadata.file_attributes() & 0x400 != 0
}

#[cfg(not(windows))]
fn is_reparse_point(_: &std::fs::Metadata) -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_is_stable_sensitive_and_excludes_receipt() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::create_dir(tmp.path().join("x")).unwrap();
        std::fs::write(tmp.path().join("x/a"), b"one").unwrap();
        let first = tree_digest(tmp.path()).unwrap();
        std::fs::write(tmp.path().join(super::super::provenance::FILE), b"receipt").unwrap();
        assert_eq!(first, tree_digest(tmp.path()).unwrap());
        std::fs::write(tmp.path().join("x/a"), b"two").unwrap();
        assert_ne!(first, tree_digest(tmp.path()).unwrap());
    }

    #[cfg(unix)]
    #[test]
    fn rejects_symlinks() {
        use std::os::unix::fs::symlink;
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("target"), b"x").unwrap();
        symlink("target", tmp.path().join("link")).unwrap();
        assert!(tree_digest(tmp.path()).is_err());
    }
}
