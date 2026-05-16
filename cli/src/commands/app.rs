//! `aware app ...` — manage installed apps.
//!
//! Phase v0.1 (read-only): `list`, `show`.
//! Phase v0.2 (install + validate): `install`, `uninstall`, `validate`, `export`.
//! Phase v0.3 (runtime):  `run`, `stop`, `logs`.

use std::time::Instant;

use clap::Subcommand;
use serde::Serialize;

use crate::context::Context;
use crate::envelope;
use crate::error::AwareError;
use crate::manifest::loader::discover_apps;
use crate::render::table::Table;

#[derive(Subcommand, Debug)]
pub enum AppCommand {
    /// Print a table of installed apps.
    List,
    /// Print an app's topology (ASCII) + provenance.
    Show { app: String },
    /// Install an app from a local path or registry name. (v0.2)
    Install { path_or_name: String },
    /// Uninstall an app. (v0.2)
    Uninstall { app: String },
    /// Validate an app file against the app-spec. (v0.2)
    Validate { path: std::path::PathBuf },
    /// Export an installed app's .flo file to a path. (v0.2)
    Export {
        app: String,
        output: std::path::PathBuf,
    },
    /// Execute an app. Blocks for long-running apps; exits for one-shot apps. (v0.3)
    Run {
        app: String,
        /// Instance id for multiple concurrent runs.
        #[arg(long)]
        instance: Option<String>,
        /// Override app input as `key=value`. Repeatable.
        #[arg(long, action = clap::ArgAction::Append)]
        config: Vec<String>,
    },
    /// Stop a running app. (v0.3)
    Stop {
        app: String,
        #[arg(long)]
        instance: Option<String>,
    },
    /// Read execution traces. (v0.3)
    Logs {
        app: String,
        #[arg(long)]
        instance: Option<String>,
        /// Follow the log as it's written (like `tail -f`).
        #[arg(long)]
        tail: bool,
    },
}

pub fn dispatch(cmd: AppCommand, ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        AppCommand::List => list(ctx),
        AppCommand::Show { app } => show(ctx, &app),
        AppCommand::Install { path_or_name } => install(ctx, &path_or_name),
        AppCommand::Uninstall { .. } => Err(AwareError::NotYetImplemented("app uninstall")),
        AppCommand::Validate { .. } => Err(AwareError::NotYetImplemented("app validate")),
        AppCommand::Export { .. } => Err(AwareError::NotYetImplemented("app export")),
        AppCommand::Run { .. } => Err(AwareError::NotYetImplemented("app run")),
        AppCommand::Stop { .. } => Err(AwareError::NotYetImplemented("app stop")),
        AppCommand::Logs { .. } => Err(AwareError::NotYetImplemented("app logs")),
    }
}

fn install(ctx: &Context, spec: &str) -> Result<(), AwareError> {
    use std::path::PathBuf;
    let path = PathBuf::from(spec);
    if !path.is_dir() {
        return Err(AwareError::Validation(format!(
            "app install: {} is not a directory (registry-hosted apps are not yet supported)",
            path.display()
        )));
    }
    let app_id = crate::install::install_app_from_path(&path, &ctx.paths)?;

    // Locate the installed .flo / .app file
    let app_dir = ctx.paths.apps_dir().join(&app_id);
    let manifest_path = std::fs::read_dir(&app_dir)?
        .flatten()
        .map(|e| e.path())
        .find(|p| {
            matches!(
                p.extension().and_then(|e| e.to_str()),
                Some("flo") | Some("app")
            )
        })
        .ok_or_else(|| {
            AwareError::Internal(format!("installed app {app_id} missing .flo/.app file"))
        })?;

    let app = crate::manifest::loader::load_app(&manifest_path)?;

    // Resolve each `requires` entry to an installed agent's exact version.
    let mut resolved = std::collections::BTreeMap::new();
    for req in &app.requires {
        let (id, _version_spec) = match req.split_once('@') {
            Some((i, v)) => (i, v),
            None => (req.as_str(), "*"),
        };
        let agent_manifest_path = ctx.paths.agents_dir().join(id).join("manifest.yaml");
        if let Ok(agent_manifest) = crate::manifest::loader::load_agent(&agent_manifest_path) {
            resolved.insert(id.to_string(), agent_manifest.version);
        }
        // If the agent isn't installed, silently skip — install proceeds; lockfile
        // simply omits it. v0.2 doesn't enforce that all requires must already be present.
    }

    let lockfile_path = app_dir.join("lockfile.yaml");
    crate::lockfile::write(&app_id, &app.version, resolved, &lockfile_path)?;

    println!("\u{2713} installed {app_id} (lockfile written)");
    Ok(())
}

#[derive(Serialize)]
struct AppListRow {
    id: String,
    version: String,
    nodes: usize,
    connections: usize,
    layout: String,
}

#[derive(Serialize)]
struct AppListData {
    apps: Vec<AppListRow>,
}

fn show(ctx: &Context, app_id: &str) -> Result<(), AwareError> {
    use crate::render::topology::format_topology;

    let discovered = discover_apps(&ctx.paths)?;
    let d = discovered
        .into_iter()
        .find(|d| d.manifest.app == app_id)
        .ok_or_else(|| AwareError::NotFound(format!("app: {app_id}")))?;

    let m = &d.manifest;
    println!("app:           {}", m.app);
    println!("version:       {}", m.version);
    if let Some(dn) = &m.display_name {
        println!("display-name:  {dn}");
    }
    println!(
        "description:   {}",
        m.description.lines().next().unwrap_or("").trim()
    );
    println!("exposes-as-agent: {}", m.exposes_as_agent);
    println!(
        "layout:        {}",
        format!("{:?}", m.layout).to_lowercase()
    );
    println!();

    println!("Requires:");
    for r in &m.requires {
        println!("  - {r}");
    }
    println!();

    print!("{}", format_topology(m));
    Ok(())
}

fn list(ctx: &Context) -> Result<(), AwareError> {
    let started = Instant::now();
    let discovered = discover_apps(&ctx.paths)?;

    if ctx.json {
        let data = AppListData {
            apps: discovered
                .iter()
                .map(|d| AppListRow {
                    id: d.manifest.app.clone(),
                    version: d.manifest.version.clone(),
                    nodes: d.manifest.node_count(),
                    connections: d.manifest.connection_count(),
                    layout: format!("{:?}", d.manifest.layout).to_lowercase(),
                })
                .collect(),
        };
        envelope::print_ok("app list", data, started).ok();
        return Ok(());
    }

    let mut t = Table::new(["ID", "VERSION", "NODES", "CONNS", "LAYOUT"]);
    for d in &discovered {
        t.row([
            d.manifest.app.clone(),
            d.manifest.version.clone(),
            d.manifest.node_count().to_string(),
            d.manifest.connection_count().to_string(),
            format!("{:?}", d.manifest.layout).to_lowercase(),
        ]);
    }
    print!("{}", t.render());
    Ok(())
}
