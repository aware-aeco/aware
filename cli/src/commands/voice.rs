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
/// A symlinked LEAF is harmless — `remove_dir_all` unlinks the link and leaves
/// the target alone — which is exactly why testing only the leaf case reads as
/// "symlinks are fine" and is wrong. Reproduced against the real binary.
///
/// So: canonicalise, and demand the result still sit under a canonicalised
/// `voices/`. Returns the CANONICAL path, so the caller acts on what it checked
/// rather than on the spelling that got it here.
fn contained_in_voices(ctx: &Context, resolved: &std::path::Path) -> Result<PathBuf, AwareError> {
    let root = voices_dir(ctx).canonicalize().map_err(|e| {
        AwareError::Internal(format!("canonicalise {}: {e}", voices_dir(ctx).display()))
    })?;
    let real = resolved
        .canonicalize()
        .map_err(|e| AwareError::Internal(format!("canonicalise {}: {e}", resolved.display())))?;
    if !real.starts_with(&root) {
        return Err(AwareError::Validation(format!(
            "voice pack path {} resolves to {}, outside {} — refusing, because a \
             symlink cannot be allowed to move what `install` writes or what \
             `uninstall` deletes",
            resolved.display(),
            real.display(),
            root.display()
        )));
    }
    Ok(real)
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
    std::fs::create_dir_all(&dst)
        .map_err(|e| AwareError::Internal(format!("create {}: {e}", dst.display())))?;
    // Fenced lexically above, but a symlink already sitting at `voices/<scope>`
    // would still put the copy somewhere else entirely. Checked after
    // `create_dir_all` (which needs the path to exist to canonicalise) and
    // BEFORE any file is written, so an escape costs empty directories rather
    // than the pack's contents.
    let dst = contained_in_voices(ctx, &dst)?;
    copy_dir_recursive(src, &dst)?;
    println!(
        "\u{2713} installed voice pack {scope}/{id}@{version} \u{2192} {}",
        dst.display()
    );
    Ok(())
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

    /// A symlinked LEAF is the harmless case — `remove_dir_all` unlinks the
    /// link and leaves the target alone. Pinned so the distinction survives:
    /// testing only this shape is what made "symlinks are fine" look true.
    #[test]
    fn a_symlinked_pack_version_removes_the_link_not_the_target() {
        let (tmp, ctx) = ctx_with(&[]);
        let outside = tmp.path().join("outside");
        std::fs::create_dir_all(&outside).unwrap();
        std::fs::write(outside.join("data.txt"), "keep\n").unwrap();
        let parent = voices_dir(&ctx).join("ise").join("reviewer");
        std::fs::create_dir_all(&parent).unwrap();
        #[cfg(unix)]
        std::os::unix::fs::symlink(&outside, parent.join("1.0.0")).unwrap();
        #[cfg(windows)]
        std::os::windows::fs::symlink_dir(&outside, parent.join("1.0.0")).unwrap();

        // The leaf canonicalises outside voices/, so the containment check
        // refuses it too — and the target is untouched either way.
        assert!(resolve_pack_dir(&ctx, "ise/reviewer@1.0.0").is_err());
        assert!(
            outside.join("data.txt").is_file(),
            "the symlink target's contents were destroyed"
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
        assert!(
            !tmp.path().join("pwned").exists(),
            "the pack escaped to {}",
            tmp.path().join("pwned").display()
        );
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
        assert!(!tmp.path().join("pwned-scope").exists());
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
        assert_eq!(
            resolve_pack_dir(&ctx, "@ise/uk-reviewer@2025").unwrap(),
            dst
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
