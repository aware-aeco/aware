//! `aware voice ...` — manage voice packs (v0.25).
//!
//! Voice packs are a new distribution primitive per
//! `10-core/app-spec.md § Panel review`. A voice pack is a markdown
//! system-prompt + reference-codes folder published by an institution
//! / authoring engineer / firm. Forkable, version-pinnable, citable.
//!
//! Storage: `~/.aware/voices/<scope>/<id>/<version>/`

use std::path::PathBuf;

use clap::{Args, Subcommand};

use crate::context::Context;
use crate::error::AwareError;

#[derive(Subcommand, Debug)]
pub enum VoiceCommand {
    /// List installed voice packs.
    List,
    /// Print the manifest + system prompt of a voice pack.
    Describe { pack: String },
    /// Install a voice pack from a local path. (Registry installer
    /// lands in v0.25.x once the voice-pack registry shape is settled.)
    Install(InstallArgs),
    /// Remove an installed voice pack.
    Uninstall { pack: String },
}

#[derive(Args, Debug)]
pub struct InstallArgs {
    /// Path to a voice pack folder containing `manifest.yaml` + `system-prompt.md`.
    pub path: PathBuf,
    /// Scope (vendor / institution / user) — e.g. `ise`, `aware-aeco`.
    #[arg(long)]
    pub scope: Option<String>,
}

pub fn dispatch(cmd: VoiceCommand, ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        VoiceCommand::List => list(ctx),
        VoiceCommand::Describe { pack } => describe(ctx, &pack),
        VoiceCommand::Install(args) => install(ctx, &args),
        VoiceCommand::Uninstall { pack } => uninstall(ctx, &pack),
    }
}

fn voices_dir(ctx: &Context) -> PathBuf {
    ctx.paths.aware_home.join("voices")
}

fn list(ctx: &Context) -> Result<(), AwareError> {
    let voices = voices_dir(ctx);
    if !voices.exists() {
        println!("(no voice packs installed)");
        println!();
        println!("Install a pack with:  aware voice install <path>");
        return Ok(());
    }
    let mut found: Vec<(String, String, String)> = Vec::new();
    if let Ok(scopes) = std::fs::read_dir(&voices) {
        for scope_entry in scopes.flatten() {
            if !scope_entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                continue;
            }
            let scope = scope_entry.file_name().to_string_lossy().into_owned();
            if let Ok(packs) = std::fs::read_dir(scope_entry.path()) {
                for pack_entry in packs.flatten() {
                    if !pack_entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                        continue;
                    }
                    let id = pack_entry.file_name().to_string_lossy().into_owned();
                    if let Ok(versions) = std::fs::read_dir(pack_entry.path()) {
                        for ver_entry in versions.flatten() {
                            if ver_entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                                let version = ver_entry.file_name().to_string_lossy().into_owned();
                                found.push((scope.clone(), id.clone(), version));
                            }
                        }
                    }
                }
            }
        }
    }
    if found.is_empty() {
        println!("(no voice packs installed)");
        return Ok(());
    }
    println!("SCOPE        ID                                  VERSION");
    for (scope, id, version) in &found {
        println!("{scope:<12} {id:<35} {version}");
    }
    Ok(())
}

fn describe(ctx: &Context, pack: &str) -> Result<(), AwareError> {
    let pack_dir = resolve_pack_dir(ctx, pack)?;
    let manifest_path = pack_dir.join("manifest.yaml");
    if !manifest_path.is_file() {
        return Err(AwareError::NotFound(format!(
            "voice pack manifest at {}",
            manifest_path.display()
        )));
    }
    let manifest = std::fs::read_to_string(&manifest_path)
        .map_err(|e| AwareError::Internal(format!("read {}: {e}", manifest_path.display())))?;
    println!("# manifest.yaml");
    println!("{manifest}");
    let sp_path = pack_dir.join("system-prompt.md");
    if sp_path.is_file() {
        let sp = std::fs::read_to_string(&sp_path)
            .map_err(|e| AwareError::Internal(format!("read {}: {e}", sp_path.display())))?;
        println!();
        println!("# system-prompt.md");
        println!("{sp}");
    }
    let refs_dir = pack_dir.join("references");
    if refs_dir.is_dir() {
        println!();
        println!("# references/");
        if let Ok(entries) = std::fs::read_dir(&refs_dir) {
            for entry in entries.flatten() {
                println!("  - {}", entry.file_name().to_string_lossy());
            }
        }
    }
    Ok(())
}

/// Every part of a pack coordinate becomes one directory level under
/// `~/.aware/voices/`, so each must be a plain segment — same fence, same
/// reasoning as `apps/<id>/` (#365) and `agents/<id>/`.
///
/// Without it `resolve_pack_dir` handed `voices/../..` to callers, and
/// `uninstall` calls `remove_dir_all` on whatever it is handed: `aware voice
/// uninstall '../..'` deleted the whole of `~/.aware` — apps, agents, stored
/// credentials — and printed `✓ uninstalled`, exit 0. `install` is the same
/// escape one step earlier, writing the copied pack wherever a manifest's
/// `id:` pointed. Both verified against the real binary before this was
/// written; `tests/voice_pack_ids_are_segments.rs` pins them.
fn fence_segment(part: &str, what: &str, pack: &str) -> Result<(), AwareError> {
    if crate::manifest::loader::is_safe_segment(part) {
        return Ok(());
    }
    Err(AwareError::Validation(format!(
        "voice pack {pack:?} has a {what} ({part:?}) that is not a plain name — \
         each part becomes a directory under `voices/`, so it may not be `.` or \
         `..`, contain a path separator, or carry a drive/UNC prefix"
    )))
}

