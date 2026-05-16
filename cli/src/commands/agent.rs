//! `aware agent ...` — manage installed agents.
//!
//! Phase v0.1 (read-only): `list`, `describe`, `skill`.
//! Phase v0.2 (install + validate): `install`, `uninstall`, `update`,
//!   `validate`, `publish`.

use std::time::Instant;

use clap::Subcommand;
use serde::Serialize;

use crate::context::Context;
use crate::envelope;
use crate::error::AwareError;
use crate::manifest::loader::discover_agents;
use crate::render::table::Table;

#[derive(Subcommand, Debug)]
pub enum AgentCommand {
    /// Print a table of installed agents.
    List,
    /// Print an agent's manifest summary + skill index + command list.
    Describe {
        /// Agent id (e.g. `tekla`, `trimble-connect`).
        agent: String,
    },
    /// Print a skill's content.
    Skill {
        /// Agent id.
        agent: String,
        /// Skill filename (e.g. `drawing-identity.md`) or skill name from frontmatter.
        skill: String,
    },
    /// Install an agent from the registry or a local path. (v0.2)
    Install {
        /// Agent spec (`<name>[@version]`) or a local folder path.
        spec: String,
    },
    /// Uninstall an agent. (v0.2)
    Uninstall { agent: String },
    /// Re-pull the latest matching version of an agent. (v0.2)
    Update { agent: String },
    /// Validate an agent folder against the agent-spec. (v0.2)
    Validate {
        /// Path to an agent folder containing manifest.yaml.
        path: std::path::PathBuf,
    },
    /// Open a PR to the GitHub registry. (v0.2+)
    Publish { path: std::path::PathBuf },
}

pub fn dispatch(cmd: AgentCommand, ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        AgentCommand::List => list(ctx),
        AgentCommand::Describe { agent } => describe(ctx, &agent),
        AgentCommand::Skill { agent, skill } => skill_cmd(ctx, &agent, &skill),
        AgentCommand::Install { spec } => install(ctx, &spec),
        AgentCommand::Uninstall { agent } => {
            crate::install::uninstall_agent(&agent, &ctx.paths)?;
            println!("✓ uninstalled {agent}");
            let _ = auto_regenerate_plugins(ctx);
            let _ = crate::commands::diagram::auto_regenerate(ctx);
            Ok(())
        }
        AgentCommand::Update { agent } => update(ctx, &agent),
        AgentCommand::Validate { path } => validate_cmd(ctx, &path),
        AgentCommand::Publish { .. } => Err(AwareError::NotYetImplemented("agent publish")),
    }
}

fn install(ctx: &Context, spec: &str) -> Result<(), AwareError> {
    use std::path::PathBuf;
    let path = PathBuf::from(spec);
    if path.is_dir() {
        let installed = crate::install::install_agent_from_path(&path, &ctx.paths)?;
        println!("✓ installed {installed} from {}", path.display());
        // Auto-regenerate host plugins (best-effort — failures don't tear down the install)
        let _ = auto_regenerate_plugins(ctx);
        let _ = crate::commands::diagram::auto_regenerate(ctx);
        return Ok(());
    }

    // Otherwise: treat as registry id [@version] or bundle name.
    let index = crate::registry::fetch::fetch_index(&ctx.paths.cache_dir())?;
    if index.bundles.contains_key(spec) {
        let report = crate::install::install_bundle(spec, &ctx.paths, &index)?;
        println!(
            "✓ bundle {}: {} installed, {} failed",
            report.bundle,
            report.installed.len(),
            report.failed.len()
        );
        for s in &report.installed {
            println!("  ✓ {s}");
        }
        for (s, e) in &report.failed {
            println!("  ✗ {s}: {e}");
        }
        // Auto-regenerate host plugins (best-effort — failures don't tear down the install)
        let _ = auto_regenerate_plugins(ctx);
        let _ = crate::commands::diagram::auto_regenerate(ctx);
        return Ok(());
    }
    let (id, version_pin) = match spec.split_once('@') {
        Some((i, v)) => (i, Some(v)),
        None => (spec, None),
    };
    let installed =
        crate::install::install_agent_from_registry(id, version_pin, &ctx.paths, &index)?;
    println!("✓ installed {installed}");
    // Auto-regenerate host plugins (best-effort — failures don't tear down the install)
    let _ = auto_regenerate_plugins(ctx);
    let _ = crate::commands::diagram::auto_regenerate(ctx);
    Ok(())
}

fn update(ctx: &Context, id: &str) -> Result<(), AwareError> {
    let index = crate::registry::fetch::fetch_index(&ctx.paths.cache_dir())?;
    // Best-effort: remove existing installation (ignore NotFound).
    let _ = crate::install::uninstall_agent(id, &ctx.paths);
    let installed = crate::install::install_agent_from_registry(id, None, &ctx.paths, &index)?;
    println!("✓ updated {installed}");
    let _ = auto_regenerate_plugins(ctx);
    let _ = crate::commands::diagram::auto_regenerate(ctx);
    Ok(())
}

fn validate_cmd(_ctx: &Context, path: &std::path::Path) -> Result<(), AwareError> {
    let manifest_path = path.join("manifest.yaml");
    let agent = crate::manifest::loader::load_agent(&manifest_path)?;
    let issues = crate::validate::validate_agent_on_disk(&agent, path);
    if issues.is_empty() {
        println!("✓ {} is valid", path.display());
        return Ok(());
    }
    for i in &issues {
        let tag = match i.severity {
            crate::validate::Severity::Error => "✗",
            crate::validate::Severity::Warning => "⚠",
        };
        println!("{tag} [{}] {}", i.code, i.message);
    }
    if crate::validate::has_errors(&issues) {
        return Err(AwareError::Validation("agent failed validation".into()));
    }
    Ok(())
}

