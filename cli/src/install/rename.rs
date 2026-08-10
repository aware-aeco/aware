//! Rename / duplicate an installed app (aware-aeco#226).
//!
//! `aware app` had `install` / `uninstall` but no first-class `rename` or
//! `duplicate`, forcing an external tool (e.g. floless.app) to reach into
//! `~/.aware/apps/<id>/` directly: move the dir, rewrite the source's `app:`
//! field, drop the stale lock, recompile — and to do all of that AGAIN for the
//! synthesized agent of an `exposes-as-agent` app, which a bare directory move
//! orphans. That is a lot of substrate-internal knowledge to carry, it is
//! fragile across versions, and it cannot be done correctly for a baked app. So
//! the substrate owns the dance here, and tools call one verb.
//!
//! An app's identity is its top-level `app:` field, stamped in BOTH the source
//! and the compiled `<id>.lock`, and it lives in `apps/<id>/` (dir name == `app:`
//! field by convention; that is how the by-id verbs resolve it).
//!
//! Both verbs **stage a fresh `apps/<new>/` first** — copy, rewrite the `app:`
//! field, rename the source to `<new>.<ext>`, regenerate the lock so its
//! `source-hash` matches the renamed bytes (no drift — the Run gate stays green),
//! and for a baked app synthesize the agent under `agents/<new>/` — and only then
//! does `rename` remove the original (`duplicate` keeps it). So a failure while
//! building the new copy leaves the original completely untouched; the partial
//! copy is cleaned up. The source id is resolved strictly by directory name (a
//! destructive move should act on exactly the directory named); a desynced app
//! is re-synced with `aware app rename <dir-name> <app-field>`.

use std::path::{Path, PathBuf};

use crate::error::AwareError;
use crate::fs_tree::copy_dir_recursive;
use crate::install::local::{is_app_backed_agent, write_app_lockfile, write_synthesized_agent};
use crate::manifest::loader::{find_app_manifest, is_safe_segment, load_app};
use crate::paths::Paths;

/// What happened to the compiled `<id>.lock` during a rename/duplicate, so the
/// caller can tell the user whether the app is still ready to run.
#[derive(Debug, PartialEq, Eq)]
pub enum LockOutcome {
    /// The app had a compiled lock and it was regenerated — still ready to run.
    Refreshed,
    /// The app had no compiled lock; none was created (its "not yet compiled"
    /// state is preserved).
    None,
    /// The app had a lock but recompiling it failed for a benign reason (e.g. it
    /// now references an uninstalled agent) — `aware app compile` is needed. A
    /// non-benign (I/O) failure is NOT this; it aborts the whole op instead.
    NeedsRefresh,
}

/// Outcome of a rename/duplicate: the resulting app id and what became of its lock.
#[derive(Debug)]
pub struct AppMoveOutcome {
    pub id: String,
    pub lock: LockOutcome,
}