/// The physical half of the fence. [`fence_segment`] is **lexical** — it asks
/// what `join` spells, which is the only question `is_safe_segment` answers, and
/// its own doc says so: *"A permitted id that names a symlink can still resolve
/// outside `dir`. Containment against symlinks is a different check (canonicalise
/// and compare), not this one."*
///
/// For `apps/` and `agents/` the lexical half is where the repo stopped. It
/// cannot stop there here, because `uninstall` ends in `remove_dir_all` and a
/// symlink anywhere along the resolved path — not just at the leaf — carries it
/// straight through:
///
/// ```text
/// $ ln -s /elsewhere ~/.aware/voices/ise      # scope dir is a symlink
/// $ aware voice uninstall 'ise/secret-pack'   # every part a plain segment
/// ✓ uninstalled voice pack at ~/.aware/voices/ise/secret-pack/1.0.0
/// $ ls /elsewhere/secret-pack/                # emptied
/// ```
///
/// Both ids pass `fence_segment`; the escape is in what the path *resolves to*.
///
/// The test is not "does it land under `voices/`" but "did it get there without
/// following a link". `starts_with(root)`, which this used at first, passes a
/// link that stays *inside* `voices/` — and that is not harmless, because
/// `contained_in_voices` returns the CANONICAL path and `uninstall` calls
/// `remove_dir_all` on what it returns. With only the prefix test:
///
/// ```text
/// $ ln -s ../reviewer/2.0.0 ~/.aware/voices/ise/reviewer/1.0.0
/// $ aware voice uninstall 'ise/reviewer@1.0.0'
/// ✓ uninstalled voice pack at ~/.aware/voices/ise/reviewer/2.0.0   # exit 0
/// ```
///
/// 2.0.0 is destroyed and the dangling 1.0.0 link is what survives — a delete of
/// something the operator did not name, introduced by the very check meant to
/// stop that. An earlier draft of this doc claimed a symlinked leaf was harmless
/// because `remove_dir_all` unlinks the link. That was true before `9ada65e4`
/// added any containment check at all, and canonicalising here is what stopped it
/// being true — so by the time that sentence was written it described neither the
/// code above it nor the code below it.
///
/// So the demand is equality, not prefix: the canonical path must be the
/// canonical `voices/` root joined with the components as spelled. Note where the
/// line falls — `root` is canonicalised too, so this constrains only the
/// components BELOW `voices/`. The root itself, `AWARE_HOME`, and anything above
/// them may be links, and must stay allowed: a symlinked `~` is an ordinary
/// dotfile-manager layout, and macOS's default `TMPDIR` is `/var` → `/private/var`,
/// which the tests in this file rely on. What may not be a link is any component
/// the operator names in a coordinate, because those are what `install` writes to
/// and `uninstall` deletes. Returns the CANONICAL path, so the caller acts on
/// what it checked rather than on the spelling that got it here.
fn contained_in_voices(ctx: &Context, resolved: &std::path::Path) -> Result<PathBuf, AwareError> {
    let voices = voices_dir(ctx);
    let root = canonicalize_named(&voices)?;
    // Both callers build `resolved` by joining onto `voices_dir(ctx)`, so this
    // cannot fail for them; it is a `Result` rather than an `expect` because a
    // future third caller getting it wrong should be an error, not a panic.
    let rel = resolved.strip_prefix(&voices).map_err(|_| {
        AwareError::Internal(format!(
            "{} is not built under {}",
            resolved.display(),
            voices.display()
        ))
    })?;
    let real = canonicalize_named(resolved)?;
    let expected = root.join(rel);
    if real != expected {
        // States what was MEASURED, not an inferred cause. An earlier wording said
        // "resolves through a symlink", which is only reachable via a link while
        // `fence_segment` runs first — but with that fence mutated off, a plain
        // `../../../` traversal produced the same sentence, sending a reader
        // hunting for a link that was never there. It also named `expected`, which
        // on any ordinary home is byte-identical to `resolved`, so it told the
        // operator that a path does not resolve to itself. Name the boundary
        // instead: that is the thing being enforced.
        return Err(AwareError::Validation(format!(
            "voice pack path {} resolves to {} — refusing, because every part of \
             a coordinate must reach its target under {} without passing through \
             a symlink, or a link could move what `install` writes and what \
             `uninstall` deletes",
            resolved.display(),
            real.display(),
            root.display()
        )));
    }
    Ok(real)
}

/// `canonicalize`, re-raised carrying the path and preserving the io kind, the
/// way `app.rs`'s installed-manifest probe does (`app.rs`, the
/// `symlink_metadata` match — same `io::Error::new(e.kind(), …)` re-raise).
///
/// Scope, stated honestly because an earlier version of this comment overclaimed
/// and a reviewer measured it: this does NOT fix what an operator with a
/// chmod-000 `~/.aware` sees. That operator never reaches this function —
/// `list`'s `if let Ok(…) = read_dir` returns "(no voice packs installed)" at
/// exit 0, `resolve_pack_dir`'s `parent.is_dir()` returns "not installed" at exit
/// 7, and `create_dir_all` raises its own `Internal` — all before any
/// canonicalise. Those three are the real defect for that operator and they are
/// pre-existing; this is not a substitute for fixing them.
///
/// What it does do is stop THIS call site printing `error: internal:` for a
/// failure that is environmental (EACCES, ELOOP, a directory removed under us),
/// and keep the path in the message, since a bare `Permission denied (os error
/// 13)` names nothing. Both `Io` and `Internal` map to exit 1, so no script sees
/// a difference — `AwareError::PermissionDenied` (exit 5) exists and would be the
/// right variant if this is ever made to carry the classification too.
fn canonicalize_named(path: &std::path::Path) -> Result<PathBuf, AwareError> {
    path.canonicalize().map_err(|e| {
        std::io::Error::new(e.kind(), format!("canonicalise {}: {e}", path.display())).into()
    })
}

/// Order two voice-pack FOLDER NAMES: the newest is the one this returns `Greater` for.
///
/// Two comparators, because pack folders are two different things in practice and using
/// either alone gets one of them backwards:
///
/// - when both names are strict SemVer, `compare_version_keys` decides — it knows that a
///   release outranks its own prerelease and that build metadata carries no precedence
///   (§10/§11), neither of which is visible to a component-wise scan;
/// - otherwise `compare_dot_components`, which is what a calendar-shaped folder needs
///   (`2025.10` after `2025.9`) and what this whole issue is about (#377).
///
/// Composing them rather than using only the second is a correction to the first cut of
/// this fix, which regressed a shape `main` had right: `1.0.0-rc.1` split as
/// `["1","0","0-rc","1"]`, and §11's "numeric ranks below alphanumeric" rule then made
/// `0-rc` beat the `1` of `1.0.1` — so a release candidate outranked a HIGHER patch. A
/// string compare, for all its faults, got that pair right. Nothing stops a pack shipping
/// a semver version (`install` copies whatever the manifest says), so this was reachable.
///
/// Equal-comparing names fall back to the raw string so the order is total: `2025.01` and
/// `2025.1` are Equal component-wise, and leaving them tied would decide the winner by
/// `read_dir` order — the same non-determinism this is meant to remove.
fn compare_pack_versions(a: &str, b: &str) -> std::cmp::Ordering {
    use crate::validate::{compare_dot_components, compare_version_keys, parse_semver};
    match (parse_semver(a), parse_semver(b)) {
        (Some(_), Some(_)) => compare_version_keys(a, b),
        _ => compare_dot_components(a, b),
    }
    .then_with(|| a.cmp(b))
}

/// Resolve a pack identifier like `@ise/uk-structural-reviewer@2025` or
/// `aware-aeco/structural-engineer` to a folder path under `~/.aware/voices/`.
/// Picks the latest installed version if none specified.
///
/// The version is introduced by `@`. This doc used to advertise a third
/// spelling, `<scope>/<id>/<version>`, which never worked: `scope_id` splits on
/// the FIRST `/`, so the id became `uk-structural-reviewer/2025`, `parent`
/// became `voices/ise/uk-structural-reviewer/2025`, and the "latest version"
/// scan then picked a lexically-latest *subdirectory of the version folder* —
/// `references/`, say, which `uninstall` would go on to delete. The fence turns
/// that into a clean refusal, so the promise is removed rather than left
/// contradicting the code (`a_slash_separated_version_is_refused` pins it).
fn resolve_pack_dir(ctx: &Context, pack: &str) -> Result<PathBuf, AwareError> {
    let cleaned = pack.trim_start_matches('@');
    let (scope_id, version) = if let Some((s, v)) = cleaned.split_once('@') {
        (s, Some(v.to_string()))
    } else {
        (cleaned, None)
    };
    let (scope, id) = scope_id
        .split_once('/')
        .ok_or_else(|| AwareError::Validation(format!("invalid pack id: {pack}")))?;

    fence_segment(scope, "scope", pack)?;
    fence_segment(id, "id", pack)?;
    if let Some(v) = version.as_deref() {
        fence_segment(v, "version", pack)?;
    }

    let parent = voices_dir(ctx).join(scope).join(id);
    if !parent.is_dir() {
        return Err(AwareError::NotFound(format!(
            "voice pack {scope}/{id} not installed"
        )));
    }
    if let Some(v) = version {
        let p = parent.join(&v);
        if p.is_dir() {
            return contained_in_voices(ctx, &p);
        }
        return Err(AwareError::NotFound(format!(
            "voice pack {scope}/{id}@{v} not installed"
        )));
    }
    // Pick the newest version by COMPONENT, not by string. This compared directory
    // names directly until #377, with a comment conceding it was "good enough for
    // semver-tagged folders" — and it is not: `"2025.10" < "2025.9"` as strings, so a
    // pack whose minor reaches double digits loses to its own predecessor. Same shape
    // as #371, which fixed it for the registry; the strict-SemVer comparator that
    // issue added cannot serve here, because a pack folder is `2025.10`, not a triple,
    // so `parse_semver` returns `None` and it degrades to the same string compare.
    // `compare_dot_components` is the part of that work which does apply.
    let mut latest: Option<PathBuf> = None;
    let mut latest_name: Option<String> = None;
    if let Ok(entries) = std::fs::read_dir(&parent) {
        for entry in entries.flatten() {
            if !entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                continue;
            }
            let name = entry.file_name().to_string_lossy().into_owned();
            let newer = match &latest_name {
                Some(best) => compare_pack_versions(&name, best) == std::cmp::Ordering::Greater,
                // The first directory wins outright, and the empty-string sentinel this
                // replaced could not survive the comparator: `compare_pack_versions("2025",
                // "")` is LESS, because `""` fails to parse as a number and a numeric
                // component ranks below an alphanumeric one. Keeping the sentinel would
                // have let `""` win every comparison, so no calendar-named pack would
                // resolve at all. This is not tidying — it is load-bearing.
                None => true,
            };
            if newer {
                latest_name = Some(name);
                latest = Some(entry.path());
            }
        }
    }
    let latest = latest
        .ok_or_else(|| AwareError::NotFound(format!("no installed versions of {scope}/{id}")))?;
    contained_in_voices(ctx, &latest)
}

