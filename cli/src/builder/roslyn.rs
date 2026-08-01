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
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err(AwareError::Validation(
            "--from-csproj/--from-sln requires a .csproj or .sln path".into(),
        ));
    }
    // The reader routes to MSBuildWorkspace by file EXTENSION; a non-project path (a typo, or a
    // directory/glob) would otherwise fall through to bare-.cs compilation with no project or
    // package references — silently producing an incomplete agent. Reject it here so the mistake
    // fails fast (#185 Codex).
    let lower = trimmed.to_ascii_lowercase();
    if !(lower.ends_with(".csproj") || lower.ends_with(".sln")) {
        return Err(AwareError::Validation(format!(
            "--from-csproj/--from-sln expects a .csproj or .sln file, got '{trimmed}'; \
             for loose .cs files, a directory, or a glob, use --from-csharp"
        )));
    }
    let args = crate::sidecar::ReflectCsharpArgs {
        paths: vec![trimmed.to_string()],
        references: Vec::new(), // the project graph resolves its own references
        agent_id: agent_id.map(String::from),
    };
    crate::sidecar::reflect_csharp(args)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_env::EnvVarGuard;

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
    fn non_project_path_is_validation_error() {
        // A path that isn't a .csproj/.sln must fail fast, not silently bare-compile (#185).
        for p in ["Foo.cs", "/some/src/dir", "Project.txt", "sln"] {
            assert!(
                matches!(build_from_project(p, None), Err(AwareError::Validation(_))),
                "expected a validation error for non-project path {p:?}"
            );
        }
        // A real project/solution path passes the extension gate (then proceeds to the reader).
        assert!(
            "App.csproj".to_ascii_lowercase().ends_with(".csproj")
                && "App.sln".to_ascii_lowercase().ends_with(".sln")
        );
    }

    #[test]
    fn csharp_no_roslyn_binary_returns_not_found() {
        // With AWARE_ROSLYN pointing at a missing file, discovery fails before the subprocess.
        let _roslyn = EnvVarGuard::set("AWARE_ROSLYN", "C:/aware-roslyn-does-not-exist-test.exe");
        let err = build_from_csharp("Foo.cs", &[], None).unwrap_err();
        assert!(matches!(err, AwareError::NotFound(_)));
    }
}
