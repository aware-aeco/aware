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
//! field by convention; that is how the by-id verbs resolve it). Renaming
//! therefore: moves the dir, rewrites the field, renames the source file to
//! `<new>.<ext>`, regenerates the lock so its `source-hash` matches the renamed
//! bytes (no drift — the Run gate stays green), and — for a baked app —
//! regenerates the synthesized agent under `agents/<new>/` and removes the old
//! one. Duplicate does the same on a copy, leaving the original untouched.

use std::path::{Path, PathBuf};

use crate::error::AwareError;
use crate::install::local::{
    copy_dir_recursive, is_app_backed_agent, write_app_lockfile, write_synthesized_agent,
};
use crate::manifest::loader::{find_app_manifest, load_app};
use crate::paths::Paths;

/// Outcome of a rename/duplicate: the resulting app id and whether a fresh
/// compiled `<id>.lock` was produced (so the caller can report
/// runnable-vs-needs-compile without re-reading the directory).
#[derive(Debug)]
pub struct AppMoveOutcome {
    pub id: String,
    pub compiled: bool,
}

/// Windows reserved device names. The app id becomes a directory name, so an id
/// that collides with one of these is unusable on Windows even though it passes
/// the slug charset — reject it up front rather than create an unopenable dir.
const RESERVED_NAMES: &[&str] = &[
    "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
    "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
];

/// Validate a candidate app id: the slug charset installed apps already use
/// (`[A-Za-z0-9._-]`, first char alphanumeric — which also rejects `.`/`..` and
/// any leading-dot name), no path separators, and not a Windows reserved device
/// name. Mirrors the `APP_ID` charset on the floless side; the first-char rule +
/// charset is what fences out directory traversal.
fn validate_app_id(id: &str) -> Result<(), AwareError> {
    let charset_ok = !id.is_empty()
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
/// computed before any write, so a missing `app:` field fails the whole op
/// without having mutated the directory.
fn restamp_dir(dir: &Path, new_id: &str) -> Result<(PathBuf, bool), AwareError> {
    let source = find_app_manifest(dir).ok_or_else(|| {
        AwareError::Validation(format!("app dir {} has no .flo/.app source", dir.display()))
    })?;
    let text = std::fs::read_to_string(&source)?;
    let rewritten = rewrite_app_field(&text, new_id)?; // errors before any write

    let ext = source.extension().and_then(|e| e.to_str()).unwrap_or("flo");
    let new_source = dir.join(format!("{new_id}.{ext}"));
    std::fs::write(&new_source, rewritten)?;
    if new_source != source {
        // Best-effort: the canonical-name lookup already prefers `<dir>.<ext>`,
        // so a lingering old-named source would be ignored — but drop it anyway.
        let _ = std::fs::remove_file(&source);
    }

    // Drop stale lock artifacts — a `*.lock` carries the old id and a now-wrong
    // source-hash; `lockfile.yaml` carries the old id. Both are regenerated by
    // the caller. Only a `*.lock` counts as "was compiled".
    let mut had_compiled_lock = false;
    if let Ok(read) = std::fs::read_dir(dir) {
        for entry in read.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.ends_with(".lock") {
                had_compiled_lock = true;
                let _ = std::fs::remove_file(entry.path());
            } else if name == "lockfile.yaml" {
                let _ = std::fs::remove_file(entry.path());
            }
        }
    }
    Ok((new_source, had_compiled_lock))
}

/// Regenerate the install-time `lockfile.yaml` and, when the app already had a
/// compiled `<id>.lock`, recompile it so a renamed/duplicated app's on-disk
/// shape matches a freshly installed-and-compiled one. Recompile is best-effort:
/// an app that references a now-uninstalled agent can't compile, which is an
/// honest "needs refresh" state, not a failed rename — so it warns rather than
/// unwinding the move. Returns whether a fresh `<id>.lock` now exists.
fn refresh_locks(new_source: &Path, dir: &Path, had_compiled_lock: bool, paths: &Paths) -> bool {
    if let Ok(app) = load_app(new_source) {
        let _ = write_app_lockfile(&app, dir, paths);
    }
    if !had_compiled_lock {
        return false;
    }
    match crate::app_lock::compile_to_disk(new_source, paths) {
        Ok(_) => true,
        Err(e) => {
            eprintln!(
                "warning: app moved, but recompiling its lock failed ({e}); run `aware app compile` to refresh"
            );
            false
        }
    }
}

