//! `aware agent ...` — manage installed agents.
//!
//! Phase v0.1 (read-only): `list`, `describe`, `skill`.
//! Phase v0.2 (install + validate): `install`, `uninstall`, `update`,
//!   `validate`, `publish`.

use clap::Subcommand;

use crate::context::Context;
use crate::error::AwareError;

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

pub fn dispatch(cmd: AgentCommand, _ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        AgentCommand::List => Err(AwareError::NotYetImplemented("agent list")),
        AgentCommand::Describe { .. } => Err(AwareError::NotYetImplemented("agent describe")),
        AgentCommand::Skill { .. } => Err(AwareError::NotYetImplemented("agent skill")),
        AgentCommand::Install { .. } => Err(AwareError::NotYetImplemented("agent install")),
        AgentCommand::Uninstall { .. } => Err(AwareError::NotYetImplemented("agent uninstall")),
        AgentCommand::Update { .. } => Err(AwareError::NotYetImplemented("agent update")),
        AgentCommand::Validate { .. } => Err(AwareError::NotYetImplemented("agent validate")),
        AgentCommand::Publish { .. } => Err(AwareError::NotYetImplemented("agent publish")),
    }
}
