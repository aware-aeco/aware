//! Rust client for the C# NativeAOT sidecar (`aware-sidecar.exe`).
//!
//! Discovery order: `AWARE_SIDECAR` env var → sibling of `aware.exe` → `aware-sidecar`
//! on PATH. Spawns the sidecar as a subprocess; one JSON request goes in on stdin,
//! one JSON response comes back on stdout. Process exits between requests.

use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

use serde::{Deserialize, Serialize};

use crate::builder::{
    GeneratedAgent, GeneratedCommand, GeneratedSkill, Provenance, now_iso, quote_yaml_scalar,
};
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

    // 2. Beside the running aware binary. Check a same-named SUBDIRECTORY first, then a
    //    direct sibling. The subdir form supports a helper that must ship as a self-contained
    //    FOLDER rather than a single file: `aware-roslyn`'s MSBuildWorkspace (`.csproj`/`.sln`)
    //    path can't run from a single-file bundle (its `Assembly.Location` is empty there, so
    //    MSBuild path resolution fails), so release staging places it at `<stem>/<stem>.exe`
    //    next to `aware` (#185). Subdir-FIRST matters on upgrade: extracting the new archive
    //    over an old install can leave a stale top-level single-file `aware-roslyn.exe` beside
    //    the new `aware-roslyn/` folder — prefer the folder so the project path keeps working
    //    (#185 Codex). A single-file helper (aware-sidecar) has no subdir, so it falls through
    //    to the direct sibling.
    if let Ok(current) = std::env::current_exe()
        && let Some(dir) = current.parent()
    {
        let in_subdir = dir.join(stem).join(&bin_name);
        if in_subdir.is_file() {
            return Ok(in_subdir);
        }
        let direct = dir.join(&bin_name);
        if direct.is_file() {
            return Ok(direct);
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
    /// Protocol version the sidecar stamps on every reply. Never read — the CLI
    /// pins the sidecar it downloads, so there is nothing to negotiate. It is
    /// required rather than `Option` on purpose: that makes a reply which omits
    /// it fail to parse, so we never accept output from a process that isn't
    /// speaking this envelope. Deleting the field would drop that check (there
    /// is no `deny_unknown_fields` here — serde ignores keys it has no field
    /// for, so an unmapped `version` would simply pass unnoticed).
    #[allow(dead_code)]
    version: String,
    /// Echo of the request's `op`. Never read — dispatch already knows which op
    /// it sent — and being `Option`, it asserts nothing either: a reply parses
    /// whether or not it carries one. Kept purely as a record of the wire shape.
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
    /// Explicit read/write mode (recipe commands set `write`); `None` → infer from name.
    #[serde(default)]
    mode: Option<String>,
    /// Declared inputs (recipe commands carry their parameter contract); empty for plain commands.
    #[serde(default)]
    inputs: Vec<SidecarInput>,
}

#[derive(Deserialize, Debug)]
struct SidecarInput {
    name: String,
    #[serde(rename = "type")]
    ty: String,
    #[serde(default)]
    optional: bool,
    #[serde(default)]
    default: Option<String>,
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

/// Render structured command inputs into the manifest's `inputs:` YAML body (the same
/// `name:\n  type: …\n  required: …` shape the OpenAPI builder emits). Names, types and
/// defaults are YAML-quoted so untrusted plug-in field names can't break the manifest.
fn render_inputs_yaml(inputs: &[SidecarInput]) -> String {
    let mut out = String::new();
    for i in inputs {
        out.push_str(&format!(
            "{}:\n  type: {}\n",
            quote_yaml_scalar(&i.name),
            quote_yaml_scalar(&i.ty),
        ));
        if !i.optional {
            out.push_str("  required: true\n");
        }
        if let Some(d) = &i.default {
            out.push_str(&format!("  default: {}\n", quote_yaml_scalar(d)));
        }
    }
    out
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
                inputs_yaml: render_inputs_yaml(&c.inputs),
                outputs_yaml: String::new(),
                mode: c.mode,
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
    use crate::test_env::EnvVarGuard;

    #[test]
    fn discover_respects_env_var_when_file_exists() {
        let tmp = tempfile::tempdir().unwrap();
        let fake = tmp.path().join("aware-sidecar.exe");
        std::fs::write(&fake, b"").unwrap();
        let _sidecar = EnvVarGuard::set("AWARE_SIDECAR", &fake);
        let result = discover();
        assert_eq!(result.unwrap(), fake);
    }

    #[test]
    fn discover_env_var_with_missing_file_is_not_found() {
        let _sidecar = EnvVarGuard::set("AWARE_SIDECAR", "C:/this-does-not-exist-12345");
        let result = discover();
        assert!(matches!(result, Err(AwareError::NotFound(_))));
    }

    /// `OkResponse::version` is never read, so what earns it its place is the
    /// parse itself: required-and-not-`Option` means a reply without a version
    /// stamp is rejected rather than silently accepted. Asserted here so the
    /// field's justification is enforced instead of merely claimed — delete the
    /// field and this test fails.
    #[test]
    fn reply_without_version_is_rejected() {
        let missing_version = r#"{"ok":true,"op":"reflect"}"#;
        assert!(serde_json::from_str::<OkResponse>(missing_version).is_err());

        let with_version = r#"{"ok":true,"version":"1.0.0","op":"reflect"}"#;
        assert!(serde_json::from_str::<OkResponse>(with_version).is_ok());
    }

    /// The flip side, and the reason the comment on `version` does not claim
    /// unknown keys are rejected: there is no `deny_unknown_fields` here, so a
    /// key the struct has no field for is ignored, not an error.
    #[test]
    fn unmapped_keys_are_ignored() {
        let extra = r#"{"ok":true,"version":"1.0.0","unmapped-key":42}"#;
        assert!(serde_json::from_str::<OkResponse>(extra).is_ok());
    }

    /// Parse a sidecar `agent` payload off the wire rather than building the
    /// struct field by field, so the `#[serde(default)]` fallbacks on
    /// `SidecarCommand::mode` / `::inputs` are exercised by the same fixtures
    /// that exercise the mapping.
    fn sidecar_agent(json: &str) -> SidecarAgent {
        serde_json::from_str(json).expect("fixture must parse as a sidecar agent")
    }

    #[test]
    fn required_is_emitted_only_for_non_optional_inputs() {
        let inputs = vec![
            SidecarInput {
                name: "profile".into(),
                ty: "string".into(),
                optional: false,
                default: None,
            },
            SidecarInput {
                name: "dry-run".into(),
                ty: "boolean".into(),
                optional: true,
                default: None,
            },
        ];
        assert_eq!(
            render_inputs_yaml(&inputs),
            "profile:\n  type: string\n  required: true\ndry-run:\n  type: boolean\n"
        );
    }

    #[test]
    fn defaults_are_emitted_only_when_the_input_declares_one() {
        let with_default = vec![SidecarInput {
            name: "units".into(),
            ty: "string".into(),
            optional: true,
            default: Some("mm".into()),
        }];
        assert_eq!(
            render_inputs_yaml(&with_default),
            "units:\n  type: string\n  default: mm\n"
        );

        let without_default = vec![SidecarInput {
            name: "units".into(),
            ty: "string".into(),
            optional: true,
            default: None,
        }];
        assert_eq!(
            render_inputs_yaml(&without_default),
            "units:\n  type: string\n"
        );
    }

    /// Names, types and defaults come from a plug-in's own metadata, so all
    /// three go through `quote_yaml_scalar`. Each field here carries a
    /// different trigger — a `:` in the name, a leading `*` in the type, a `#`
    /// in the default — so dropping the quoting on any one of them shows up as
    /// a distinct failure rather than being masked by the other two.
    #[test]
    fn hostile_input_metadata_is_yaml_quoted_in_every_field() {
        let inputs = vec![SidecarInput {
            name: "Tekla::Beam".into(),
            ty: "*ref".into(),
            optional: false,
            default: Some("grade # A36".into()),
        }];
        let yaml = render_inputs_yaml(&inputs);
        assert_eq!(
            yaml,
            "\"Tekla::Beam\":\n  type: \"*ref\"\n  required: true\n  default: \"grade # A36\"\n"
        );

        // The point of the quoting is that the result still parses, and parses
        // back to the values the plug-in declared.
        let parsed: serde_yaml::Value = serde_yaml::from_str(&yaml).expect("must stay parseable");
        let entry = &parsed["Tekla::Beam"];
        assert_eq!(entry["type"].as_str(), Some("*ref"));
        assert_eq!(entry["required"].as_bool(), Some(true));
        assert_eq!(entry["default"].as_str(), Some("grade # A36"));
    }

    /// The sidecar's `version` is the *SDK* it reflected, not the agent's own
    /// version — the generated agent is always a fresh `0.1.0`. Getting these
    /// two the wrong way round would stamp e.g. Tekla's `2024.0` as the agent
    /// version and lose the SDK target entirely.
    #[test]
    fn sidecar_version_becomes_sdk_target_and_the_agent_starts_at_0_1_0() {
        let agent = to_local_agent(
            sidecar_agent(
                r#"{"id":"tekla","version":"2024.0","description":"d","license":"MIT",
                    "stateful":true,"commands":[],"skills":[]}"#,
            ),
            "dlls",
        );
        assert_eq!(agent.version, "0.1.0");
        assert_eq!(agent.sdk_target.as_deref(), Some("2024.0"));
        assert_eq!(agent.id, "tekla");
        assert_eq!(agent.license, "MIT");
        assert!(agent.stateful);
        // Nothing on this path describes a REST transport.
        assert!(agent.rest.is_none());
    }

    #[test]
    fn commands_are_keyed_by_name_and_carry_mode_and_rendered_inputs() {
        let agent = to_local_agent(
            sidecar_agent(
                r#"{"id":"tekla","version":"2024.0","description":"d","license":"MIT",
                    "stateful":false,"skills":[],"commands":[
                      {"name":"insert-beam","lifecycle":"session","description":"Insert a beam",
                       "mode":"write","inputs":[{"name":"profile","type":"string"}]},
                      {"name":"list-beams","lifecycle":"oneshot","description":"List beams"}
                    ]}"#,
            ),
            "dlls",
        );

        let insert = &agent.commands["insert-beam"];
        assert_eq!(insert.lifecycle, "session");
        assert_eq!(insert.description, "Insert a beam");
        assert_eq!(insert.mode.as_deref(), Some("write"));
        assert_eq!(
            insert.inputs_yaml,
            "profile:\n  type: string\n  required: true\n"
        );

        // A command that declares neither `mode` nor `inputs` must leave both
        // empty so the manifest loader falls back to name-based inference.
        let list = &agent.commands["list-beams"];
        assert_eq!(list.mode, None);
        assert_eq!(list.inputs_yaml, "");
    }

    #[test]
    fn skills_are_carried_through_verbatim() {
        let agent = to_local_agent(
            sidecar_agent(
                r##"{"id":"tekla","version":"1","description":"d","license":"MIT",
                    "stateful":false,"commands":[],"skills":[
                      {"filename":"insert-beam.md","body":"# Insert a beam\n"}
                    ]}"##,
            ),
            "dlls",
        );
        assert_eq!(agent.skills.len(), 1);
        assert_eq!(agent.skills[0].filename, "insert-beam.md");
        assert_eq!(agent.skills[0].body, "# Insert a beam\n");
    }

    /// Provenance is what lets a reader tell a decompiled agent from a
    /// COM-derived one, so the `source_kind` each public verb passes has to
    /// reach the manifest rather than being flattened to a constant.
    #[test]
    fn provenance_records_the_source_kind_the_caller_passed() {
        for kind in ["dlls", "decompile", "com", "headers", "csharp"] {
            let agent = to_local_agent(
                sidecar_agent(
                    r#"{"id":"a","version":"1","description":"d","license":"MIT",
                        "stateful":false,"commands":[],"skills":[]}"#,
                ),
                kind,
            );
            assert_eq!(agent.provenance.source["type"], kind);
            assert_eq!(agent.provenance.source["via"], "aware-sidecar");
            assert_eq!(agent.provenance.generated_by, "aware-agent-builder");
            assert_eq!(
                agent.provenance.generator_version,
                env!("CARGO_PKG_VERSION")
            );
        }
    }

    /// Stand-in for the C# sidecar: reads and records the request on stdin,
    /// then replays fixed `stdout` / `stderr` bodies. Lets the wire handling in
    /// [`invoke_raw_with`] be driven without the real NativeAOT binary, which
    /// is not built in CI.
    ///
    /// Unix-only because it is a `#!/bin/sh` script; the code under test is
    /// platform-independent, so covering it on one platform covers it.
    #[cfg(unix)]
    fn fake_sidecar(dir: &std::path::Path, stdout_body: &str, stderr_body: &str) -> PathBuf {
        use std::os::unix::fs::PermissionsExt;

        let out = dir.join("stdout.txt");
        let err = dir.join("stderr.txt");
        let seen = dir.join("request.json");
        std::fs::write(&out, stdout_body).unwrap();
        std::fs::write(&err, stderr_body).unwrap();

        let script = dir.join("aware-sidecar");
        std::fs::write(
            &script,
            format!(
                "#!/bin/sh\ncat > {seen}\ncat {out}\ncat {err} >&2\n",
                seen = seen.display(),
                out = out.display(),
                err = err.display(),
            ),
        )
        .unwrap();
        std::fs::set_permissions(&script, std::fs::Permissions::from_mode(0o755)).unwrap();
        script
    }

    #[cfg(unix)]
    fn reflect_against(stdout_body: &str, stderr_body: &str) -> Result<GeneratedAgent, AwareError> {
        let tmp = tempfile::tempdir().unwrap();
        let script = fake_sidecar(tmp.path(), stdout_body, stderr_body);
        let _sidecar = EnvVarGuard::set("AWARE_SIDECAR", &script);
        reflect_dlls(ReflectDllsArgs {
            globs: vec!["*.dll".into()],
            agent_id: Some("tekla".into()),
        })
    }

    /// The request contract: exactly one JSON object, naming the op and
    /// carrying the caller's args, terminated by a newline so a sidecar reading
    /// a line at a time sees a complete request.
    #[cfg(unix)]
    #[test]
    fn request_is_one_newline_terminated_json_object_naming_the_op() {
        let tmp = tempfile::tempdir().unwrap();
        let script = fake_sidecar(
            tmp.path(),
            r#"{"ok":true,"version":"1.0.0","data":{"agent":{"id":"tekla","version":"1",
               "description":"d","license":"MIT","stateful":false,"commands":[],"skills":[]}}}"#,
            "",
        );
        let _sidecar = EnvVarGuard::set("AWARE_SIDECAR", &script);
        reflect_dlls(ReflectDllsArgs {
            globs: vec!["a.dll".into(), "b.dll".into()],
            agent_id: Some("tekla".into()),
        })
        .expect("fixture reply is well-formed");

        let request = std::fs::read_to_string(tmp.path().join("request.json")).unwrap();
        assert!(
            request.ends_with('\n'),
            "request must be newline-terminated, got {request:?}"
        );
        assert_eq!(
            request.trim_end().lines().count(),
            1,
            "one request per spawn"
        );

        let parsed: serde_json::Value = serde_json::from_str(&request).unwrap();
        assert_eq!(parsed["op"], "reflect-dlls");
        assert_eq!(
            parsed["args"]["globs"],
            serde_json::json!(["a.dll", "b.dll"])
        );
        assert_eq!(parsed["args"]["agent_id"], "tekla");
    }

    #[cfg(unix)]
    #[test]
    fn reply_is_mapped_into_a_generated_agent() {
        let agent = reflect_against(
            r#"{"ok":true,"version":"1.0.0","op":"reflect-dlls","data":{"agent":{
                 "id":"tekla","version":"2024.0","description":"Tekla Structures",
                 "license":"proprietary","stateful":true,
                 "commands":[{"name":"insert-beam","lifecycle":"session","description":"b"}],
                 "skills":[{"filename":"s.md","body":"x"}]}}}"#,
            "",
        )
        .expect("well-formed reply must map cleanly");

        assert_eq!(agent.id, "tekla");
        assert_eq!(agent.sdk_target.as_deref(), Some("2024.0"));
        assert_eq!(agent.provenance.source["type"], "dlls");
        assert!(agent.commands.contains_key("insert-beam"));
        assert_eq!(agent.skills.len(), 1);
    }

    /// `ok: false` is the sidecar reporting a failure it diagnosed itself. The
    /// message it wrote is the only diagnostic the user gets, so it has to
    /// survive into the error rather than being replaced by a generic one.
    #[cfg(unix)]
    #[test]
    fn sidecar_reported_failure_surfaces_its_message_and_the_op() {
        let err = reflect_against(
            r#"{"ok":false,"version":"1.0.0","error":"assembly load failed: Tekla.Structures.dll"}"#,
            "",
        )
        .expect_err("ok:false must not be treated as success");

        let AwareError::Network(message) = err else {
            panic!("expected a Network error, got {err:?}");
        };
        assert!(
            message.contains("assembly load failed: Tekla.Structures.dll"),
            "sidecar's own diagnostic must survive: {message}"
        );
        assert!(
            message.contains("reflect-dlls"),
            "op must be named: {message}"
        );
    }

    /// A failure reply with no `error` still has to fail — the missing message
    /// is a gap in the diagnostic, not a reason to treat the call as having
    /// worked.
    #[cfg(unix)]
    #[test]
    fn failure_without_a_message_still_fails() {
        let err = reflect_against(r#"{"ok":false,"version":"1.0.0"}"#, "")
            .expect_err("ok:false must fail whether or not it carries a message");
        assert!(matches!(err, AwareError::Network(_)), "got {err:?}");
    }

    /// `ok: true` with an empty envelope is a sidecar bug, and the three layers
    /// each catch a different depth of it. Missing `data` is caught in
    /// `invoke_raw`; `data` present but with no `agent` is caught one layer up
    /// in `invoke`. Neither may fall through as an empty-but-valid agent.
    #[cfg(unix)]
    #[test]
    fn success_without_a_data_payload_is_rejected() {
        let err = reflect_against(r#"{"ok":true,"version":"1.0.0"}"#, "")
            .expect_err("ok:true with no data must not yield an agent");

        let AwareError::Validation(message) = err else {
            panic!("expected a Validation error, got {err:?}");
        };
        assert!(message.contains("no data"), "{message}");
        assert!(
            message.contains("reflect-dlls"),
            "op must be named: {message}"
        );
    }

    #[cfg(unix)]
    #[test]
    fn success_without_an_agent_is_rejected() {
        let err = reflect_against(r#"{"ok":true,"version":"1.0.0","data":{}}"#, "")
            .expect_err("ok:true with no agent must not yield an agent");

        let AwareError::Validation(message) = err else {
            panic!("expected a Validation error, got {err:?}");
        };
        assert!(message.contains("no agent"), "{message}");
        assert!(
            message.contains("reflect-dlls"),
            "op must be named: {message}"
        );
    }

    /// When the sidecar dies before writing a reply — a missing .NET runtime,
    /// a native crash — stdout is empty or noise and stderr holds the only
    /// clue. Dropping stderr from the error leaves the user with an
    /// unactionable "not JSON".
    #[cfg(unix)]
    #[test]
    fn unparseable_stdout_reports_both_streams() {
        let err = reflect_against(
            "Unhandled exception. System.IO.FileNotFoundException",
            "libhostfxr.so not found",
        )
        .expect_err("non-JSON stdout must not be treated as a reply");

        let AwareError::Validation(message) = err else {
            panic!("expected a Validation error, got {err:?}");
        };
        assert!(
            message.contains("Unhandled exception"),
            "stdout must be quoted back: {message}"
        );
        assert!(
            message.contains("libhostfxr.so not found"),
            "stderr must be quoted back: {message}"
        );
    }

    /// A reply that parses but omits the version stamp is rejected at the same
    /// place a garbled one is — driven end to end here so the guarantee is
    /// checked on the real path, not only against `OkResponse` in isolation.
    #[cfg(unix)]
    #[test]
    fn reply_without_a_version_stamp_is_rejected_on_the_live_path() {
        let err = reflect_against(
            r#"{"ok":true,"data":{"agent":{"id":"tekla","version":"1","description":"d",
               "license":"MIT","stateful":false,"commands":[],"skills":[]}}}"#,
            "",
        )
        .expect_err("an unstamped reply is not this protocol");
        assert!(matches!(err, AwareError::Validation(_)), "got {err:?}");
    }

    /// `coverage-validate` is the one verb whose payload is deserialized into a
    /// typed result rather than re-emitted as JSON, and `ok = false` on the
    /// *result* means schema violations — not a failed call. It must come back
    /// as a value the caller can render, not an error.
    #[cfg(unix)]
    #[test]
    fn coverage_validate_returns_violations_as_a_value_not_an_error() {
        let tmp = tempfile::tempdir().unwrap();
        let script = fake_sidecar(
            tmp.path(),
            r#"{"ok":true,"version":"1.0.0","data":{"coverage_validate":{
                 "ok":false,"ir_path":"ir.json","ir_violations":["/commands: required"],
                 "catalog_violations":{"catalog/a.json":["/x: type"]},
                 "catalog_files_validated":3}}}"#,
            "",
        );
        let _sidecar = EnvVarGuard::set("AWARE_SIDECAR", &script);

        let result = coverage_validate(CoverageValidateArgs {
            ir_path: "ir.json".into(),
            ..Default::default()
        })
        .expect("schema violations are a result, not a transport failure");

        assert!(!result.ok);
        assert_eq!(result.ir_violations, ["/commands: required"]);
        assert_eq!(result.catalog_files_validated, 3);
        assert_eq!(
            result.catalog_violations["catalog/a.json"],
            ["/x: type".to_string()]
        );
    }

    /// Each coverage verb reads its own field of `data`. A reply carrying the
    /// other verb's payload must not be accepted as this one's.
    #[cfg(unix)]
    #[test]
    fn coverage_verbs_do_not_accept_each_others_payloads() {
        let tmp = tempfile::tempdir().unwrap();
        let script = fake_sidecar(
            tmp.path(),
            r#"{"ok":true,"version":"1.0.0","data":{"coverage":{"skills":7}}}"#,
            "",
        );
        let _sidecar = EnvVarGuard::set("AWARE_SIDECAR", &script);

        // `coverage-generate`'s field is present, so that verb succeeds…
        let generated = coverage_generate(CoverageGenerateArgs {
            ir_path: "ir.json".into(),
            out_dir: "out".into(),
            agent_id: "a".into(),
            vendor: "v".into(),
            vertical: "aec".into(),
        })
        .expect("the coverage field is present");
        assert_eq!(generated["skills"], serde_json::json!(7));

        // …while `coverage-validate` reads a different field and must not
        // silently accept it.
        let err = coverage_validate(CoverageValidateArgs {
            ir_path: "ir.json".into(),
            ..Default::default()
        })
        .expect_err("coverage_validate must not read the coverage field");
        let AwareError::Validation(message) = err else {
            panic!("expected a Validation error, got {err:?}");
        };
        assert!(message.contains("coverage_validate"), "{message}");
    }

    /// The Roslyn reader is discovered under its own env var and reports under
    /// its own name — sharing `invoke_raw_with` with the sidecar must not make
    /// it fall back to `AWARE_SIDECAR`.
    #[cfg(unix)]
    #[test]
    fn roslyn_reflection_uses_its_own_binary_and_source_kind() {
        let tmp = tempfile::tempdir().unwrap();
        let script = fake_sidecar(
            tmp.path(),
            r#"{"ok":true,"version":"1.0.0","data":{"agent":{"id":"acme","version":"9.0",
               "description":"d","license":"MIT","stateful":false,"commands":[],"skills":[]}}}"#,
            "",
        );
        let _roslyn = EnvVarGuard::set("AWARE_ROSLYN", &script);

        let agent = reflect_csharp(ReflectCsharpArgs {
            paths: vec!["src".into()],
            references: vec![],
            agent_id: Some("acme".into()),
        })
        .expect("AWARE_ROSLYN must be the binary reflect_csharp spawns");

        assert_eq!(agent.id, "acme");
        assert_eq!(agent.provenance.source["type"], "csharp");
    }

    #[test]
    fn roslyn_discovery_does_not_fall_back_to_the_sidecar_variable() {
        let tmp = tempfile::tempdir().unwrap();
        let fake = tmp.path().join("aware-sidecar");
        std::fs::write(&fake, b"").unwrap();
        let _sidecar = EnvVarGuard::set("AWARE_SIDECAR", &fake);

        // `AWARE_ROSLYN` is unset, so discovery falls through to the
        // sibling/PATH search. Whatever it finds there, it must never resolve
        // to whatever `AWARE_SIDECAR` points at — and when it finds nothing it
        // must say so under its own name, not the sidecar's. (Written to
        // tolerate a real `aware-roslyn` on the runner's PATH rather than
        // assuming its absence.)
        match discover_roslyn() {
            Ok(found) => assert_ne!(found, fake, "AWARE_ROSLYN must not read AWARE_SIDECAR"),
            Err(AwareError::NotFound(message)) => {
                assert!(message.contains("aware-roslyn"), "{message}");
                assert!(message.contains("AWARE_ROSLYN"), "{message}");
            }
            other => panic!("expected a NotFound naming aware-roslyn, got {other:?}"),
        }
    }
}
