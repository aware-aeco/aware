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

pub async fn dispatch(cmd: AppCommand, ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        AppCommand::List => list(ctx),
        AppCommand::Show { app } => show(ctx, &app),
        AppCommand::Install { path_or_name } => install(ctx, &path_or_name),
        AppCommand::Uninstall { app } => {
            crate::install::uninstall_app(&app, &ctx.paths)?;
            println!("\u{2713} uninstalled {app}");
            Ok(())
        }
        AppCommand::Validate { path } => validate_cmd(ctx, &path),
        AppCommand::Export { app, output } => export(ctx, &app, &output),
        AppCommand::Run {
            app,
            instance,
            config,
        } => run(ctx, &app, instance.as_deref(), &config).await,
        AppCommand::Stop { app, instance } => stop(ctx, &app, instance.as_deref()),
        AppCommand::Logs {
            app,
            instance,
            tail,
        } => logs(ctx, &app, instance.as_deref(), tail, false, None).await,
    }
}

async fn run(
    ctx: &Context,
    app_id: &str,
    instance: Option<&str>,
    configs: &[String],
) -> Result<(), AwareError> {
    use crate::runtime::context::RuntimeContext;
    use crate::runtime::invoker::CliInvoker;
    use crate::runtime::orchestrator::Orchestrator;
    use crate::runtime::provenance::{ProvenanceWriter, log_path_for, run_id_now};

    let instance = instance.unwrap_or("default").to_string();
    let run_id = run_id_now();

    // Load the app.
    let app_dir = ctx.paths.apps_dir().join(app_id);
    if !app_dir.is_dir() {
        return Err(AwareError::NotFound(format!("app: {app_id}")));
    }
    let manifest_path = std::fs::read_dir(&app_dir)?
        .flatten()
        .map(|e| e.path())
        .find(|p| {
            matches!(
                p.extension().and_then(|e| e.to_str()),
                Some("flo") | Some("app")
            )
        })
        .ok_or_else(|| AwareError::Validation(format!("app {app_id} has no .flo/.app file")))?;
    let app = crate::manifest::loader::load_app(&manifest_path)?;

    // Parse k=v config overrides.
    let mut inputs = serde_json::Map::new();
    for c in configs {
        if let Some((k, v)) = c.split_once('=') {
            inputs.insert(k.to_string(), serde_json::Value::String(v.to_string()));
        }
    }

    // Detect mode: any node whose agent has stateful: true + lifecycle: start = long-running.
    // For v0.3 one-shot path, we check installed manifests. If no installed manifest for a node,
    // treat the node as stateless. Task 14 wires the actual long-running path.
    let is_long_running = app.nodes.iter().any(|n| {
        if let Some(agent_id) = &n.agent {
            let mp = ctx.paths.agents_dir().join(agent_id).join("manifest.yaml");
            if let Ok(m) = crate::manifest::loader::load_agent(&mp)
                && m.stateful
                && let Some(cmd_name) = &n.command
                && let Some(c) = m.commands.get(cmd_name)
            {
                return matches!(c.lifecycle, crate::manifest::agent::Lifecycle::Start);
            }
        }
        false
    });

    if is_long_running {
        use crate::runtime::lifecycle::{install_ctrl_c_handler, stop_channel};
        use crate::runtime::pidfile;

        let log_path = log_path_for(&ctx.paths.logs_dir(), app_id, &instance, &run_id);
        let provenance = ProvenanceWriter::open(&log_path).await?;
        let invoker = std::sync::Arc::new(CliInvoker {
            agents_dir: ctx.paths.agents_dir(),
        });

        let mut rt_ctx = RuntimeContext {
            inputs: serde_json::Value::Object(inputs.clone()),
            ..Default::default()
        };
        let creds_dir = ctx.paths.credentials_dir();
        if creds_dir.is_dir()
            && let Ok(read) = std::fs::read_dir(&creds_dir)
        {
            for entry in read.flatten() {
                let p = entry.path();
                if let Some(stem) = p.file_stem().and_then(|s| s.to_str()) {
                    let _ = crate::runtime::context::load_secret(&mut rt_ctx, &creds_dir, stem);
                }
            }
        }

        let orch = Orchestrator {
            app,
            agents_dir: ctx.paths.agents_dir(),
            run_id: run_id.clone(),
            instance: instance.clone(),
            invoker,
            provenance,
            ctx: rt_ctx,
            inputs: serde_json::Value::Object(serde_json::Map::new()),
            fan_in: Default::default(),
        };

        // Write pidfile.
        let instance_dir = ctx.paths.app_instance_dir(app_id, &instance);
        let pf = pidfile::Pidfile {
            app: app_id.to_string(),
            instance: instance.clone(),
            pid: std::process::id(),
            started_at: crate::runtime::provenance::now_iso(),
            run_id: run_id.clone(),
        };
        pidfile::write(&pf, &instance_dir)?;

        // Set up stop channel + Ctrl+C handler.
        let (stop_tx, stop_rx) = stop_channel();
        let _ctrl_handle = install_ctrl_c_handler(stop_tx);

        println!("\u{25b6} run {app_id} (instance {instance}, run-id {run_id})");
        println!(
            "  long-running \u{2014} press Ctrl+C to stop, or run `aware app stop {app_id}` from another terminal"
        );

        let result = orch.run_long_running(stop_rx).await;

        // Always remove pidfile on exit (success or interrupt).
        pidfile::remove(&instance_dir);

        return match result {
            Ok(()) => {
                println!("\u{2713} run ended; trace at {}", log_path.display());
                Ok(())
            }
            Err(e) => Err(e),
        };
    }

    // One-shot path.
    let log_path = log_path_for(&ctx.paths.logs_dir(), app_id, &instance, &run_id);
    let provenance = ProvenanceWriter::open(&log_path).await?;
    let invoker = std::sync::Arc::new(CliInvoker {
        agents_dir: ctx.paths.agents_dir(),
    });

    let mut rt_ctx = RuntimeContext {
        inputs: serde_json::Value::Object(inputs),
        ..Default::default()
    };

    // Load any credential files into the secrets map.
    let creds_dir = ctx.paths.credentials_dir();
    if creds_dir.is_dir()
        && let Ok(read) = std::fs::read_dir(&creds_dir)
    {
        for entry in read.flatten() {
            let p = entry.path();
            if let Some(stem) = p.file_stem().and_then(|s| s.to_str()) {
                let _ = crate::runtime::context::load_secret(&mut rt_ctx, &creds_dir, stem);
            }
        }
    }

    let orch = Orchestrator {
        app,
        agents_dir: ctx.paths.agents_dir(),
        run_id: run_id.clone(),
        instance: instance.clone(),
        invoker,
        provenance,
        ctx: rt_ctx,
        inputs: serde_json::Value::Object(serde_json::Map::new()),
        fan_in: Default::default(),
    };

    println!("\u{25b6} run {app_id} (instance {instance}, run-id {run_id})");
    orch.run_one_shot().await?;
    println!("\u{2713} run complete; trace at {}", log_path.display());
    Ok(())
}

