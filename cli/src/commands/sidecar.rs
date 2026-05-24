//! `aware sidecar` — manage host-execution bridge binaries.
//!
//! Host-execution bridges (`aware-tekla`, `aware-rhino`, `aware-sketchup`,
//! `aware-revit`) are separate Windows binaries that the CLI spawns when an
//! agent declares `transport: cli: binary: aware-<host>`. They are NOT
//! bundled in the main release zip (they are Windows-only and some ship with
//! platform-specific assets), so they are published as separate release
//! assets and fetched on demand by this command group.
//!
//! # Commands
//!
//! ```
//! aware sidecar list                 # show which bridges are present / missing
//! aware sidecar install <host>       # download + install the bridge binary
//! aware sidecar uninstall <host>     # remove the bridge binary
//! ```
//!
//! # Install location
//!
//! Bridges are placed in `~/.aware/bridges/` — a persistent, version-independent
//! directory that survives `npm install -g` upgrades (#148). The runtime resolves
//! them by absolute path, so they do not need to be on PATH.

use std::io::Write as _;
use std::path::PathBuf;

use clap::Subcommand;

use crate::context::Context;
use crate::error::AwareError;

/// Known host bridges. Each entry: (host-id, binary-name, asset-suffix)
/// The asset name in a GitHub release is:
///   `<binary-name>-<version>-win-x64.<ext>`
/// where ext is zip (tekla, sketchup) or exe (rhino, revit sidecar).
const BRIDGES: &[Bridge] = &[
    Bridge {
        id: "tekla",
        binary: "aware-tekla",
        asset_kind: AssetKind::Zip,
        description: "Tekla Structures 2026 (.NET Framework 4.8, Roslyn scripting)",
        note: None,
    },
    Bridge {
        id: "rhino",
        binary: "aware-rhino",
        asset_kind: AssetKind::Exe,
        description: "Rhino 8 / Grasshopper (net10, single-file, wraps rhinocode CLI)",
        note: None,
    },
    Bridge {
        id: "sketchup",
        binary: "aware-sketchup",
        asset_kind: AssetKind::Zip,
        description: "SketchUp 2026 (net10, single-file + Ruby bridge assets)",
        note: Some("Load the Ruby plugin: run `\"{dir}/aware-sketchup.exe\" --install-bridge`"),
    },
    Bridge {
        id: "revit",
        binary: "aware-revit",
        asset_kind: AssetKind::Zip,
        description: "Revit 2026 (net8 sidecar + IExternalApplication add-in)",
        note: Some("Register the Revit add-in: run `pwsh \"{dir}/install-addin.ps1\"`"),
    },
];

#[derive(Debug)]
struct Bridge {
    id: &'static str,
    binary: &'static str,
    asset_kind: AssetKind,
    description: &'static str,
    note: Option<&'static str>,
}

#[derive(Clone, Copy, Debug)]
enum AssetKind {
    Exe,
    Zip,
}

impl AssetKind {
    fn ext(self) -> &'static str {
        match self {
            AssetKind::Exe => "exe",
            AssetKind::Zip => "zip",
        }
    }
}

#[derive(Subcommand, Debug)]
pub enum SidecarCommand {
    /// Show installed and missing host bridges.
    List,

    /// Download and install a host bridge binary.
    ///
    /// Downloads from the GitHub release matching the current `aware` version
    /// and installs into `~/.aware/bridges/` (persistent across CLI upgrades).
    /// Accepted host IDs: tekla, rhino, sketchup, revit.
    Install {
        /// Host bridge ID: `tekla`, `rhino`, `sketchup`, or `revit`.
        host: String,
    },

    /// Remove an installed host bridge binary.
    Uninstall {
        /// Host bridge ID to remove.
        host: String,
    },
}

pub fn dispatch(action: SidecarCommand, ctx: &Context) -> Result<(), AwareError> {
    match action {
        SidecarCommand::List => list(ctx),
        SidecarCommand::Install { host } => install(ctx, &host),
        SidecarCommand::Uninstall { host } => uninstall(ctx, &host),
    }
}

// ── list ─────────────────────────────────────────────────────────────────────

