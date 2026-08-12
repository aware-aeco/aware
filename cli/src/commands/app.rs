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
use crate::runtime::invoker::{TransportKind, effective_transport};

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
    /// Rename an installed app in place. Moves its directory, rewrites the
    /// `app:` field, regenerates the lock (so it stays runnable, no drift), and
    /// — for an `exposes-as-agent` app — the synthesized agent. (v0.67)
    Rename {
        /// The installed app to rename.
        app: String,
        /// The new app id (slug: letters, digits, dash, underscore, dot).
        #[arg(value_name = "NEW_NAME")]
        new_name: String,
    },
    /// Duplicate an installed app into an independent copy under a new id. The
    /// original is left untouched. (v0.67)
    Duplicate {
        /// The installed app to copy.
        app: String,
        /// The new app id for the copy.
        #[arg(value_name = "NEW_NAME")]
        new_name: String,
    },
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
    /// Copy a run-owned large artifact to an explicit destination. (v0.121)
    Artifact {
        app: String,
        /// Opaque artifact id from a `$aware-artifact` node output.
        id: String,
        #[arg(long)]
        instance: Option<String>,
        #[arg(long)]
        run_id: Option<String>,
        /// Destination file. Required so large payloads never accidentally enter a terminal.
        #[arg(long)]
        output: std::path::PathBuf,
    },
    /// Freeze a node: pin its last run output into the source as a `frozen:` block, so Run skips
    /// it (emits the pinned value, never re-runs the agent) until unfrozen. Recompiles the lock.
    Freeze {
        /// The installed app.
        app: String,
        /// The node id to freeze.
        node: String,
    },
    /// Unfreeze a node: remove its `frozen:` block so it runs normally again. Recompiles the lock.
    Unfreeze {
        /// The installed app.
        app: String,
        /// The node id to unfreeze.
        node: String,
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
        AppCommand::Rename { app, new_name } => rename_cmd(ctx, &app, &new_name),
        AppCommand::Duplicate { app, new_name } => duplicate_cmd(ctx, &app, &new_name),
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
        AppCommand::Artifact {
            app,
            id,
            instance,
            run_id,
            output,
        } => {
            artifact(
                ctx,
                &app,
                &id,
                instance.as_deref(),
                run_id.as_deref(),
                &output,
            )
            .await
        }
        AppCommand::Freeze { app, node } => freeze_cmd(ctx, &app, &node).await,
        AppCommand::Unfreeze { app, node } => unfreeze_cmd(ctx, &app, &node),
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

    // Resolve the app's directory (by directory name, else by `app:` field — see
    // resolve_app_dir, #226) and load its source.
    let app_dir = crate::manifest::loader::resolve_app_dir(&ctx.paths, app_id)?;
    // Key all run state (pidfile, logs, instances under apps/<id>/) on the actual
    // DIRECTORY name, never the caller's id — so resolving a desynced app by its
    // `app:` field can't spawn a stray apps/<field>/ dir alongside the real one.
    let app_id = app_dir
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(app_id);
    let manifest_path = crate::manifest::loader::find_app_manifest(&app_dir)
        .ok_or_else(|| AwareError::Validation(format!("app {app_id} has no .flo/.app file")))?;
    let app = crate::manifest::loader::load_app(&manifest_path)?;

    // Safety-contract pre-flight: refuse to run an app whose write-mode
    // nodes are missing `safety:` blocks. Skipped in --dry-run (a dry-run
    // is precisely how you'd test an app's safety contract before adding
    // the blocks). See `10-core/app-spec.md § Safety contract`.
    // Pre-flight checks need the agent catalogue. `--simulate` stubs every node
    // and contacts no binary, so it skips both checks; a plain `--dry-run` still
    // dispatches read-mode nodes, so it gets the planned-agent check.

    // …but an unreadable `requires:` pin is checked FIRST, outside that exemption
    // (#349). Simulation is excused from the catalogue checks because it contacts
    // no binary — a fact about the environment. Whether a constraint can be *read*
    // is a fact about the file, true on every machine, so the same exemption must
    // not swallow it. `run` never calls `validate_app`, so without this an app
    // edited in place under `~/.aware/apps/` simulated clean with a constraint
    // nothing could parse.
    if let Some(err) = crate::validate::malformed_requires(&app).first() {
        eprintln!("error: {}", err.message);
        return Err(AwareError::Validation(format!("[{}]", err.code)));
    }

    // …and the same file-level rule one level down. A node may dispatch to an
    // app-backed agent, and the app behind it carries a `requires:` block of its
    // own. On a real run `DispatchInvoker::resolve_exposed` reads those pins when
    // the node dispatches; `--simulate` never reaches it, because the
    // orchestrator returns a synthesized output before the app transport, so the
    // backing app is never loaded and an unreadable pin one level down was
    // reported nowhere at all.
    if let Some(err) = nested_malformed_requires(&ctx.paths, &app)?.first() {
        eprintln!("error: {}", err.message);
        return Err(AwareError::Validation(format!("[{}]", err.code)));
    }

    if !simulate {
        let agents = crate::manifest::loader::discover_agents(&ctx.paths)?;

        // Planned-agent check: a plain `--dry-run` still dispatches to live read-mode
        // binaries (only `--simulate`, excluded above, stubs everything), so refuse a
        // not-yet-runnable agent with a clear reason instead of a downstream
        // "program not found" (#161).
        if let Some(err) = crate::validate::validate_app_agents(&app, &agents).first() {
            eprintln!("error: {}", err.message);
            return Err(AwareError::Validation(format!("[{}]", err.code)));
        }

        // Missing-agent check (#308): a node whose agent isn't installed has
        // nothing to dispatch to. Without this the run reached the transport and
        // died reading `<home>/agents/<id>/manifest.yaml` — surfacing a bare
        // `io: ... (os error 3)` that named neither the node nor the agent.
        // Install/compile only warn (an app may be installed before its agents,
        // #170); by run time the agent must exist. `--simulate` stubs every node
        // and contacts no binary, so it is excluded above and stays the way to
        // check a composition before its agents are installed.
        if let Some(err) =
            crate::validate::missing_agents(&app, &agents, crate::validate::Severity::Error).first()
        {
            eprintln!("error: {}", err.message);
            return Err(AwareError::Validation(format!("[{}]", err.code)));
        }

        // Pin check (#349): the app's `requires:` names the agent contract it was
        // written against. Running against a version outside that pin is how a
        // breaking agent change used to reach an app silently — the #343
        // coordinate-frame change is the case that surfaced it. Checked against
        // the live catalogue rather than the lock, so an agent swapped out after
        // the app was compiled is caught too. `--simulate` is excluded with the
        // other catalogue checks above: it stubs every node and dispatches to
        // nothing, and it is the documented way to check a composition before the
        // agents around it are in place.
        if let Some(err) =
            crate::validate::unsatisfied_pins(&app, &agents, crate::validate::Severity::Error)
                .first()
        {
            eprintln!("error: {}", err.message);
            return Err(AwareError::Validation(format!("[{}]", err.code)));
        }

        // Safety pre-flight only gates real runs (dry-run is precisely how you test
        // an app's safety contract before adding the blocks).
        if !dry_run {
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
            let agents = ctx.paths.agents_dir();
            if let Ok(m) = crate::manifest::loader::load_agent_by_id(&agents, agent_id)
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
        let artifact_dir = crate::runtime::provenance::artifact_dir_for(
            &ctx.paths.logs_dir(),
            app_id,
            &instance,
            &run_id,
        );
        tokio::fs::create_dir_all(&artifact_dir).await?;
        let invoker = std::sync::Arc::new(DispatchInvoker::new(
            &ctx.paths,
            dry_run,
            simulate,
            Some(artifact_dir),
        ));

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
        // Load `<app-dir>/config.yaml` into the `config` namespace so
        // `{{ config.<key> }}` resolves (app-spec § Templating; #230).
        crate::runtime::context::load_app_config(&mut rt_ctx, &app_dir)?;

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
            exposed_tx: None,
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
    let artifact_dir = crate::runtime::provenance::artifact_dir_for(
        &ctx.paths.logs_dir(),
        app_id,
        &instance,
        &run_id,
    );
    tokio::fs::create_dir_all(&artifact_dir).await?;
    let invoker = std::sync::Arc::new(DispatchInvoker::new(
        &ctx.paths,
        dry_run,
        simulate,
        Some(artifact_dir),
    ));

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
    // Load `<app-dir>/config.yaml` into the `config` namespace so
    // `{{ config.<key> }}` resolves (app-spec § Templating; #230).
    crate::runtime::context::load_app_config(&mut rt_ctx, &app_dir)?;

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
        exposed_tx: None,
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

/// Unreadable `requires:` pins in the apps behind this app's app-backed agents.
///
/// Whether a constraint can be *read* is a fact about a file — true on every
/// machine, needing no binary — so the `--simulate` exemption, which is about
/// the environment, must not swallow it one level down any more than it does at
/// the top level. Under a real run the nested pins are read at dispatch by
/// [`crate::runtime::invoker::DispatchInvoker::resolve_exposed`]; under
/// `--simulate` the orchestrator short-circuits with a synthesized output before
/// the app transport, so nothing ever loaded the backing app to look.
///
/// Deliberately narrow, and the narrowness is the point:
///
/// - It reads a **file**, and only for the `requires:` *syntax*. It does not
///   dispatch to the nested app, run it, or apply the catalogue checks
///   (installed / version-satisfied) that `--simulate` is legitimately excused
///   from because it contacts no binary.
/// - Anything not installed, or installed but not app-backed, is skipped in
///   silence — the same posture as `Orchestrator::synthesize_output`, which
///   already reads agent manifests under `--simulate` and falls back quietly
///   when one is missing. So `--simulate` stays the way to check a composition
///   before the agents around it exist. That silence is deliberately **per
///   agent**: the manifests are loaded one at a time rather than through
///   `discover_agents`, which aborts the whole walk on the first unreadable
///   manifest in the catalogue — so an unrelated broken agent elsewhere under
///   `~/.aware/agents/` would otherwise switch this check off entirely.
/// - A file that is *present and unreadable* is NOT silence, though — neither the
///   agent manifest nor the backing app. A check that cannot read the files it
///   must follow cannot report "no unreadable pin below them" either, so the
///   loader's error propagates. A real run gives the same answer below: on the
///   agent manifest always (`discover_agents` walks the catalogue), on the
///   backing app when that node dispatches (`resolve_exposed` loads it). So the
///   pre-flight is the stricter of the two by exactly the nodes that never run —
///   deliberately, since it is the only gate `--simulate` reaches at all.
///   One residual it does NOT catch: `find_app_manifest` flattens a failed
///   `read_dir` to "no manifest here" (`read_dir(..).ok()?`), so a backing-app
///   directory this process cannot *enumerate* reads as an absence. Narrow in
///   practice — the canonical `<dir>/<dir>.flo` is probed by name first, which
///   needs traverse rather than list permission, so a standard install still
///   resolves; it bites a non-canonically-named `.flo`, or a directory that
///   cannot be traversed either. Tracked on #365 rather than left here.
/// - Every id it joins onto a directory is fenced with
///   [`crate::manifest::loader::is_safe_segment`] first — the node's `agent:` and
///   the manifest's `backed-by:`, both of which come from a FILE and neither of
///   which is validated as a path anywhere else.
/// - Scoped to *dispatchable* agents via [`crate::validate::dispatchable_agents`],
///   so a frozen-only nested app — which never runs — is not gated, matching
///   [`crate::validate::unsatisfied_pins`].
/// - Scoped to agents that are app-backed *as dispatch resolves it* — through
///   [`effective_transport`], not through the raw `transport.app` field. A
///   manifest declaring both `cli:` and `app:` runs on `cli` and never loads the
///   backing app, so reading that app's pins here would refuse a run over a file
///   the node never touches (#215 review).
///
/// One level is the whole depth: a nested app may not itself compose another
/// `exposes-as-agent` app in v0 (`DispatchInvoker::nested_leaf` passes
/// `app_ctx: None`), so there is no deeper hop to recurse into.
fn nested_malformed_requires(
    paths: &crate::paths::Paths,
    app: &crate::manifest::app::App,
) -> Result<Vec<crate::validate::ValidationIssue>, AwareError> {
    let agents_dir = paths.agents_dir();
    let apps_dir = paths.apps_dir();
    // Sorted, so which of several broken nested apps gets reported first is the
    // same on every machine — `dispatchable_agents` returns a set, whose order is
    // not.
    let mut agent_ids: Vec<&str> = crate::validate::dispatchable_agents(app)
        .into_iter()
        .collect();
    agent_ids.sort_unstable();
    let mut out = Vec::new();
    for agent_id in agent_ids {
        // This id comes from the app FILE and nothing validates it as a path, so
        // `agent: ../../somewhere` would otherwise make the pre-flight read a
        // `manifest.yaml` from outside `agents/` entirely. Skipping is the right
        // answer as well as the safe one: no path-shaped id can name an installed
        // agent, so this is the "not installed" case.
        //
        // Through `agent_manifest_path` — the fenced path, not a hand-rolled join
        // plus a hand-rolled guard. This site needs the PATH rather than the
        // parsed manifest (the absent-vs-unreadable split below is about the file
        // itself), which is exactly what that function exists for. Doing it by
        // hand was safe but invisible to `tests/agent_id_joins_are_fenced.rs`, so
        // it would have taught the next author a pattern the guard cannot check
        // (#365 review).
        let Ok(manifest_path) = crate::manifest::loader::agent_manifest_path(&agents_dir, agent_id)
        else {
            continue; // not a plain segment, so it names no installed agent
        };
        // Absent means "not installed", and that is the silence `--simulate`
        // depends on to check a composition before its agents exist. Anything
        // else — a directory where the manifest should be, a broken symlink, a
        // metadata read this process is not allowed to make — is a fault and
        // propagates. `is_file()` could not tell those apart: it answers `false`
        // for all of them, which would have left a narrower version of the very
        // fail-open below.
        //
        // `symlink_metadata` rather than `metadata` so a DANGLING symlink is a
        // fault too: `metadata` follows the link and reports its missing target
        // as `NotFound`, which would read as "not installed" for an entry that is
        // plainly there and plainly broken.
        //
        // One residual, stated rather than papered over: on Windows an invalid
        // *intermediate* component can also surface as `NotFound`
        // (`ERROR_PATH_NOT_FOUND`), so a file sitting where `agents/<id>/` should
        // be still reads as an absence here. That shape is "no agent directory",
        // which is the same answer, so it is left alone rather than special-cased
        // on an error code that means two things.
        match std::fs::symlink_metadata(&manifest_path) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            // Re-raised carrying the path, the way `loader::read_manifest` does:
            // `AwareError::Io` cannot hold one, and a bare "Access is denied.
            // (os error 5)" names neither the agent nor the file, which is no
            // better than the silence this replaced. The kind is preserved, so
            // the class and exit code are unchanged.
            Err(e) => {
                return Err(std::io::Error::new(
                    e.kind(),
                    format!("{}: {e}", manifest_path.display()),
                )
                .into());
            }
            Ok(_) => {}
        }
        // Loaded one at a time on purpose: `discover_agents` walks the whole
        // catalogue and returns `Err` on the first manifest that won't parse, so
        // routing this through it would let one unrelated broken agent silence
        // the check for every app on the machine. Installed-but-unreadable
        // propagates, though — see the doc comment: it is a fault, not an absence,
        // and a check that cannot read this manifest cannot clear the pins below
        // it either.
        let manifest = crate::manifest::loader::load_agent(&manifest_path)?;
        // Only an app-backed agent has an app — and therefore a `requires:` block
        // — behind it, and "app-backed" means what DISPATCH means by it. Resolved
        // through `effective_transport`, the single owner of the priority order
        // (cli > rest > app > builtin) that `DispatchInvoker::transport_kind`
        // resolves through, because a manifest carrying both `cli:` and `app:`
        // runs on `cli` and never loads the backing app: probing `transport.app`
        // raw would refuse the run over a file the node would never touch. That
        // rule is #215's, written at the definition of `effective_transport`.
        let Ok(TransportKind::App) = effective_transport(&manifest, agent_id) else {
            continue; // dispatches on some other transport, or on none at all
        };
        let Some(app_transport) = manifest.transport.app.as_ref() else {
            continue; // unreachable: `TransportKind::App` is that field being set
        };
        // `backed-by` is manifest-controlled too, and lands in a join of its own —
        // an agent manifest edited to `backed-by: ../../elsewhere` would read an
        // app from outside `apps/`. Same fence, same reasoning as the id above.
        if !crate::manifest::loader::is_safe_segment(&app_transport.backed_by) {
            continue;
        }
        let backing_dir = apps_dir.join(&app_transport.backed_by);
        let Some(manifest_path) = crate::manifest::loader::find_app_manifest(&backing_dir) else {
            continue;
        };
        // The backing app is the file whose `requires:` this whole function
        // exists to read, so an unparseable one is the LEAST excusable silence of
        // the lot — it is the check failing at its own subject. Absent (no
        // `.flo`/`.app` in the directory) already continued above; this is
        // "present and unreadable", which propagates for the same reason the
        // agent manifest above does.
        //
        // Named with the hop, as every issue below is — but matched on the
        // VARIANT rather than stringified, for two reasons a `format!("{e}")` got
        // wrong. `AwareError` Displays its own class prefix, so wrapping the
        // rendered error printed "validation failed:" twice, and for a read
        // failure it nested one class inside another. And coercing both arms to
        // `Validation` moved an unreadable file from exit 1 to exit 3,
        // contradicting the re-raise above and disagreeing with `resolve_exposed`,
        // which yields `Io` for the same file on a real run. `cli-spec.md` keeps 1
        // ("general failure") and 3 ("validation failed") distinct: a file that
        // cannot be read is not an invalid one.
        let backing = crate::manifest::loader::load_app(&manifest_path).map_err(|e| {
            let hop = format!(
                "app-backed agent {:?} (backing app {:?})",
                agent_id, app_transport.backed_by
            );
            match e {
                AwareError::Validation(m) => AwareError::Validation(format!("{hop}: {m}")),
                AwareError::Io(io) => std::io::Error::new(io.kind(), format!("{hop}: {io}")).into(),
                // `load_app` yields only those two; anything else keeps its own
                // class and loses only the hop, which fails safe.
                other => other,
            }
        })?;
        out.extend(
            crate::validate::malformed_requires(&backing)
                .into_iter()
                .map(|issue| crate::validate::ValidationIssue {
                    // Name the hop, or the operator reads a pin that appears in
                    // neither the app they named nor anything they can see.
                    message: format!(
                        "app-backed agent {:?} (backing app {:?}): {}",
                        agent_id, app_transport.backed_by, issue.message
                    ),
                    ..issue
                }),
        );
    }
    Ok(out)
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
                RunEvent::NodeProgress { ts, node, data, .. } => {
                    println!("[{ts}] \u{2026} {node}  {}", render_progress(data));
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

/// One `node-progress` record as a line a person can follow (#405).
///
/// Rendered rather than dumped as JSON because this is the view a human watches a 40-second read
/// through, and the three things that matter — where it is, how far along, and whether a segment is
/// ready to fetch — are otherwise buried in a record whose other fields are producer-specific. The
/// raw record is still one `--replay`-free `aware app logs` away, since the JSONL is the trace.
fn render_progress(data: &serde_json::Value) -> String {
    let phase = data
        .get("phase")
        .and_then(|v| v.as_str())
        .unwrap_or("progress");
    let mut line = format!("progress {phase}");
    if let Some(done) = data.get("done").and_then(|v| v.as_u64()) {
        match data.get("total").and_then(|v| v.as_u64()) {
            Some(total) => line.push_str(&format!(" {done}/{total}")),
            None => line.push_str(&format!(" {done}")),
        }
    }
    if let Some(id) = data
        .get("artifact")
        .and_then(|a| a.get("id"))
        .and_then(|v| v.as_str())
    {
        line.push_str(&format!(" segment {id}"));
    }
    if let Some(message) = data.get("message").and_then(|v| v.as_str()) {
        line.push_str(&format!(" — {message}"));
    }
    line
}

async fn artifact(
    ctx: &Context,
    app_id: &str,
    id: &str,
    instance: Option<&str>,
    run_id_override: Option<&str>,
    output: &std::path::Path,
) -> Result<(), AwareError> {
    let instance = instance.unwrap_or("default");
    crate::runtime::provenance::validate_artifact_component(app_id, "app")?;
    crate::runtime::provenance::validate_artifact_component(instance, "instance")?;
    let run_id = match run_id_override {
        Some(id) => id.to_string(),
        None => {
            crate::runtime::provenance::most_recent_run_id(&ctx.paths.logs_dir(), app_id, instance)
                .ok_or_else(|| AwareError::NotFound(format!("no runs for {app_id}/{instance}")))?
        }
    };
    let source = crate::runtime::provenance::artifact_path_for(
        &ctx.paths.logs_dir(),
        app_id,
        instance,
        &run_id,
        id,
    )?;
    if !source.is_file() {
        return Err(AwareError::NotFound(format!(
            "artifact {id:?} for run {run_id} not found"
        )));
    }
    tokio::fs::copy(&source, output).await?;
    println!("✓ copied artifact {id} → {}", output.display());
    Ok(())
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
    // Missing agents are reported separately from `issues` so they surface even
    // on an otherwise-clean install (the `issues` loop below only prints when
    // something is a hard error).
    let mut missing = Vec::new();
    if let Ok(agents) = crate::manifest::loader::discover_agents(&ctx.paths) {
        issues.extend(crate::validate::validate_app_safety(&src_app, &agents));
        // Don't install an app that references a not-yet-runnable agent — install
        // must enforce the same contract as validate/compile (#161).
        issues.extend(crate::validate::validate_app_agents(&src_app, &agents));
        // #308: a node whose agent isn't installed can't run. Installing an app
        // before its agents is legitimate (#170), so this is a warning here
        // rather than a refusal — but a loud one, naming the node, the agent and
        // the remedy, so the gap is known now instead of surfacing later as a
        // bare `os error 3`. `aware app run` refuses it.
        missing =
            crate::validate::missing_agents(&src_app, &agents, crate::validate::Severity::Warning);
        // #349: same posture for an unsatisfied `requires:` pin. Installing an
        // app before the agent it pins is legitimate, and the matching version
        // may still be on its way — so name the mismatch here and let `compile`
        // and `run` be the refusals.
        missing.extend(crate::validate::unsatisfied_pins(
            &src_app,
            &agents,
            crate::validate::Severity::Warning,
        ));
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
    for m in &missing {
        eprintln!("\u{26a0} [{}] {}", m.code, m.message);
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

    // Resolve `requires` → installed agent versions and write `lockfile.yaml`.
    // Shared with rename/duplicate so a moved app's on-disk shape matches a
    // freshly-installed one.
    crate::install::local::write_app_lockfile(&app, &app_dir, &ctx.paths)?;

    println!("\u{2713} installed {app_id} (lockfile written)");
    Ok(())
}

/// `aware app rename <app> <new-name>` — rename an installed app in place. (v0.67)
fn rename_cmd(ctx: &Context, old: &str, new: &str) -> Result<(), AwareError> {
    let out = crate::install::rename_app(old, new, &ctx.paths)?;
    println!("\u{2713} renamed {old} \u{2192} {}", out.id);
    print_lock_outcome(out.lock);
    Ok(())
}

/// `aware app duplicate <app> <new-name>` — copy an installed app to a new id. (v0.67)
fn duplicate_cmd(ctx: &Context, src: &str, new: &str) -> Result<(), AwareError> {
    let out = crate::install::duplicate_app(src, new, &ctx.paths)?;
    println!("\u{2713} duplicated {src} \u{2192} {}", out.id);
    print_lock_outcome(out.lock);
    Ok(())
}

/// Report what happened to the compiled lock after a rename/duplicate, so a user
/// who had a ready-to-run app learns immediately if it now needs a recompile
/// (rather than the failure being an absent line they have to notice).
fn print_lock_outcome(lock: crate::install::LockOutcome) {
    use crate::install::LockOutcome;
    match lock {
        LockOutcome::Refreshed => println!("  lock refreshed \u{2014} ready to run"),
        LockOutcome::NeedsRefresh => println!(
            "  \u{26a0} lock could not be refreshed \u{2014} run `aware app compile` before running"
        ),
        LockOutcome::None => {}
    }
}

/// `aware app freeze <app> <node>` — pin the node's last run output as a `frozen:` block on the
/// source, then recompile. The orchestrator then emits that value and skips the agent on Run.
async fn freeze_cmd(ctx: &Context, app_id: &str, node_id: &str) -> Result<(), AwareError> {
    use crate::runtime::provenance::{RunEvent, log_path_for, most_recent_run_id, read_run_events};

    let app_dir = crate::manifest::loader::resolve_app_dir(&ctx.paths, app_id)?;
    let manifest_path = crate::manifest::loader::find_app_manifest(&app_dir)
        .ok_or_else(|| AwareError::Validation(format!("app {app_id} has no .flo/.app file")))?;
    let app = crate::manifest::loader::load_app(&manifest_path)?;
    if !app.nodes.iter().any(|n| n.id == node_id) {
        return Err(AwareError::NotFound(format!(
            "node {node_id:?} not found in app {app_id:?}"
        )));
    }

    // "Whatever's there" = the node's last output, from the most recent run trace.
    let no_output = || {
        AwareError::Validation(format!(
            "node {node_id:?} has no recorded output \u{2014} run the app first, then freeze"
        ))
    };
    let run_id =
        most_recent_run_id(&ctx.paths.logs_dir(), app_id, "default").ok_or_else(no_output)?;
    let log_path = log_path_for(&ctx.paths.logs_dir(), app_id, "default", &run_id);
    let events = read_run_events(&log_path).await?;
    let last_output = events
        .iter()
        .rev()
        .find_map(|e| match e {
            RunEvent::NodeOutput { node, data, .. } if node == node_id => Some(data.clone()),
            _ => None,
        })
        .ok_or_else(no_output)?;

    let value_yaml = serde_yaml::to_string(&last_output)
        .map_err(|e| AwareError::Internal(format!("serialize frozen value: {e}")))?;
    let source = std::fs::read_to_string(&manifest_path)
        .map_err(|e| AwareError::Internal(format!("read {}: {e}", manifest_path.display())))?;
    let edited = set_node_frozen(&source, node_id, &value_yaml)?;
    std::fs::write(&manifest_path, edited)
        .map_err(|e| AwareError::Internal(format!("write {}: {e}", manifest_path.display())))?;
    println!(
        "\u{2713} froze {app_id}/{node_id} \u{2014} Run emits its pinned output and skips the agent"
    );
    recompile_after_freeze(ctx, &manifest_path);
    Ok(())
}

/// `aware app unfreeze <app> <node>` — remove the node's `frozen:` block, then recompile.
fn unfreeze_cmd(ctx: &Context, app_id: &str, node_id: &str) -> Result<(), AwareError> {
    let app_dir = crate::manifest::loader::resolve_app_dir(&ctx.paths, app_id)?;
    let manifest_path = crate::manifest::loader::find_app_manifest(&app_dir)
        .ok_or_else(|| AwareError::Validation(format!("app {app_id} has no .flo/.app file")))?;
    // Validate the node exists (catches typos + scopes the edit to a real top-level node), the same
    // guard freeze applies.
    let app = crate::manifest::loader::load_app(&manifest_path)?;
    if !app.nodes.iter().any(|n| n.id == node_id) {
        return Err(AwareError::NotFound(format!(
            "node {node_id:?} not found in app {app_id:?}"
        )));
    }
    let source = std::fs::read_to_string(&manifest_path)
        .map_err(|e| AwareError::Internal(format!("read {}: {e}", manifest_path.display())))?;
    let edited = clear_node_frozen(&source, node_id)?;
    if edited == source {
        println!("node {node_id} is not frozen \u{2014} nothing to do");
        return Ok(());
    }
    std::fs::write(&manifest_path, edited)
        .map_err(|e| AwareError::Internal(format!("write {}: {e}", manifest_path.display())))?;
    println!("\u{2713} unfroze {app_id}/{node_id} \u{2014} it runs normally again");
    recompile_after_freeze(ctx, &manifest_path);
    Ok(())
}

/// Recompile after a freeze/unfreeze edit so the lock matches the new source (the Run gate). A
/// compile failure is surfaced as a warning — the source edit already landed.
fn recompile_after_freeze(ctx: &Context, manifest_path: &std::path::Path) {
    match crate::app_lock::compile_to_disk(manifest_path, &ctx.paths) {
        Ok(_) => println!("  lock refreshed \u{2014} ready to run"),
        Err(e) => println!(
            "  \u{26a0} recompile failed ({e}) \u{2014} run `aware app compile` before running"
        ),
    }
}

// ── Comment-preserving `frozen:` text edits ───────────────────────────────────────────────────
// serde_yaml (0.9) drops comments on round-trip, so we edit the source text in a line-targeted way
// (mirrors install::rename's `app:` rewrite). Assumes the .flo's 2-space list indentation.

fn leading_indent(line: &str) -> usize {
    line.len() - line.trim_start().len()
}

/// The value after `id:` on a node line, normalized: strip an inline `# comment` and surrounding
/// quotes, so `id: read`, `id: "read"`, and `- id: read  # note` all yield `read` (node ids are
/// slugs, so a `#` inside the value can't occur).
fn parse_id_value(rest: &str) -> &str {
    let no_comment = rest.find(" #").map_or(rest, |i| &rest[..i]);
    let t = no_comment.trim();
    t.strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))
        .or_else(|| t.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')))
        .unwrap_or(t)
}

/// Locate a node's `id:` line and its field indentation (the column where `id:` starts — where
/// sibling fields like `agent:` / `frozen:` live). Matches both `- id: x` and own-line `id: x`.
fn find_node_id_line(lines: &[&str], node_id: &str) -> Option<(usize, usize)> {
    for (i, line) in lines.iter().enumerate() {
        let t = line.trim_start();
        let body = t.strip_prefix("- ").unwrap_or(t);
        if let Some(rest) = body.strip_prefix("id:")
            && parse_id_value(rest) == node_id
        {
            return Some((i, line.find("id:").unwrap_or(0)));
        }
    }
    None
}

/// Remove a node's `frozen:` block (the `frozen:` line + its more-indented value lines). Returns the
/// source unchanged if the node isn't found or isn't frozen. Comment-preserving.
pub fn clear_node_frozen(source: &str, node_id: &str) -> Result<String, AwareError> {
    let lines: Vec<&str> = source.lines().collect();
    let Some((idx, indent)) = find_node_id_line(&lines, node_id) else {
        return Ok(source.to_string());
    };
    let marker = indent.saturating_sub(2);
    // Find `frozen:` at field indent within this node's block (stop at the node's dedent/next item).
    let mut frozen_start = None;
    let mut i = idx + 1;
    while i < lines.len() {
        let line = lines[i];
        if !line.trim().is_empty() {
            let li = leading_indent(line);
            if li <= marker {
                break; // next node or dedent out of this node
            }
            if li == indent && line.trim_start().starts_with("frozen:") {
                frozen_start = Some(i);
                break;
            }
        }
        i += 1;
    }
    let Some(fs) = frozen_start else {
        return Ok(source.to_string());
    };
    // The block is the `frozen:` line + following lines more-indented than the field indent.
    let mut fe = fs + 1;
    while fe < lines.len() {
        let line = lines[fe];
        if !line.trim().is_empty() && leading_indent(line) <= indent {
            break;
        }
        fe += 1;
    }
    let kept: Vec<&str> = lines
        .iter()
        .enumerate()
        .filter(|(j, _)| *j < fs || *j >= fe)
        .map(|(_, l)| *l)
        .collect();
    let mut out = kept.join("\n");
    if source.ends_with('\n') {
        out.push('\n');
    }
    Ok(restore_line_endings(out, source))
}

/// Re-apply CRLF line endings if the source used them — `lines()` strips `\r`, so the LF-rebuilt
/// text would otherwise silently convert a CRLF-authored .flo to all-LF (a whole-file diff).
fn restore_line_endings(lf: String, source: &str) -> String {
    if source.contains("\r\n") {
        lf.replace('\n', "\r\n")
    } else {
        lf
    }
}

/// Pin a node's output as a `frozen:` block (replacing any existing one), comment-preserving.
/// `value_yaml` is the YAML serialization of the value (e.g. from `serde_yaml::to_string`).
pub fn set_node_frozen(
    source: &str,
    node_id: &str,
    value_yaml: &str,
) -> Result<String, AwareError> {
    let cleared = clear_node_frozen(source, node_id)?; // idempotent replace
    let lines: Vec<&str> = cleared.lines().collect();
    let (idx, indent) = find_node_id_line(&lines, node_id)
        .ok_or_else(|| AwareError::NotFound(format!("node {node_id:?} not found in app source")))?;
    let vpad = " ".repeat(indent + 2);
    let mut block = format!("{}frozen:", " ".repeat(indent));
    for vline in value_yaml.lines() {
        block.push('\n');
        if !vline.is_empty() {
            block.push_str(&vpad);
            block.push_str(vline);
        }
    }
    let mut out: Vec<String> = Vec::with_capacity(lines.len() + 1);
    for (i, l) in lines.iter().enumerate() {
        out.push((*l).to_string());
        if i == idx {
            out.push(block.clone());
        }
    }
    let mut joined = out.join("\n");
    if source.ends_with('\n') {
        joined.push('\n');
    }
    Ok(restore_line_endings(joined, source))
}

/// Resolve `aware app validate <PATH>` to the manifest to load.
///
/// Accepts EITHER the `.flo`/`.app` file itself or the directory holding it. The file
/// form is the natural invocation — `aware app validate my-app.app`, and the only one
/// possible for the loose `.app` files under `30-apps/_examples/` — but used to hit a
/// bare `read_dir` and fail with an unexplained OS error (`ERROR_DIRECTORY` on Windows,
/// `ENOTDIR` elsewhere) rather than anything a user could act on.
///
/// A missing path reports that, instead of the misleading "no .flo or .app file in …"
/// a directory scan would produce for a path that is not a directory at all.
fn resolve_validate_target(path: &std::path::Path) -> Result<std::path::PathBuf, AwareError> {
    if path.is_file() {
        return Ok(path.to_path_buf());
    }
    if path.is_dir() {
        // Probe first so a genuine enumeration failure (permissions, IO) propagates as
        // itself. `find_app_manifest` flattens its `read_dir` error to `None`, which
        // would otherwise be reported as "no .flo or .app file" — the previous
        // `read_dir(path)?` did surface it, and losing that would be a regression.
        std::fs::read_dir(path)?;
        // Shared with `app explain` / install: prefers `<dir-name>.flo`, then any `.flo`,
        // then any `.app` — deterministic, where the previous inline scan returned
        // whatever `read_dir` happened to yield first.
        return crate::manifest::loader::find_app_manifest(path).ok_or_else(|| {
            AwareError::Validation(format!("no .flo or .app file in {}", path.display()))
        });
    }
    Err(AwareError::NotFound(format!(
        "{} does not exist",
        path.display()
    )))
}

fn validate_cmd(ctx: &Context, path: &std::path::Path) -> Result<(), AwareError> {
    let manifest_path = resolve_validate_target(path)?;

    let app = crate::manifest::loader::load_app(&manifest_path)?;
    let mut issues = crate::validate::validate_app(&app);

    // Safety-contract check requires the agent catalogue. Best-effort — if
    // the agents aren't discovered we skip rather than fail (the caller may
    // be validating an app before installing its agents).
    if let Ok(agents) = crate::manifest::loader::discover_agents(&ctx.paths) {
        issues.extend(crate::validate::validate_app_safety(&app, &agents));
        issues.extend(crate::validate::validate_app_agents(&app, &agents));
        // Deliberately NOT the #308 missing-agent check: `app validate` judges the
        // app *file*, and whether an agent happens to be installed is a fact about
        // the environment, not the composition. (It would also make the verdict
        // depend on ambient `~/.aware` state — an app would be "valid" on one
        // machine and not another.) `install`/`compile` warn and `run` refuses.
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

    let app_dir = crate::manifest::loader::resolve_app_dir(&ctx.paths, app_id)?;
    let manifest_path = crate::manifest::loader::find_app_manifest(&app_dir)
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
        // Honor an explicit node-level `mode:` on `mode-overridable` commands
        // (e.g. `exec`) so explain/glass-box classifies a read-only exec node
        // as a read, matching the validator + lockfile compiler (#165).
        let mode = d.manifest.effective_mode(cmd_name, cmd, node.mode).mode;
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
    // compile_to_disk validates before locking, so an unrunnable construct (e.g.
    // an inline kind the runtime rejects) fails here rather than at run (#160).
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
    let app_dir = crate::manifest::loader::resolve_app_dir(&ctx.paths, app_id)?;
    let manifest_path = crate::manifest::loader::find_app_manifest(&app_dir).ok_or_else(|| {
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

    let app_dir = crate::manifest::loader::resolve_app_dir(&ctx.paths, app_id)?;
    let manifest_path = crate::manifest::loader::find_app_manifest(&app_dir)
        .ok_or_else(|| AwareError::NotFound(format!("app: {app_id}")))?;
    let m = crate::manifest::loader::load_app(&manifest_path)?;
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

    print!("{}", format_topology(&m));
    Ok(())
}

fn list(ctx: &Context) -> Result<(), AwareError> {
    let started = Instant::now();
    let discovered = discover_apps(&ctx.paths)?;

    // Surface the #226 footgun: an app whose directory name and `app:` field
    // disagree (e.g. after a manual `mv`) is only half-addressable. Warn in the
    // human view; `--json` stdout stays clean for machine consumers.
    if !ctx.json {
        for d in &discovered {
            if let Some(dir) = d.root.file_name().and_then(|s| s.to_str())
                && dir != d.manifest.app
            {
                eprintln!(
                    "warning: app {:?} is in directory {:?} — name and `app:` field disagree; run `aware app rename` to re-sync",
                    d.manifest.app, dir
                );
            }
        }
    }

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

#[cfg(test)]
mod validate_target_tests {
    use super::resolve_validate_target;

    /// The regression: passing the manifest file itself used to reach `read_dir` and
    /// fail with a raw OS error instead of validating.
    #[test]
    fn accepts_the_manifest_file_directly() {
        let dir = tempfile::tempdir().unwrap();
        let app = dir.path().join("demo.app");
        std::fs::write(&app, "app: demo\n").unwrap();
        assert_eq!(resolve_validate_target(&app).unwrap(), app);
    }

    #[test]
    fn accepts_the_containing_directory() {
        let dir = tempfile::tempdir().unwrap();
        let app = dir.path().join("demo.app");
        std::fs::write(&app, "app: demo\n").unwrap();
        assert_eq!(resolve_validate_target(dir.path()).unwrap(), app);
    }

    /// `<dir-name>.flo` wins over other candidates, so the answer does not depend on
    /// directory iteration order.
    #[test]
    fn directory_prefers_the_canonical_manifest() {
        let dir = tempfile::tempdir().unwrap();
        let name = dir
            .path()
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_string();
        let canonical = dir.path().join(format!("{name}.flo"));
        std::fs::write(dir.path().join("aaa-other.app"), "app: other\n").unwrap();
        std::fs::write(&canonical, "app: demo\n").unwrap();
        assert_eq!(resolve_validate_target(dir.path()).unwrap(), canonical);
    }

    #[test]
    fn missing_path_says_so_rather_than_blaming_a_directory_scan() {
        let dir = tempfile::tempdir().unwrap();
        let err = resolve_validate_target(&dir.path().join("nope.app")).unwrap_err();
        assert!(
            err.to_string().contains("does not exist"),
            "expected a not-found message, got: {err}"
        );
    }

    #[test]
    fn empty_directory_reports_no_manifest() {
        let dir = tempfile::tempdir().unwrap();
        let err = resolve_validate_target(dir.path()).unwrap_err();
        assert!(
            err.to_string().contains("no .flo or .app file"),
            "expected the no-manifest message, got: {err}"
        );
    }
}

#[cfg(test)]
mod freeze_tests {
    use super::{clear_node_frozen, set_node_frozen};

    const FLO: &str = "# top comment\napp: demo\nversion: 0.1.0\ndescription: x\nnodes:\n  - id: read\n    agent: tekla\n    command: exec\n    config:\n      code: return 1\n  - id: view\n    agent: viewer-3d\n    command: render\nconnections: []\nrequires: []\n";

    #[test]
    fn set_then_clear_round_trips_and_preserves_comments() {
        let frozen = set_node_frozen(FLO, "read", "kept: true\nn: 7\n").unwrap();
        assert!(frozen.contains("# top comment"), "comments preserved");
        assert!(frozen.contains("    frozen:"), "frozen at the field indent");
        assert!(
            frozen.contains("      kept: true"),
            "value indented under frozen"
        );
        // The frozen value parses back, and only the targeted node is frozen.
        let app: crate::manifest::App = serde_yaml::from_str(&frozen).unwrap();
        assert!(
            app.nodes
                .iter()
                .find(|n| n.id == "read")
                .unwrap()
                .frozen
                .is_some()
        );
        assert!(
            app.nodes
                .iter()
                .find(|n| n.id == "view")
                .unwrap()
                .frozen
                .is_none()
        );
        // Clearing returns the exact original (comment-preserving round-trip).
        assert_eq!(clear_node_frozen(&frozen, "read").unwrap(), FLO);
    }

    #[test]
    fn refreeze_replaces_not_duplicates() {
        let once = set_node_frozen(FLO, "read", "a: 1\n").unwrap();
        let twice = set_node_frozen(&once, "read", "b: 2\n").unwrap();
        assert_eq!(
            twice.matches("frozen:").count(),
            1,
            "one frozen block, replaced"
        );
        assert!(twice.contains("b: 2") && !twice.contains("a: 1"));
    }

    #[test]
    fn clear_is_a_noop_when_not_frozen_or_unknown_node() {
        assert_eq!(clear_node_frozen(FLO, "read").unwrap(), FLO);
        assert_eq!(clear_node_frozen(FLO, "nope").unwrap(), FLO);
    }

    #[test]
    fn set_unknown_node_errors() {
        assert!(set_node_frozen(FLO, "nope", "a: 1\n").is_err());
    }

    #[test]
    fn matches_quoted_and_inline_comment_id_forms() {
        let flo = "nodes:\n  - id: \"read\"  # the reader\n    agent: x\n";
        let out = set_node_frozen(flo, "read", "k: 1\n").unwrap();
        assert!(
            out.contains("    frozen:"),
            "matched a quoted id carrying an inline comment"
        );
    }

    #[test]
    fn preserves_crlf_line_endings() {
        let flo = "nodes:\r\n  - id: read\r\n    agent: x\r\n";
        let frozen = set_node_frozen(flo, "read", "k: 1\n").unwrap();
        assert!(frozen.contains("\r\n"), "kept CRLF");
        assert!(
            !frozen.replace("\r\n", "").contains('\r'),
            "no bare \\r left behind"
        );
        assert_eq!(
            clear_node_frozen(&frozen, "read").unwrap(),
            flo,
            "CRLF set→clear round-trip"
        );
    }
}
