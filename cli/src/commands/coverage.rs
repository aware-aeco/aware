//! `aware coverage ...` — manage host-coverage IR + reviews (v0.30).
//!
//! Verbs:
//!   aware coverage generate <host> <version> --from-ir <ir.json>
//!   aware coverage validate <ir.json> [--agent-dir <dir>]
//!   aware coverage review <agent-id>
//!
//! `generate` regenerates a raw host agent from a coverage IR file (delegates
//! to `builder::coverage::build_from_coverage`, the same code path used by
//! `aware build agent --from-coverage`).
//!
//! `validate` performs Draft 2020-12 JSON Schema validation against
//! `cli-sidecar/Ingest/Schema/host-coverage.schema.json` (the root IR schema)
//! and, when an agent directory is supplied, against
//! `host-coverage-type.schema.json` for every `<agent>/catalog/*.json`. Both
//! schemas are embedded into the sidecar binary; this command is the single
//! canonical replacement for the out-of-tree `ajv@8` / Python `jsonschema`
//! step that the codex-coverage review protocol previously ran externally.
//!
//! `review` is a stub for v0.30 alpha — it prints the path to the review
//! protocol document. Real invocation of the codex-rescue reviewer lands in
//! Phase C subagent dispatch, not as a CLI command.

use clap::Subcommand;
use std::path::PathBuf;

use crate::context::Context;
use crate::error::AwareError;
use crate::sidecar::{CoverageValidateArgs, coverage_validate};

#[derive(Subcommand, Debug)]
pub enum CoverageCommand {
    /// Regenerate a raw host agent from a coverage IR file.
    Generate {
        host: String,
        version: String,
        #[arg(long)]
        from_ir: PathBuf,
        #[arg(long, default_value = "engineering")]
        vertical: String,
        #[arg(long)]
        vendor: Option<String>,
    },
    /// Validate an IR file against the host-coverage schema.
    Validate {
        ir: PathBuf,
        /// Optional: an installed agent directory (containing a `catalog/`
        /// subfolder). When set, every `catalog/*.json` is also validated
        /// against the per-Type schema.
        #[arg(long)]
        agent_dir: Option<PathBuf>,
    },
    /// Invoke the codex-rescue reviewer on an installed agent.
    Review { agent_id: String },
}

pub fn dispatch(cmd: CoverageCommand, ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        CoverageCommand::Generate {
            host,
            version,
            from_ir,
            vertical,
            vendor,
        } => {
            let agent_id = format!("{host}-{version}");
            let target = ctx.paths.aware_home.join("agents").join(&agent_id);
            let v = vendor.as_deref().unwrap_or("unknown");
            let result = crate::builder::coverage::build_from_coverage(
                &from_ir, &target, &agent_id, v, &vertical,
            )?;
            println!("{}", serde_json::to_string_pretty(&result).unwrap());
            Ok(())
        }
        CoverageCommand::Validate { ir, agent_dir } => {
            // Front-load NotFound before paying the sidecar spawn cost. The
            // sidecar would surface the same condition as a FileNotFoundException
            // (mapped to a Network error here) but the local check gives the
            // user a more direct error path with the correct exit code (7).
            if !ir.is_file() {
                return Err(AwareError::NotFound(format!("IR file: {}", ir.display())));
            }
            if let Some(ref ad) = agent_dir
                && !ad.is_dir()
            {
                return Err(AwareError::NotFound(format!("agent dir: {}", ad.display())));
            }

            let result = coverage_validate(CoverageValidateArgs {
                ir_path: ir.to_string_lossy().into_owned(),
                schema_root: None,
                schema_type: None,
                agent_dir: agent_dir.as_ref().map(|p| p.to_string_lossy().into_owned()),
            })?;

            // Always emit a JSON line so callers (codex-coverage reviewers,
            // CI scripts, IDEs) get a deterministic, parseable result. When
            // --json is set we emit ONLY the JSON; otherwise we follow with a
            // human-readable summary on stderr-free stdout.
            let json = serde_json::to_string(&result)
                .map_err(|e| AwareError::Internal(format!("serialize result: {e}")))?;
            println!("{json}");

            if !ctx.json {
                if result.ok {
                    println!(
                        "status: ok — IR valid, {n} catalog file(s) validated.",
                        n = result.catalog_files_validated
                    );
                } else {
                    if !result.ir_violations.is_empty() {
                        println!("IR violations ({}):", result.ir_violations.len());
                        for v in &result.ir_violations {
                            println!("  {v}");
                        }
                    }
                    if !result.catalog_violations.is_empty() {
                        println!(
                            "Catalog violations ({} file(s) of {} validated):",
                            result.catalog_violations.len(),
                            result.catalog_files_validated
                        );
                        for (path, lines) in &result.catalog_violations {
                            println!("  {path}:");
                            for line in lines {
                                println!("    {line}");
                            }
                        }
                    }
                }
            }

            if !result.ok {
                // Exit code 3 (Validation) per cli-spec.md § Exit codes.
                return Err(AwareError::Validation(format!(
                    "{} IR violation(s), {} catalog file(s) with violations",
                    result.ir_violations.len(),
                    result.catalog_violations.len()
                )));
            }
            Ok(())
        }
        CoverageCommand::Review { agent_id } => {
            // Stub: codex-rescue invocation is documented in
            // docs/superpowers/specs/host-coverage-review-protocol.md.
            // Real CLI-driven review wiring lands in Phase C (subagent
            // dispatch), not in v0.30 alpha.
            let proto = "docs/superpowers/specs/host-coverage-review-protocol.md";
            println!("Review {agent_id}: see {proto}");
            Ok(())
        }
    }
}