/// Windows reserved device names. The app id becomes a directory name, so an id
/// that collides with one of these is unusable on Windows even though it passes
/// the slug charset — reject it up front rather than create an unopenable dir.
const RESERVED_NAMES: &[&str] = &[
    "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
    "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

/// Validate a candidate NEW app id: the slug charset installed apps already use
/// (`[A-Za-z0-9._-]`, first char alphanumeric — which also rejects `.`/`..` and
/// any leading-dot name), no trailing dot (Windows strips it, aliasing `foo.`
/// onto `foo`), no path separators, and not a Windows reserved device name.
/// Mirrors the `APP_ID` charset on the floless side; the first-char rule + charset
/// fence out directory traversal.
fn validate_app_id(id: &str) -> Result<(), AwareError> {
    let charset_ok = !id.is_empty()
        && !id.ends_with('.')
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
        && id.chars().next().is_some_and(|c| c.is_ascii_alphanumeric());
    if !charset_ok {
        return Err(AwareError::Validation(format!(
            "invalid app name {id:?}: use letters, digits, dash, underscore or dot, starting with a letter or digit"
        )));
    }
    // Compare the pre-extension stem (e.g. `nul.flo` → `NUL`) case-insensitively.
    let stem = id.split('.').next().unwrap_or(id).to_ascii_uppercase();
    if RESERVED_NAMES.contains(&stem.as_str()) {
        return Err(AwareError::Validation(format!(
            "invalid app name {id:?}: reserved on Windows"
        )));
    }
    Ok(())
}

/// Rewrite the top-level `app:` scalar in a source file's text to `new_id`,
/// preserving line endings, indentation-free key formatting, the existing quote
/// style, and any trailing inline comment. ONLY a line with no leading
/// whitespace whose key is exactly `app` is matched, so an indented/nested
/// `app:` inside a node's config is left untouched. Errors if no top-level
/// `app:` line is present (computed before any caller writes to disk).
fn rewrite_app_field(text: &str, new_id: &str) -> Result<String, AwareError> {
    let mut out = String::with_capacity(text.len() + new_id.len());
    let mut replaced = false;
    for segment in text.split_inclusive('\n') {
        if replaced {
            out.push_str(segment);
            continue;
        }
        // Split the logical line from its trailing newline (LF or CRLF), so the
        // original ending is preserved exactly on the rewritten line.
        let (content, eol) = match segment.strip_suffix('\n') {
            Some(rest) => match rest.strip_suffix('\r') {
                Some(r) => (r, "\r\n"),
                None => (rest, "\n"),
            },
            None => (segment, ""),
        };
        match rewrite_app_line(content, new_id) {
            Some(rewritten) => {
                out.push_str(&rewritten);
                out.push_str(eol);
                replaced = true;
            }
            None => out.push_str(segment),
        }
    }
    if !replaced {
        return Err(AwareError::Validation(
            "source has no top-level `app:` field to rename".into(),
        ));
    }
    Ok(out)
}

/// If `line` is a top-level `app:` declaration, return it with the value swapped
/// for `new_id` (keeping the key, the spacing after the colon, the quote style,
/// and any inline comment); otherwise `None`.
fn rewrite_app_line(line: &str, new_id: &str) -> Option<String> {
    if line.starts_with([' ', '\t']) {
        return None; // indented → nested key, not the top-level identity
    }
    let rest = line.strip_prefix("app:")?;
    // Preserve the whitespace between the colon and the value.
    let ws_len = rest.len() - rest.trim_start().len();
    let (lead_ws, after) = rest.split_at(ws_len);
    // Split a trailing inline comment (a `#` preceded by whitespace) from the value.
    let (value_part, comment) = split_inline_comment(after);
    let trimmed_val = value_part.trim_end();
    let trailing_ws = &value_part[trimmed_val.len()..];
    // Keep the existing quote style, if any.
    let quote = trimmed_val
        .chars()
        .next()
        .filter(|c| *c == '"' || *c == '\'')
        .map(|c| c.to_string())
        .unwrap_or_default();
    Some(format!(
        "app:{lead_ws}{quote}{new_id}{quote}{trailing_ws}{comment}"
    ))
}

/// Split `s` into (value, inline-comment) at the first `#` that begins a YAML
/// comment (preceded by whitespace, or at the very start). The comment retains
/// its leading `#`. A value with no comment returns `(s, "")`.
fn split_inline_comment(s: &str) -> (&str, &str) {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'#' && (i == 0 || bytes[i - 1] == b' ' || bytes[i - 1] == b'\t') {
            return (&s[..i], &s[i..]);
        }
    }
    (s, "")
}

