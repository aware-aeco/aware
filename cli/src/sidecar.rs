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

/// Discover the sidecar binary location.
pub fn discover() -> Result<PathBuf, AwareError> {
    // 1. Explicit override via env var
    if let Ok(p) = std::env::var("AWARE_SIDECAR") {
        let path = PathBuf::from(&p);
        if path.is_file() {
            return Ok(path);
        }
        return Err(AwareError::NotFound(format!(
            "AWARE_SIDECAR={p} but file not found"
        )));
    }

    // 2. Sibling of the running aware binary
    if let Ok(current) = std::env::current_exe()
        && let Some(dir) = current.parent()
    {
        let candidate = dir.join(if cfg!(windows) {
            "aware-sidecar.exe"
        } else {
            "aware-sidecar"
        });
        if candidate.is_file() {
            return Ok(candidate);
        }
    }

    // 3. On PATH
    let bin_name = if cfg!(windows) {
        "aware-sidecar.exe"
    } else {
        "aware-sidecar"
    };
    if let Ok(path_var) = std::env::var("PATH") {
        let sep = if cfg!(windows) { ';' } else { ':' };
        for entry in path_var.split(sep) {
            let candidate = PathBuf::from(entry).join(bin_name);
            if candidate.is_file() {
                return Ok(candidate);
            }
        }
    }

    Err(AwareError::NotFound(
        "aware-sidecar binary not found. Install from \
         https://github.com/aware-aeco/aware/releases or set AWARE_SIDECAR=<path>"
            .to_string(),
    ))
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

/// Spawn the sidecar, send a reflect-dlls request, return the parsed agent.
pub fn reflect_dlls(args: ReflectDllsArgs) -> Result<GeneratedAgent, AwareError> {
    let agent = invoke("reflect-dlls", &args)?;
    Ok(to_local_agent(agent, "dlls"))
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

fn invoke<T: Serialize>(op: &str, args: &T) -> Result<SidecarAgent, AwareError> {
    let bin = discover()?;
    let request = Envelope { op, args };
    let body = serde_json::to_string(&request)
        .map_err(|e| AwareError::Internal(format!("serialize request: {e}")))?;

    let mut child = Command::new(&bin)
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
        .and_then(|d| d.agent)
        .ok_or_else(|| AwareError::Validation(format!("sidecar {op} returned ok but no agent")))
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