fn install(ctx: &Context, args: &InstallArgs) -> Result<(), AwareError> {
    let src = &args.path;
    if !src.is_dir() {
        return Err(AwareError::Validation(format!(
            "source {} is not a directory",
            src.display()
        )));
    }
    let manifest_path = src.join("manifest.yaml");
    if !manifest_path.is_file() {
        return Err(AwareError::Validation(format!(
            "no manifest.yaml in {}",
            src.display()
        )));
    }
    // Minimal manifest fields we need: id + version (+ optional scope).
    let manifest_text = std::fs::read_to_string(&manifest_path)
        .map_err(|e| AwareError::Internal(format!("read manifest: {e}")))?;
    let manifest: serde_yaml::Value = serde_yaml::from_str(&manifest_text)
        .map_err(|e| AwareError::Validation(format!("manifest YAML: {e}")))?;
    let id = yaml_to_string(manifest.get("id"))
        .ok_or_else(|| AwareError::Validation("manifest missing `id`".into()))?;
    let version = yaml_to_string(manifest.get("version"))
        .ok_or_else(|| AwareError::Validation("manifest missing `version`".into()))?;
    let scope = args
        .scope
        .clone()
        .or_else(|| yaml_to_string(manifest.get("scope")))
        .ok_or_else(|| {
            AwareError::Validation(
                "scope required — pass --scope <name> or set `scope:` in manifest".into(),
            )
        })?;

    // The three fields that decide WHERE the copy lands all come from a file the
    // installer did not write, so they are fenced before the join — not after,
    // and not only on `resolve_pack_dir`'s side. `create_dir_all` +
    // `copy_dir_recursive` below take whatever `dst` says.
    let coord = format!("{scope}/{id}@{version}");
    fence_segment(&scope, "scope", &coord)?;
    fence_segment(&id, "id", &coord)?;
    fence_segment(&version, "version", &coord)?;

    let dst = voices_dir(ctx).join(&scope).join(&id).join(&version);
    // Whether THIS call is the one bringing the version directory into being.
    // `false` means it already existed and this is a re-install over it — see the
    // rollback below for why that case is left alone rather than cleaned up.
    let created_dst = !dst.exists();
    std::fs::create_dir_all(&dst)
        .map_err(|e| AwareError::Internal(format!("create {}: {e}", dst.display())))?;
    // Fenced lexically above, but a symlink already sitting at `voices/<scope>`
    // would still put the copy somewhere else entirely, so the physical check
    // has to run too — and it needs the path to exist in order to canonicalise,
    // which is why it is here rather than before `create_dir_all`.
    //
    // A FIRST install of this coordinate is undone on failure. Without that, a
    // refusal left the version directory standing and `list` — which reads the
    // directory tree, not a manifest — reported the pack as installed:
    //
    // ```text
    // $ aware voice install ./pack     # ships one symlink
    // error: validation failed: voice pack contains a symlink (…)   exit 3
    // $ aware voice list
    // ise    reviewer    1.0.0                                      exit 0
    // ```
    //
    // and `read_dir` order decides whether `describe` then prints a manifest for
    // a pack whose `references/` never arrived.
    //
    // A RE-INSTALL is NOT undone, and saying so plainly rather than claiming a
    // transaction this does not have: `copy_dir_recursive` writes into the live
    // directory, so by the time a refusal arrives the previously installed pack
    // is already a blend of old and new. Rolling back would then delete a working
    // pack the operator still has; not rolling back leaves it corrupted. Neither
    // is right, and the fix is to stage into a sibling and `rename` into place —
    // a behaviour change owed its own PR (see the review thread on #381). What is
    // fixed here is the first-install husk; the re-install hazard is recorded,
    // not closed.
    let dst = match contained_in_voices(ctx, &dst)
        .and_then(|checked| copy_dir_recursive(src, &checked).map(|()| checked))
    {
        Ok(checked) => checked,
        Err(e) => {
            if created_dst {
                undo_created_pack_dir(ctx, &dst);
            }
            return Err(e);
        }
    };
    println!(
        "\u{2713} installed voice pack {scope}/{id}@{version} \u{2192} {}",
        dst.display()
    );
    Ok(())
}

/// Undo the directory a failed FIRST install created, and nothing else.
///
/// The blast radius is the point. An earlier cut of this walked up to the
/// topmost directory that had not existed before the install and
/// `remove_dir_all`-ed that — which on a home with no `voices/` yet is
/// `~/.aware/voices` **itself**, and on a home whose parents did not exist
/// reached above `AWARE_HOME` entirely. Reproduced: a second `aware voice
/// install` that completed successfully during the first one's copy had its pack
/// deleted when the first was refused —
///
/// ```text
/// ✓ installed voice pack other/q@1.0.0          # B, exit 0
/// error: validation failed: … contains a symlink # A, exit 3
/// $ aware voice list
/// (no voice packs installed)                     # B's pack, gone
/// ```
///
/// — "a delete of a pack nobody named", which is the exact bug the fence at the
/// top of this file exists to stop, reintroduced by the cleanup for a lesser one.
/// Before any rollback existed, a refused install left litter but destroyed
/// nothing, so that first cut was a regression and not merely an imperfection.
///
/// So: remove only the version directory this call created, then prune ancestors
/// with `remove_dir`, which removes a directory only when it is EMPTY. A sibling
/// pack another process installed keeps its parent non-empty and stops the walk
/// on its own — no lock needed, and no window in which a concurrent success can
/// be swallowed. The walk stops at `voices/` and never considers a path outside
/// it, so `AWARE_HOME` and everything above it are out of reach by construction.
///
/// Best-effort by necessity — the install has already failed and there is no
/// second error to return — but not silent: a cleanup that leaves something
/// behind means `voice list` will report a pack that is not there, and the
/// operator needs to know which one.
fn undo_created_pack_dir(ctx: &Context, dst: &std::path::Path) {
    if let Err(e) = std::fs::remove_dir_all(dst) {
        eprintln!(
            "warning: the failed install could not be undone — {} may be left \
             behind, and `aware voice list` will report it as installed: {e}",
            dst.display()
        );
        return;
    }
    // `remove_dir` on an empty directory only. Stops at the first non-empty one,
    // which is what makes a concurrent install safe, and never touches `voices/`.
    let voices = voices_dir(ctx);
    let mut cursor = dst.parent();
    while let Some(dir) = cursor {
        if dir == voices || !dir.starts_with(&voices) || std::fs::remove_dir(dir).is_err() {
            break;
        }
        cursor = dir.parent();
    }
}

