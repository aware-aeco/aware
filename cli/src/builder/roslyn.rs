//! `aware build agent --from-csharp` / `--from-csproj` / `--from-sln` — generate an agent
//! from C# *source* via the out-of-process `aware-roslyn` reader (the JIT twin of the AOT
//! `aware-sidecar`).
//!
//! `--from-csharp` accepts a `.cs` file, a directory, or a glob; `--reference-dir` adds DLL
//! directories (e.g. an SDK's bin) so base types and attributes resolve and the recipe can
//! fire. `--from-csproj` / `--from-sln` load a project/solution graph via MSBuildWorkspace —
//! the `.cs` set + every `PackageReference`/`ProjectReference` resolve automatically (no
//! `--reference-dir`), at the cost of requiring the host .NET SDK (#185).

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

/// `--from-csproj` / `--from-sln` — load a project or solution graph via MSBuildWorkspace in
/// the `aware-roslyn` reader. The reader auto-resolves the `.cs` set + package/project
/// references from the project graph, so no `--reference-dir` is passed. The reader detects a
/// `.csproj`/`.sln` path by extension and routes it to the MSBuild path; it requires the host
/// .NET SDK (a clear error is returned if it's absent). (#185)
pub fn build_from_project(
    path: &str,
    agent_id: Option<&str>,
) -> Result<GeneratedAgent, AwareError> {
    if path.trim().is_empty() {
        return Err(AwareError::Validation(
            "--from-csproj/--from-sln requires a .csproj or .sln path".into(),
        ));
    }
    let args = crate::sidecar::ReflectCsharpArgs {
        paths: vec![path.to_string()],
        references: Vec::new(), // the project graph resolves its own references
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
    fn empty_project_path_is_validation_error() {
        let err = build_from_project("   ", None).unwrap_err();
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
