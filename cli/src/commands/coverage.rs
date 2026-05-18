//! `aware coverage ...` — manage host-coverage IR + reviews (v0.30).
//!
//! Verbs:
//!   aware coverage generate <host> <version> --from-ir <ir.json>
//!   aware coverage validate <ir.json>
//!   aware coverage review <agent-id>
//!
//! `generate` regenerates a raw host agent from a coverage IR file (delegates
//! to `builder::coverage::build_from_coverage`, the same code path used by
//! `aware build agent --from-coverage`).
//!
//! `validate` is a stub for v0.30 alpha — it parses the JSON for syntactic
//! validity and reports file size. Full schema validation against the
//! host-coverage IR schema is a follow-up via a sidecar `coverage-validate`
//! verb.
//!
//! `review` is a stub for v0.30 alpha — it prints the path to the review
//! protocol document. Real invocation of the codex-rescue reviewer lands in
//! Phase C subagent dispatch, not as a CLI command.

use clap::Subcommand;
use std::path::PathBuf;

use crate::context::Context;
use crate::error::AwareError;

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
    Validate { ir: PathBuf },
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
        CoverageCommand::Validate { ir } => {
            // Stub: schema validation will land via a sidecar coverage-validate
            // verb in a follow-up task. For now, confirm the file exists and is
            // syntactically valid JSON.
            if !ir.is_file() {
                return Err(AwareError::NotFound(format!("IR file: {}", ir.display())));
            }
            let bytes = std::fs::read(&ir)?;
            let _: serde_json::Value = serde_json::from_slice(&bytes)
                .map_err(|e| AwareError::Validation(format!("IR not JSON: {e}")))?;
            println!(
                "{{\"status\":\"ok\",\"file\":\"{}\",\"size_bytes\":{}}}",
                ir.display(),
                bytes.len()
            );
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