/// Coerce a YAML scalar (string, number, bool) to a string. Returns None
/// for missing / null / non-scalar values.
fn yaml_to_string(v: Option<&serde_yaml::Value>) -> Option<String> {
    match v? {
        serde_yaml::Value::String(s) => Some(s.clone()),
        serde_yaml::Value::Number(n) => Some(n.to_string()),
        serde_yaml::Value::Bool(b) => Some(b.to_string()),
        _ => None,
    }
}

/// Copy the pack tree, refusing to FOLLOW symlinks out of it.
///
/// `Path::is_dir` follows links, and the walk used it. A pack shipping
/// `up -> ..` therefore descended `pack/up/pack/up/…` until the OS stopped it
/// with `ELOOP` — 85 directories and half a megabyte written under
/// `~/.aware/voices/` on the way, left behind because there is no rollback. The
/// same mechanism means `references -> /etc` copies `/etc` into the pack.
/// Reproduced against the real binary.
///
/// So the type is read with `symlink_metadata`, which does NOT follow, and a
/// link is refused by name rather than silently skipped: a pack that ships one
/// is asking for something this installer will not do, and dropping it quietly
/// would install a pack missing files its manifest lists.
fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) -> Result<(), AwareError> {
    for entry in std::fs::read_dir(src)
        .map_err(|e| AwareError::Internal(format!("read_dir {}: {e}", src.display())))?
        .flatten()
    {
        let from = entry.path();
        let to = dst.join(entry.file_name());
        let meta = std::fs::symlink_metadata(&from)
            .map_err(|e| AwareError::Internal(format!("stat {}: {e}", from.display())))?;
        if meta.file_type().is_symlink() {
            return Err(AwareError::Validation(format!(
                "voice pack contains a symlink ({}) — refusing, because following \
                 it would copy from outside the pack, and a link pointing back at \
                 an ancestor recurses until the filesystem stops it",
                from.display()
            )));
        }
        if meta.is_dir() {
            std::fs::create_dir_all(&to)
                .map_err(|e| AwareError::Internal(format!("create {}: {e}", to.display())))?;
            copy_dir_recursive(&from, &to)?;
        } else {
            std::fs::copy(&from, &to).map_err(|e| {
                AwareError::Internal(format!("copy {} -> {}: {e}", from.display(), to.display()))
            })?;
        }
    }
    Ok(())
}