fn list(ctx: &Context) -> Result<(), AwareError> {
    let install_dir = bridge_install_dir(ctx);
    println!("Host bridges  (install dir: {})", install_dir.display());
    println!();
    let version = env!("CARGO_PKG_VERSION");
    for b in BRIDGES {
        match find_bridge_in_dir(b, &install_dir) {
            Some(p) if bridge_is_current(b, &install_dir, version) => {
                println!("  \u{2713} {:<12}  {}", b.id, p.display());
            }
            Some(p) => {
                // Present in the managed dir but from a different CLI version.
                println!(
                    "  \u{21bb} {:<12}  {}  (other version — run: aware sidecar install {})",
                    b.id,
                    p.display(),
                    b.id
                );
            }
            None => {
                // Not in the managed dir — a legacy copy on PATH still spawns, but
                // it's outside ~/.aware/bridges and will be wiped on the next npm
                // upgrade (#148), so prompt migration rather than a clean check.
                if let Some(legacy) = which_binary(b.binary) {
                    println!(
                        "  \u{26a0} {:<12}  {}  (legacy/on PATH — run: aware sidecar install {} to persist it)",
                        b.id,
                        legacy.display(),
                        b.id
                    );
                } else {
                    println!(
                        "  \u{2717} {:<12}  not found — run: aware sidecar install {}",
                        b.id, b.id
                    );
                }
            }
        }
        println!("               {}", b.description);
        if let Some(msg) = note_message(b, &install_dir) {
            println!("               \u{26a0} {msg}");
        }
        println!();
    }
    Ok(())
}

// ── install ───────────────────────────────────────────────────────────────────

fn install(ctx: &Context, host: &str) -> Result<(), AwareError> {
    let bridge = lookup_bridge(host)?;
    let install_dir = bridge_install_dir(ctx);
    std::fs::create_dir_all(&install_dir).map_err(|e| {
        AwareError::Internal(format!("create {}: {e}", install_dir.display()))
    })?;
    let version = env!("CARGO_PKG_VERSION");

    // Already installed in the persistent dir AND matching this CLI version? Skip.
    // Two reasons we check the version, not just presence:
    //  - A legacy on-PATH copy must NOT satisfy this (we check the dir only), or
    //    the bridge never migrates and the next npm upgrade wipes it (#148).
    //  - Release assets are versioned per CLI release, so a bridge left over from
    //    an older `aware` must be refreshed after upgrade rather than reused stale.
    if bridge_is_current(bridge, &install_dir, version) {
        println!(
            "\u{2713} {} already installed (v{version}) in {}",
            bridge.binary,
            install_dir.display()
        );
        print_note(bridge, &install_dir);
        return Ok(());
    }

    let asset_name = format!(
        "{}-{}-win-x64.{}",
        bridge.binary,
        version,
        bridge.asset_kind.ext()
    );
    let url = format!(
        "https://github.com/aware-aeco/aware/releases/download/v{version}/{asset_name}"
    );

    println!("Downloading {} from:", bridge.binary);
    println!("  {url}");

    let bytes = http_get_bytes(&url)?;

    match bridge.asset_kind {
        AssetKind::Exe => {
            let dest = install_dir.join(format!("{}.exe", bridge.binary));
            std::fs::write(&dest, &bytes)
                .map_err(|e| AwareError::Internal(format!("write {}: {e}", dest.display())))?;
            println!("\u{2713} Installed {} to {}", bridge.binary, dest.display());
        }
        AssetKind::Zip => {
            extract_zip(&bytes, &install_dir, bridge.binary)?;
            println!(
                "\u{2713} Extracted {} to {}",
                bridge.binary,
                install_dir.display()
            );
        }
    }

    // Stamp the installed version so a later CLI upgrade refreshes the bridge.
    std::fs::write(version_marker_path(&install_dir, bridge.binary), version).map_err(|e| {
        AwareError::Internal(format!("write version marker for {}: {e}", bridge.binary))
    })?;

    print_note(bridge, &install_dir);
    Ok(())
}

/// Path of the `<binary>.version` marker recording which CLI release installed
/// the bridge currently in `install_dir`.
fn version_marker_path(install_dir: &std::path::Path, binary: &str) -> PathBuf {
    install_dir.join(format!("{binary}.version"))
}

/// The CLI version recorded for an installed bridge, if any.
fn installed_bridge_version(install_dir: &std::path::Path, binary: &str) -> Option<String> {
    std::fs::read_to_string(version_marker_path(install_dir, binary))
        .ok()
        .map(|s| s.trim().to_string())
}

/// Whether the bridge is present in the managed dir AND was installed by this
/// CLI version. A missing or mismatched marker counts as not-current, so install
/// re-downloads the matching release asset.
fn bridge_is_current(bridge: &Bridge, install_dir: &std::path::Path, version: &str) -> bool {
    find_bridge_in_dir(bridge, install_dir).is_some()
        && installed_bridge_version(install_dir, bridge.binary).as_deref() == Some(version)
}

