//! `aware build ...` — agent-builder commands.
//!
//! Phase v0.5. See `20-agents/_core/aware-agent-builder/` for the spec.

use clap::Subcommand;

use crate::error::AwareError;

#[derive(Subcommand, Debug)]
pub enum BuildCommand {
    /// Generate an agent from a source surface.
    Agent {
        /// Local DLL folder glob.
        #[arg(long, conflicts_with_all = ["from_nuget", "from_openapi", "from_com", "from_cli", "from_headers", "from_python"])]
        from_dlls: Option<String>,
        /// NuGet package id (optionally `@version`).
        #[arg(long, conflicts_with_all = ["from_dlls", "from_openapi", "from_com", "from_cli", "from_headers", "from_python"])]
        from_nuget: Option<String>,
        /// OpenAPI spec URL or file path.
        #[arg(long, conflicts_with_all = ["from_dlls", "from_nuget", "from_com", "from_cli", "from_headers", "from_python"])]
        from_openapi: Option<String>,
        /// COM ProgID.
        #[arg(long, conflicts_with_all = ["from_dlls", "from_nuget", "from_openapi", "from_cli", "from_headers", "from_python"])]
        from_com: Option<String>,
        /// CLI binary name (parses `<bin> --help`).
        #[arg(long, conflicts_with_all = ["from_dlls", "from_nuget", "from_openapi", "from_com", "from_headers", "from_python"])]
        from_cli: Option<String>,
        /// C++ headers glob.
        #[arg(long, conflicts_with_all = ["from_dlls", "from_nuget", "from_openapi", "from_com", "from_cli", "from_python"])]
        from_headers: Option<String>,
        /// Python module name (importable from local environment).
        #[arg(long, conflicts_with_all = ["from_dlls", "from_nuget", "from_openapi", "from_com", "from_cli", "from_headers"])]
        from_python: Option<String>,

        /// Override the generated agent id.
        #[arg(long)]
        output: Option<String>,

        /// Run ILSpy on DLLs missing XML docs. License-checked.
        #[arg(long)]
        decompile: bool,

        /// Skill organization strategy.
        #[arg(long, default_value = "auto")]
        tier_strategy: String,

        /// Print what would happen without writing.
        #[arg(long)]
        dry_run: bool,
    },
}

pub fn dispatch(cmd: BuildCommand) -> Result<(), AwareError> {
    match cmd {
        BuildCommand::Agent { .. } => Err(AwareError::NotYetImplemented("build agent")),
    }
}
