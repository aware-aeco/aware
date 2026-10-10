//! Deterministic installed-agent bundle hashing.

use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::error::AwareError;
use crate::fs::is_reparse_point;

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
    digest_files(digest_inputs(&root)?)
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
            if !is_install_metadata(&normalized) {
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

/// Domain for the stat fingerprint; distinct from [`DOMAIN`] so a fingerprint
/// can never be mistaken for a content digest.
const FINGERPRINT_DOMAIN: &[u8] = b"aware-agent-bundle-stat-fingerprint-v1\0";

/// A file whose mtime is this close to "now" may still be rewritten within the
/// filesystem's timestamp granularity without its (size, mtime) changing, so a
/// digest measured then is not remembered ("racy git").
const RACY_WINDOW: std::time::Duration = std::time::Duration::from_secs(3);

#[derive(serde::Serialize, serde::Deserialize)]
struct DigestCacheEntry {
    root: String,
    fingerprint: String,
    digest: String,
}

/// Where the remembered digest of `root` lives: outside the bundle, so the tree
/// itself (and therefore every other reader of it) is untouched.
fn digest_cache_path(paths: &crate::paths::Paths, root: &Path) -> PathBuf {
    let mut h = Sha256::new();
    h.update(root.to_string_lossy().as_bytes());
    let key = format!("{:x}", h.finalize());
    paths
        .cache_dir()
        .join("tree-digest")
        .join(format!("{}.json", &key[..32]))
}

/// Cheap identity of a tree's current state: a hash over every hashed file's
/// relative path, size and modification time. Reads no file content. Returns
/// the fingerprint and whether any file is too recent to trust (see
/// [`RACY_WINDOW`]).
fn stat_fingerprint(files: &[HashedFile]) -> Result<(String, bool), AwareError> {
    let now = std::time::SystemTime::now();
    let mut racy = false;
    let mut h = Sha256::new();
    h.update(FINGERPRINT_DOMAIN);
    for (relative, _, metadata) in files {
        let modified = metadata
            .modified()
            .ok()
            .and_then(|m| m.duration_since(std::time::UNIX_EPOCH).ok());
        let Some(modified) = modified else {
            // No usable timestamp: nothing to bind a remembered digest to.
            racy = true;
            h.update(relative.as_bytes());
            continue;
        };
        if now
            .duration_since(std::time::UNIX_EPOCH)
            .map(|n| n < modified + RACY_WINDOW)
            .unwrap_or(true)
        {
            racy = true;
        }
        h.update((relative.len() as u64).to_be_bytes());
        h.update(relative.as_bytes());
        h.update(metadata.len().to_be_bytes());
        h.update(modified.as_secs().to_be_bytes());
        h.update(modified.subsec_nanos().to_be_bytes());
        // A file re-created with the same size and mtime (a reproducible
        // archive of a same-length edit, an update swapping the folder) still
        // is a different file: bind the fingerprint to when it was made.
        h.update(file_identity(metadata));
    }
    Ok((format!("{:x}", h.finalize()), racy))
}

/// What distinguishes a file from a re-created one that has the same size and
/// mtime: its creation time on Windows; its inode and change time elsewhere.
#[cfg(windows)]
fn file_identity(metadata: &std::fs::Metadata) -> Vec<u8> {
    let created = metadata
        .created()
        .ok()
        .and_then(|c| c.duration_since(std::time::UNIX_EPOCH).ok())
        .unwrap_or_default();
    let mut out = created.as_secs().to_be_bytes().to_vec();
    out.extend(created.subsec_nanos().to_be_bytes());
    out
}

#[cfg(unix)]
fn file_identity(metadata: &std::fs::Metadata) -> Vec<u8> {
    use std::os::unix::fs::MetadataExt;
    let mut out = metadata.ino().to_be_bytes().to_vec();
    out.extend(metadata.dev().to_be_bytes());
    out.extend(metadata.ctime().to_be_bytes());
    out.extend(metadata.ctime_nsec().to_be_bytes());
    out
}

#[cfg(not(any(windows, unix)))]
fn file_identity(_metadata: &std::fs::Metadata) -> Vec<u8> {
    Vec::new()
}

/// [`tree_digest`], but a tree whose hashed files all still have the same
/// relative path, size, modification time and file identity as when its digest was last
/// measured answers from that memory instead of re-reading every byte
/// (aware-aeco/aware#678: reading ~23k files per `agent refs` took minutes
/// on Windows, where every open is also scanned by antivirus).
///
/// Never weaker than "same as the last full hash unless a file changed
/// without changing its size or mtime". That residual case — bytes edited
/// in place with the timestamp restored — is NOT detected here, so this is
/// for reporting paths (`agent refs`, GC planning). Anything that decides
/// whether bytes may be executed or restored must keep calling
/// [`tree_digest`]. A missing, corrupt or mismatching memory, a root that
/// moved, or a tree modified within [`RACY_WINDOW`] all fall back to the
/// full hash, which then refreshes the memory (best effort; a failure to
/// write it never fails the digest).
pub fn tree_digest_cached(paths: &crate::paths::Paths, root: &Path) -> Result<String, AwareError> {
    let root_metadata = std::fs::symlink_metadata(root)?;
    if root_metadata.file_type().is_symlink() || is_reparse_point(&root_metadata) {
        // Same refusal as the uncached form.
        return tree_digest(root);
    }
    let canonical = root.canonicalize()?;
    let measured = digest_inputs_with_metadata(&canonical)?;
    let (fingerprint, racy) = stat_fingerprint(&measured)?;
    let cache_path = digest_cache_path(paths, &canonical);
    let root_text = canonical.to_string_lossy().into_owned();
    if let Some(entry) = std::fs::read(&cache_path)
        .ok()
        .and_then(|bytes| serde_json::from_slice::<DigestCacheEntry>(&bytes).ok())
        && entry.root == root_text
        && entry.fingerprint == fingerprint
        && entry.digest.starts_with("sha256:")
    {
        return Ok(entry.digest);
    }
    let digest = digest_files(measured.into_iter().map(|(r, p, _)| (r, p)).collect())?;
    if !racy {
        // Re-take the fingerprint after hashing: if the tree moved while it
        // was being read, the digest describes no single state; remember
        // nothing.
        let settled = digest_inputs_with_metadata(&canonical)
            .and_then(|again| stat_fingerprint(&again))
            .map(|(after, _)| after == fingerprint)
            .unwrap_or(false);
        if settled {
            let entry = DigestCacheEntry {
                root: root_text,
                fingerprint,
                digest: digest.clone(),
            };
            let _ = write_digest_cache(&cache_path, &entry);
        }
    }
    Ok(digest)
}

/// Forget remembered digests whose tree no longer exists (a removed agent, a
/// pruned snapshot) so the cache directory cannot grow without bound. Best
/// effort: a file that cannot be read or removed is left for the next pass.
pub fn prune_digest_cache(paths: &crate::paths::Paths) {
    let dir = paths.cache_dir().join("tree-digest");
    let Ok(entries) = std::fs::read_dir(&dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue; // an in-flight temp file of a writer: not ours to judge
        }
        let alive = std::fs::read(&path)
            .ok()
            .and_then(|bytes| serde_json::from_slice::<DigestCacheEntry>(&bytes).ok())
            .is_some_and(|e| Path::new(&e.root).is_dir());
        if !alive {
            let _ = std::fs::remove_file(&path);
        }
    }
}