fn uninstall(ctx: &Context, pack: &str) -> Result<(), AwareError> {
    let pack_dir = resolve_pack_dir(ctx, pack)?;
    std::fs::remove_dir_all(&pack_dir)
        .map_err(|e| AwareError::Internal(format!("remove {}: {e}", pack_dir.display())))?;
    println!("\u{2713} uninstalled voice pack at {}", pack_dir.display());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::paths::Paths;
    use crate::validate::compare_dot_components;
    use std::cmp::Ordering::*;

    /// A voices tree with the given `scope/id/version` folders, plus an
    /// `apps/keep-me/` sibling that no voice operation should ever touch —
    /// it is what makes the escape assertions below say something.
    ///
    /// The home is nested `deep/enough/to/catch/aware` inside the tempdir on
    /// purpose. An earlier version put it one level down, so a scope of
    /// `../../../pwned` escaped past the tempdir into the system `/tmp` — and
    /// the assertion written to catch that looked inside the tempdir, where
    /// nothing would ever appear. It passed by construction, and a real fence
    /// regression littered `/tmp` on every machine that ran the suite. Any
    /// `..`-chain a test uses must stay inside the directory the test inspects.
    fn ctx_with(packs: &[(&str, &str, &str)]) -> (tempfile::TempDir, Context) {
        let tmp = tempfile::tempdir().unwrap();
        let home = tmp.path().join("deep/enough/to/catch/aware");
        std::fs::create_dir_all(home.join("apps/keep-me")).unwrap();
        for (scope, id, version) in packs {
            let dir = home.join("voices").join(scope).join(id).join(version);
            std::fs::create_dir_all(&dir).unwrap();
            std::fs::write(
                dir.join("manifest.yaml"),
                format!("id: {id}\nversion: {version}\n"),
            )
            .unwrap();
        }
        let ctx = Context {
            paths: Paths { aware_home: home },
            json: false,
        };
        (tmp, ctx)
    }

    /// Assert nothing whose name begins with `prefix` exists ANYWHERE under
    /// `root`, and say where it landed if it does.
    ///
    /// The escape assertions used to name one path and check that. Two of the
    /// three named the wrong one — `<tmp>/pwned` where the escape actually lands
    /// at `<tmp>/deep/enough/to/catch/pwned` — so both passed with
    /// `fence_segment` deleted outright, which is the exact failure `ctx_with`'s
    /// own doc comment warns about, repeated one level up. Fixing the arithmetic
    /// would have left the next person to get it wrong; searching the whole tree
    /// removes the arithmetic from the assertion altogether.
    fn assert_nothing_escaped(root: &std::path::Path, prefix: &str) {
        let mut found = Vec::new();
        let mut stack = vec![root.to_path_buf()];
        while let Some(dir) = stack.pop() {
            let Ok(entries) = std::fs::read_dir(&dir) else {
                continue;
            };
            for entry in entries.flatten() {
                if entry.file_name().to_string_lossy().starts_with(prefix) {
                    found.push(entry.path());
                }
                // `file_type` does not follow links, so a symlinked fixture
                // cannot walk this out of `root` or into a cycle.
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    stack.push(entry.path());
                }
            }
        }
        assert!(
            found.is_empty(),
            "the pack escaped to {found:?} — nothing named {prefix:?}* may exist under {}",
            root.display()
        );
    }

    #[test]
    fn no_version_picks_the_latest_installed_one() {
        let (_tmp, ctx) = ctx_with(&[("ise", "reviewer", "1.0.0"), ("ise", "reviewer", "2.0.0")]);
        let dir = resolve_pack_dir(&ctx, "ise/reviewer").unwrap();
        assert_eq!(
            dir.file_name().unwrap(),
            "2.0.0",
            "an unversioned id must resolve to the newest installed version, got {}",
            dir.display()
        );
    }

    #[test]
    fn an_explicit_version_wins_over_the_latest() {
        let (_tmp, ctx) = ctx_with(&[("ise", "reviewer", "1.0.0"), ("ise", "reviewer", "2.0.0")]);
        // Both spellings the doc comment promises: bare and `@`-prefixed.
        // (It used to promise a third, `scope/id/version`; see
        // `a_slash_separated_version_is_refused` for why that is gone.)
        for pack in ["ise/reviewer@1.0.0", "@ise/reviewer@1.0.0"] {
            let dir = resolve_pack_dir(&ctx, pack).unwrap();
            assert_eq!(
                dir.file_name().unwrap(),
                "1.0.0",
                "{pack} asked for 1.0.0 and got {}",
                dir.display()
            );
        }
    }

    #[test]
    fn a_version_that_is_not_installed_is_not_silently_downgraded() {
        let (_tmp, ctx) = ctx_with(&[("ise", "reviewer", "1.0.0")]);
        // The failure this guards is resolving to 1.0.0 anyway — describe would
        // then print a different pack than the one named, and uninstall would
        // delete it.
        let err = resolve_pack_dir(&ctx, "ise/reviewer@9.9.9").unwrap_err();
        assert!(
            matches!(err, AwareError::NotFound(_)),
            "expected NotFound, got {err:?}"
        );
    }

    #[test]
    fn a_pack_id_without_a_scope_is_refused() {
        let (_tmp, ctx) = ctx_with(&[("ise", "reviewer", "1.0.0")]);
        let err = resolve_pack_dir(&ctx, "reviewer").unwrap_err();
        assert!(
            matches!(err, AwareError::Validation(_)),
            "expected Validation, got {err:?}"
        );
    }

    /// The resolver's half of the escape. `uninstall` hands whatever this
    /// returns straight to `remove_dir_all`, so a resolved path that is not
    /// under `voices/` is a delete of something else.
    ///
    /// Each input must be refused BY THE FENCE — `Validation` — not merely
    /// refused. Accepting any `Err` let inputs into the list that the fence
    /// never judged: `../../apps/keep-me` names a directory that does not
    /// exist relative to `voices/`, so it returned `NotFound` on the unfenced
    /// code too and carried no weight. Demanding the fence's own verdict is
    /// what makes every row here count.
    #[test]
    fn no_pack_id_resolves_outside_the_voices_directory() {
        let (_tmp, ctx) = ctx_with(&[("ise", "reviewer", "1.0.0")]);
        let voices = voices_dir(&ctx);
        for pack in [
            "../..",
            "../../apps/keep-me",
            "ise/../../apps",
            "ise/reviewer@../../../apps",
            "./../voices/ise/reviewer",
        ] {
            match resolve_pack_dir(&ctx, pack) {
                Err(AwareError::Validation(_)) => {}
                Err(other) => panic!(
                    "{pack:?} was refused as {other:?}, not by the fence — it would \
                     have been refused for an unrelated reason on unfenced code too"
                ),
                Ok(dir) => panic!(
                    "{pack:?} resolved to {} — outside {}",
                    dir.display(),
                    voices.display()
                ),
            }
        }
        // And the guard did not achieve that by refusing everything.
        assert!(resolve_pack_dir(&ctx, "ise/reviewer").is_ok());
    }

    /// #3 from review: the doc comment advertised `<scope>/<id>/<version>` and
    /// the code never honoured it. Now that the fence turns it into a clean
    /// refusal, this pins code and doc together so they cannot drift apart
    /// again silently.
    #[test]
    fn a_slash_separated_version_is_refused() {
        let (_tmp, ctx) = ctx_with(&[("ise", "reviewer", "1.0.0")]);
        let err = resolve_pack_dir(&ctx, "ise/reviewer/1.0.0").unwrap_err();
        assert!(
            matches!(err, AwareError::Validation(_)),
            "expected Validation, got {err:?}"
        );
        // The `@` spelling of the same coordinate is the supported one.
        assert!(resolve_pack_dir(&ctx, "ise/reviewer@1.0.0").is_ok());
    }

    /// The fence is lexical; this is the other half. Every part of
    /// `ise/secret-pack` is a plain segment, so `fence_segment` passes it —
    /// and `uninstall` would then `remove_dir_all` through the symlink,
    /// emptying a directory outside `voices/` entirely.
    #[test]
    fn a_symlinked_scope_directory_cannot_smuggle_the_target_outside_voices() {
        let (tmp, ctx) = ctx_with(&[]);
        let outside = tmp.path().join("outside");
        std::fs::create_dir_all(outside.join("secret-pack/1.0.0")).unwrap();
        std::fs::write(outside.join("secret-pack/1.0.0/creds.txt"), "secret\n").unwrap();
        std::fs::create_dir_all(voices_dir(&ctx)).unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(&outside, voices_dir(&ctx).join("ise")).unwrap();
        #[cfg(windows)]
        std::os::windows::fs::symlink_dir(&outside, voices_dir(&ctx).join("ise")).unwrap();

        let err = resolve_pack_dir(&ctx, "ise/secret-pack").unwrap_err();
        assert!(
            matches!(err, AwareError::Validation(_)),
            "expected Validation, got {err:?}"
        );
        // What actually matters: uninstall did not reach through the link.
        assert!(uninstall(&ctx, "ise/secret-pack").is_err());
        assert!(
            outside.join("secret-pack/1.0.0/creds.txt").is_file(),
            "remove_dir_all followed the symlinked scope directory and deleted \
             a file outside voices/"
        );
    }

    /// A symlinked LEAF, both ways it can point — and it must be refused both
    /// ways, with `uninstall` actually run.
    ///
    /// This test previously covered only the outside-`voices/` link and never
    /// called `uninstall`, so `assert!(target still there)` could not fail: no
    /// delete was ever attempted. Its name claimed a link-vs-target distinction
    /// it asserted nothing about.
    ///
    /// The sibling case is the one that mattered. A link to `../2.0.0` stays
    /// inside `voices/`, so the old prefix test passed it, `resolve_pack_dir`
    /// returned the CANONICAL 2.0.0 path, and `uninstall ise/reviewer@1.0.0`
    /// deleted 2.0.0 — a pack the operator never named, destroyed by the check
    /// added to stop exactly that.
    #[cfg(unix)]
    #[test]
    fn a_symlinked_pack_version_is_refused_wherever_it_points() {
        // (a) pointing out of voices/ — the target's contents must survive.
        let (tmp, ctx) = ctx_with(&[]);
        let outside = tmp.path().join("elsewhere");
        std::fs::create_dir_all(&outside).unwrap();
        std::fs::write(outside.join("data.txt"), "keep\n").unwrap();
        let parent = voices_dir(&ctx).join("ise").join("reviewer");
        std::fs::create_dir_all(&parent).unwrap();
        std::os::unix::fs::symlink(&outside, parent.join("1.0.0")).unwrap();

        assert!(matches!(
            resolve_pack_dir(&ctx, "ise/reviewer@1.0.0"),
            Err(AwareError::Validation(_))
        ));
        assert!(
            uninstall(&ctx, "ise/reviewer@1.0.0").is_err(),
            "uninstall accepted a symlinked pack version"
        );
        assert!(
            outside.join("data.txt").is_file(),
            "uninstall followed the link and destroyed the target's contents"
        );

        // (b) pointing at a SIBLING inside voices/ — canonically contained, and
        // still a delete of a pack that was not named.
        let (_tmp2, ctx2) = ctx_with(&[("ise", "reviewer", "2.0.0")]);
        let parent2 = voices_dir(&ctx2).join("ise").join("reviewer");
        std::os::unix::fs::symlink("2.0.0", parent2.join("1.0.0")).unwrap();

        assert!(
            matches!(
                resolve_pack_dir(&ctx2, "ise/reviewer@1.0.0"),
                Err(AwareError::Validation(_))
            ),
            "a link that stays inside voices/ was accepted — `starts_with` is \
             not enough, the path must reach its target without following one"
        );
        assert!(uninstall(&ctx2, "ise/reviewer@1.0.0").is_err());
        assert!(
            parent2.join("2.0.0/manifest.yaml").is_file(),
            "uninstalling the 1.0.0 LINK deleted the real 2.0.0 pack"
        );
    }

    /// #2 from review. `Path::is_dir` follows links, so a pack shipping
    /// `up -> ..` walked `pack/up/pack/up/…` until the OS stopped it, leaving
    /// ~85 directories under `voices/` with no rollback.
    #[test]
    fn a_pack_containing_a_symlink_is_refused_rather_than_followed() {
        let (tmp, ctx) = ctx_with(&[]);
        let src = tmp.path().join("src").join("pack");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("manifest.yaml"), "id: p\nversion: 1.0.0\n").unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink("..", src.join("up")).unwrap();
        #[cfg(windows)]
        std::os::windows::fs::symlink_dir("..", src.join("up")).unwrap();

        let err = install(
            &ctx,
            &InstallArgs {
                path: src,
                scope: Some("s".into()),
            },
        )
        .unwrap_err();
        assert!(
            matches!(err, AwareError::Validation(_)),
            "expected Validation, got {err:?}"
        );
        // The runaway wrote a directory per level; nothing that deep may exist.
        let deep = voices_dir(&ctx).join("s/p/1.0.0/up/pack/up");
        assert!(
            !deep.exists(),
            "the walk followed the link into {}",
            deep.display()
        );
    }

    /// `install` joins three fields read out of a file nobody here wrote.
    /// Fencing only the resolver would leave this half open.
    #[test]
    fn a_manifest_cannot_write_its_pack_outside_the_voices_directory() {
        let (tmp, ctx) = ctx_with(&[]);
        let src = tmp.path().join("pack");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(
            src.join("manifest.yaml"),
            "id: ../../../pwned\nversion: 1.0.0\nscope: ise\n",
        )
        .unwrap();
        std::fs::write(src.join("system-prompt.md"), "you are a reviewer\n").unwrap();

        let err = install(
            &ctx,
            &InstallArgs {
                path: src,
                scope: None,
            },
        )
        .unwrap_err();
        assert!(
            matches!(err, AwareError::Validation(_)),
            "expected Validation, got {err:?}"
        );
        assert_nothing_escaped(tmp.path(), "pwned");
    }

    #[test]
    fn a_traversing_scope_flag_cannot_write_outside_the_voices_directory() {
        let (tmp, ctx) = ctx_with(&[]);
        let src = tmp.path().join("pack");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("manifest.yaml"), "id: reviewer\nversion: 1.0.0\n").unwrap();

        let err = install(
            &ctx,
            &InstallArgs {
                path: src,
                scope: Some("../../../pwned-scope".into()),
            },
        )
        .unwrap_err();
        assert!(
            matches!(err, AwareError::Validation(_)),
            "expected Validation, got {err:?}"
        );
        assert_nothing_escaped(tmp.path(), "pwned-scope");
    }

    /// The install-side containment call, and ONLY it.
    ///
    /// `--scope ise` is a plain segment, so `fence_segment` waves it through;
    /// the link sits at `voices/ise`, so the whole escape is in what the path
    /// resolves to. Deleting the `contained_in_voices` call in `install` left
    /// the entire suite green before this existed — the three tests that did
    /// redden under that mutation were every one of them resolve/uninstall
    /// tests, so the reason commit `9ada65e4` exists had no coverage at all.
    #[cfg(unix)]
    #[test]
    fn a_symlinked_scope_directory_cannot_smuggle_an_install_outside_voices() {
        let (tmp, ctx) = ctx_with(&[]);
        let outside = tmp.path().join("elsewhere");
        std::fs::create_dir_all(&outside).unwrap();
        std::fs::create_dir_all(voices_dir(&ctx)).unwrap();
        std::os::unix::fs::symlink(&outside, voices_dir(&ctx).join("ise")).unwrap();

        let src = tmp.path().join("pack");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("manifest.yaml"), "id: reviewer\nversion: 1.0.0\n").unwrap();
        std::fs::write(src.join("system-prompt.md"), "you are a reviewer\n").unwrap();

        let err = install(
            &ctx,
            &InstallArgs {
                path: src,
                scope: Some("ise".into()),
            },
        )
        .unwrap_err();
        assert!(
            matches!(err, AwareError::Validation(_)),
            "expected Validation, got {err:?}"
        );
        // Not one byte through the link — the assertion the exit code cannot make.
        assert!(
            !outside.join("reviewer/1.0.0/system-prompt.md").exists(),
            "the pack was written through the symlinked scope directory"
        );
        // And the rollback swept the empty directories `create_dir_all` made at
        // the link's target on the way.
        assert!(
            !outside.join("reviewer").exists(),
            "a refused install left {} behind",
            outside.join("reviewer").display()
        );
    }

    /// A refused install must leave NOTHING, because `list` reads the directory
    /// tree rather than a manifest: a bare `voices/s/p/1.0.0/` is reported as an
    /// installed pack, and `describe` will print a manifest for one whose
    /// `references/` never arrived. Which of the two happens is decided by
    /// `read_dir` order, so the husk is the bug either way.
    #[cfg(unix)]
    #[test]
    fn a_refused_install_leaves_no_half_written_pack_behind() {
        let (tmp, ctx) = ctx_with(&[]);
        let src = tmp.path().join("pack");
        std::fs::create_dir_all(src.join("references")).unwrap();
        std::fs::write(src.join("manifest.yaml"), "id: p\nversion: 1.0.0\n").unwrap();
        std::fs::write(src.join("system-prompt.md"), "you are a reviewer\n").unwrap();
        std::fs::write(src.join("references/bs5950.md"), "clause 4\n").unwrap();
        std::os::unix::fs::symlink("/etc", src.join("references-link")).unwrap();

        let err = install(
            &ctx,
            &InstallArgs {
                path: src,
                scope: Some("s".into()),
            },
        )
        .unwrap_err();
        assert!(
            matches!(err, AwareError::Validation(_)),
            "expected Validation, got {err:?}"
        );
        assert!(
            !voices_dir(&ctx).join("s").exists(),
            "a refused install left {} standing — `list` reports that as installed",
            voices_dir(&ctx).join("s").display()
        );
    }

    /// A refused re-install does not DELETE the pack it would have replaced.
    ///
    /// Deliberately narrow, and named for what it actually pins. An earlier
    /// version of this test claimed the whole "undone as a unit" property and
    /// asserted only `manifest.yaml.is_file()` — on a file the install itself
    /// rewrites, with the fixture's manifest and the source's manifest written
    /// byte-identical, so it could not have failed. Two reviewers measured what
    /// it was passing over: the refused install had already overwritten the live
    /// system prompt and left stray files in the pack.
    ///
    /// That corruption is real and is NOT fixed — see `install`'s comment. Since
    /// a green test must not imply otherwise, this asserts only the one guarantee
    /// that does hold (nothing was deleted) and
    /// `a_refused_reinstall_leaves_the_live_pack_modified_in_place` below pins the
    /// hazard that remains, so neither can be mistaken for the other.
    #[cfg(unix)]
    #[test]
    fn a_refused_reinstall_does_not_delete_the_pack_it_would_have_replaced() {
        let (tmp, ctx) = ctx_with(&[("s", "p", "1.0.0")]);
        let live = voices_dir(&ctx).join("s/p/1.0.0");
        std::fs::write(live.join("keep-me.md"), "ORIGINAL\n").unwrap();

        let src = tmp.path().join("pack");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("manifest.yaml"), "id: p\nversion: 1.0.0\n").unwrap();
        std::os::unix::fs::symlink("/etc", src.join("zz-link")).unwrap();

        assert!(
            install(
                &ctx,
                &InstallArgs {
                    path: src,
                    scope: Some("s".into()),
                },
            )
            .is_err()
        );
        assert!(
            live.join("manifest.yaml").is_file(),
            "the rollback deleted the pack that was already installed"
        );
        // A file the incoming pack does not carry, so only a delete could remove
        // it — unlike `manifest.yaml`, which the copy overwrites.
        assert_eq!(
            std::fs::read_to_string(live.join("keep-me.md")).unwrap(),
            "ORIGINAL\n",
            "the rollback removed a file the failed re-install never wrote"
        );
    }

    /// The hazard that is NOT fixed, pinned so it cannot be forgotten and so the
    /// day someone makes `install` stage-and-rename, this test fails and tells
    /// them to delete it.
    ///
    /// `copy_dir_recursive` writes into the live directory, so a re-install that
    /// is refused partway leaves the installed pack a blend of old and new, with
    /// `describe` serving the REFUSED pack's content at exit 0. Rolling back is
    /// not the answer either — that would delete a working pack over a bad input.
    /// The fix is to stage into a sibling and rename, which is a behaviour change
    /// owed its own PR.
    #[cfg(unix)]
    #[test]
    fn a_refused_reinstall_leaves_the_live_pack_modified_in_place() {
        let (tmp, ctx) = ctx_with(&[("s", "p", "1.0.0")]);
        let live = voices_dir(&ctx).join("s/p/1.0.0");
        std::fs::write(live.join("system-prompt.md"), "ORIGINAL PROMPT\n").unwrap();

        let src = tmp.path().join("pack");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("manifest.yaml"), "id: p\nversion: 1.0.0\n").unwrap();
        std::fs::write(src.join("system-prompt.md"), "REPLACEMENT PROMPT\n").unwrap();
        std::os::unix::fs::symlink("/etc", src.join("zz-link")).unwrap();

        assert!(
            install(
                &ctx,
                &InstallArgs {
                    path: src,
                    scope: Some("s".into()),
                },
            )
            .is_err()
        );
        assert_eq!(
            std::fs::read_to_string(live.join("system-prompt.md")).unwrap(),
            "REPLACEMENT PROMPT\n",
            "install now appears to be atomic on the re-install path — if that is \
             deliberate, delete this test and the carve-out in `install`'s comment"
        );
    }

    /// The rollback must not be able to swallow a pack another process installed
    /// while this one was copying.
    ///
    /// The first cut walked up to the topmost directory that had not existed
    /// before the install and `remove_dir_all`-ed it — which on a fresh home is
    /// `voices/` itself. A concurrent install that COMPLETED was then deleted by
    /// the failing one's cleanup. Simulated here without threads by creating the
    /// neighbour during the window: the refusal is what triggers cleanup, so a
    /// neighbour present at that moment is exactly the race.
    #[cfg(unix)]
    #[test]
    fn the_rollback_cannot_delete_a_pack_installed_alongside_it() {
        let (tmp, ctx) = ctx_with(&[]);
        let src = tmp.path().join("pack");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("manifest.yaml"), "id: p\nversion: 1.0.0\n").unwrap();
        std::os::unix::fs::symlink("/etc", src.join("zz-link")).unwrap();

        // Stands in for the concurrent install: a completed pack under a
        // different scope, sitting in the `voices/` the failing install created.
        let neighbour = voices_dir(&ctx).join("other/q/1.0.0");
        std::fs::create_dir_all(&neighbour).unwrap();
        std::fs::write(neighbour.join("manifest.yaml"), "id: q\n").unwrap();

        assert!(
            install(
                &ctx,
                &InstallArgs {
                    path: src,
                    scope: Some("s".into()),
                },
            )
            .is_err()
        );
        assert!(
            neighbour.join("manifest.yaml").is_file(),
            "the rollback deleted a pack installed alongside it at {}",
            neighbour.display()
        );
        assert!(
            voices_dir(&ctx).is_dir(),
            "the rollback removed the voices/ root itself"
        );
        // Its own litter is still gone.
        assert!(!voices_dir(&ctx).join("s").exists());
    }

    /// A home reached THROUGH a symlink must keep working, end to end.
    ///
    /// This is the boundary `contained_in_voices`'s doc draws: the canonical root
    /// absorbs anything above `voices/`, so links there are fine and only the
    /// coordinate's own components are constrained. Nothing pinned it, and the
    /// hole was not theoretical — replacing `root.join(rel)` with
    /// `resolved.to_path_buf()` left the ENTIRE suite green while making every
    /// `voice` command refuse permanently for any user whose `~` or `~/.aware`
    /// sits behind a link. That is macOS's default `TMPDIR` shape (`/var` →
    /// `/private/var`) and an ordinary dotfile-manager layout on Linux.
    #[cfg(unix)]
    #[test]
    fn a_home_reached_through_a_symlink_still_installs_resolves_and_uninstalls() {
        let tmp = tempfile::tempdir().unwrap();
        let real = tmp.path().join("real-home");
        std::fs::create_dir_all(&real).unwrap();
        let linked = tmp.path().join("home-link");
        std::os::unix::fs::symlink(&real, &linked).unwrap();
        let ctx = Context {
            paths: Paths {
                aware_home: linked.clone(),
            },
            json: false,
        };

        let src = tmp.path().join("pack");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("manifest.yaml"), "id: reviewer\nversion: 1.0.0\n").unwrap();
        std::fs::write(src.join("system-prompt.md"), "you are a reviewer\n").unwrap();

        install(
            &ctx,
            &InstallArgs {
                path: src,
                scope: Some("ise".into()),
            },
        )
        .expect("install refused a home that is merely reached through a symlink");
        let resolved = resolve_pack_dir(&ctx, "ise/reviewer@1.0.0")
            .expect("resolve refused a symlinked home after install accepted it");
        assert!(resolved.join("system-prompt.md").is_file());
        uninstall(&ctx, "ise/reviewer@1.0.0").expect("uninstall refused a symlinked home");
        assert!(!real.join("voices/ise/reviewer/1.0.0").exists());
    }

    /// The rollback is bounded by `voices/` and cannot climb above `AWARE_HOME`.
    /// The first cut removed directories outside the home entirely when the
    /// home's own parents did not exist yet.
    #[cfg(unix)]
    #[test]
    fn the_rollback_cannot_climb_above_the_aware_home() {
        let tmp = tempfile::tempdir().unwrap();
        let outer = tmp.path().join("outer");
        let home = outer.join("nested").join("aware");
        std::fs::create_dir_all(&outer).unwrap();
        let ctx = Context {
            paths: Paths {
                aware_home: home.clone(),
            },
            json: false,
        };

        let src = tmp.path().join("pack");
        std::fs::create_dir_all(&src).unwrap();
        std::fs::write(src.join("manifest.yaml"), "id: p\nversion: 1.0.0\n").unwrap();
        std::os::unix::fs::symlink("/etc", src.join("zz-link")).unwrap();

        assert!(
            install(
                &ctx,
                &InstallArgs {
                    path: src,
                    scope: Some("s".into()),
                },
            )
            .is_err()
        );
        assert!(
            outer.is_dir(),
            "the rollback removed {} — above AWARE_HOME",
            outer.display()
        );
        assert!(
            home.join("voices").is_dir(),
            "the rollback removed the voices/ root itself"
        );
    }

    /// The point of the fence is that legal packs still install, and install
    /// WHERE they say. Also pins the YAML-scalar coercion: `version: 2025`
    /// parses as a number, and a string-only read would reject it as missing.
    #[test]
    fn a_plain_pack_installs_under_its_own_coordinate() {
        let (tmp, ctx) = ctx_with(&[]);
        let src = tmp.path().join("pack");
        std::fs::create_dir_all(src.join("references")).unwrap();
        std::fs::write(
            src.join("manifest.yaml"),
            "id: uk-reviewer\nversion: 2025\n",
        )
        .unwrap();
        std::fs::write(src.join("system-prompt.md"), "you are a reviewer\n").unwrap();
        std::fs::write(src.join("references").join("bs5950.md"), "clause 4\n").unwrap();

        install(
            &ctx,
            &InstallArgs {
                path: src,
                scope: Some("ise".into()),
            },
        )
        .unwrap();

        let dst = voices_dir(&ctx)
            .join("ise")
            .join("uk-reviewer")
            .join("2025");
        assert!(dst.join("system-prompt.md").is_file(), "{}", dst.display());
        assert!(
            dst.join("references").join("bs5950.md").is_file(),
            "copy_dir_recursive skipped the nested references/ folder"
        );
        // And it is addressable afterwards by the coordinate it was filed under.
        // Both sides canonicalised: `resolve_pack_dir` returns the canonical
        // path, and comparing it against the spelled `dst` fails wherever
        // `TMPDIR` is itself a link — which is the macOS default (`/var` ->
        // `/private/var`), and macOS is a release target (`release.yml`) even
        // though the test job is ubuntu-only.
        assert_eq!(
            resolve_pack_dir(&ctx, "@ise/uk-reviewer@2025").unwrap(),
            dst.canonicalize().unwrap()
        );
    }

    #[test]
    fn uninstall_removes_one_version_and_leaves_its_siblings() {
        let (_tmp, ctx) = ctx_with(&[
            ("ise", "reviewer", "1.0.0"),
            ("ise", "reviewer", "2.0.0"),
            ("aware-aeco", "structural", "1.0.0"),
        ]);
        uninstall(&ctx, "ise/reviewer@1.0.0").unwrap();

        let voices = voices_dir(&ctx);
        assert!(!voices.join("ise/reviewer/1.0.0").exists());
        assert!(
            voices.join("ise/reviewer/2.0.0").is_dir(),
            "uninstalling one version took the other with it"
        );
        assert!(
            voices.join("aware-aeco/structural/1.0.0").is_dir(),
            "uninstalling one pack took an unrelated pack with it"
        );
    }

    #[test]
    fn a_double_digit_component_ranks_above_a_single_digit_one() {
        // #377: pack folders were compared as strings, so `"2025.10" < "2025.9"` and a
        // pack lost to its own predecessor the moment a component reached double digits.
        // These are the folder shapes this registry actually publishes.
        for (lo, hi) in [
            ("2025.9", "2025.10"),
            ("2025", "2025.1"),
            ("2025.0.9", "2025.0.10"),
            ("1.9", "1.10"),
            ("9", "10"),
        ] {
            assert_eq!(
                compare_dot_components(lo, hi),
                Less,
                "{lo} must rank below {hi} — a string compare says the opposite"
            );
            assert_eq!(compare_dot_components(hi, lo), Greater);
            assert_eq!(compare_dot_components(lo, lo), Equal);
        }
    }

    #[test]
    fn a_non_numeric_component_still_has_a_defined_order() {
        use std::cmp::Ordering::*;
        // Folder names are not validated, so the comparator must total-order whatever is
        // on disk rather than panic or call two different names equal. Numeric below
        // alphanumeric is SemVer §11's rule, kept because this IS that function.
        assert_eq!(compare_dot_components("2025", "beta"), Less);
        assert_eq!(compare_dot_components("alpha", "beta"), Less);
        assert_eq!(compare_dot_components("beta", "beta"), Equal);
    }

    /// Build `<home>/voices/<scope>/<id>/<version>/` for each version, then ask the real
    /// resolver which one it picks.
    fn resolve_latest(versions: &[&str]) -> String {
        let tmp = tempfile::tempdir().unwrap();
        let parent = tmp.path().join("voices").join("acme").join("reviewer");
        for v in versions {
            std::fs::create_dir_all(parent.join(v)).unwrap();
        }
        let ctx = Context {
            paths: crate::paths::Paths {
                aware_home: tmp.path().to_path_buf(),
            },
            json: false,
        };
        let dir = resolve_pack_dir(&ctx, "acme/reviewer").unwrap();
        dir.file_name().unwrap().to_string_lossy().into_owned()
    }

    #[test]
    fn the_resolver_picks_the_newest_pack_not_the_lexically_greatest() {
        // The issue's repro, through the function `aware voice` actually calls.
        assert_eq!(resolve_latest(&["2025.9", "2025.10"]), "2025.10");

        // The DISPLACEMENT branch — a later entry beating the incumbent — needs a case
        // where the winner is discovered SECOND, and creation order does not give you
        // that: NTFS returns `read_dir` in collation order whatever order you create in
        // (measured), so `["2025.10", "2025.9"]` enumerates identically to the line
        // above. `2026.1` sorts after `2025.9`, so it is genuinely found last. Without
        // this, "always keep the first directory" passes every assertion here.
        assert_eq!(resolve_latest(&["2025.9", "2026.1"]), "2026.1");

        // A single version is still found — the first candidate wins outright now. It
        // could not win by comparison: `compare_pack_versions("2025.1", "")` is Less.
        assert_eq!(resolve_latest(&["2025.1"]), "2025.1");
    }

    #[test]
    fn a_semver_shaped_pack_orders_by_semver_not_by_components() {
        // The first cut of this fix regressed a shape `main` had RIGHT. Splitting
        // `1.0.0-rc.1` on '.' yields `["1","0","0-rc","1"]`, and §11's "numeric ranks
        // below alphanumeric" then makes `0-rc` beat the `1` of `1.0.1` — a release
        // candidate outranking a higher patch. Nothing stops a pack shipping a semver
        // version: `install` copies whatever its manifest says.
        assert_eq!(resolve_latest(&["1.0.0-rc.1", "1.0.1"]), "1.0.1");
        // …and a release still outranks its own prerelease (§11).
        assert_eq!(resolve_latest(&["1.0.0-rc.1", "1.0.0"]), "1.0.0");
        // The component path is still what handles the folder shapes semver cannot
        // parse — that is the whole issue, and it must not regress to satisfy the above.
        assert_eq!(resolve_latest(&["2025.9", "2025.10"]), "2025.10");
    }

    #[test]
    fn names_that_compare_equal_still_resolve_deterministically() {
        // `2025.01` and `2025.1` are Equal component-wise (both parse as 1), so without
        // a tiebreak the winner would be decided by `read_dir` order — the same
        // non-determinism this fix exists to remove. The raw-string fallback settles it.
        let first = resolve_latest(&["2025.01", "2025.1"]);
        assert_eq!(first, resolve_latest(&["2025.1", "2025.01"]));
        assert_eq!(
            first, "2025.1",
            "the tiebreak is the raw name, and `1` > `01`"
        );
    }

    #[test]
    fn an_unpinned_uninstall_removes_the_newest_pack() {
        // `resolve_pack_dir` is shared with `uninstall`, where the resolved path goes
        // straight to `remove_dir_all` — so this fix silently changed which directory an
        // unpinned `aware voice uninstall` DELETES (it used to take 2025.9). Pinned
        // there, because a destructive command changing target deserves a test rather
        // than a paragraph.
        let tmp = tempfile::tempdir().unwrap();
        let parent = tmp.path().join("voices").join("acme").join("reviewer");
        for v in ["2025.9", "2025.10"] {
            std::fs::create_dir_all(parent.join(v)).unwrap();
        }
        let ctx = Context {
            paths: crate::paths::Paths {
                aware_home: tmp.path().to_path_buf(),
            },
            json: false,
        };
        let doomed = resolve_pack_dir(&ctx, "acme/reviewer").unwrap();
        assert_eq!(doomed.file_name().unwrap(), "2025.10");
        assert!(
            parent.join("2025.9").is_dir(),
            "the older pack is untouched"
        );
    }
}