async fn logs(
    ctx: &Context,
    app_id: &str,
    instance: Option<&str>,
    tail: bool,
    _replay: bool,
    _run_id_override: Option<&str>,
) -> Result<(), AwareError> {
    let instance = instance.unwrap_or("default");
    let run_id =
        crate::runtime::provenance::most_recent_run_id(&ctx.paths.logs_dir(), app_id, instance)
            .ok_or_else(|| AwareError::NotFound(format!("no runs for {app_id}/{instance}")))?;
    let log_path =
        crate::runtime::provenance::log_path_for(&ctx.paths.logs_dir(), app_id, instance, &run_id);

    if !tail {
        // Read and print raw JSONL
        let body = tokio::fs::read_to_string(&log_path).await?;
        print!("{body}");
        return Ok(());
    }

    // Tail: open, seek to end, poll for new lines every 200ms
    use tokio::io::{AsyncBufReadExt, AsyncSeekExt, BufReader, SeekFrom};
    let mut file = tokio::fs::File::open(&log_path).await?;
    let mut pos = file.seek(SeekFrom::End(0)).await?;
    loop {
        let _ = file.seek(SeekFrom::Start(pos)).await?;
        let mut reader = BufReader::new(file);
        let mut line = String::new();
        loop {
            line.clear();
            let n = reader.read_line(&mut line).await?;
            if n == 0 {
                break;
            } // EOF
            print!("{line}");
            pos += n as u64;
        }
        file = reader.into_inner();
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    }
}

fn stop(ctx: &Context, app_id: &str, instance: Option<&str>) -> Result<(), AwareError> {
    let instance = instance.unwrap_or("default");
    let instance_dir = ctx.paths.app_instance_dir(app_id, instance);
    let pid = crate::runtime::pidfile::read(&instance_dir)?;
    println!(
        "Stopping {} instance {} (pid {})",
        app_id, instance, pid.pid
    );
    #[cfg(unix)]
    {
        use std::process::Command;
        // SIGTERM via `kill -TERM` (no libc dep needed)
        let _ = Command::new("kill")
            .args(["-TERM", &pid.pid.to_string()])
            .status();
    }
    #[cfg(windows)]
    {
        use std::process::Command;
        let _ = Command::new("taskkill")
            .args(["/PID", &pid.pid.to_string(), "/T", "/F"])
            .status();
    }
    crate::runtime::pidfile::remove(&instance_dir);
    println!("\u{2713} stopped {app_id} (instance {instance})");
    Ok(())
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

fn validate_cmd(_ctx: &Context, path: &std::path::Path) -> Result<(), AwareError> {
    // Find .flo/.app in the dir
    let manifest_path = std::fs::read_dir(path)?
        .flatten()
        .map(|e| e.path())
        .find(|p| {
            matches!(
                p.extension().and_then(|e| e.to_str()),
                Some("flo") | Some("app")
            )
        })
        .ok_or_else(|| {
            AwareError::Validation(format!("no .flo or .app file in {}", path.display()))
        })?;

    let app = crate::manifest::loader::load_app(&manifest_path)?;
    let issues = crate::validate::validate_app(&app);
    if issues.is_empty() {
        println!("\u{2713} {} is valid", manifest_path.display());
        return Ok(());
    }
    for i in &issues {
        let tag = match i.severity {
            crate::validate::Severity::Error => "\u{2717}",
            crate::validate::Severity::Warning => "\u{26a0}",
        };
        println!("{tag} [{}] {}", i.code, i.message);
    }
    if crate::validate::has_errors(&issues) {
        return Err(AwareError::Validation("app failed validation".into()));
    }
    Ok(())
}

fn export(ctx: &Context, app_id: &str, output: &std::path::Path) -> Result<(), AwareError> {
    let app_dir = ctx.paths.apps_dir().join(app_id);
    if !app_dir.is_dir() {
        return Err(AwareError::NotFound(format!("app: {app_id}")));
    }
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

    std::fs::copy(&manifest_path, output)?;
    println!("\u{2713} exported {app_id} \u{2192} {}", output.display());
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