/// Whether a managed-dir bridge for `binary` exists but was installed by a
/// *different* CLI version (or has no version marker) — i.e. it should be
/// refreshed via `aware sidecar install`. `false` for a PATH-only/legacy copy,
/// an absent bridge, or an unknown binary name. Consulted at spawn time so the
/// runtime can warn before running a potentially mismatched sidecar.
pub fn managed_bridge_is_stale(binary: &str, install_dir: &std::path::Path, version: &str) -> bool {
    let Some(b) = BRIDGES.iter().find(|b| b.binary == binary) else {
        return false;
    };
    find_bridge_in_dir(b, install_dir).is_some()
        && installed_bridge_version(install_dir, b.binary).as_deref() != Some(version)
}

/// Print a bridge's post-install note, resolving `{dir}` to the (off-PATH)
/// install directory so the host-registration command is actually runnable.
fn print_note(bridge: &Bridge, install_dir: &std::path::Path) {
    if let Some(msg) = note_message(bridge, install_dir) {
        println!("\u{26a0} {msg}");
    }
}

/// Build the resolved post-install note text, substituting `{dir}` with the
/// install directory. `None` for bridges without a note.
fn note_message(bridge: &Bridge, install_dir: &std::path::Path) -> Option<String> {
    bridge
        .note
        .map(|n| n.replace("{dir}", &install_dir.display().to_string()))
}

// ── uninstall ─────────────────────────────────────────────────────────────────

fn uninstall(ctx: &Context, host: &str) -> Result<(), AwareError> {
    let bridge = lookup_bridge(host)?;
    let install_dir = bridge_install_dir(ctx);

    // Only remove what we manage in the persistent dir — never a PATH/legacy copy.
    let path = find_bridge_in_dir(bridge, &install_dir)
        .ok_or_else(|| AwareError::NotFound(format!("{} is not installed", bridge.binary)))?;

    std::fs::remove_file(&path)
        .map_err(|e| AwareError::Internal(format!("remove {}: {e}", path.display())))?;
    // Best-effort: drop the version marker too so it doesn't linger.
    let _ = std::fs::remove_file(version_marker_path(&install_dir, bridge.binary));
    println!("\u{2713} Removed {}", path.display());
    Ok(())
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// Resolve the directory where bridge binaries are installed.
///
/// `~/.aware/bridges/` — a persistent, version-independent location alongside the
/// rest of `~/.aware` durable state. Earlier versions placed bridges next to the
/// running `aware` binary (so they landed on PATH), but for npm installs that is
/// the package's own dir, which `npm install -g` wipes wholesale on every upgrade
/// — silently dropping every installed bridge (#148). Since the runtime resolves
/// bridges by absolute path (see `find_bridge_by_binary`), they no longer need to
/// be on PATH.
pub fn bridge_install_dir(ctx: &Context) -> PathBuf {
    ctx.paths.aware_home.join("bridges")
}

/// Find a bridge binary on disk by host id (e.g. `tekla`). Checks
/// `install_dir/<binary>.exe`, the extracted sub-dir, then PATH.
pub fn find_bridge_by_id(id: &str, install_dir: &std::path::Path) -> Option<PathBuf> {
    let b = BRIDGES.iter().find(|b| b.id == id)?;
    find_bridge(b, install_dir)
}

/// Find a bridge binary on disk by binary name (e.g. `aware-tekla`, as it appears
/// in an agent manifest's `transport.cli.binary`). Returns `None` for names that
/// are not known host bridges, so the caller can fall back to PATH resolution.
pub fn find_bridge_by_binary(binary: &str, install_dir: &std::path::Path) -> Option<PathBuf> {
    let b = BRIDGES.iter().find(|b| b.binary == binary)?;
    find_bridge(b, install_dir)
}

/// Find a bridge for RUNTIME resolution: the managed install dir first, then a
/// `which`-style PATH lookup (so a legacy on-PATH bridge still spawns during the
/// migration window).
fn find_bridge(bridge: &Bridge, install_dir: &std::path::Path) -> Option<PathBuf> {
    find_bridge_in_dir(bridge, install_dir).or_else(|| which_binary(bridge.binary))
}

/// Find a bridge ONLY within the managed install dir (`~/.aware/bridges`), never
/// PATH. Used by install/uninstall: a legacy on-PATH copy must NOT make `install`
/// think the bridge is already present (which would skip writing it to the
/// persistent dir and let the next npm upgrade delete the only copy — #148), nor
/// should `uninstall` reach out and delete a binary outside the dir we manage.
fn find_bridge_in_dir(bridge: &Bridge, install_dir: &std::path::Path) -> Option<PathBuf> {
    // Flat: <dir>/<binary>.exe
    let local = install_dir.join(format!("{}.exe", bridge.binary));
    if local.is_file() {
        return Some(local);
    }
    // Sub-dir: tekla/sketchup zips extract to <dir>/<binary>/<binary>.exe
    let subdir = install_dir
        .join(bridge.binary)
        .join(format!("{}.exe", bridge.binary));
    if subdir.is_file() {
        return Some(subdir);
    }
    None
}

fn which_binary(name: &str) -> Option<PathBuf> {
    let name_exe = if cfg!(windows) {
        format!("{name}.exe")
    } else {
        name.to_string()
    };
    std::env::var_os("PATH")
        .map(|paths| std::env::split_paths(&paths).collect::<Vec<_>>())
        .unwrap_or_default()
        .into_iter()
        .map(|dir| dir.join(&name_exe))
        .find(|p| p.is_file())
}

fn lookup_bridge(host: &str) -> Result<&'static Bridge, AwareError> {
    BRIDGES
        .iter()
        .find(|b| b.id == host)
        .ok_or_else(|| {
            let ids: Vec<_> = BRIDGES.iter().map(|b| b.id).collect();
            AwareError::Validation(format!(
                "unknown host '{host}' — known bridges: {}",
                ids.join(", ")
            ))
        })
}

