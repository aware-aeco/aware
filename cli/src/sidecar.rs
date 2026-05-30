//! Rust client for the C# NativeAOT sidecar (`aware-sidecar.exe`).
//!
//! Discovery order: `AWARE_SIDECAR` env var → sibling of `aware.exe` → `aware-sidecar`
//! on PATH. Spawns the sidecar as a subprocess; one JSON request goes in on stdin,
//! one JSON response comes back on stdout. Process exits between requests.

#![allow(dead_code)]

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use serde::{Deserialize, Serialize};

use crate::builder::{GeneratedAgent, GeneratedCommand, GeneratedSkill, Provenance, now_iso};
use crate::error::AwareError;

/// Discover the C# reflection sidecar (`aware-sidecar`).
pub fn discover() -> Result<PathBuf, AwareError> {
    discover_named("AWARE_SIDECAR", "aware-sidecar")
}

/// Discover the Roslyn source reader (`aware-roslyn`).
pub fn discover_roslyn() -> Result<PathBuf, AwareError> {
    discover_named("AWARE_ROSLYN", "aware-roslyn")
}

/// Discover a sibling helper binary by env-var override → sibling of `aware` → PATH.
///
/// `env_var` is an explicit absolute-path override; `stem` is the binary's base name
/// (the platform `.exe` suffix is appended on Windows).
fn discover_named(env_var: &str, stem: &str) -> Result<PathBuf, AwareError> {
    let bin_name = if cfg!(windows) {
        format!("{stem}.exe")
    } else {
        stem.to_string()
    };

    // 1. Explicit override via env var
    if let Ok(p) = std::env::var(env_var) {
        let path = PathBuf::from(&p);
        if path.is_file() {
            return Ok(path);
        }
        return Err(AwareError::NotFound(format!(
            "{env_var}={p} but file not found"
        )));
    }

    // 2. Sibling of the running aware binary
    if let Ok(current) = std::env::current_exe()
        && let Some(dir) = current.parent()
    {
        let candidate = dir.join(&bin_name);
        if candidate.is_file() {
            return Ok(candidate);
        }
    }

    // 3. On PATH
    if let Ok(path_var) = std::env::var("PATH") {
        let sep = if cfg!(windows) { ';' } else { ':' };
        for entry in path_var.split(sep) {
            let candidate = PathBuf::from(entry).join(&bin_name);
            if candidate.is_file() {
                return Ok(candidate);
            }
        }
    }

    Err(AwareError::NotFound(format!(
        "{stem} binary not found. Install from \
         https://github.com/aware-aeco/aware/releases or set {env_var}=<path>"
    )))
}

#[derive(Serialize)]
struct Envelope<'a, T: Serialize> {
    op: &'a str,
    args: T,
}

#[derive(Deserialize, Debug)]
struct OkResponse {
    ok: bool,
    #[allow(dead_code)]
    version: String,
    #[allow(dead_code)]
    op: Option<String>,
    data: Option<ResponseData>,
    error: Option<String>,
}

#[derive(Deserialize, Debug)]
struct ResponseData {
    agent: Option<SidecarAgent>,
    coverage: Option<serde_json::Value>,
    /// Result of the `coverage-validate` verb. We deserialize the inner
    /// payload to a strongly-typed `CoverageValidateResult` so the CLI can
    /// render it (instead of just re-emitting the JSON blob like
    /// `coverage-generate` does).
    coverage_validate: Option<CoverageValidateResult>,
}

/// Mirrors the C# `GeneratedAgent` shape. Converted to `crate::builder::GeneratedAgent` for return.
#[derive(Deserialize, Debug)]
struct SidecarAgent {
    id: String,
    version: String,
    description: String,
    license: String,
    stateful: bool,
    commands: Vec<SidecarCommand>,
    skills: Vec<SidecarSkill>,
}

#[derive(Deserialize, Debug)]
struct SidecarCommand {
    name: String,
    lifecycle: String,
    description: String,
}

#[derive(Deserialize, Debug)]
struct SidecarSkill {
    filename: String,
    body: String,
}

#[derive(Serialize)]
pub struct ReflectDllsArgs {
    pub globs: Vec<String>,
    pub agent_id: Option<String>,
}

#[derive(Serialize)]
pub struct DecompileArgs {
    pub package_path: String,
    pub version: String,
    pub agent_id: Option<String>,
    pub accept_license: bool,
}

#[derive(Serialize)]
pub struct FromComArgs {
    pub progid: String,
    pub agent_id: Option<String>,
}

