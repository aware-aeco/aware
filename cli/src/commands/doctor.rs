//! `aware doctor` — health check.
//!
//! Phase v0.1 (limited): config + filesystem only.

use crate::context::Context;
use crate::error::AwareError;
use crate::manifest::loader::{discover_agents, discover_apps};

pub fn run(ctx: &Context) -> Result<(), AwareError> {
    if ctx.json {
        run_json(ctx)
    } else {
        run_text(ctx)
    }
}

// ── JSON output ───────────────────────────────────────────────────────────────

/// #627: finish or roll back every interrupted agent swap, before anything
/// is listed, so the report describes complete agents. Touches nothing (not
/// even the store lock file) on a home that has never swapped an agent.
fn recover_agent_swaps(ctx: &Context) -> Result<Vec<crate::install::swap::DoctorFinding>, String> {
    if !ctx.paths.agent_swap_dir().is_dir() {
        return Ok(Vec::new());
    }
    let guard = crate::agent_store::open(&ctx.paths).map_err(|e| e.to_string())?;
    crate::install::swap::recover_all(&ctx.paths, &guard).map_err(|e| e.to_string())
}

fn run_json(ctx: &Context) -> Result<(), AwareError> {
    let aware_home = &ctx.paths.aware_home;
    let swaps = recover_agent_swaps(ctx);
    let agents = discover_agents(&ctx.paths).unwrap_or_default();
    // An unreadable manifest no longer empties the whole app list (#659): the
    // readable apps are listed and each unreadable one is reported by name.
    let app_discovery = discover_apps(&ctx.paths).unwrap_or_default();
    let apps = &app_discovery.apps;

    // Integrity issues
    let mut integrity_issues: Vec<serde_json::Value> = Vec::new();
    for d in &agents {
        for issue in crate::validate::validate_agent_on_disk(&d.manifest, &d.root) {
            integrity_issues.push(serde_json::json!({
                "agent": d.manifest.agent,
                "severity": issue.severity.to_string(),
                "code": issue.code,
                "message": issue.message,
            }));
        }
    }

    // Running instances
    let apps_dir = ctx.paths.apps_dir();
    let mut running: Vec<serde_json::Value> = Vec::new();
    if let Ok(read) = std::fs::read_dir(&apps_dir) {
        for app_entry in read.flatten() {
            let app_path = app_entry.path();
            if !app_path.is_dir() {
                continue;
            }
            let instances_dir = app_path.join("instances");
            if let Ok(instance_read) = std::fs::read_dir(&instances_dir) {
                for inst_entry in instance_read.flatten() {
                    let inst_path = inst_entry.path();
                    if !inst_path.is_dir() {
                        continue;
                    }
                    if let Ok(pid) = crate::runtime::pidfile::read(&inst_path) {
                        running.push(serde_json::json!({
                            "app": pid.app,
                            "instance": pid.instance,
                            "pid": pid.pid,
                            "run_id": pid.run_id,
                            "started_at": pid.started_at,
                        }));
                    }
                }
            }
        }
    }

    // Credentials — use the read-only status renderer. Doctor must never
    // materialize legacy metadata or create credential lock files.
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let credentials: Vec<serde_json::Value> =
        ["trimble-connect", "microsoft-365", "google-workspace"]
            .iter()
            .map(|integration| {
                crate::commands::connect::credential_status_json_read_only(
                    integration,
                    None,
                    aware_home,
                    now,
                )
            })
            .collect();

    // Host bridges
    let install_dir = crate::commands::sidecar::bridge_install_dir(ctx);
    let bridge_ids = ["tekla", "rhino", "sketchup", "revit"];
    let host_bridges: Vec<serde_json::Value> = bridge_ids
        .iter()
        .map(
            |id| match crate::commands::sidecar::find_bridge_by_id(id, &install_dir) {
                Some(p) => serde_json::json!({
                    "id": id,
                    "installed": true,
                    "path": p.display().to_string(),
                }),
                None => serde_json::json!({
                    "id": id,
                    "installed": false,
                    "path": null,
                }),
            },
        )
        .collect();

    let output = serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
        "aware_home": aware_home.display().to_string(),
        "aware_home_exists": aware_home.is_dir(),
        "config_path": ctx.paths.config_path().display().to_string(),
        "config_exists": ctx.paths.config_path().is_file(),
        "agents": agents.iter().map(|a| serde_json::json!({
            "id": a.manifest.agent,
            "version": a.manifest.version,
        })).collect::<Vec<_>>(),
        "apps": apps.iter().map(|a| serde_json::json!({
            "id": a.manifest.app,
            "version": a.manifest.version,
        })).collect::<Vec<_>>(),
        "invalid_apps": app_discovery.unreadable.iter().map(|u| serde_json::json!({
            "id": u.dir_name(),
            "path": u.manifest_path.display().to_string(),
            "message": u.error.to_string(),
        })).collect::<Vec<_>>(),
        "integrity": {
            "all_pass": integrity_issues.is_empty(),
            "issues": integrity_issues,
        },
        "running": running,
        "agent_swaps": match &swaps {
            Ok(findings) => serde_json::json!({ "ok": true, "transactions": findings }),
            Err(error) => serde_json::json!({ "ok": false, "error": error }),
        },
        "credentials": credentials,
        "host_bridges": host_bridges,
    });

    println!("{}", serde_json::to_string_pretty(&output)?);
    Ok(())
}

