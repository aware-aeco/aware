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
//! Bridges are placed in the same directory as the running `aware` binary so
//! they are automatically on PATH. Fallback: `~/.aware/bin/` (created if
//! absent) with a doctor warning to add it to PATH.

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
        note: Some("Run `aware-sketchup --install-bridge` after install to load the Ruby plugin"),
    },
    Bridge {
        id: "revit",
        binary: "aware-revit",
        asset_kind: AssetKind::Zip,
        description: "Revit 2026 (net8 sidecar + IExternalApplication add-in)",
        note: Some("Run `install-addin.ps1` after install to register the Revit add-in"),
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
    /// and places the binary next to `aware.exe` (or in `~/.aware/bin/` as a
    /// fallback). Accepted host IDs: tekla, rhino, sketchup, revit.
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
    for b in BRIDGES {
        let path = find_bridge(b, &install_dir);
        if let Some(p) = &path {
            println!("  \u{2713} {:<12}  {}", b.id, p.display());
        } else {
            println!(
                "  \u{2717} {:<12}  not found — run: aware sidecar install {}",
                b.id, b.id
            );
        }
        println!("               {}", b.description);
        if let Some(note) = b.note {
            println!("               \u{26a0} {note}");
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

    // Already installed?
    if let Some(p) = find_bridge(bridge, &install_dir) {
        println!("\u{2713} {} already installed at {}", bridge.binary, p.display());
        if let Some(note) = bridge.note {
            println!("\u{26a0} {note}");
        }
        return Ok(());
    }

    let version = env!("CARGO_PKG_VERSION");
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

    if let Some(note) = bridge.note {
        println!("\u{26a0} {note}");
    }
    Ok(())
}

// ── uninstall ─────────────────────────────────────────────────────────────────

fn uninstall(ctx: &Context, host: &str) -> Result<(), AwareError> {
    let bridge = lookup_bridge(host)?;
    let install_dir = bridge_install_dir(ctx);

    let path = find_bridge(bridge, &install_dir).ok_or_else(|| {
        AwareError::NotFound(format!("{} is not installed", bridge.binary))
    })?;

    std::fs::remove_file(&path)
        .map_err(|e| AwareError::Internal(format!("remove {}: {e}", path.display())))?;
    println!("\u{2713} Removed {}", path.display());
    Ok(())
}

// ── helpers ───────────────────────────────────────────────────────────────────

/// Resolve the directory where bridge binaries are installed.
///
/// Priority:
/// 1. Same directory as the running `aware` binary (so bridges land on PATH).
/// 2. `~/.aware/bin/` fallback (with a doctor warning to add to PATH).
pub fn bridge_install_dir(ctx: &Context) -> PathBuf {
    // Same dir as `aware.exe` — always on PATH when installed via npm/cargo/MSI.
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            if dir.is_dir() {
                return dir.to_path_buf();
            }
        }
    }
    // Fallback: ~/.aware/bin/
    ctx.paths.aware_home.join("bin")
}

/// Find a bridge binary on disk. Checks `install_dir/<binary>.exe` and PATH.
pub fn find_bridge_by_id(id: &str, install_dir: &std::path::Path) -> Option<PathBuf> {
    let b = BRIDGES.iter().find(|b| b.id == id)?;
    find_bridge(b, install_dir)
}

fn find_bridge(bridge: &Bridge, install_dir: &std::path::Path) -> Option<PathBuf> {
    // 1. Check install dir (where `aware sidecar install` places it)
    let local = install_dir.join(format!("{}.exe", bridge.binary));
    if local.is_file() {
        return Some(local);
    }
    // Also check a sub-dir (tekla/sketchup zips extract to a subdir)
    let subdir = install_dir.join(bridge.binary).join(format!("{}.exe", bridge.binary));
    if subdir.is_file() {
        return Some(subdir);
    }
    // 2. Check PATH via `which`-style lookup
    which_binary(bridge.binary)
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
}