fn write_digest_cache(path: &Path, entry: &DigestCacheEntry) -> std::io::Result<()> {
    let dir = path
        .parent()
        .ok_or_else(|| std::io::Error::other("cache path has no parent"))?;
    std::fs::create_dir_all(dir)?;
    let bytes = serde_json::to_vec(entry).map_err(std::io::Error::other)?;
    let mut temp = tempfile::NamedTempFile::new_in(dir)?;
    std::io::Write::write_all(&mut temp, &bytes)?;
    temp.persist(path).map_err(|e| e.error)?;
    Ok(())
}

/// Files read ahead of the hasher at once. Reading is the slow part (every open is
/// also scanned by antivirus on Windows), so a batch is read by several threads;
/// the bytes are still fed to the hash in sorted order, so the digest is
/// identical to a one-file-at-a-time read.
const READ_BATCH: usize = 64;

/// ...and no more than this many bytes held ahead of the hasher at once (a batch
/// always holds at least one file, however large).
const READ_BATCH_BYTES: u64 = 32 * 1024 * 1024;

fn read_workers() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
        .clamp(2, 8)
}

fn digest_files(mut files: Vec<(String, PathBuf)>) -> Result<String, AwareError> {
    files.sort_by(|a, b| a.0.as_bytes().cmp(b.0.as_bytes()));
    let mut h = Sha256::new();
    h.update(DOMAIN);
    let workers = read_workers();
    let mut start = 0;
    while start < files.len() {
        // Read ahead by count and by bytes: many small files are the slow case
        // worth overlapping; a large file is read on its own, as before.
        let mut end = start;
        let mut bytes_ahead = 0u64;
        while end < files.len() && end - start < READ_BATCH {
            let size = std::fs::metadata(&files[end].1)
                .map(|m| m.len())
                .unwrap_or(0);
            if end > start && bytes_ahead + size > READ_BATCH_BYTES {
                break;
            }
            bytes_ahead += size;
            end += 1;
        }
        let batch = &files[start..end];
        let contents = read_batch(batch, workers)?;
        for ((relative, _), bytes) in batch.iter().zip(contents) {
            let name = relative.as_bytes();
            h.update((name.len() as u64).to_be_bytes());
            h.update(name);
            h.update((bytes.len() as u64).to_be_bytes());
            h.update(&bytes);
        }
        start = end;
    }
    Ok(format!("sha256:{:x}", h.finalize()))
}