/// Regenerate the synthesized agent for a baked (`exposes-as-agent`) app under
/// its new id. `old_id` is `None` for duplicate (the source's agent stays);
/// `Some(old)` for rename (the old synthesized agent is removed after the new
/// one is written). On failure the partially-written new agent is cleaned up and
/// the error returned so the caller can unwind.
fn move_synth_agent(
    new_source: &Path,
    new_id: &str,
    old_id: Option<&str>,
    paths: &Paths,
) -> Result<(), AwareError> {
    let app = load_app(new_source)?;
    if !app.exposes_as_agent {
        return Ok(());
    }
    if let Err(e) = write_synthesized_agent(&app, paths) {
        let _ = std::fs::remove_dir_all(paths.agents_dir().join(new_id));
        return Err(e);
    }
    if let Some(old_id) = old_id {
        let old_agent = paths.agents_dir().join(old_id);
        if old_agent.exists()
            && is_app_backed_agent(&old_agent, old_id)
            && let Err(e) = std::fs::remove_dir_all(&old_agent)
        {
            eprintln!(
                "warning: app renamed, but removing the old synthesized agent {old_id} failed ({e})"
            );
        }
    }
    Ok(())
}

/// Pre-flight: a baked app would register a synthesized agent at `agents/<new>/`.
/// Refuse if a real (not app-backed-by-`backed_by`) agent already squats that
/// name, mirroring `install`'s guard, so we never half-move an app.
fn assert_agent_name_free(new_id: &str, backed_by: &str, paths: &Paths) -> Result<(), AwareError> {
    let new_agent = paths.agents_dir().join(new_id);
    if new_agent.exists() && !is_app_backed_agent(&new_agent, backed_by) {
        return Err(AwareError::Conflict(format!(
            "an agent named {new_id} is already installed"
        )));
    }
    Ok(())
}

/// Rename an installed app `old_id` → `new_id`, in place. The app keeps running
/// the same plan under the new identity (lock regenerated, no drift).
pub fn rename_app(old_id: &str, new_id: &str, paths: &Paths) -> Result<AppMoveOutcome, AwareError> {
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
    // Guard the synthesized-agent name collision BEFORE moving anything.
    let old_source = find_app_manifest(&old_dir)
        .ok_or_else(|| AwareError::Validation(format!("app {old_id} has no .flo/.app source")))?;
    let exposes = load_app(&old_source)?.exposes_as_agent;
    if exposes {
        assert_agent_name_free(new_id, old_id, paths)?;
    }

    // The load-bearing step. After it, unwind by renaming back.
    std::fs::rename(&old_dir, &new_dir)?;

    let (new_source, had_lock) = match restamp_dir(&new_dir, new_id) {
        Ok(v) => v,
        Err(e) => {
            let _ = std::fs::rename(&new_dir, &old_dir);
            return Err(e);
        }
    };

    if exposes && let Err(e) = move_synth_agent(&new_source, new_id, Some(old_id), paths) {
        let _ = std::fs::rename(&new_dir, &old_dir);
        return Err(e);
    }

    let compiled = refresh_locks(&new_source, &new_dir, had_lock, paths);
    Ok(AppMoveOutcome {
        id: new_id.to_string(),
        compiled,
    })
}

