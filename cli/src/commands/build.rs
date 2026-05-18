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
    /// (v0.7.2) Rubygems package spec: <gem>@<version>.
    #[arg(long = "from-ruby")]
    pub from_ruby: Option<String>,
    /// (v0.7.2) YARD-rendered docs URL or local directory.
    #[arg(long = "from-yard")]
    pub from_yard: Option<String>,
    /// (v0.8.1) npm package spec: <pkg>@<version>. Reflects from TypeScript .d.ts files.
    #[arg(long = "from-npm")]
    pub from_npm: Option<String>,
    /// (v0.30) Build an agent from a host-coverage IR file.
    #[arg(long = "from-coverage")]
    pub from_coverage: Option<std::path::PathBuf>,
    /// (tier C — v0.5.1) Run decompile pass on DLLs / NuGet.
    #[arg(long)]
    pub decompile: bool,
    /// Accept a non-permissive license on --from-nuget.
    #[arg(long = "accept-license")]
    pub accept_license: bool,
    /// Override the generated agent id (default: derived from source).
    #[arg(long)]
    pub output: Option<String>,
    /// (v0.30) Agent id for --from-coverage (required for that source).
    #[arg(long = "agent-id")]
    pub agent_id: Option<String>,
    /// (v0.30) Vendor name for --from-coverage (e.g. "trimble", "autodesk").
    #[arg(long)]
    pub vendor: Option<String>,
    /// (v0.30) Vertical for --from-coverage (e.g. "engineering", "construction").
    #[arg(long)]
    pub vertical: Option<String>,
}

pub fn dispatch(cmd: BuildCommand, ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        BuildCommand::Agent(args) => build_agent(ctx, &args),
    }
}

fn build_agent(ctx: &Context, args: &BuildAgentArgs) -> Result<(), AwareError> {
    use crate::builder;

    // v0.30 `--from-coverage` writes manifest + skills + catalog directly via the
    // C# sidecar (it doesn't build a GeneratedAgent in-process). Handle it first
    // and return — the rest of the dispatch is for builders that produce a
    // GeneratedAgent + then go through write_agent().
    if let Some(ir_path) = &args.from_coverage {
        let agent_id = args
            .agent_id
            .as_deref()
            .ok_or_else(|| AwareError::Validation("--from-coverage requires --agent-id".into()))?;
        let agent_id = normalize_agent_id(agent_id)?;
        let vendor = args.vendor.as_deref().unwrap_or("unknown");
        let vertical = args.vertical.as_deref().unwrap_or("engineering");
        let out = ctx.paths.aware_home.join("build").join(&agent_id);
        let result =
            builder::coverage::build_from_coverage(ir_path, &out, &agent_id, vendor, vertical)?;
        println!("{}", serde_json::to_string_pretty(&result)?);
        return Ok(());
    }

    let normalized_output: Option<String> =
        args.output.as_deref().map(normalize_agent_id).transpose()?;
    let id_override = normalized_output.as_deref();

    let generated = if args.decompile {
        builder::stubs::build_with_decompile(args.from_nuget.as_deref().unwrap_or(""))?
    } else if let Some(s) = &args.from_openapi {
        builder::openapi::build_from_url_or_path(s, id_override)?
    } else if let Some(s) = &args.from_cli {
        builder::cli_help::build_from_cli(s, id_override)?
    } else if let Some(s) = &args.from_nuget {
        builder::nuget::build_from_nuget(s, id_override, args.accept_license)?
    } else if let Some(s) = &args.from_python {
        builder::python::build_from_python(s, id_override)?
    } else if let Some(s) = &args.from_dlls {
        builder::stubs::build_from_dlls(s, id_override)?
    } else if let Some(s) = &args.from_com {
        builder::stubs::build_from_com(s, id_override)?
    } else if let Some(s) = &args.from_headers {
        builder::stubs::build_from_headers(s, id_override)?
    } else if let Some(s) = &args.from_ruby {
        builder::ruby::build_from_ruby(s, id_override)?
    } else if let Some(s) = &args.from_yard {
        builder::yard::build_from_url_or_dir(s, id_override)?
    } else if let Some(s) = &args.from_npm {
        builder::npm::build_from_npm(s, id_override)?
    } else {
        return Err(AwareError::Validation(
            "aware build agent: must specify one of --from-openapi, --from-cli, --from-nuget, --from-python, --from-dlls, --from-com, --from-headers, --from-ruby, --from-yard, --from-npm, --from-coverage".into()
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

/// Normalize the user-supplied `--output` into a clean agent id.
///
/// `--output` is documented as an "agent id override" but users naturally pass
/// path-like strings (e.g. `/tmp/scratch/tekla-2025`). Without sanitization,
/// the raw path is baked into the manifest's `agent:` field and the
/// `transport.cli.binary:` field, producing unusable agents.
///
/// Cross-platform basename: splits on both `/` and `\` since `Path::file_name`
/// only recognizes the host OS's separator (so a Windows path on a Linux
/// runner would otherwise pass through unchanged).
fn normalize_agent_id(raw: &str) -> Result<String, AwareError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(AwareError::Validation("--output cannot be empty".into()));
    }

    let basename = trimmed.rsplit(['/', '\\']).next().unwrap_or(trimmed).trim();

    if basename.is_empty() {
        return Err(AwareError::Validation(format!(
            "--output '{raw}' yields empty agent id"
        )));
    }

    if !basename
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.'))
    {
        return Err(AwareError::Validation(format!(
            "--output '{raw}' (resolved id: '{basename}') contains characters outside [a-z0-9-_.]"
        )));
    }

    Ok(basename.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalize_simple_id_passes_through() {
        assert_eq!(normalize_agent_id("tekla-2025").unwrap(), "tekla-2025");
    }

    #[test]
    fn normalize_unix_path_extracts_basename() {
        assert_eq!(
            normalize_agent_id("/tmp/scratch/tekla-2025").unwrap(),
            "tekla-2025"
        );
    }

    #[test]
    fn normalize_windows_path_extracts_basename() {
        assert_eq!(
            normalize_agent_id("C:\\Users\\foo\\tekla-2025").unwrap(),
            "tekla-2025"
        );
    }

    #[test]
    fn normalize_mixed_separators_extracts_basename() {
        assert_eq!(
            normalize_agent_id("C:/Users/foo/tekla-2025").unwrap(),
            "tekla-2025"
        );
    }

    #[test]
    fn normalize_rejects_empty() {
        assert!(normalize_agent_id("").is_err());
        assert!(normalize_agent_id("   ").is_err());
    }

    #[test]
    fn normalize_rejects_trailing_separator() {
        assert!(normalize_agent_id("/tmp/scratch/").is_err());
        assert!(normalize_agent_id("C:\\Users\\foo\\").is_err());
    }

    #[test]
    fn normalize_rejects_invalid_chars() {
        assert!(normalize_agent_id("tekla@2025").is_err());
        assert!(normalize_agent_id("tekla 2025").is_err());
        assert!(normalize_agent_id("tekla$2025").is_err());
    }

    #[test]
    fn normalize_allows_dot_underscore_dash() {
        assert_eq!(normalize_agent_id("tekla_2025").unwrap(), "tekla_2025");
        assert_eq!(normalize_agent_id("tekla.2025").unwrap(), "tekla.2025");
        assert_eq!(normalize_agent_id("Tekla-2025.0").unwrap(), "Tekla-2025.0");
    }
}
