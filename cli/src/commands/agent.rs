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
        AgentCommand::Install { .. } => Err(AwareError::NotYetImplemented("agent install")),
        AgentCommand::Uninstall { .. } => Err(AwareError::NotYetImplemented("agent uninstall")),
        AgentCommand::Update { .. } => Err(AwareError::NotYetImplemented("agent update")),
        AgentCommand::Validate { .. } => Err(AwareError::NotYetImplemented("agent validate")),
        AgentCommand::Publish { .. } => Err(AwareError::NotYetImplemented("agent publish")),
    }
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