// ── Human-readable output ─────────────────────────────────────────────────────

fn run_text(ctx: &Context) -> Result<(), AwareError> {
    let aware_home = &ctx.paths.aware_home;
    let agents_dir = ctx.paths.agents_dir();
    let apps_dir = ctx.paths.apps_dir();
    let config_path = ctx.paths.config_path();

    println!("CLI:");
    println!(
        "  \u{2713} aware v{} (build profile)",
        env!("CARGO_PKG_VERSION")
    );
    if config_path.is_file() {
        println!("  \u{2713} Config at {}", config_path.display());
    } else {
        println!("  \u{00b7} No config.yaml yet (will be created on first write)");
    }
    println!();

    println!("Filesystem:");
    if aware_home.is_dir() {
        println!("  \u{2713} {} exists", aware_home.display());
    } else {
        println!("  \u{00b7} {} does not exist yet", aware_home.display());
    }
    println!();

    let swaps = recover_agent_swaps(ctx);
    let agents = discover_agents(&ctx.paths).unwrap_or_default();
    // An unreadable manifest no longer empties the whole app list (#659): the
    // readable apps are listed and each unreadable one is reported by name.
    let app_discovery = discover_apps(&ctx.paths).unwrap_or_default();
    let apps = &app_discovery.apps;

    match &swaps {
        Ok(findings) if findings.is_empty() => {}
        Ok(findings) => {
            println!("Agent swaps:");
            for finding in findings {
                let tag = if finding.outcome == "needs-you" {
                    "\u{2717}"
                } else {
                    "\u{2713}"
                };
                println!(
                    "  {tag} {} ({}): {}{}",
                    finding.txn,
                    finding.ids.join(", "),
                    finding.outcome,
                    finding
                        .detail
                        .as_deref()
                        .map(|d| format!(" \u{2014} {d}"))
                        .unwrap_or_default()
                );
            }
            println!();
        }
        Err(error) => {
            println!("Agent swaps:");
            println!("  \u{2717} could not check interrupted agent swaps: {error}");
            println!();
        }
    }

    println!("Agents:");
    println!(
        "  \u{2713} {} installed (under {})",
        agents.len(),
        agents_dir.display()
    );
    for a in &agents {
        println!("    - {}@{}", a.manifest.agent, a.manifest.version);
    }
    println!();

    println!("Apps:");
    println!(
        "  \u{2713} {} installed (under {})",
        apps.len(),
        apps_dir.display()
    );
    for a in apps {
        println!("    - {}@{}", a.manifest.app, a.manifest.version);
    }
    for u in &app_discovery.unreadable {
        println!("  \u{2717} {} unreadable: {}", u.dir_name(), u.error);
    }

    println!();
    println!("Integrity:");
    let mut total_issues = 0usize;
    for d in &agents {
        let issues = crate::validate::validate_agent_on_disk(&d.manifest, &d.root);
        if issues.is_empty() {
            continue;
        }
        total_issues += issues.len();
        println!("  Agent {}:", d.manifest.agent);
        for i in &issues {
            let tag = match i.severity {
                crate::validate::Severity::Error => "\u{2717}",
                crate::validate::Severity::Warning => "\u{26a0}",
            };
            println!("    {tag} [{}] {}", i.code, i.message);
        }
    }
    if total_issues == 0 {
        println!("  \u{2713} all installed agents pass validation");
    }

    println!();
    println!("Running:");
    let mut found_any = false;
    if let Ok(read) = std::fs::read_dir(&apps_dir) {
        for app_entry in read.flatten() {
            let app_path = app_entry.path();
            if !app_path.is_dir() {
                continue;
            }
            let instances_dir = app_path.join("instances");
            if let Ok(instance_read) = std::fs::read_dir(&instances_dir) {
                for inst_entry in instance_read.flatten() {
                    let inst_path = inst_entry.path();
                    if !inst_path.is_dir() {
                        continue;
                    }
                    if let Ok(pid) = crate::runtime::pidfile::read(&inst_path) {
                        found_any = true;
                        println!(
                            "  \u{25b6} {}/{} (pid {}, run-id {}, started {})",
                            pid.app, pid.instance, pid.pid, pid.run_id, pid.started_at
                        );
                    }
                }
            }
        }
    }
    if !found_any {
        println!("  \u{00b7} no running instances");
    }

    println!();
    println!("Credentials:");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    for integration in &["trimble-connect", "microsoft-365", "google-workspace"] {
        crate::commands::connect::print_credential_status_text_read_only(
            integration,
            None,
            aware_home,
            now,
        );
    }

    println!();
    println!("Host Bridges:");
    let install_dir = crate::commands::sidecar::bridge_install_dir(ctx);
    let bridge_ids = ["tekla", "rhino", "sketchup", "revit"];
    let mut any_missing = false;
    for id in &bridge_ids {
        match crate::commands::sidecar::find_bridge_by_id(id, &install_dir) {
            Some(p) => {
                println!("  \u{2713} aware-{id:<10}  {}", p.display());
            }
            None => {
                any_missing = true;
                println!("  \u{2717} aware-{id:<10}  not found — run: aware sidecar install {id}");
            }
        }
    }
    if !any_missing {
        println!("  all host bridges present");
    }

    Ok(())
}
