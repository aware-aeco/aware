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
                Some(best) => {
                    crate::validate::compare_dot_components(&name, best)
                        == std::cmp::Ordering::Greater
                }
                // The first directory wins by default rather than by comparison: an
                // empty-string sentinel made this "is it greater than nothing", which
                // a legitimately named version can only lose against by accident.
                None => true,
            };
            if newer {
                latest_name = Some(name);
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
    use crate::validate::compare_dot_components;
    use std::cmp::Ordering::*;

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
        // Order of discovery must not decide it: `read_dir` order is not guaranteed, and
        // the fix replaced an empty-string sentinel with a first-wins branch.
        assert_eq!(resolve_latest(&["2025.10", "2025.9"]), "2025.10");
        // A single version is still found — the first candidate wins by default now,
        // rather than by comparing greater than "".
        assert_eq!(resolve_latest(&["2025.1"]), "2025.1");
    }
}