/// Restamp an app directory IN PLACE to `new_id`: rewrite the source `app:`
/// field, rename the source file to `<new_id>.<ext>`, and remove every stale
/// lock artifact (`*.lock` + the legacy `lockfile.yaml`). Returns the new source
/// path and whether a compiled `*.lock` was present beforehand (so the caller
/// only regenerates a lock for an app that already had one). The text rewrite is
/// computed before any write, so a missing `app:` field fails without having
/// mutated the directory. Best-effort removals warn (rather than fail) so a
/// lingering stale source / lock is at least diagnosable.
fn restamp_dir(dir: &Path, new_id: &str) -> Result<(PathBuf, bool), AwareError> {
    let source = find_app_manifest(dir).ok_or_else(|| {
        AwareError::Validation(format!("app dir {} has no .flo/.app source", dir.display()))
    })?;
    let text = std::fs::read_to_string(&source)?;
    let rewritten = rewrite_app_field(&text, new_id)?; // errors before any write

    let ext = source.extension().and_then(|e| e.to_str()).unwrap_or("flo");
    let new_source = dir.join(format!("{new_id}.{ext}"));
    std::fs::write(&new_source, rewritten)?;
    if new_source != source
        && let Err(e) = std::fs::remove_file(&source)
    {
        // The canonical-name lookup already prefers `<dir>.<ext>`, so a leftover
        // old-named source is ignored — but warn so it's diagnosable.
        eprintln!(
            "warning: wrote {}, but removing the old source {} failed ({e})",
            new_source.display(),
            source.display()
        );
    }

    // Drop stale lock artifacts — a `*.lock` carries the old id and a now-wrong
    // source-hash; `lockfile.yaml` carries the old id. Both are regenerated by
    // the caller. Only a `*.lock` counts as "was compiled".
    let mut had_compiled_lock = false;
    if let Ok(read) = std::fs::read_dir(dir) {
        for entry in read.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            let is_lock = name.ends_with(".lock");
            if is_lock || name == "lockfile.yaml" {
                had_compiled_lock |= is_lock;
                if let Err(e) = std::fs::remove_file(entry.path()) {
                    eprintln!("warning: could not remove stale {name} ({e})");
                }
            }
        }
    }
    Ok((new_source, had_compiled_lock))
}

/// Regenerate the install-time `lockfile.yaml` and, when the app already had a
/// compiled `<id>.lock`, recompile it so a renamed/duplicated app's on-disk shape
/// matches a freshly installed-and-compiled one. A recompile failure is split:
/// a `Validation` error (the app references a now-uninstalled agent) is the honest
/// "needs refresh" state and degrades to `NeedsRefresh`; any other error (I/O,
/// serialize) means the move could not be completed cleanly and is propagated so
/// the caller unwinds — never a silent lockless app reported as success.
fn refresh_locks(
    new_source: &Path,
    dir: &Path,
    had_compiled_lock: bool,
    paths: &Paths,
) -> Result<LockOutcome, AwareError> {
    if let Ok(app) = load_app(new_source)
        && let Err(e) = write_app_lockfile(&app, dir, paths)
    {
        eprintln!("warning: app moved, but rewriting lockfile.yaml failed ({e})");
    }
    if !had_compiled_lock {
        return Ok(LockOutcome::None);
    }
    match crate::app_lock::compile_to_disk(new_source, paths) {
        Ok(_) => Ok(LockOutcome::Refreshed),
        Err(e @ AwareError::Validation(_)) => {
            eprintln!(
                "warning: app moved, but recompiling its lock failed ({e}); run `aware app compile` to refresh"
            );
            Ok(LockOutcome::NeedsRefresh)
        }
        Err(e) => Err(e),
    }
}

/// A baked app registers a synthesized agent at `agents/<new_id>/`. Refuse if a
/// REAL (not app-backed-by-`new_id`) agent already holds that name; an orphaned
/// app-backed agent of the same name is reclaimable (overwritten), mirroring
/// `install`'s guard. Runs before any directory is created, so we never half-stage.
fn assert_agent_name_free(new_id: &str, paths: &Paths) -> Result<(), AwareError> {
    let new_agent = paths.agents_dir().join(new_id);
    if new_agent.exists() && !is_app_backed_agent(&new_agent, new_id) {
        return Err(AwareError::Conflict(format!(
            "an agent named {new_id} is already installed"
        )));
    }
    Ok(())
}

