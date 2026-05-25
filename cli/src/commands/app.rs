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
        /// Override an app input as `key=value`. Repeatable. (Named `--input`,
        /// not `--config`, to avoid colliding with the global `--config`
        /// config-file flag — see #117.)
        #[arg(long = "input", action = clap::ArgAction::Append)]
        input: Vec<String>,
        /// Preview the run without committing any write-mode side effects.
        /// Each write-mode node emits a `would-write:` block in the trace
        /// instead of calling the agent's mutation transport. (v0.11)
        #[arg(long)]
        dry_run: bool,
        /// Fully-stubbed run: stub read-mode nodes too. Every node yields a
        /// schema-shaped placeholder from its command's `output-schema` and no
        /// host sidecar is contacted, so a composition can be validated
        /// end-to-end without a live app installed. Implies `--dry-run`. (#103)
        #[arg(long)]
        simulate: bool,
    },
    /// Print a one-screen summary of an app's reads, writes, and external
    /// posts, plus the union of required permissions. (v0.11)
    Explain { app: String },
    /// Compile an app to its deterministic `<app>.lock` sidecar.
    /// Engineers read the lockfile; the AI reads the source. (v0.24)
    Compile { path: std::path::PathBuf },
    /// Open Glass Box — a single-file HTML viewer of the lockfile —
    /// in the user's default browser. (v0.24)
    Inspect { path: std::path::PathBuf },
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
        /// Pretty-print each event from the most recent (or given) run.
        #[arg(long)]
        replay: bool,
        /// Override which run-id to inspect (default: most recent).
        #[arg(long)]
        run_id: Option<String>,
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
            input,
            dry_run,
            simulate,
        } => run(ctx, &app, instance.as_deref(), &input, dry_run, simulate).await,
        AppCommand::Explain { app } => explain(ctx, &app),
        AppCommand::Compile { path } => compile_cmd(ctx, &path),
        AppCommand::Inspect { path } => inspect_cmd(ctx, &path),
        AppCommand::Stop { app, instance } => stop(ctx, &app, instance.as_deref()),
        AppCommand::Logs {
            app,
            instance,
            tail,
            replay,
            run_id,
        } => {
            logs(
                ctx,
                &app,
                instance.as_deref(),
                tail,
                replay,
                run_id.as_deref(),
            )
            .await
        }
    }
}

