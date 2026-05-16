//! `aware doctor` — health check.
//!
//! Phase v0.1 (limited): config + filesystem only.

use crate::context::Context;
use crate::error::AwareError;
use crate::manifest::loader::{discover_agents, discover_apps};

pub fn run(ctx: &Context) -> Result<(), AwareError> {
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

    let agents = discover_agents(&ctx.paths).unwrap_or_default();
    let apps = discover_apps(&ctx.paths).unwrap_or_default();

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
    for a in &apps {
        println!("    - {}@{}", a.manifest.app, a.manifest.version);
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
    Ok(())
}