/// Build a fresh `apps/<new_id>/` from `src_dir`: copy, restamp to `new_id`,
/// synthesize the agent for a baked app, regenerate the locks. On ANY failure the
/// partial new directory (and the synth agent, if we created it) is removed, so
/// `src_dir` is never left in a half-state. `agent_preexisted` tells cleanup not
/// to delete a synth agent that was already there before we started.
fn stage_app(
    src_dir: &Path,
    new_dir: &Path,
    new_id: &str,
    agent_preexisted: bool,
    paths: &Paths,
) -> Result<LockOutcome, AwareError> {
    let staged = stage_app_inner(src_dir, new_dir, new_id, paths);
    if staged.is_err() {
        let _ = std::fs::remove_dir_all(new_dir);
        if !agent_preexisted {
            let _ = std::fs::remove_dir_all(paths.agents_dir().join(new_id));
        }
    }
    staged
}

fn stage_app_inner(
    src_dir: &Path,
    new_dir: &Path,
    new_id: &str,
    paths: &Paths,
) -> Result<LockOutcome, AwareError> {
    copy_dir_recursive(src_dir, new_dir).map_err(AwareError::Io)?;
    let (new_source, had_lock) = restamp_dir(new_dir, new_id)?;
    let app = load_app(&new_source)?;
    if app.exposes_as_agent {
        write_synthesized_agent(&app, paths)?;
    }
    refresh_locks(&new_source, new_dir, had_lock, paths)
}

/// Rename an installed app `old_id` → `new_id`. Stages a fresh `apps/<new_id>/`
/// (original untouched), then removes the original on success — so a failure
/// mid-build never corrupts the original.
pub fn rename_app(old_id: &str, new_id: &str, paths: &Paths) -> Result<AppMoveOutcome, AwareError> {
    if !is_safe_segment(old_id) {
        return Err(AwareError::NotFound(format!("app: {old_id}")));
    }
    let old_dir = paths.apps_dir().join(old_id);
    if !old_dir.is_dir() {
        return Err(AwareError::NotFound(format!("app: {old_id}")));
    }
    validate_app_id(new_id)?;
    if new_id == old_id {
        return Err(AwareError::Validation(format!(
            "app is already named {old_id:?}"
        )));
    }
    let new_dir = paths.apps_dir().join(new_id);
    if new_dir.exists() {
        return Err(AwareError::Conflict(format!(
            "an app named {new_id} already exists"
        )));
    }
    let old_source = find_app_manifest(&old_dir)
        .ok_or_else(|| AwareError::Validation(format!("app {old_id} has no .flo/.app source")))?;
    let exposes = load_app(&old_source)?.exposes_as_agent;
    let agent_preexisted = paths.agents_dir().join(new_id).exists();
    if exposes {
        assert_agent_name_free(new_id, paths)?;
    }

    let lock = stage_app(&old_dir, &new_dir, new_id, agent_preexisted, paths)?;

    // The new app is fully built and runnable — commit by removing the original.
    // A failure here leaves BOTH (recoverable), so warn with the fix rather than
    // unwind a perfectly good new app.
    if let Err(e) = std::fs::remove_dir_all(&old_dir) {
        eprintln!(
            "warning: created {new_id}, but removing the old app {old_id} failed (is it running?) — remove it with `aware app uninstall {old_id}` ({e})"
        );
    }
    if exposes {
        let old_agent = paths.agents_dir().join(old_id);
        if old_agent.exists()
            && is_app_backed_agent(&old_agent, old_id)
            && let Err(e) = std::fs::remove_dir_all(&old_agent)
        {
            eprintln!(
                "warning: renamed the agent, but removing the old synthesized agent {old_id} failed — remove it with `aware agent uninstall {old_id}` ({e})"
            );
        }
    }
    Ok(AppMoveOutcome {
        id: new_id.to_string(),
        lock,
    })
}