/// Simple HTTP GET returning bytes. Uses `ureq` (already a Cargo dependency).
fn http_get_bytes(url: &str) -> Result<Vec<u8>, AwareError> {
    let resp = ureq::get(url)
        .call()
        .map_err(|e| AwareError::Network(format!("download {url}: {e}")))?;

    let len: usize = resp
        .header("content-length")
        .and_then(|v| v.parse().ok())
        .unwrap_or(8 * 1024 * 1024); // 8 MB default buf

    let mut bytes = Vec::with_capacity(len);
    resp.into_reader()
        .read_to_end(&mut bytes)
        .map_err(|e| AwareError::Network(format!("read body {url}: {e}")))?;
    Ok(bytes)
}

/// Extract `<binary>.exe` (and any sibling files) from an in-memory zip into
/// `dest_dir`. Strips one path component if all entries share a common prefix
/// directory (as produced by `dotnet publish` zips).
fn extract_zip(bytes: &[u8], dest_dir: &std::path::Path, _binary: &str) -> Result<(), AwareError> {
    use std::io::Read as _;
    let cursor = std::io::Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(cursor)
        .map_err(|e| AwareError::Internal(format!("open zip: {e}")))?;

    // Detect common prefix (strip one leading path component if all entries share one).
    let prefix: Option<String> = {
        let first = archive.by_index(0).ok().and_then(|f| {
            let name = f.name().to_string();
            let parts: Vec<_> = name.splitn(2, '/').collect();
            if parts.len() == 2 {
                Some(format!("{}/", parts[0]))
            } else {
                None
            }
        });
        if let Some(ref p) = first {
            let all_share = (0..archive.len()).all(|i| {
                archive
                    .by_index(i)
                    .map(|f| f.name().starts_with(p.as_str()))
                    .unwrap_or(false)
            });
            if all_share { first } else { None }
        } else {
            None
        }
    };

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| AwareError::Internal(format!("zip entry {i}: {e}")))?;

        let entry_name = file.name().to_string();
        let stripped = if let Some(ref pfx) = prefix {
            entry_name.strip_prefix(pfx.as_str()).unwrap_or(&entry_name)
        } else {
            &entry_name
        };

        if stripped.is_empty() || stripped.ends_with('/') {
            continue; // directory entry
        }

        let dest_path = dest_dir.join(stripped.replace('/', std::path::MAIN_SEPARATOR_STR));
        if let Some(parent) = dest_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                AwareError::Internal(format!("mkdir {}: {e}", parent.display()))
            })?;
        }

        let mut out = std::fs::File::create(&dest_path)
            .map_err(|e| AwareError::Internal(format!("create {}: {e}", dest_path.display())))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .map_err(|e| AwareError::Internal(format!("read entry: {e}")))?;
        out.write_all(&buf)
            .map_err(|e| AwareError::Internal(format!("write {}: {e}", dest_path.display())))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lookup_known_bridges() {
        assert!(lookup_bridge("tekla").is_ok());
        assert!(lookup_bridge("rhino").is_ok());
        assert!(lookup_bridge("sketchup").is_ok());
        assert!(lookup_bridge("revit").is_ok());
    }

    #[test]
    fn lookup_unknown_bridge_errors() {
        let err = lookup_bridge("autocad").unwrap_err();
        assert!(err.to_string().contains("autocad"));
        assert!(err.to_string().contains("tekla"));
    }

    #[test]
    fn find_bridge_returns_none_when_missing() {
        let tmp = tempfile::tempdir().unwrap();
        let result = find_bridge_by_id("tekla", tmp.path());
        assert!(result.is_none());
    }

    #[test]
    fn find_bridge_detects_installed_exe() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("aware-tekla.exe"), b"fake").unwrap();
        let result = find_bridge_by_id("tekla", tmp.path());
        assert!(result.is_some());
    }

    #[test]
    fn install_dir_is_persistent_under_aware_home() {
        // Must be ~/.aware/bridges — NOT the (volatile) npm package dir that
        // `npm install -g` wipes on upgrade (#148).
        let tmp = tempfile::tempdir().unwrap();
        let ctx = Context {
            paths: crate::paths::Paths {
                aware_home: tmp.path().to_path_buf(),
            },
            json: false,
        };
        assert_eq!(bridge_install_dir(&ctx), tmp.path().join("bridges"));
    }

    #[test]
    fn find_bridge_in_dir_ignores_path() {
        // The dir-only check (used by install/uninstall) must not be satisfied by
        // a copy that only exists elsewhere/on PATH — empty dir → None, present → Some.
        let tmp = tempfile::tempdir().unwrap();
        let bridge = lookup_bridge("tekla").unwrap();
        assert!(find_bridge_in_dir(bridge, tmp.path()).is_none());
        std::fs::write(tmp.path().join("aware-tekla.exe"), b"fake").unwrap();
        assert!(find_bridge_in_dir(bridge, tmp.path()).is_some());
    }

    #[test]
    fn bridge_is_current_requires_matching_version_marker() {
        let tmp = tempfile::tempdir().unwrap();
        let bridge = lookup_bridge("tekla").unwrap();
        // Absent → not current.
        assert!(!bridge_is_current(bridge, tmp.path(), "0.43.0"));
        // Present but no version marker (stale/unknown) → not current.
        std::fs::write(tmp.path().join("aware-tekla.exe"), b"fake").unwrap();
        assert!(!bridge_is_current(bridge, tmp.path(), "0.43.0"));
        // Marker for a different version → not current (refresh on upgrade).
        std::fs::write(version_marker_path(tmp.path(), "aware-tekla"), "0.42.0").unwrap();
        assert!(!bridge_is_current(bridge, tmp.path(), "0.43.0"));
        // Marker matches → current.
        std::fs::write(version_marker_path(tmp.path(), "aware-tekla"), "0.43.0\n").unwrap();
        assert!(bridge_is_current(bridge, tmp.path(), "0.43.0"));
    }

    #[test]
    fn managed_bridge_is_stale_detects_version_drift() {
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path();
        let v = "0.43.0";
        // Absent → not stale (missing, not stale).
        assert!(!managed_bridge_is_stale("aware-tekla", dir, v));
        // Present, no marker → stale (unknown version).
        std::fs::write(dir.join("aware-tekla.exe"), b"fake").unwrap();
        assert!(managed_bridge_is_stale("aware-tekla", dir, v));
        // Marker for a different version → stale.
        std::fs::write(version_marker_path(dir, "aware-tekla"), "0.42.0").unwrap();
        assert!(managed_bridge_is_stale("aware-tekla", dir, v));
        // Marker matches → not stale.
        std::fs::write(version_marker_path(dir, "aware-tekla"), v).unwrap();
        assert!(!managed_bridge_is_stale("aware-tekla", dir, v));
        // Unknown binary → not stale.
        assert!(!managed_bridge_is_stale("ripgrep", dir, v));
    }

    #[test]
    fn note_message_resolves_install_dir_for_off_path_bridges() {
        let dir = std::path::Path::new("/home/u/.aware/bridges");
        let sketchup = lookup_bridge("sketchup").unwrap();
        let msg = note_message(sketchup, dir).unwrap();
        assert!(msg.contains("/home/u/.aware/bridges"));
        assert!(msg.contains("aware-sketchup.exe"));
        assert!(!msg.contains("{dir}"));
        // tekla has no note.
        assert!(note_message(lookup_bridge("tekla").unwrap(), dir).is_none());
    }

    #[test]
    fn find_bridge_by_binary_matches_manifest_name() {
        let tmp = tempfile::tempdir().unwrap();
        std::fs::write(tmp.path().join("aware-tekla.exe"), b"fake").unwrap();
        // Manifest transport uses the binary name (`aware-tekla`), not the host id.
        assert!(find_bridge_by_binary("aware-tekla", tmp.path()).is_some());
        // Unknown / non-bridge binary → None (caller falls back to PATH).
        assert!(find_bridge_by_binary("ripgrep", tmp.path()).is_none());
    }
}
