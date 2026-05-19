//! `aware build agent --from-coverage <ir.json>` (v0.30) — invoke the C#
//! cli-sidecar `coverage-generate` verb to produce manifest + skills + catalog
//! from an IR file (the host-coverage IR produced upstream by A1–A7).
//!
//! Unlike the other `--from-*` sources, coverage-generate writes its outputs
//! directly via the sidecar into `<output_dir>/<agent_id>/...` and returns a
//! JSON summary (manifest path + skill/catalog counts + written file lists).
//! The CLI prints that summary back to the caller.

#![allow(dead_code)]

use std::path::Path;

use crate::error::AwareError;
use crate::sidecar::{CoverageGenerateArgs, coverage_generate};

/// Validate the IR file exists, then route to the sidecar's `coverage-generate`
/// verb. Returns the raw `coverage` payload from the sidecar response so the
/// CLI can pretty-print it.
pub fn build_from_coverage(
    ir_path: &Path,
    output_dir: &Path,
    agent_id: &str,
    vendor: &str,
    vertical: &str,
) -> Result<serde_json::Value, AwareError> {
    if !ir_path.is_file() {
        return Err(AwareError::NotFound(format!(
            "IR file: {}",
            ir_path.display()
        )));
    }
    coverage_generate(CoverageGenerateArgs {
        ir_path: ir_path.to_string_lossy().into_owned(),
        out_dir: output_dir.to_string_lossy().into_owned(),
        agent_id: agent_id.to_string(),
        vendor: vendor.to_string(),
        vertical: vertical.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_ir_returns_not_found() {
        let missing = Path::new("/aware-test-does-not-exist-12345.ir.json");
        let out = Path::new("/tmp/out");
        let err = build_from_coverage(missing, out, "id", "vendor", "vertical").unwrap_err();
        assert!(matches!(err, AwareError::NotFound(_)));
        assert!(err.to_string().contains("IR file"));
    }
}
