//! `aware build agent --from-csharp` — generate an agent from C# *source* via the
//! out-of-process `aware-roslyn` reader (the JIT twin of the AOT `aware-sidecar`).
//!
//! Accepts a `.cs` file, a directory, or a glob; `--reference-dir` adds DLL directories
//! (e.g. an SDK's bin) so base types and attributes resolve and the recipe can fire.

#![allow(dead_code)]

use crate::builder::GeneratedAgent;
use crate::error::AwareError;

pub fn build_from_csharp(
    path: &str,
    references: &[String],
    agent_id: Option<&str>,
) -> Result<GeneratedAgent, AwareError> {
    if path.trim().is_empty() {
        return Err(AwareError::Validation(
            "--from-csharp requires a .cs file, directory, or glob".into(),
        ));
    }
    let args = crate::sidecar::ReflectCsharpArgs {
        paths: vec![path.to_string()],
        references: references.to_vec(),
        agent_id: agent_id.map(String::from),
    };
    crate::sidecar::reflect_csharp(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_path_is_validation_error() {
        let err = build_from_csharp("   ", &[], None).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }

    #[test]
    fn csharp_no_roslyn_binary_returns_not_found() {
        // With AWARE_ROSLYN pointing at a missing file, discovery fails before the subprocess.
        // SAFETY: single-threaded test; env var is restored immediately after.
        unsafe {
            std::env::set_var("AWARE_ROSLYN", "C:/aware-roslyn-does-not-exist-test.exe");
        }
        let err = build_from_csharp("Foo.cs", &[], None).unwrap_err();
        unsafe {
            std::env::remove_var("AWARE_ROSLYN");
        }
        assert!(matches!(err, AwareError::NotFound(_)));
    }
}
