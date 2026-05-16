//! `aware plugins ...` — manage host-side plugin folders.

use clap::Subcommand;

use crate::context::Context;
use crate::error::AwareError;

#[derive(Subcommand, Debug)]
pub enum PluginsCommand {
    /// Rebuild host plugin folders from installed agents.
    Regenerate,
}

pub fn dispatch(cmd: PluginsCommand, ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        PluginsCommand::Regenerate => regenerate(ctx),
    }
}

fn regenerate(ctx: &Context) -> Result<(), AwareError> {
    let agents = crate::manifest::loader::discover_agents(&ctx.paths)?;
    println!(
        "Regenerating host plugins from {} installed agents...",
        agents.len()
    );

    let home = dirs::home_dir()
        .ok_or_else(|| AwareError::Internal("could not determine home directory".into()))?;

    // claude-code
    let claude_target = std::env::var_os("AWARE_PLUGINS_CLAUDE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| home.join(".claude/plugins"));
    if claude_target.exists() || std::env::var_os("AWARE_PLUGINS_CLAUDE").is_some() {
        let count = crate::plugins::claude_code::generate(&agents, &claude_target)?;
        println!("  \u{2713} claude-code: {count} commands");
    } else {
        println!("  \u{00b7} claude-code: ~/.claude/plugins not present (skipped)");
    }

    // codex
    let codex_target = std::env::var_os("AWARE_PLUGINS_CODEX")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| home.join(".codex/plugins"));
    if codex_target.exists() || std::env::var_os("AWARE_PLUGINS_CODEX").is_some() {
        crate::plugins::codex::generate(&agents, &codex_target)?;
        println!("  \u{2713} codex: scaffold (format pending)");
    } else {
        println!("  \u{00b7} codex: ~/.codex/plugins not present (skipped)");
    }

    // opencode
    let opencode_target = std::env::var_os("AWARE_PLUGINS_OPENCODE")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| home.join(".opencode/plugins"));
    if opencode_target.exists() || std::env::var_os("AWARE_PLUGINS_OPENCODE").is_some() {
        crate::plugins::opencode::generate(&agents, &opencode_target)?;
        println!("  \u{2713} opencode: scaffold (format pending)");
    } else {
        println!("  \u{00b7} opencode: ~/.opencode/plugins not present (skipped)");
    }

    Ok(())
}
