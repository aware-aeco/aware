//! `aware` — the AWARE CLI.
//!
//! See `10-core/cli-spec.md` for the full surface contract and
//! `10-core/cli-roadmap.md` for phased delivery.
//!
//! The command surface spans v0.1–v0.27 (agent/app/connect/skill/build/
//! coverage/doctor/plugins/diagram/tree/search/report/voice/key/receipt).
//! Most groups are implemented. Known `NotYetImplemented` stubs:
//! `agent publish`, the Windows-only `build --from-com` guard,
//! `CliInvoker::invoke_stream` (streaming/stateful-agent transport), and
//! the v0.19 substrate primitives other than `assert:` (parsed +
//! validated; runtime execution lands in v0.19.x patches).

mod app_lock;
mod auth;
mod builder;
mod commands;
mod receipt;
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

    /// Manage host-coverage IR files and reviews (v0.30).
    Coverage {
        #[command(subcommand)]
        action: commands::coverage::CoverageCommand,
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

    /// Print a text tree of an agent's commands grouped by owning class.
    Tree(commands::tree::TreeArgs),

    /// Search command names + descriptions across installed agents.
    Search(commands::search::SearchArgs),

    /// Render reports about the substrate (HTML, etc.).
    Report {
        #[command(subcommand)]
        action: commands::report::ReportCommand,
    },

    /// Manage voice packs for panel review (v0.25).
    Voice {
        #[command(subcommand)]
        action: commands::voice::VoiceCommand,
    },

    /// Manage ed25519 signing keys for receipts (v0.27).
    Key {
        #[command(subcommand)]
        action: commands::key::KeyCommand,
    },

    /// Inspect / verify signed JSONL receipts (v0.27).
    Receipt {
        #[command(subcommand)]
        action: commands::receipt_cli::ReceiptCommand,
    },

    /// Download and manage host-execution bridge binaries
    /// (tekla, rhino, sketchup, revit).
    Sidecar {
        #[command(subcommand)]
        action: commands::sidecar::SidecarCommand,
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
        Command::Coverage { action } => commands::coverage::dispatch(action, &ctx),
        Command::Doctor => commands::doctor::run(&ctx),
        Command::Plugins { action } => commands::plugins::dispatch(action, &ctx),
        Command::Diagram { action } => commands::diagram::dispatch(action, &ctx),
        Command::Tree(args) => commands::tree::run(&ctx, &args),
        Command::Search(args) => commands::search::run(&ctx, &args),
        Command::Report { action } => commands::report::dispatch(action, &ctx),
        Command::Voice { action } => commands::voice::dispatch(action, &ctx),
        Command::Key { action } => commands::key::dispatch(action, &ctx),
        Command::Receipt { action } => commands::receipt_cli::dispatch(action, &ctx),
        Command::Sidecar { action } => commands::sidecar::dispatch(action, &ctx),
    };

    match result {
        Ok(()) => Ok(()),
        Err(err) => {
            eprintln!("error: {err}");
            std::process::exit(err.exit_code());
        }
    }
}