#[derive(Serialize)]
pub struct FromHeadersArgs {
    pub files: Vec<String>,
    pub agent_id: Option<String>,
}

#[derive(Serialize)]
pub struct CoverageGenerateArgs {
    pub ir_path: String,
    pub out_dir: String,
    pub agent_id: String,
    pub vendor: String,
    pub vertical: String,
}

/// Arguments for the sidecar's `coverage-validate` verb. The schema paths are
/// optional — when omitted, the sidecar resolves them from embedded
/// resources, which is the default for in-tree review work. Override paths
/// exist so the test suite can validate against a scratched-up schema
/// fragment.
#[derive(Serialize, Default)]
pub struct CoverageValidateArgs {
    pub ir_path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_root: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_dir: Option<String>,
}

/// Strongly-typed mirror of `CoverageValidateResult` in
/// `Protocol/Response.cs`. Held internally; surface re-exported via
/// [`coverage_validate`].
#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct CoverageValidateResult {
    pub ok: bool,
    pub ir_path: String,
    #[serde(default)]
    pub ir_violations: Vec<String>,
    #[serde(default)]
    pub catalog_violations: std::collections::BTreeMap<String, Vec<String>>,
    #[serde(default)]
    pub catalog_files_validated: u32,
}

/// Spawn the sidecar, send a reflect-dlls request, return the parsed agent.
pub fn reflect_dlls(args: ReflectDllsArgs) -> Result<GeneratedAgent, AwareError> {
    let agent = invoke("reflect-dlls", &args)?;
    Ok(to_local_agent(agent, "dlls"))
}

/// Arguments for the Roslyn source reader's `reflect-csharp` verb.
#[derive(Serialize)]
pub struct ReflectCsharpArgs {
    /// `.cs` files, directories, or globs to reflect.
    pub paths: Vec<String>,
    /// Extra directories of reference DLLs (e.g. an SDK bin) so base types/attributes resolve.
    pub references: Vec<String>,
    pub agent_id: Option<String>,
}

/// Spawn `aware-roslyn`, send a reflect-csharp request, return the parsed agent.
pub fn reflect_csharp(args: ReflectCsharpArgs) -> Result<GeneratedAgent, AwareError> {
    let bin = discover_roslyn()?;
    let data = invoke_raw_with(&bin, "reflect-csharp", &args)?;
    let agent = data.agent.ok_or_else(|| {
        AwareError::Validation("aware-roslyn reflect-csharp returned ok but no agent".into())
    })?;
    Ok(to_local_agent(agent, "csharp"))
}

/// Spawn the sidecar, send a decompile request, return the parsed agent.
pub fn decompile(args: DecompileArgs) -> Result<GeneratedAgent, AwareError> {
    let agent = invoke("decompile", &args)?;
    Ok(to_local_agent(agent, "decompile"))
}

/// Spawn the sidecar, send a from-com request, return the parsed agent.
pub fn from_com(args: FromComArgs) -> Result<GeneratedAgent, AwareError> {
    let agent = invoke("from-com", &args)?;
    Ok(to_local_agent(agent, "com"))
}

/// Spawn the sidecar, send a from-headers request, return the parsed agent.
pub fn from_headers(args: FromHeadersArgs) -> Result<GeneratedAgent, AwareError> {
    let agent = invoke("from-headers", &args)?;
    Ok(to_local_agent(agent, "headers"))
}

/// Spawn the sidecar, send a coverage-generate request, return the raw
/// `coverage` payload (manifest path, skill/catalog counts, written paths).
///
/// Unlike the other verbs which deserialize into a `GeneratedAgent`, the
/// coverage-generate verb writes its outputs directly to `out_dir` and returns
/// a summary describing what was produced. Callers print the JSON value as-is.
pub fn coverage_generate(args: CoverageGenerateArgs) -> Result<serde_json::Value, AwareError> {
    invoke_raw("coverage-generate", &args).and_then(|data| {
        data.coverage.ok_or_else(|| {
            AwareError::Validation(
                "sidecar coverage-generate returned ok but no coverage data".into(),
            )
        })
    })
}

/// Spawn the sidecar, send a coverage-validate request, return the result.
///
/// The verb performs Draft 2020-12 JSON Schema validation against the
/// embedded host-coverage schemas. A returned `ok = false` indicates schema
/// violations — the verb itself succeeded; it's the IR (or one of the catalog
/// files) that failed validation. Callers translate `ok = false` into a
/// non-zero exit code at the CLI surface.
pub fn coverage_validate(args: CoverageValidateArgs) -> Result<CoverageValidateResult, AwareError> {
    invoke_raw("coverage-validate", &args).and_then(|data| {
        data.coverage_validate.ok_or_else(|| {
            AwareError::Validation(
                "sidecar coverage-validate returned ok but no coverage_validate data".into(),
            )
        })
    })
}