/// Duplicate an installed app `src_id` → `new_id` as an independent copy. The
/// original is left entirely untouched.
pub fn duplicate_app(
    src_id: &str,
    new_id: &str,
    paths: &Paths,
) -> Result<AppMoveOutcome, AwareError> {
    if !is_safe_segment(src_id) {
        return Err(AwareError::NotFound(format!("app: {src_id}")));
    }
    let src_dir = paths.apps_dir().join(src_id);
    if !src_dir.is_dir() {
        return Err(AwareError::NotFound(format!("app: {src_id}")));
    }
    validate_app_id(new_id)?;
    let new_dir = paths.apps_dir().join(new_id);
    if new_dir.exists() {
        return Err(AwareError::Conflict(format!(
            "an app named {new_id} already exists"
        )));
    }
    let src_source = find_app_manifest(&src_dir)
        .ok_or_else(|| AwareError::Validation(format!("app {src_id} has no .flo/.app source")))?;
    let exposes = load_app(&src_source)?.exposes_as_agent;
    let agent_preexisted = paths.agents_dir().join(new_id).exists();
    if exposes {
        assert_agent_name_free(new_id, paths)?;
    }

    // Stage the copy; the original is intentionally never touched (no commit step).
    let lock = stage_app(&src_dir, &new_dir, new_id, agent_preexisted, paths)?;
    Ok(AppMoveOutcome {
        id: new_id.to_string(),
        lock,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn paths_in(tmp: &Path) -> Paths {
        Paths {
            aware_home: tmp.to_path_buf(),
        }
    }

    /// Install a minimal app `id` directly on disk (source + a stand-in compiled
    /// `<id>.lock`). `exposes` adds an exposes-as-agent block and synthesizes the
    /// agent, mirroring an `aware app install`.
    fn seed_app(paths: &Paths, id: &str, exposes: bool) {
        let dir = paths.apps_dir().join(id);
        std::fs::create_dir_all(&dir).unwrap();
        let body = if exposes {
            format!(
                "app: {id}\nversion: 0.1.0\ndescription: a test app\nexposes-as-agent: true\n\
                 exposed-commands:\n  run:\n    lifecycle: single\n    inputs:\n      phase:\n        type: string\n\
                 nodes:\n  - id: gate\n    inline:\n      kind: predicate\n      description: pass\n      code: 'true'\nrequires: []\n"
            )
        } else {
            format!(
                "app: {id}\nversion: 0.1.0\ndescription: a test app\n\
                 nodes:\n  - id: gate\n    inline:\n      kind: predicate\n      description: pass\n      code: 'true'\nrequires: []\n"
            )
        };
        std::fs::write(dir.join(format!("{id}.flo")), body).unwrap();
        std::fs::write(
            dir.join(format!("{id}.lock")),
            "source-hash: sha256:stale\n",
        )
        .unwrap();
        if exposes {
            let app = load_app(&dir.join(format!("{id}.flo"))).unwrap();
            write_synthesized_agent(&app, paths).unwrap();
        }
    }

    /// Write a REAL (non-app-backed, cli-transport) agent that merely shares a name.
    fn seed_real_agent(paths: &Paths, id: &str) {
        let dir = paths.agents_dir().join(id);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("manifest.yaml"),
            format!(
                "agent: {id}\nversion: 1.0\ndescription: real\nstateful: false\nlicense: MIT\n\
                 transport: {{ cli: {{ binary: aware-{id} }} }}\ncommands: {{ go: {{ lifecycle: single, description: x }} }}\n"
            ),
        )
        .unwrap();
    }

    fn read_app_field(paths: &Paths, id: &str) -> String {
        let src = find_app_manifest(&paths.apps_dir().join(id)).unwrap();
        load_app(&src).unwrap().app
    }

    #[test]
    fn rename_moves_dir_and_rewrites_field_and_regenerates_lock() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_in(tmp.path());
        seed_app(&paths, "old-name", false);

        let out = rename_app("old-name", "new-name", &paths).unwrap();
        assert_eq!(out.id, "new-name");
        assert_eq!(out.lock, LockOutcome::Refreshed);

        assert!(!paths.apps_dir().join("old-name").exists());
        assert!(paths.apps_dir().join("new-name/new-name.flo").is_file());
        assert!(!paths.apps_dir().join("new-name/old-name.flo").exists());
        assert_eq!(read_app_field(&paths, "new-name"), "new-name");
        let lock =
            std::fs::read_to_string(paths.apps_dir().join("new-name/new-name.lock")).unwrap();
        assert!(
            lock.contains("app: new-name"),
            "lock not regenerated: {lock}"
        );
        assert!(
            !lock.contains("sha256:stale"),
            "stale lock survived: {lock}"
        );
    }

    #[test]
    fn rename_without_prior_lock_does_not_force_compile() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_in(tmp.path());
        seed_app(&paths, "src", false);
        std::fs::remove_file(paths.apps_dir().join("src/src.lock")).unwrap();

        let out = rename_app("src", "dst", &paths).unwrap();
        assert_eq!(out.lock, LockOutcome::None);
        assert!(!paths.apps_dir().join("dst/dst.lock").exists());
    }

    #[test]
    fn rename_relocates_synthesized_agent_for_baked_app() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_in(tmp.path());
        seed_app(&paths, "baked", true);
        assert!(paths.agents_dir().join("baked/manifest.yaml").is_file());

        rename_app("baked", "rebaked", &paths).unwrap();

        assert!(
            !paths.agents_dir().join("baked").exists(),
            "old synth agent must be removed"
        );
        let new_manifest = paths.agents_dir().join("rebaked/manifest.yaml");
        assert!(new_manifest.is_file(), "new synth agent must be written");
        let agent = crate::manifest::loader::load_agent(&new_manifest).unwrap();
        assert_eq!(agent.agent, "rebaked");
        assert_eq!(agent.transport.app.unwrap().backed_by, "rebaked");
    }

    #[test]
    fn rename_rejects_existing_target() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_in(tmp.path());
        seed_app(&paths, "a", false);
        seed_app(&paths, "b", false);
        let err = rename_app("a", "b", &paths).unwrap_err();
        assert!(matches!(err, AwareError::Conflict(_)), "got {err:?}");
        assert!(paths.apps_dir().join("a/a.flo").is_file());
        assert!(paths.apps_dir().join("b/b.flo").is_file());
    }

    #[test]
    fn rename_onto_a_real_agent_name_conflicts_and_changes_nothing() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_in(tmp.path());
        seed_app(&paths, "baked", true);
        seed_real_agent(&paths, "taken");

        let err = rename_app("baked", "taken", &paths).unwrap_err();
        assert!(matches!(err, AwareError::Conflict(_)), "got {err:?}");
        // Nothing moved: the source app, its agent, and the real agent are intact.
        assert!(paths.apps_dir().join("baked/baked.flo").is_file());
        assert!(paths.agents_dir().join("baked/manifest.yaml").is_file());
        assert!(!paths.apps_dir().join("taken").exists());
        assert!(paths.agents_dir().join("taken/manifest.yaml").is_file());
    }

    #[test]
    fn rename_missing_app_is_not_found() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_in(tmp.path());
        let err = rename_app("nope", "new", &paths).unwrap_err();
        assert!(matches!(err, AwareError::NotFound(_)), "got {err:?}");
    }

    #[test]
    fn rename_to_same_name_errors() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_in(tmp.path());
        seed_app(&paths, "same", false);
        let err = rename_app("same", "same", &paths).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)), "got {err:?}");
    }

    #[test]
    fn rename_rejects_invalid_new_id() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_in(tmp.path());
        seed_app(&paths, "ok", false);
        for bad in [
            "../escape",
            "has space",
            ".hidden",
            "nul",
            "a/b",
            "trailingdot.",
        ] {
            let err = rename_app("ok", bad, &paths).unwrap_err();
            assert!(
                matches!(err, AwareError::Validation(_)),
                "expected Validation for {bad:?}, got {err:?}"
            );
        }
        assert!(paths.apps_dir().join("ok/ok.flo").is_file());
    }

    #[test]
    fn rename_rejects_unsafe_source_id() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_in(tmp.path());
        seed_app(&paths, "ok", false);
        for bad in ["../ok", "a/b", "..", "a\\b"] {
            let err = rename_app(bad, "new", &paths).unwrap_err();
            assert!(
                matches!(err, AwareError::NotFound(_)),
                "expected NotFound for source {bad:?}, got {err:?}"
            );
        }
        assert!(paths.apps_dir().join("ok/ok.flo").is_file());
    }

    #[test]
    fn duplicate_leaves_original_intact_and_is_independent() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_in(tmp.path());
        seed_app(&paths, "orig", false);

        let out = duplicate_app("orig", "copy", &paths).unwrap();
        assert_eq!(out.id, "copy");
        assert_eq!(out.lock, LockOutcome::Refreshed);

        assert!(paths.apps_dir().join("orig/orig.flo").is_file());
        assert_eq!(read_app_field(&paths, "orig"), "orig");
        assert!(paths.apps_dir().join("copy/copy.flo").is_file());
        assert_eq!(read_app_field(&paths, "copy"), "copy");
        assert!(paths.apps_dir().join("copy/copy.lock").is_file());
    }

    #[test]
    fn duplicate_baked_app_creates_new_agent_and_keeps_the_original() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_in(tmp.path());
        seed_app(&paths, "baked", true);

        duplicate_app("baked", "baked-copy", &paths).unwrap();

        assert!(paths.agents_dir().join("baked/manifest.yaml").is_file());
        let copy_manifest = paths.agents_dir().join("baked-copy/manifest.yaml");
        assert!(copy_manifest.is_file());
        let agent = crate::manifest::loader::load_agent(&copy_manifest).unwrap();
        assert_eq!(agent.transport.app.unwrap().backed_by, "baked-copy");
    }

    #[test]
    fn duplicate_onto_a_real_agent_name_conflicts_and_changes_nothing() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_in(tmp.path());
        seed_app(&paths, "baked", true);
        seed_real_agent(&paths, "taken");

        let err = duplicate_app("baked", "taken", &paths).unwrap_err();
        assert!(matches!(err, AwareError::Conflict(_)), "got {err:?}");
        assert!(!paths.apps_dir().join("taken").exists());
        assert!(paths.agents_dir().join("taken/manifest.yaml").is_file());
        // The source is untouched.
        assert!(paths.apps_dir().join("baked/baked.flo").is_file());
    }

    #[test]
    fn rewrite_app_field_preserves_quotes_and_inline_comment() {
        assert_eq!(
            rewrite_app_field("app: old\nversion: 1\n", "new").unwrap(),
            "app: new\nversion: 1\n"
        );
        assert_eq!(
            rewrite_app_field("app: 'old'  # id\n", "new").unwrap(),
            "app: 'new'  # id\n"
        );
        assert_eq!(
            rewrite_app_field("app: \"old\"\n", "new").unwrap(),
            "app: \"new\"\n"
        );
        // CRLF line endings are preserved.
        assert_eq!(
            rewrite_app_field("app: old\r\nx: 1\r\n", "new").unwrap(),
            "app: new\r\nx: 1\r\n"
        );
    }

    #[test]
    fn rewrite_app_field_ignores_nested_app_keys() {
        let src = "app: real\nnodes:\n  - id: n\n    config:\n      app: nested-should-stay\n";
        let out = rewrite_app_field(src, "renamed").unwrap();
        assert!(out.contains("app: renamed\n"));
        assert!(out.contains("      app: nested-should-stay\n"));
    }

    #[test]
    fn rewrite_app_field_errors_without_top_level_field() {
        let err = rewrite_app_field("version: 1\ndescription: x\n", "new").unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }

    #[test]
    fn validate_app_id_accepts_slugs_and_rejects_unsafe() {
        for ok in ["hello", "hello-world", "a.b_c-2", "App2"] {
            assert!(validate_app_id(ok).is_ok(), "{ok} should be valid");
        }
        for bad in [
            "",
            "..",
            "../x",
            "a/b",
            "a\\b",
            ".hidden",
            "-leading",
            "has space",
            "nul",
            "COM1",
            "trailing.",
        ] {
            assert!(validate_app_id(bad).is_err(), "{bad} should be invalid");
        }
    }
}