type ReadResult = Result<Vec<u8>, AwareError>;

/// The contents of `batch`, in the same order, read by up to `workers` threads.
fn read_batch(batch: &[(String, PathBuf)], workers: usize) -> Result<Vec<Vec<u8>>, AwareError> {
    let read = |path: &Path| {
        std::fs::read(path).map_err(|e| {
            AwareError::Validation(format!("cannot read bundle file {}: {e}", path.display()))
        })
    };
    if workers <= 1 || batch.len() <= 1 {
        return batch.iter().map(|(_, path)| read(path)).collect();
    }
    let next = std::sync::atomic::AtomicUsize::new(0);
    let slots: Vec<std::sync::Mutex<Option<ReadResult>>> =
        batch.iter().map(|_| std::sync::Mutex::new(None)).collect();
    std::thread::scope(|scope| {
        for _ in 0..workers.min(batch.len()) {
            scope.spawn(|| {
                loop {
                    let i = next.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                    let Some((_, path)) = batch.get(i) else { break };
                    let result = read(path);
                    if let Ok(mut slot) = slots[i].lock() {
                        *slot = Some(result);
                    }
                }
            });
        }
    });
    let mut out = Vec::with_capacity(batch.len());
    for slot in slots {
        let result = slot
            .into_inner()
            .ok()
            .flatten()
            .ok_or_else(|| AwareError::Internal("bundle file read did not finish".into()))?;
        out.push(result?);
    }
    Ok(out)
}

type HashedFile = (String, PathBuf, std::fs::Metadata);

/// [`digest_inputs`] with each file's metadata from the walk's own stat.
fn digest_inputs_with_metadata(root: &Path) -> Result<Vec<HashedFile>, AwareError> {
    Ok(
        crate::fs::plain_files_with_metadata_under(root, "agent bundle")?
            .into_iter()
            .filter(|(relative, _)| !is_install_metadata(relative))
            .map(|(relative, (path, metadata))| (relative, path, metadata))
            .collect(),
    )
}

/// Every bundle file that contributes to the digest: the tree walk, minus the
/// install receipt and the agent-store package record.
///
/// The exclusion lives here and not in [`crate::fs::plain_files_under`] because
/// it is this caller's rule, not the walk's — the receipt is written *after* a
/// bundle is hashed and promoted, so including it would make a bundle's digest
/// change the moment it was installed. The store's `.aware-package.yaml` is the
/// same kind of file (#626): it is written into a snapshot *about* the bytes, so
/// a snapshot's digest must equal the working copy it was taken from. The shared
/// walk stays a walk.
fn digest_inputs(root: &Path) -> Result<Vec<(String, PathBuf)>, AwareError> {
    Ok(crate::fs::plain_files_under(root, "agent bundle")?
        .into_iter()
        .filter(|(relative, _)| !is_install_metadata(relative))
        .collect())
}

