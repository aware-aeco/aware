//! Deterministic installed-agent bundle hashing.

use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::error::AwareError;

const DOMAIN: &[u8] = b"aware-agent-bundle-tree-v1\0";

pub fn tree_digest(root: &Path) -> Result<String, AwareError> {
    // Inspect the path as named before canonicalizing it. Canonicalization follows
    // a root symlink/junction and would otherwise erase the evidence that the
    // caller crossed an indirection boundary before `collect` can reject it.
    let root_metadata = std::fs::symlink_metadata(root)?;
    if root_metadata.file_type().is_symlink() || is_reparse_point(&root_metadata) {
        return Err(AwareError::Validation(format!(
            "agent bundle root contains symlink/reparse indirection: {}",
            root.display()
        )));
    }
    let root = root.canonicalize()?;
    let mut files = Vec::new();
    collect(&root, &root, &mut files)?;
    digest_files(files)
}

/// Hash the exact Git index blobs a repository archive will contain. Staged
/// additions/changes are supported; unstaged or untracked bundle content is
/// refused rather than hashing stale index bytes or checkout-translated bytes.
pub fn checkout_tree_digest(repo_root: &Path, root: &Path) -> Result<String, AwareError> {
    checkout_tree_digests(repo_root, &[root.to_path_buf()])?
        .remove(root)
        .ok_or_else(|| AwareError::Validation("bundle digest was not computed".into()))
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
    let mut paths = Vec::new();
    for root in &unique {
        paths.push(root.strip_prefix(repo_root).map_err(|_| {
            AwareError::Validation(format!("{} is outside registry checkout", root.display()))
        })?);
    }
    let mut status = std::process::Command::new("git");
    status.current_dir(repo_root).args([
        "status",
        "--porcelain=v1",
        "-z",
        "--untracked-files=all",
        "--",
    ]);
    status.args(&paths);
    let status = status.output().map_err(|error| {
        AwareError::Validation(format!(
            "cannot run `git status` while hashing registry bundles: {error}; run publish/reindex inside a Git checkout with Git available"
        ))
    })?;
    if !status.status.success() {
        let stderr = String::from_utf8_lossy(&status.stderr);
        let detail = stderr.trim();
        return Err(AwareError::Validation(format!(
            "`git status` failed while hashing registry bundles{}; run publish/reindex inside a Git checkout and stage the exact bundle content first",
            if detail.is_empty() {
                String::new()
            } else {
                format!(": {detail}")
            }
        )));
    }
    for item in status.stdout.split(|b| *b == 0).filter(|p| !p.is_empty()) {
        if item.len() < 3 {
            return Err(AwareError::Validation(
                "malformed `git status --porcelain` response while hashing agent bundle".into(),
            ));
        }
        let index_status = item[0];
        let worktree_status = item[1];
        if worktree_status != b' ' || index_status == b'?' || matches!(index_status, b'R' | b'C') {
            return Err(AwareError::Validation(
                "agent bundle has unstaged, untracked, or renamed content; run `git add <agent-folder>` (and commit/resolve renames) before publish/reindex so the digest binds the exact archive bytes".into(),
            ));
        }
    }
    // `git status --untracked-files=all` deliberately follows ignore rules. An
    // ignored file can still affect on-disk manifest validation while being absent
    // from both the Git archive and the digest, so inventory ignored untracked
    // content separately and reject it rather than blessing two different trees.
    let mut ignored = std::process::Command::new("git");
    ignored.current_dir(repo_root).args([
        "ls-files",
        "--others",
        "--ignored",
        "--exclude-standard",
        "-z",
        "--",
    ]);
    ignored.args(&paths);
    let ignored = ignored.output().map_err(|error| {
        AwareError::Validation(format!(
            "cannot inspect ignored agent bundle content with `git ls-files`: {error}"
        ))
    })?;
    if !ignored.status.success() {
        return Err(AwareError::Validation(
            "git ls-files failed while inspecting ignored agent bundle content".into(),
        ));
    }
    if !ignored.stdout.is_empty() {
        return Err(AwareError::Validation(
            "agent bundle contains ignored untracked content; remove it or explicitly force-add and stage it so validation, digest, and archive bind the same files".into(),
        ));
    }
    let mut listed = std::process::Command::new("git");
    listed
        .current_dir(repo_root)
        .args(["ls-files", "--stage", "-z", "--"]);
    listed.args(&paths);
    let listed = listed.output()?;
    if !listed.status.success() {
        return Err(AwareError::Validation(
            "git ls-files failed while hashing registry bundles".into(),
        ));
    }
    #[derive(Clone)]
    struct BlobRecord {
        root: PathBuf,
        relative: String,
        oid: String,
    }
    let mut records = Vec::new();
    for raw in listed.stdout.split(|b| *b == 0).filter(|p| !p.is_empty()) {
        let text = std::str::from_utf8(raw)
            .map_err(|_| AwareError::Validation("bundle path is not UTF-8".into()))?;
        let (header, repo_relative) = text
            .split_once('\t')
            .ok_or_else(|| AwareError::Validation("malformed git index entry".into()))?;
        let mut fields = header.split_whitespace();
        let mode = fields.next().unwrap_or("");
        let oid = fields.next().unwrap_or("");
        let stage = fields.next().unwrap_or("");
        if stage != "0" || !matches!(mode, "100644" | "100755") {
            return Err(AwareError::Validation(format!(
                "bundle contains conflicted, symlink, gitlink, or non-regular index entry: {repo_relative}"
            )));
        }
        let path = repo_root.join(repo_relative);
        let containing_roots = unique
            .iter()
            .filter(|root| path.starts_with(root))
            .collect::<Vec<_>>();
        if containing_roots.is_empty() {
            return Err(AwareError::Validation(
                "git returned a path outside requested bundles".into(),
            ));
        }
        // Overlapping release roots are uncommon but valid. A file below the
        // child is also part of the parent's archive subtree, so feed it to every
        // containing root just as independent `checkout_tree_digest` calls would.
        for root in containing_roots {
            let relative = path
                .strip_prefix(root)
                .map_err(|_| AwareError::Validation("bundle path escaped root".into()))?;
            let normalized = relative
                .components()
                .map(|c| c.as_os_str().to_str())
                .collect::<Option<Vec<_>>>()
                .ok_or_else(|| AwareError::Validation("bundle path is not UTF-8".into()))?
                .join("/");
            if normalized != super::provenance::FILE {
                records.push(BlobRecord {
                    root: root.clone(),
                    relative: normalized,
                    oid: oid.into(),
                });
            }
        }
    }
    for root in &unique {
        if !records.iter().any(|record| &record.root == root) {
            return Err(AwareError::Validation(format!(
                "agent bundle {} has no staged regular files; ignored or empty bundle trees cannot be hashed for publish/reindex",
                root.display()
            )));
        }
    }
    records.sort_by(|a, b| {
        a.relative
            .as_bytes()
            .cmp(b.relative.as_bytes())
            .then(a.root.cmp(&b.root))
    });
    let mut child = std::process::Command::new("git")
        .current_dir(repo_root)
        .args(["cat-file", "--batch"])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()?;
    let mut input = child
        .stdin
        .take()
        .ok_or_else(|| AwareError::Internal("git cat-file stdin unavailable".into()))?;
    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| AwareError::Internal("git cat-file stdout unavailable".into()))?;
    let mut reader = std::io::BufReader::new(stdout);
    let mut hashers: std::collections::BTreeMap<PathBuf, Sha256> = unique
        .iter()
        .cloned()
        .map(|root| {
            let mut h = Sha256::new();
            h.update(DOMAIN);
            (root, h)
        })
        .collect();
    for record in records {
        use std::io::{BufRead, Read, Write};
        writeln!(input, "{}", record.oid)?;
        input.flush()?;
        let mut header = String::new();
        reader.read_line(&mut header)?;
        let mut fields = header.split_whitespace();
        let _oid = fields.next();
        if fields.next() != Some("blob") {
            return Err(AwareError::Validation(format!(
                "{} is not a Git blob",
                record.oid
            )));
        }
        let size: usize = fields
            .next()
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| AwareError::Validation("malformed git cat-file response".into()))?;
        let mut bytes = vec![0; size];
        reader.read_exact(&mut bytes)?;
        let mut newline = [0];
        reader.read_exact(&mut newline)?;
        if newline[0] != b'\n' {
            return Err(AwareError::Validation(
                "malformed git cat-file framing".into(),
            ));
        }
        let h = hashers
            .get_mut(&record.root)
            .ok_or_else(|| AwareError::Internal("missing bundle hasher".into()))?;
        let name = record.relative.as_bytes();
        h.update((name.len() as u64).to_be_bytes());
        h.update(name);
        h.update((bytes.len() as u64).to_be_bytes());
        h.update(&bytes);
    }
    drop(input);
    if !child.wait()?.success() {
        return Err(AwareError::Validation(
            "git cat-file failed while hashing bundles".into(),
        ));
    }
    Ok(hashers
        .into_iter()
        .map(|(root, h)| (root, format!("sha256:{:x}", h.finalize())))
        .collect())
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

    #[test]
    fn checkout_hash_uses_git_blob_bytes_not_crlf_worktree_bytes() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        let git = |args: &[&str]| {
            let status = std::process::Command::new("git")
                .current_dir(repo)
                .args(args)
                .status()
                .unwrap();
            assert!(status.success(), "git {args:?}");
        };
        git(&["init", "--quiet"]);
        git(&["config", "core.autocrlf", "true"]);
        std::fs::create_dir(repo.join("agent")).unwrap();
        std::fs::write(
            repo.join("agent/manifest.yaml"),
            b"agent: probe\r\nversion: 1\r\n",
        )
        .unwrap();
        git(&["add", "agent/manifest.yaml"]);

        let actual = checkout_tree_digest(repo, &repo.join("agent")).unwrap();
        let mut expected = Sha256::new();
        expected.update(DOMAIN);
        let name = b"manifest.yaml";
        let blob = b"agent: probe\nversion: 1\n";
        expected.update((name.len() as u64).to_be_bytes());
        expected.update(name);
        expected.update((blob.len() as u64).to_be_bytes());
        expected.update(blob);
        assert_eq!(actual, format!("sha256:{:x}", expected.finalize()));
        assert_ne!(actual, tree_digest(&repo.join("agent")).unwrap());
    }

    #[test]
    fn checkout_hash_fails_closed_when_git_status_fails() {
        let tmp = tempfile::tempdir().unwrap();
        let bundle = tmp.path().join("agent");
        std::fs::create_dir(&bundle).unwrap();
        std::fs::write(bundle.join("manifest.yaml"), b"agent: probe\n").unwrap();

        let error = checkout_tree_digest(tmp.path(), &bundle).unwrap_err();
        let message = error.to_string();
        assert!(message.contains("git status"), "{message}");
        assert!(message.contains("Git checkout"), "{message}");
    }

    #[test]
    fn checkout_hash_rejects_unstaged_worktree_modification() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        let git = |args: &[&str]| {
            let status = std::process::Command::new("git")
                .current_dir(repo)
                .args(args)
                .status()
                .unwrap();
            assert!(status.success(), "git {args:?}");
        };
        git(&["init", "--quiet"]);
        std::fs::create_dir(repo.join("agent")).unwrap();
        std::fs::write(repo.join("agent/manifest.yaml"), b"agent: staged\n").unwrap();
        git(&["add", "agent/manifest.yaml"]);
        std::fs::write(repo.join("agent/manifest.yaml"), b"agent: unstaged\n").unwrap();

        let error = checkout_tree_digest(repo, &repo.join("agent")).unwrap_err();
        assert!(error.to_string().contains("unstaged"), "{error}");
    }

    #[test]
    fn checkout_hash_rejects_entirely_ignored_new_bundle() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        let git = |args: &[&str]| {
            let status = std::process::Command::new("git")
                .current_dir(repo)
                .args(args)
                .status()
                .unwrap();
            assert!(status.success(), "git {args:?}");
        };
        git(&["init", "--quiet"]);
        std::fs::write(repo.join(".gitignore"), b"agent/\n").unwrap();
        git(&["add", ".gitignore"]);
        std::fs::create_dir(repo.join("agent")).unwrap();
        std::fs::write(repo.join("agent/manifest.yaml"), b"agent: ignored\n").unwrap();

        let error = checkout_tree_digest(repo, &repo.join("agent")).unwrap_err();
        assert!(error.to_string().contains("ignored"), "{error}");
    }

    #[test]
    fn checkout_hash_rejects_empty_bundle_tree() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        let status = std::process::Command::new("git")
            .current_dir(repo)
            .args(["init", "--quiet"])
            .status()
            .unwrap();
        assert!(status.success());
        let bundle = repo.join("agent");
        std::fs::create_dir(&bundle).unwrap();

        let error = checkout_tree_digest(repo, &bundle).unwrap_err();
        assert!(
            error.to_string().contains("no staged regular files"),
            "{error}"
        );
    }

    #[test]
    fn batch_checkout_hash_includes_nested_root_files_in_parent() {
        let tmp = tempfile::tempdir().unwrap();
        let repo = tmp.path();
        let git = |args: &[&str]| {
            let status = std::process::Command::new("git")
                .current_dir(repo)
                .args(args)
                .status()
                .unwrap();
            assert!(status.success(), "git {args:?}");
        };
        git(&["init", "--quiet"]);
        let parent = repo.join("releases/parent");
        let child = parent.join("nested-child");
        std::fs::create_dir_all(&child).unwrap();
        std::fs::write(parent.join("parent.txt"), b"parent\n").unwrap();
        std::fs::write(child.join("child.txt"), b"child\n").unwrap();
        git(&["add", "releases"]);

        let batched = checkout_tree_digests(repo, &[parent.clone(), child.clone()]).unwrap();
        let parent_alone = checkout_tree_digest(repo, &parent).unwrap();
        let child_alone = checkout_tree_digest(repo, &child).unwrap();

        assert_eq!(batched.get(&parent), Some(&parent_alone));
        assert_eq!(batched.get(&child), Some(&child_alone));
    }

    #[cfg(unix)]
    #[test]
    fn rejects_nested_symlinks() {
        use std::os::unix::fs::symlink;
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("target"), b"x").unwrap();
        symlink("target", tmp.path().join("link")).unwrap();
        assert!(tree_digest(tmp.path()).is_err());
    }

    #[cfg(unix)]
    #[test]
    fn rejects_symlink_bundle_root_before_canonicalizing() {
        use std::os::unix::fs::symlink;
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("target");
        std::fs::create_dir(&target).unwrap();
        std::fs::write(target.join("manifest.yaml"), b"agent: probe\n").unwrap();
        let link = tmp.path().join("bundle-link");
        symlink(&target, &link).unwrap();

        let error = tree_digest(&link).unwrap_err();
        assert!(error.to_string().contains("bundle root"), "{error}");
    }

    #[cfg(windows)]
    #[test]
    fn rejects_junction_bundle_root_before_canonicalizing() {
        let tmp = tempfile::tempdir().unwrap();
        let target = tmp.path().join("target");
        std::fs::create_dir(&target).unwrap();
        std::fs::write(target.join("manifest.yaml"), b"agent: probe\n").unwrap();
        let junction = tmp.path().join("bundle-junction");
        // Directory junction creation does not require the symlink privilege and
        // exercises the NTFS reparse-point case that canonicalize would follow.
        let status = std::process::Command::new("cmd.exe")
            .args(["/C", "mklink", "/J"])
            .arg(&junction)
            .arg(&target)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .unwrap();
        assert!(status.success(), "failed to create test junction");

        let error = tree_digest(&junction).unwrap_err();
        assert!(error.to_string().contains("bundle root"), "{error}");
    }
}