async fn run(
    ctx: &Context,
    app_id: &str,
    instance: Option<&str>,
    input_overrides: &[String],
    dry_run: bool,
    simulate: bool,
) -> Result<(), AwareError> {
    // `--simulate` is a strict superset of `--dry-run`: it also stubs read
    // nodes. Fold it into `dry_run` so every write-mode safety path downstream
    // (pre-flight skip, would-write events) treats a simulate run as a dry run.
    let dry_run = dry_run || simulate;
    use crate::runtime::context::RuntimeContext;
    use crate::runtime::invoker::DispatchInvoker;
    use crate::runtime::orchestrator::Orchestrator;
    use crate::runtime::provenance::{ProvenanceWriter, log_path_for, run_id_now};

    let instance = instance.unwrap_or("default").to_string();
    let run_id = run_id_now();
    // Ambient `{{ run.* }}` context (run.id / run.date / run.operator), shared by
    // every node so they render the same values within one run (#127).
    let run_ctx = crate::runtime::context::run_context(&run_id);

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

    // Safety-contract pre-flight: refuse to run an app whose write-mode
    // nodes are missing `safety:` blocks. Skipped in --dry-run (a dry-run
    // is precisely how you'd test an app's safety contract before adding
    // the blocks). See `10-core/app-spec.md § Safety contract`.
    if !dry_run {
        let agents = crate::manifest::loader::discover_agents(&ctx.paths)?;
        let safety_issues = crate::validate::validate_app_safety(&app, &agents);
        if !safety_issues.is_empty() {
            eprintln!("error: app failed safety pre-flight (use --dry-run to preview):");
            for issue in &safety_issues {
                eprintln!("  \u{2717} [{}] {}", issue.code, issue.message);
            }
            return Err(AwareError::Validation(
                "write-mode node(s) missing `safety:` block".into(),
            ));
        }
        // Refuse a real run that dispatches to a not-yet-runnable agent, with a
        // clear reason instead of a downstream "program not found" (#161).
        let agent_issues = crate::validate::validate_app_agents(&app, &agents);
        if let Some(err) = agent_issues.first() {
            eprintln!("error: {}", err.message);
            return Err(AwareError::Validation(format!("[{}]", err.code)));
        }
    }

    // Parse `--input key=value` overrides into the app's input map.
    let mut inputs = serde_json::Map::new();
    for kv in input_overrides {
        if let Some((k, v)) = kv.split_once('=') {
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
        let invoker = std::sync::Arc::new(DispatchInvoker {
            agents_dir: ctx.paths.agents_dir(),
        });

        let mut rt_ctx = RuntimeContext {
            inputs: serde_json::Value::Object(inputs.clone()),
            run: run_ctx.clone(),
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
            dry_run,
            simulate,
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
    let invoker = std::sync::Arc::new(DispatchInvoker {
        agents_dir: ctx.paths.agents_dir(),
    });

    let mut rt_ctx = RuntimeContext {
        inputs: serde_json::Value::Object(inputs),
        run: run_ctx,
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
        dry_run,
        simulate,
    };

    if simulate {
        println!("\u{25b6} simulate {app_id} (instance {instance}, run-id {run_id})");
        println!(
            "  every node is stubbed with a schema-shaped placeholder; no host sidecar is contacted"
        );
    } else if dry_run {
        println!("\u{25b6} dry-run {app_id} (instance {instance}, run-id {run_id})");
        println!("  write-mode nodes will emit `would-write:` events instead of mutating state");
    } else {
        println!("\u{25b6} run {app_id} (instance {instance}, run-id {run_id})");
    }
    orch.run_one_shot().await?;
    println!("\u{2713} run complete; trace at {}", log_path.display());
    Ok(())
}

async fn logs(
    ctx: &Context,
    app_id: &str,
    instance: Option<&str>,
    tail: bool,
    replay: bool,
    run_id_override: Option<&str>,
) -> Result<(), AwareError> {
    let instance = instance.unwrap_or("default");
    let run_id = if let Some(id) = run_id_override {
        id.to_string()
    } else {
        crate::runtime::provenance::most_recent_run_id(&ctx.paths.logs_dir(), app_id, instance)
            .ok_or_else(|| AwareError::NotFound(format!("no runs for {app_id}/{instance}")))?
    };
    let log_path =
        crate::runtime::provenance::log_path_for(&ctx.paths.logs_dir(), app_id, instance, &run_id);

    if replay {
        use crate::runtime::provenance::{RunEvent, read_run_events};
        let events = read_run_events(&log_path).await?;
        for event in &events {
            match event {
                RunEvent::RunStart {
                    ts,
                    run_id,
                    app,
                    instance,
                    ..
                } => {
                    println!("[{ts}] \u{25b6} run-start  {app}/{instance} (run {run_id})");
                }
                RunEvent::NodeStart {
                    ts,
                    node,
                    agent,
                    command,
                    ..
                } => {
                    let kind = agent
                        .as_deref()
                        .map(|a| format!("({a}/{})", command.as_deref().unwrap_or("")))
                        .unwrap_or_default();
                    println!("[{ts}] \u{25b6} {node}  {kind}");
                }
                RunEvent::NodeOutput { ts, node, data, .. } => {
                    println!("[{ts}] \u{2192} {node}  output {data}");
                }
                RunEvent::NodeError {
                    ts, node, error, ..
                } => {
                    println!("[{ts}] \u{2717} {node}  error: {error}");
                }
                RunEvent::NodeStop {
                    ts, node, reason, ..
                } => {
                    println!("[{ts}] \u{25fc} {node}  stop: {reason}");
                }
                RunEvent::WouldWrite {
                    ts,
                    node,
                    agent,
                    command,
                    ..
                } => {
                    println!(
                        "[{ts}] \u{26a0} {node}  would-write {agent}.{command} (dry-run; no side effects)"
                    );
                }
                RunEvent::RunEnd { ts, status, .. } => {
                    let mark = match status.as_str() {
                        "ok" => "\u{2713}",
                        "interrupted" => "\u{25fc}",
                        _ => "\u{2717}",
                    };
                    println!("[{ts}] {mark} run-end  {status}");
                }
            }
        }
        return Ok(());
    }

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

    // Validate BEFORE copying anything — fail fast, write nothing on error
    // (app-spec § Safety contract: "aware app validate refuses to install an
    // app missing `safety:` on a write-mode node"; install must enforce the
    // same contract as the standalone `validate` command, #134).
    let src_manifest = std::fs::read_dir(&path)?
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
    let src_app = crate::manifest::loader::load_app(&src_manifest)?;
    let mut issues = crate::validate::validate_app(&src_app);
    if let Ok(agents) = crate::manifest::loader::discover_agents(&ctx.paths) {
        issues.extend(crate::validate::validate_app_safety(&src_app, &agents));
    }
    if crate::validate::has_errors(&issues) {
        for i in &issues {
            let tag = match i.severity {
                crate::validate::Severity::Error => "\u{2717}",
                crate::validate::Severity::Warning => "\u{26a0}",
            };
            eprintln!("{tag} [{}] {}", i.code, i.message);
        }
        return Err(AwareError::Validation(
            "app install refused: app failed validation (fix errors above and retry)".into(),
        ));
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

fn validate_cmd(ctx: &Context, path: &std::path::Path) -> Result<(), AwareError> {
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
    let mut issues = crate::validate::validate_app(&app);

    // Safety-contract check requires the agent catalogue. Best-effort — if
    // the agents aren't discovered we skip rather than fail (the caller may
    // be validating an app before installing its agents).
    if let Ok(agents) = crate::manifest::loader::discover_agents(&ctx.paths) {
        issues.extend(crate::validate::validate_app_safety(&app, &agents));
        issues.extend(crate::validate::validate_app_agents(&app, &agents));
    }

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

/// Print a one-screen summary of an app's reads, writes, and external
/// posts plus the union of required permissions. Implements
/// `aware app explain <app>` (v0.11).
fn explain(ctx: &Context, app_id: &str) -> Result<(), AwareError> {
    use crate::manifest::agent::Mode;
    use std::collections::BTreeSet;

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
    let agents = crate::manifest::loader::discover_agents(&ctx.paths)?;

    let mut reads: Vec<(String, String, String)> = Vec::new();
    let mut writes: Vec<(String, String, String, bool)> = Vec::new(); // bool = safety declared
    let mut hosts: BTreeSet<String> = BTreeSet::new();
    let mut software: BTreeSet<String> = BTreeSet::new();
    let mut secrets: BTreeSet<String> = BTreeSet::new();

    for node in &app.nodes {
        let (Some(agent_id), Some(cmd_name)) = (node.agent.as_ref(), node.command.as_ref()) else {
            continue;
        };
        let Some(d) = agents.iter().find(|d| d.manifest.agent == *agent_id) else {
            continue;
        };
        if let Some(req) = &d.manifest.requires {
            for h in &req.network {
                hosts.insert(h.clone());
            }
            for s in &req.software {
                software.insert(s.clone());
            }
            for s in &req.secrets {
                secrets.insert(s.clone());
            }
        }
        let Some(cmd) = d.manifest.commands.get(cmd_name.as_str()) else {
            continue;
        };
        let mode = d.manifest.mode_of(cmd_name, cmd);
        let desc = cmd
            .description
            .lines()
            .next()
            .unwrap_or("")
            .trim()
            .to_string();
        match mode {
            Mode::Read => reads.push((node.id.clone(), format!("{agent_id}.{cmd_name}"), desc)),
            Mode::Write => writes.push((
                node.id.clone(),
                format!("{agent_id}.{cmd_name}"),
                desc,
                node.safety.is_some(),
            )),
        }
    }

    if ctx.json {
        use serde::Serialize;
        #[derive(Serialize)]
        struct ExplainData {
            app: String,
            reads: Vec<(String, String, String)>,
            writes: Vec<(String, String, String, bool)>,
            hosts: Vec<String>,
            software: Vec<String>,
            secrets: Vec<String>,
        }
        let data = ExplainData {
            app: app_id.to_string(),
            reads,
            writes,
            hosts: hosts.into_iter().collect(),
            software: software.into_iter().collect(),
            secrets: secrets.into_iter().collect(),
        };
        crate::envelope::print_ok("app explain", data, std::time::Instant::now()).ok();
        return Ok(());
    }

    println!("app: {app_id}");
    println!();
    println!("reads ({}):", reads.len());
    for (node, cmd, desc) in &reads {
        println!("  {node:<20} {cmd:<40} {desc}");
    }
    println!();
    println!("writes ({}):", writes.len());
    if writes.is_empty() {
        println!("  (none — read-only app)");
    }
    for (node, cmd, desc, safety_declared) in &writes {
        let marker = if *safety_declared {
            "\u{2713}"
        } else {
            "\u{2717}"
        };
        println!("  {marker} {node:<20} {cmd:<40} {desc}");
    }
    if writes.iter().any(|(_, _, _, s)| !s) {
        println!(
            "  ✗ = write-mode node missing `safety:` block (refused by `aware app run` per app-spec § Safety contract)"
        );
    }
    println!();
    if !hosts.is_empty() {
        println!("network hosts ({}):", hosts.len());
        for h in &hosts {
            println!("  - {h}");
        }
        println!();
    }
    if !software.is_empty() {
        println!("required software ({}):", software.len());
        for s in &software {
            println!("  - {s}");
        }
        println!();
    }
    if !secrets.is_empty() {
        println!("required secrets ({}):", secrets.len());
        for s in &secrets {
            println!("  - {s}");
        }
    }
    Ok(())
}

/// `aware app compile <path>` — emit the deterministic `<app-name>.lock`
/// sidecar (per `10-core/app-spec.md § Lockfile sidecar`, v0.24).
fn compile_cmd(ctx: &Context, path: &std::path::Path) -> Result<(), AwareError> {
    let source = crate::app_lock::find_app_source(path).ok_or_else(|| {
        AwareError::Validation(format!(
            "no app source file (.flo / .app / .flow / .aware) at {}",
            path.display()
        ))
    })?;
    let lock_path = crate::app_lock::compile_to_disk(&source, &ctx.paths)?;
    println!(
        "\u{2713} compiled {} \u{2192} {}",
        source.display(),
        lock_path.display()
    );
    Ok(())
}

/// `aware app inspect <path>` — open Glass Box (single-file HTML viewer)
/// of the lockfile in the user's default browser.
fn inspect_cmd(ctx: &Context, path: &std::path::Path) -> Result<(), AwareError> {
    let source = crate::app_lock::find_app_source(path).ok_or_else(|| {
        AwareError::Validation(format!(
            "no app source file (.flo / .app / .flow / .aware) at {}",
            path.display()
        ))
    })?;
    // Compile first so the viewer renders the freshly-resolved lockfile.
    let lock_path = crate::app_lock::compile_to_disk(&source, &ctx.paths)?;
    let app = crate::manifest::loader::load_app(&source)?;
    let agents = crate::manifest::loader::discover_agents(&ctx.paths)?;
    let lock = crate::app_lock::compile(&app, &agents, &source)?;

    let html_path = glass_box_html_path(&lock_path);
    let html = render_glass_box_html(&lock);
    std::fs::write(&html_path, &html)
        .map_err(|e| AwareError::Internal(format!("write {}: {e}", html_path.display())))?;
    println!(
        "\u{2713} compiled {} \u{2192} {}\n\u{2713} wrote Glass Box \u{2192} {}",
        source.display(),
        lock_path.display(),
        html_path.display()
    );

    if let Err(e) = open_in_browser(&html_path) {
        println!("  (couldn't auto-open browser: {e}; open the file above manually)");
    }
    Ok(())
}

fn glass_box_html_path(lock_path: &std::path::Path) -> std::path::PathBuf {
    let stem = lock_path
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("app");
    let dir = lock_path
        .parent()
        .unwrap_or_else(|| std::path::Path::new("."));
    dir.join(format!("{stem}.glass-box.html"))
}

fn open_in_browser(path: &std::path::Path) -> Result<(), std::io::Error> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", path.to_str().unwrap_or("")])
            .spawn()?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(path).spawn()?;
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        std::process::Command::new("xdg-open").arg(path).spawn()?;
    }
    Ok(())
}

/// Render the Glass Box single-file HTML viewer for a compiled lockfile.
/// Pure-string concatenation; no external deps.
fn render_glass_box_html(lock: &crate::app_lock::LockFile) -> String {
    let mut nodes_html = String::new();
    for node in &lock.nodes {
        let mode_class = if node.mode == "write" {
            "node-write"
        } else {
            "node-read"
        };
        let cmd_str = match (&node.agent, &node.command) {
            (Some(a), Some(c)) => format!(
                "<code>{}.{}</code>",
                html_escape_local(a),
                html_escape_local(c)
            ),
            _ => format!("<em>{}</em>", html_escape_local(&node.kind)),
        };
        let safety_badge = if node.safety.is_some() {
            "<span class=\"badge badge-safety\">safety \u{2713}</span>"
        } else if node.mode == "write" {
            "<span class=\"badge badge-warn\">safety MISSING</span>"
        } else {
            ""
        };
        nodes_html.push_str(&format!(
            "<div class=\"node {mode_class}\"><div class=\"node-id\">{id}</div><div class=\"node-cmd\">{cmd}</div><div class=\"node-meta\"><span class=\"badge badge-{mode}\">{mode}</span> {safety}</div></div>",
            id = html_escape_local(&node.id),
            cmd = cmd_str,
            mode = node.mode,
            safety = safety_badge,
        ));
    }

    let mut pins_html = String::new();
    for (k, v) in &lock.agent_pins {
        pins_html.push_str(&format!(
            "<li><code>{}</code> &rarr; <code>{}</code></li>",
            html_escape_local(k),
            html_escape_local(v)
        ));
    }

    format!(
        "<!DOCTYPE html>\n<html lang=\"en\"><head>\n<meta charset=\"utf-8\">\n<title>Glass Box \u{2014} {app}</title>\n<style>\n  body {{ font: 14px -apple-system, BlinkMacSystemFont, \"Segoe UI\", sans-serif; margin: 0; padding: 24px; background: #fafafa; color: #1a1a1a; }}\n  h1 {{ margin: 0 0 6px; font-size: 22px; }}\n  h2 {{ margin: 24px 0 8px; font-size: 15px; color: #555; text-transform: uppercase; letter-spacing: 0.04em; }}\n  .meta {{ color: #6b7280; font-size: 12px; }}\n  .nodes {{ display: grid; grid-template-columns: repeat(auto-fill, minmax(260px, 1fr)); gap: 8px; margin-top: 12px; }}\n  .node {{ background: white; border: 1px solid #ddd; border-radius: 6px; padding: 10px 14px; }}\n  .node-read {{ border-left: 4px solid #6b7280; }}\n  .node-write {{ border-left: 4px solid #dc2626; }}\n  .node-id {{ font-weight: 600; font-size: 13px; }}\n  .node-cmd {{ font-family: ui-monospace, SFMono-Regular, Menlo, monospace; font-size: 12px; color: #444; margin-top: 4px; }}\n  .node-meta {{ margin-top: 8px; font-size: 11px; }}\n  .badge {{ display: inline-block; padding: 1px 6px; border-radius: 3px; font-family: ui-monospace, monospace; font-size: 11px; }}\n  .badge-read {{ background: #eef2ff; color: #4338ca; }}\n  .badge-write {{ background: #fee2e2; color: #991b1b; }}\n  .badge-safety {{ background: #dcfce7; color: #166534; }}\n  .badge-warn {{ background: #fef3c7; color: #92400e; }}\n  ul {{ margin: 0; padding-left: 18px; }}\n  li {{ margin: 2px 0; font-family: ui-monospace, monospace; font-size: 12px; }}\n  .source {{ font-family: ui-monospace, monospace; font-size: 11px; color: #6b7280; word-break: break-all; }}\n</style>\n</head>\n<body>\n  <h1>{app} <small style=\"font-weight:400;color:#6b7280;\">v{version}</small></h1>\n  <div class=\"meta\">compiled {compiled} \u{2022} compiler {compiler}</div>\n  <div class=\"source\">source-hash: {hash}</div>\n\n  <h2>Agent pins ({pin_count})</h2>\n  <ul>{pins}</ul>\n\n  <h2>Nodes ({node_count}) <small style=\"font-weight:400;color:#6b7280;\">\u{2014} red = write-mode, gray = read-mode</small></h2>\n  <div class=\"nodes\">{nodes}</div>\n</body></html>",
        app = html_escape_local(&lock.app),
        version = html_escape_local(&lock.version),
        compiled = html_escape_local(&lock.compiled_at),
        compiler = html_escape_local(&lock.compiler_version),
        hash = html_escape_local(&lock.source_hash),
        pin_count = lock.agent_pins.len(),
        pins = if pins_html.is_empty() {
            "<li><em>(none)</em></li>".to_string()
        } else {
            pins_html
        },
        node_count = lock.nodes.len(),
        nodes = if nodes_html.is_empty() {
            "<em>(no nodes)</em>".to_string()
        } else {
            nodes_html
        },
    )
}

fn html_escape_local(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
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