fn invoke<T: Serialize>(op: &str, args: &T) -> Result<SidecarAgent, AwareError> {
    invoke_raw(op, args)?
        .agent
        .ok_or_else(|| AwareError::Validation(format!("sidecar {op} returned ok but no agent")))
}

/// Spawn the `aware-sidecar`, send a request, return the raw `data` payload.
///
/// Lets callers pick whichever field of `ResponseData` matches their verb
/// (`agent` for reflection verbs, `coverage` for coverage-generate, etc.).
fn invoke_raw<T: Serialize>(op: &str, args: &T) -> Result<ResponseData, AwareError> {
    let bin = discover()?;
    invoke_raw_with(&bin, op, args)
}

/// Spawn a specific helper binary (sidecar or roslyn), send one JSON request on stdin,
/// and return the parsed `data` payload. Both tools share the request/response envelope.
fn invoke_raw_with<T: Serialize>(
    bin: &PathBuf,
    op: &str,
    args: &T,
) -> Result<ResponseData, AwareError> {
    let request = Envelope { op, args };
    let body = serde_json::to_string(&request)
        .map_err(|e| AwareError::Internal(format!("serialize request: {e}")))?;

    let mut child = Command::new(bin)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| AwareError::Network(format!("spawn {}: {e}", bin.display())))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(body.as_bytes())
            .map_err(|e| AwareError::Network(format!("stdin write: {e}")))?;
        stdin
            .write_all(b"\n")
            .map_err(|e| AwareError::Network(format!("stdin newline: {e}")))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|e| AwareError::Network(format!("wait: {e}")))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    let parsed: OkResponse = serde_json::from_str(stdout.trim()).map_err(|e| {
        AwareError::Validation(format!(
            "sidecar response not JSON: {e}\nstdout: {stdout}\nstderr: {stderr}"
        ))
    })?;

    if !parsed.ok {
        return Err(AwareError::Network(format!(
            "sidecar {op} failed: {}",
            parsed.error.unwrap_or_else(|| "(no message)".into())
        )));
    }

    parsed
        .data
        .ok_or_else(|| AwareError::Validation(format!("sidecar {op} returned ok but no data")))
}

fn to_local_agent(s: SidecarAgent, source_kind: &str) -> GeneratedAgent {
    use std::collections::BTreeMap;
    let mut commands = BTreeMap::new();
    for c in s.commands {
        commands.insert(
            c.name,
            GeneratedCommand {
                lifecycle: c.lifecycle,
                description: c.description,
                inputs_yaml: String::new(),
                outputs_yaml: String::new(),
                ..Default::default()
            },
        );
    }
    let skills = s
        .skills
        .into_iter()
        .map(|sk| GeneratedSkill {
            filename: sk.filename,
            body: sk.body,
        })
        .collect();
    let provenance = Provenance {
        generated_by: "aware-agent-builder".into(),
        generator_version: env!("CARGO_PKG_VERSION").into(),
        source: serde_json::json!({ "type": source_kind, "via": "aware-sidecar" }),
        generated_at: now_iso(),
    };
    GeneratedAgent {
        id: s.id,
        version: "0.1.0".into(),
        sdk_target: Some(s.version),
        description: s.description,
        commands,
        skills,
        provenance,
        stateful: s.stateful,
        license: s.license,
        rest: None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn discover_respects_env_var_when_file_exists() {
        let tmp = tempfile::tempdir().unwrap();
        let fake = tmp.path().join("aware-sidecar.exe");
        std::fs::write(&fake, b"").unwrap();
        // SAFETY: single-threaded test; env var is restored immediately after.
        unsafe { std::env::set_var("AWARE_SIDECAR", &fake) };
        let result = discover();
        unsafe { std::env::remove_var("AWARE_SIDECAR") };
        assert_eq!(result.unwrap(), fake);
    }

    #[test]
    fn discover_env_var_with_missing_file_is_not_found() {
        // SAFETY: single-threaded test; env var is restored immediately after.
        unsafe { std::env::set_var("AWARE_SIDECAR", "C:/this-does-not-exist-12345") };
        let result = discover();
        unsafe { std::env::remove_var("AWARE_SIDECAR") };
        assert!(matches!(result, Err(AwareError::NotFound(_))));
    }
}
