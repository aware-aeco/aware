//! `aware` — the AWARE CLI.
//!
//! See `10-core/cli-spec.md` for the full surface contract and
//! `10-core/cli-roadmap.md` for phased delivery.
//!
//! v0.1 scaffold — every subcommand is a stub that returns
//! `Err(AwareError::NotYetImplemented(<command>))`. The fresh session
//! implements them per the roadmap, one phase at a time.

mod auth;
mod builder;
mod commands;
mod context;
mod envelope;
mod error;
mod install;
mod lockfile;
mod manifest;
mod paths;
mod plugins;
mod registry;
mod render;
mod runtime;
mod sidecar;
mod skill_builder;
mod validate;

use clap::{Parser, Subcommand};

use crate::error::AwareError;

/// `aware` — the open-source agentic substrate for AECO.
///
/// See https://github.com/aware-aeco/aware for the substrate (specs,
/// reference agents, manifesto, decalog).
#[derive(Parser, Debug)]
#[command(name = "aware", version, about, long_about = None, arg_required_else_help = true)]
struct Cli {
    #[command(subcommand)]
    command: Command,

    /// Output structured JSON instead of human-readable text.
    #[arg(long, global = true)]
    json: bool,

    /// Override the config file location (default: ~/.aware/config.yaml).
    #[arg(long, global = true)]
    config: Option<std::path::PathBuf>,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Manage installed agents (list, describe, install, validate, …).
    Agent {
        #[command(subcommand)]
        action: commands::agent::AgentCommand,
    },

    /// Manage installed apps (list, show, install, run, stop, …).
    App {
        #[command(subcommand)]
        action: commands::app::AppCommand,
    },

    /// Provision OAuth credentials for an integration (`trimble-connect`,
    /// `microsoft-365`, `google-workspace`, …).
    Connect(commands::connect::ConnectArgs),

    /// Delete the credential file for an integration.
    Disconnect(commands::connect::DisconnectArgs),

    /// Author / port / modify / eval AWARE agent skills.
    Skill {
        #[command(subcommand)]
        action: commands::skill::SkillCommand,
    },

    /// Generate an agent from a source (DLLs, NuGet, OpenAPI, CLI, …).
    Build {
        #[command(subcommand)]
        action: commands::build::BuildCommand,
    },

    /// Health check — config, filesystem, credentials, registry.
    Doctor,

    /// Manage host-side plugin folders for claude-code / codex / opencode.
    Plugins {
        #[command(subcommand)]
        action: commands::plugins::PluginsCommand,
    },

    /// Regenerate the master Mermaid diagram from installed agents.
    Diagram {
        #[command(subcommand)]
        action: commands::diagram::DiagramCommand,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let paths = match crate::paths::Paths::from_env() {
        Ok(p) => p,
        Err(err) => {
            eprintln!("error: {err}");
            std::process::exit(err.exit_code());
        }
    };
    let ctx = crate::context::Context {
        paths,
        json: cli.json,
    };

    let result: Result<(), AwareError> = match cli.command {
        Command::Agent { action } => commands::agent::dispatch(action, &ctx),
        Command::App { action } => commands::app::dispatch(action, &ctx).await,
        Command::Connect(args) => commands::connect::run_connect(args, &ctx),
        Command::Disconnect(args) => commands::connect::run_disconnect(args, &ctx),
        Command::Skill { action } => commands::skill::dispatch(action, &ctx),
        Command::Build { action } => commands::build::dispatch(action, &ctx),
        Command::Doctor => commands::doctor::run(&ctx),
        Command::Plugins { action } => commands::plugins::dispatch(action, &ctx),
        Command::Diagram { action } => commands::diagram::dispatch(action, &ctx),
    };

    match result {
        Ok(()) => Ok(()),
        Err(err) => {
            eprintln!("error: {err}");
            std::process::exit(err.exit_code());
        }
    }
}