/// Duplicate an installed app `src_id` → `new_id` as an independent copy. The
/// original is left entirely untouched.
pub fn duplicate_app(
    src_id: &str,
    new_id: &str,
    paths: &Paths,
) -> Result<AppMoveOutcome, AwareError> {
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
    if exposes {
        // A duplicate's synth agent is brand new, so the target name must be free
        // outright (there is no prior app-backed agent of `new_id` to reclaim).
        assert_agent_name_free(new_id, new_id, paths)?;
    }

    copy_dir_recursive(&src_dir, &new_dir)?;

    let (new_source, had_lock) = match restamp_dir(&new_dir, new_id) {
        Ok(v) => v,
        Err(e) => {
            let _ = std::fs::remove_dir_all(&new_dir);
            return Err(e);
        }
    };

    if exposes && let Err(e) = move_synth_agent(&new_source, new_id, None, paths) {
        let _ = std::fs::remove_dir_all(&new_dir);
        return Err(e);
    }

    let compiled = refresh_locks(&new_source, &new_dir, had_lock, paths);
    Ok(AppMoveOutcome {
        id: new_id.to_string(),
        compiled,
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

    /// Install a minimal app `id` directly on disk (source + a compiled-looking
    /// `<id>.lock`), returning its paths root. `exposes` adds an exposes-as-agent
    /// block and the synthesized agent, mirroring an `aware app install`.
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
        // A stand-in compiled lock so rename/duplicate treat the app as compiled.
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
        assert!(out.compiled, "an app that had a lock must be recompiled");

        // Old dir gone, new dir present with the canonical source name.
        assert!(!paths.apps_dir().join("old-name").exists());
        assert!(paths.apps_dir().join("new-name/new-name.flo").is_file());
        assert!(
            !paths.apps_dir().join("new-name/old-name.flo").exists(),
            "old-named source must be removed"
        );
        // The `app:` field followed the rename.
        assert_eq!(read_app_field(&paths, "new-name"), "new-name");
        // A fresh lock exists whose source-hash matches the renamed bytes.
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
        // Seed then delete the stand-in lock → app was never compiled.
        seed_app(&paths, "src", false);
        std::fs::remove_file(paths.apps_dir().join("src/src.lock")).unwrap();

        let out = rename_app("src", "dst", &paths).unwrap();
        assert!(!out.compiled);
        assert!(!paths.apps_dir().join("dst/dst.lock").exists());
    }

    #[test]
    fn rename_relocates_synthesized_agent_for_baked_app() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_in(tmp.path());
        seed_app(&paths, "baked", true);
        assert!(paths.agents_dir().join("baked/manifest.yaml").is_file());

        rename_app("baked", "rebaked", &paths).unwrap();

        // The synthesized agent followed the rename.
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
        // Both apps untouched.
        assert!(paths.apps_dir().join("a/a.flo").is_file());
        assert!(paths.apps_dir().join("b/b.flo").is_file());
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
    fn rename_rejects_invalid_id() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_in(tmp.path());
        seed_app(&paths, "ok", false);
        for bad in ["../escape", "has space", ".hidden", "nul", "a/b"] {
            let err = rename_app("ok", bad, &paths).unwrap_err();
            assert!(
                matches!(err, AwareError::Validation(_)),
                "expected Validation for {bad:?}, got {err:?}"
            );
        }
        // The original is intact after every rejected attempt.
        assert!(paths.apps_dir().join("ok/ok.flo").is_file());
    }

    #[test]
    fn duplicate_leaves_original_intact_and_is_independent() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_in(tmp.path());
        seed_app(&paths, "orig", false);

        let out = duplicate_app("orig", "copy", &paths).unwrap();
        assert_eq!(out.id, "copy");

        // Original untouched.
        assert!(paths.apps_dir().join("orig/orig.flo").is_file());
        assert_eq!(read_app_field(&paths, "orig"), "orig");
        // Copy exists with its own identity.
        assert!(paths.apps_dir().join("copy/copy.flo").is_file());
        assert_eq!(read_app_field(&paths, "copy"), "copy");
        assert!(out.compiled && paths.apps_dir().join("copy/copy.lock").is_file());
    }

    #[test]
    fn duplicate_baked_app_creates_new_agent_and_keeps_the_original() {
        let tmp = tempfile::tempdir().unwrap();
        let paths = paths_in(tmp.path());
        seed_app(&paths, "baked", true);

        duplicate_app("baked", "baked-copy", &paths).unwrap();

        // Original synth agent still present; copy got its own.
        assert!(paths.agents_dir().join("baked/manifest.yaml").is_file());
        let copy_manifest = paths.agents_dir().join("baked-copy/manifest.yaml");
        assert!(copy_manifest.is_file());
        let agent = crate::manifest::loader::load_agent(&copy_manifest).unwrap();
        assert_eq!(agent.transport.app.unwrap().backed_by, "baked-copy");
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
        // An indented `app:` (e.g. inside a node's transport config) must NOT be
        // rewritten — only the top-level identity.
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
        ] {
            assert!(validate_app_id(bad).is_err(), "{bad} should be invalid");
        }
    }
}
