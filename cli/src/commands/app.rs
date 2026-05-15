//! `aware app ...` — manage installed apps.
//!
//! Phase v0.1 (read-only): `list`, `show`.
//! Phase v0.2 (install + validate): `install`, `uninstall`, `validate`, `export`.
//! Phase v0.3 (runtime):  `run`, `stop`, `logs`.

use clap::Subcommand;

use crate::context::Context;
use crate::error::AwareError;

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
        /// Override app input as `key=value`. Repeatable.
        #[arg(long, action = clap::ArgAction::Append)]
        config: Vec<String>,
    },
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
    },
}

pub fn dispatch(cmd: AppCommand, _ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        AppCommand::List => Err(AwareError::NotYetImplemented("app list")),
        AppCommand::Show { .. } => Err(AwareError::NotYetImplemented("app show")),
        AppCommand::Install { .. } => Err(AwareError::NotYetImplemented("app install")),
        AppCommand::Uninstall { .. } => Err(AwareError::NotYetImplemented("app uninstall")),
        AppCommand::Validate { .. } => Err(AwareError::NotYetImplemented("app validate")),
        AppCommand::Export { .. } => Err(AwareError::NotYetImplemented("app export")),
        AppCommand::Run { .. } => Err(AwareError::NotYetImplemented("app run")),
        AppCommand::Stop { .. } => Err(AwareError::NotYetImplemented("app stop")),
        AppCommand::Logs { .. } => Err(AwareError::NotYetImplemented("app logs")),
    }
}
