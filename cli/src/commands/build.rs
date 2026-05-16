//! `aware build agent ...` — generate AWARE agents from existing software's API surface.

use clap::{Args, Subcommand};

use crate::context::Context;
use crate::error::AwareError;

#[derive(Subcommand, Debug)]
pub enum BuildCommand {
    /// Generate an agent from a source (OpenAPI / CLI / NuGet / Python / DLLs / COM / headers).
    Agent(BuildAgentArgs),
}

#[derive(Args, Debug)]
pub struct BuildAgentArgs {
    /// OpenAPI spec URL or file path.
    #[arg(long = "from-openapi")]
    pub from_openapi: Option<String>,
    /// CLI binary name (must be on PATH).
    #[arg(long = "from-cli")]
    pub from_cli: Option<String>,
    /// NuGet package spec: <pkg>@<version>.
    #[arg(long = "from-nuget")]
    pub from_nuget: Option<String>,
    /// Python module name.
    #[arg(long = "from-python")]
    pub from_python: Option<String>,
    /// (tier C — v0.5.1) DLL glob path.
    #[arg(long = "from-dlls")]
    pub from_dlls: Option<String>,
    /// (tier C — v0.5.1) COM ProgID.
    #[arg(long = "from-com")]
    pub from_com: Option<String>,
    /// (tier C — v0.5.1) C++ header glob path.
    #[arg(long = "from-headers")]
    pub from_headers: Option<String>,
    /// (tier C — v0.5.1) Run decompile pass on DLLs / NuGet.
    #[arg(long)]
    pub decompile: bool,
    /// Accept a non-permissive license on --from-nuget.
    #[arg(long = "accept-license")]
    pub accept_license: bool,
    /// Override the generated agent id (default: derived from source).
    #[arg(long)]
    pub output: Option<String>,
}

pub fn dispatch(cmd: BuildCommand, ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        BuildCommand::Agent(args) => build_agent(ctx, &args),
    }
}

fn build_agent(ctx: &Context, args: &BuildAgentArgs) -> Result<(), AwareError> {
    use crate::builder;

    let generated = if args.decompile {
        builder::stubs::build_with_decompile(args.from_nuget.as_deref().unwrap_or(""))?
    } else if let Some(s) = &args.from_openapi {
        builder::openapi::build_from_url_or_path(s, args.output.as_deref())?
    } else if let Some(s) = &args.from_cli {
        builder::cli_help::build_from_cli(s, args.output.as_deref())?
    } else if let Some(s) = &args.from_nuget {
        builder::nuget::build_from_nuget(s, args.output.as_deref(), args.accept_license)?
    } else if let Some(s) = &args.from_python {
        builder::python::build_from_python(s, args.output.as_deref())?
    } else if let Some(s) = &args.from_dlls {
        builder::stubs::build_from_dlls(s, args.output.as_deref())?
    } else if let Some(s) = &args.from_com {
        builder::stubs::build_from_com(s, args.output.as_deref())?
    } else if let Some(s) = &args.from_headers {
        builder::stubs::build_from_headers(s, args.output.as_deref())?
    } else {
        return Err(AwareError::Validation(
            "aware build agent: must specify one of --from-openapi, --from-cli, --from-nuget, --from-python, --from-dlls, --from-com, --from-headers".into()
        ));
    };

    let agents_dir = ctx.paths.agents_dir();
    std::fs::create_dir_all(&agents_dir)?;
    let dst = builder::write_agent(&generated, &agents_dir)?;
    println!(
        "\u{2713} generated {} ({} commands, {} skills) at {}",
        generated.id,
        generated.commands.len(),
        generated.skills.len(),
        dst.display()
    );
    Ok(())
}