/// The top-level metadata files that describe a bundle rather than belong to it:
/// the install receipt and the agent-store package record. Neither is hashed.
pub(crate) fn is_install_metadata(relative: &str) -> bool {
    relative == super::provenance::FILE || relative == crate::agent_store::PACKAGE_FILE
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

    /// #626: a store snapshot carries `.aware-package.yaml`, and its digest must
    /// still equal the working copy it was taken from.
    #[test]
    fn hash_excludes_the_agent_store_package_record() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(
            tmp.path().join("manifest.yaml"),
            b"agent: x
",
        )
        .unwrap();
        let first = tree_digest(tmp.path()).unwrap();
        std::fs::write(
            tmp.path().join(crate::agent_store::PACKAGE_FILE),
            b"agent: x
",
        )
        .unwrap();
        assert_eq!(first, tree_digest(tmp.path()).unwrap());
        // …but only at the top level: a nested file of that name is content.
        std::fs::create_dir(tmp.path().join("sub")).unwrap();
        std::fs::write(
            tmp.path()
                .join("sub")
                .join(crate::agent_store::PACKAGE_FILE),
            b"x",
        )
        .unwrap();
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

    // ---- tree_digest_cached (#678) ----

    /// One fixed instant an hour ago, so a rewrite can restore it exactly.
    fn old_mtime() -> std::time::SystemTime {
        static AT: std::sync::OnceLock<std::time::SystemTime> = std::sync::OnceLock::new();
        *AT.get_or_init(|| std::time::SystemTime::now() - std::time::Duration::from_secs(3600))
    }

    fn set_mtime(path: &Path, at: std::time::SystemTime) {
        let file = std::fs::OpenOptions::new().write(true).open(path).unwrap();
        file.set_modified(at).unwrap();
    }

    /// A bundle whose files all look an hour old, so the racy-timestamp guard
    /// does not suppress remembering the digest.
    fn aged_bundle(root: &Path) {
        std::fs::create_dir_all(root.join("sub")).unwrap();
        for (name, body) in [("a.txt", "one"), ("sub/b.txt", "two")] {
            let path = root.join(name);
            std::fs::write(&path, body).unwrap();
            set_mtime(&path, old_mtime());
        }
    }

    fn cache_home() -> (tempfile::TempDir, crate::paths::Paths) {
        let home = tempfile::tempdir().unwrap();
        let paths = crate::paths::Paths {
            aware_home: home.path().to_path_buf(),
        };
        (home, paths)
    }

    fn cache_files(paths: &crate::paths::Paths) -> usize {
        std::fs::read_dir(paths.cache_dir().join("tree-digest"))
            .map(|d| d.count())
            .unwrap_or(0)
    }

    #[test]
    fn cached_digest_equals_full_digest_and_is_remembered() {
        let (_home, paths) = cache_home();
        let tree = tempfile::tempdir().unwrap();
        aged_bundle(tree.path());
        let full = tree_digest(tree.path()).unwrap();
        assert_eq!(tree_digest_cached(&paths, tree.path()).unwrap(), full);
        assert_eq!(cache_files(&paths), 1);
        assert_eq!(tree_digest_cached(&paths, tree.path()).unwrap(), full);
    }

    /// Proof that a matching fingerprint is answered from memory and the files
    /// are not read again: plant a sentinel under the live fingerprint.
    #[test]
    fn cache_hit_is_answered_from_memory() {
        let (_home, paths) = cache_home();
        let tree = tempfile::tempdir().unwrap();
        aged_bundle(tree.path());
        tree_digest_cached(&paths, tree.path()).unwrap();
        let cache = digest_cache_path(&paths, &tree.path().canonicalize().unwrap());
        let mut entry: DigestCacheEntry =
            serde_json::from_slice(&std::fs::read(&cache).unwrap()).unwrap();
        entry.digest = "sha256:remembered".into();
        std::fs::write(&cache, serde_json::to_vec(&entry).unwrap()).unwrap();
        assert_eq!(
            tree_digest_cached(&paths, tree.path()).unwrap(),
            "sha256:remembered"
        );
    }

    #[test]
    fn edited_file_invalidates_the_cache() {
        let (_home, paths) = cache_home();
        let tree = tempfile::tempdir().unwrap();
        aged_bundle(tree.path());
        let before = tree_digest_cached(&paths, tree.path()).unwrap();
        std::fs::write(tree.path().join("a.txt"), "changed and longer").unwrap();
        let after = tree_digest_cached(&paths, tree.path()).unwrap();
        assert_ne!(before, after);
        assert_eq!(after, tree_digest(tree.path()).unwrap());
    }

    #[test]
    fn same_size_edit_with_new_mtime_invalidates_the_cache() {
        let (_home, paths) = cache_home();
        let tree = tempfile::tempdir().unwrap();
        aged_bundle(tree.path());
        let before = tree_digest_cached(&paths, tree.path()).unwrap();
        let file = tree.path().join("a.txt");
        std::fs::write(&file, "ONE").unwrap();
        set_mtime(&file, old_mtime() + std::time::Duration::from_secs(60));
        let after = tree_digest_cached(&paths, tree.path()).unwrap();
        assert_ne!(before, after);
        assert_eq!(after, tree_digest(tree.path()).unwrap());
    }

    #[test]
    fn added_and_removed_files_invalidate_the_cache() {
        let (_home, paths) = cache_home();
        let tree = tempfile::tempdir().unwrap();
        aged_bundle(tree.path());
        let before = tree_digest_cached(&paths, tree.path()).unwrap();
        let extra = tree.path().join("extra.txt");
        std::fs::write(&extra, "x").unwrap();
        set_mtime(&extra, old_mtime());
        let added = tree_digest_cached(&paths, tree.path()).unwrap();
        assert_ne!(before, added);
        assert_eq!(added, tree_digest(tree.path()).unwrap());
        std::fs::remove_file(&extra).unwrap();
        let removed = tree_digest_cached(&paths, tree.path()).unwrap();
        assert_eq!(removed, before);
        std::fs::remove_file(tree.path().join("sub/b.txt")).unwrap();
        let fewer = tree_digest_cached(&paths, tree.path()).unwrap();
        assert_ne!(fewer, before);
        assert_eq!(fewer, tree_digest(tree.path()).unwrap());
    }

    #[test]
    fn corrupt_or_foreign_cache_falls_back_to_the_full_hash() {
        let (_home, paths) = cache_home();
        let tree = tempfile::tempdir().unwrap();
        aged_bundle(tree.path());
        let full = tree_digest(tree.path()).unwrap();
        tree_digest_cached(&paths, tree.path()).unwrap();
        let canonical = tree.path().canonicalize().unwrap();
        let cache = digest_cache_path(&paths, &canonical);
        std::fs::write(&cache, b"{ not json").unwrap();
        assert_eq!(tree_digest_cached(&paths, tree.path()).unwrap(), full);
        // The fallback refreshed the memory.
        assert!(
            serde_json::from_slice::<DigestCacheEntry>(&std::fs::read(&cache).unwrap()).is_ok()
        );
        // A well-formed entry for another root, or a lying digest under a
        // fingerprint that cannot match, is not believed.
        std::fs::write(
            &cache,
            serde_json::to_vec(&DigestCacheEntry {
                root: "/elsewhere".into(),
                fingerprint: "x".into(),
                digest: "sha256:deadbeef".into(),
            })
            .unwrap(),
        )
        .unwrap();
        assert_eq!(tree_digest_cached(&paths, tree.path()).unwrap(), full);
    }

    #[test]
    fn recently_written_trees_are_not_remembered() {
        let (_home, paths) = cache_home();
        let tree = tempfile::tempdir().unwrap();
        std::fs::write(tree.path().join("a.txt"), "fresh").unwrap();
        let digest = tree_digest_cached(&paths, tree.path()).unwrap();
        assert_eq!(digest, tree_digest(tree.path()).unwrap());
        assert_eq!(cache_files(&paths), 0);
    }

    #[test]
    fn install_metadata_changes_do_not_invalidate_the_cache() {
        let (_home, paths) = cache_home();
        let tree = tempfile::tempdir().unwrap();
        aged_bundle(tree.path());
        let before = tree_digest_cached(&paths, tree.path()).unwrap();
        std::fs::write(tree.path().join(super::super::provenance::FILE), b"receipt").unwrap();
        assert_eq!(tree_digest_cached(&paths, tree.path()).unwrap(), before);
    }

    /// Reading is spread over threads and batches; the digest must still be the
    /// plain sorted, one-file-at-a-time definition.
    #[test]
    fn parallel_reading_gives_the_sequential_digest() {
        let tmp = tempfile::tempdir().unwrap();
        let mut names = Vec::new();
        for i in 0..(READ_BATCH * 2 + 17) {
            let name = format!("d{}/f{i:04}.txt", i % 7);
            let path = tmp.path().join(&name);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, format!("content {i}").repeat(i % 5 + 1)).unwrap();
            names.push(name);
        }
        names.sort_by(|a, b| a.as_bytes().cmp(b.as_bytes()));
        let mut h = Sha256::new();
        h.update(DOMAIN);
        for name in &names {
            let bytes = std::fs::read(tmp.path().join(name)).unwrap();
            h.update((name.len() as u64).to_be_bytes());
            h.update(name.as_bytes());
            h.update((bytes.len() as u64).to_be_bytes());
            h.update(&bytes);
        }
        assert_eq!(
            tree_digest(tmp.path()).unwrap(),
            format!("sha256:{:x}", h.finalize())
        );
    }

    #[test]
    fn parallel_reading_reports_an_unreadable_file() {
        let tmp = tempfile::tempdir().unwrap();
        let gone = tmp.path().join("gone.txt");
        let batch = vec![("gone.txt".to_string(), gone)];
        assert!(read_batch(&batch, 4).is_err());
        let two = vec![
            ("a".to_string(), tmp.path().join("a")),
            ("b".to_string(), tmp.path().join("b")),
        ];
        assert!(read_batch(&two, 4).is_err());
    }

    #[test]
    fn a_recreated_tree_with_the_same_size_and_mtime_is_not_confused_with_the_old_one() {
        let (_home, paths) = cache_home();
        let parent = tempfile::tempdir().unwrap();
        let live = parent.path().join("agent");
        aged_bundle(&live);
        let before = tree_digest_cached(&paths, &live).unwrap();
        // An update: a same-length edit shipped with the same timestamps,
        // built beside the old folder and swapped into its path.
        std::thread::sleep(std::time::Duration::from_millis(50));
        let next = parent.path().join("agent-next");
        aged_bundle(&next);
        let edited = next.join("a.txt");
        std::fs::write(&edited, "ONE").unwrap(); // same length as "one"
        set_mtime(&edited, old_mtime());
        std::fs::remove_dir_all(&live).unwrap();
        std::fs::rename(&next, &live).unwrap();
        let after = tree_digest_cached(&paths, &live).unwrap();
        assert_ne!(before, after);
        assert_eq!(after, tree_digest(&live).unwrap());
    }

    #[test]
    fn pruning_forgets_trees_that_are_gone_and_keeps_live_ones() {
        let (_home, paths) = cache_home();
        let keep = tempfile::tempdir().unwrap();
        let gone = tempfile::tempdir().unwrap();
        aged_bundle(keep.path());
        aged_bundle(gone.path());
        tree_digest_cached(&paths, keep.path()).unwrap();
        tree_digest_cached(&paths, gone.path()).unwrap();
        assert_eq!(cache_files(&paths), 2);
        drop(gone);
        prune_digest_cache(&paths);
        assert_eq!(cache_files(&paths), 1);
        assert_eq!(
            tree_digest_cached(&paths, keep.path()).unwrap(),
            tree_digest(keep.path()).unwrap()
        );
    }
}
