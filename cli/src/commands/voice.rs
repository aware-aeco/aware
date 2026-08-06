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

/// Resolve a pack identifier like `@ise/uk-structural-reviewer@2025`,
/// `ise/uk-structural-reviewer/2025`, or `aware-aeco/structural-engineer`
/// to a folder path under `~/.aware/voices/`. Picks the latest installed
/// version if none specified.
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
            return Ok(p);
        }
        return Err(AwareError::NotFound(format!(
            "voice pack {scope}/{id}@{v} not installed"
        )));
    }
    // Pick the lexically-latest version (good enough for semver-tagged folders).
    let mut latest: Option<PathBuf> = None;
    let mut latest_name = String::new();
    if let Ok(entries) = std::fs::read_dir(&parent) {
        for entry in entries.flatten() {
            if !entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                continue;
            }
            let name = entry.file_name().to_string_lossy().into_owned();
            if name > latest_name {
                latest_name = name;
                latest = Some(entry.path());
            }
        }
    }
    latest.ok_or_else(|| AwareError::NotFound(format!("no installed versions of {scope}/{id}")))
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

fn copy_dir_recursive(src: &std::path::Path, dst: &std::path::Path) -> Result<(), AwareError> {
    for entry in std::fs::read_dir(src)
        .map_err(|e| AwareError::Internal(format!("read_dir {}: {e}", src.display())))?
        .flatten()
    {
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if from.is_dir() {
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

    /// A voices tree with the given `scope/id/version` folders, plus an
    /// `apps/keep-me/` sibling that no voice operation should ever touch —
    /// it is what makes the escape assertions below say something.
    fn ctx_with(packs: &[(&str, &str, &str)]) -> (tempfile::TempDir, Context) {
        let tmp = tempfile::tempdir().unwrap();
        let home = tmp.path().join("aware");
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
                Err(_) => {}
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
}