#[derive(Serialize)]
struct AgentListRow {
    id: String,
    version: String,
    kind: String,
    skills: usize,
    commands: usize,
}

#[derive(Serialize)]
struct AgentListData {
    agents: Vec<AgentListRow>,
}

fn describe(ctx: &Context, agent_id: &str) -> Result<(), AwareError> {
    let started = Instant::now();
    let discovered = discover_agents(&ctx.paths)?;
    let d = discovered
        .into_iter()
        .find(|d| d.manifest.agent == agent_id)
        .ok_or_else(|| AwareError::NotFound(format!("agent: {agent_id}")))?;

    if ctx.json {
        #[derive(Serialize)]
        struct CommandRow {
            name: String,
            lifecycle: String,
            description: String,
        }
        #[derive(Serialize)]
        struct DescribeData<'a> {
            agent: &'a str,
            version: &'a str,
            display_name: Option<&'a str>,
            description: &'a str,
            stateful: bool,
            license: &'a str,
            vendor: Option<&'a str>,
            commands: Vec<CommandRow>,
            skills: &'a [String],
            skill_count: usize,
            command_count: usize,
        }

        let cmds: Vec<CommandRow> = d
            .manifest
            .commands
            .iter()
            .map(|(n, c)| CommandRow {
                name: n.clone(),
                lifecycle: format!("{:?}", c.lifecycle).to_lowercase(),
                description: c.description.clone(),
            })
            .collect();

        let data = DescribeData {
            agent: &d.manifest.agent,
            version: &d.manifest.version,
            display_name: d.manifest.display_name.as_deref(),
            description: &d.manifest.description,
            stateful: d.manifest.stateful,
            license: &d.manifest.license,
            vendor: d.manifest.vendor.as_deref(),
            command_count: d.manifest.command_count(),
            skill_count: d.manifest.skill_count(),
            commands: cmds,
            skills: &d.manifest.skills,
        };
        envelope::print_ok("agent describe", data, started).ok();
        return Ok(());
    }

    let m = &d.manifest;
    println!("agent:        {}", m.agent);
    println!("version:      {}", m.version);
    if let Some(dn) = &m.display_name {
        println!("display-name: {dn}");
    }
    println!(
        "description:  {}",
        m.description.lines().next().unwrap_or("").trim()
    );
    println!("stateful:     {}", m.stateful);
    if let Some(v) = &m.vendor {
        println!("vendor:       {v}");
    }
    println!("license:      {}", m.license);
    if let Some(t) = &m.transport.cli {
        println!("transport:    cli ({})", t.binary);
    }
    println!();
    println!("commands:");
    for (name, c) in &m.commands {
        let lc = format!("{:?}", c.lifecycle).to_lowercase();
        let desc = c.description.lines().next().unwrap_or("").trim();
        println!("  {name:<18} {lc:<8} {desc}");
    }
    println!();
    println!("skills ({}):", m.skill_count());
    for s in &m.skills {
        println!("  - {s}");
    }
    Ok(())
}

fn skill_cmd(ctx: &Context, agent_id: &str, skill_name: &str) -> Result<(), AwareError> {
    let discovered = discover_agents(&ctx.paths)?;
    let d = discovered
        .into_iter()
        .find(|d| d.manifest.agent == agent_id)
        .ok_or_else(|| AwareError::NotFound(format!("agent: {agent_id}")))?;

    let filename = if skill_name.ends_with(".md") {
        skill_name.to_string()
    } else {
        format!("{skill_name}.md")
    };
    let path = d.root.join("skills").join(&filename);
    if !path.is_file() {
        return Err(AwareError::NotFound(format!(
            "skill: {agent_id}/{filename}"
        )));
    }
    let body = std::fs::read_to_string(&path)?;
    // Raw print — markdown is human-readable and AI-readable as-is.
    print!("{body}");
    Ok(())
}

fn auto_regenerate_plugins(ctx: &Context) -> Result<(), AwareError> {
    let home = dirs::home_dir().ok_or_else(|| AwareError::Internal("home dir".into()))?;
    let agents = crate::manifest::loader::discover_agents(&ctx.paths)?;

    // Only regen for hosts whose plugin dir already exists (or override env var set)
    let claude_target = std::env::var_os("AWARE_PLUGINS_CLAUDE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| home.join(".claude/plugins"));
    if claude_target.exists() || std::env::var_os("AWARE_PLUGINS_CLAUDE").is_some() {
        let _ = crate::plugins::claude_code::generate(&agents, &claude_target);
    }
    // codex / opencode left as scaffolds — regen on install would write the same TODO every time

    Ok(())
}

fn list(ctx: &Context) -> Result<(), AwareError> {
    let started = Instant::now();
    let discovered = discover_agents(&ctx.paths)?;

    if ctx.json {
        let data = AgentListData {
            agents: discovered
                .iter()
                .map(|d| AgentListRow {
                    id: d.manifest.agent.clone(),
                    version: d.manifest.version.clone(),
                    kind: d.manifest.kind(),
                    skills: d.manifest.skill_count(),
                    commands: d.manifest.command_count(),
                })
                .collect(),
        };
        envelope::print_ok("agent list", data, started).ok();
        return Ok(());
    }

    let mut t = Table::new(["ID", "VERSION", "KIND", "SKILLS", "COMMANDS"]);
    for d in &discovered {
        t.row([
            d.manifest.agent.clone(),
            d.manifest.version.clone(),
            d.manifest.kind(),
            d.manifest.skill_count().to_string(),
            d.manifest.command_count().to_string(),
        ]);
    }
    print!("{}", t.render());
    Ok(())
}
