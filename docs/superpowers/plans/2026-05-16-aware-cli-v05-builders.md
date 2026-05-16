# AWARE CLI v0.5 (Builders) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development.

**Goal:** Ship the `aware build agent` meta-primitive that turns existing software's API surface into an AWARE agent, plus `aware skill ...` commands that scaffold + port + evaluate skills. After v0.5, the substrate becomes self-growing — one engineer can wire up an existing software product into an AWARE agent.

**Architecture:** A `cli::builder` module with a `Source` enum (`OpenApi`, `Cli`, `NuGet`, `Dlls`, `Com`, `Headers`, `Python`, `Decompile`) and per-source generator implementations. Each generator reads a source surface (an OpenAPI spec, a CLI binary's `--help`, etc.) and emits a complete AWARE agent folder under a target path. A `cli::skill_builder` module ships skill-level helpers (create scaffolding, port between agents, eval against a trigger corpus). AI-driven skill generation is deferred — v0.5 scaffolds + the user's own AI tooling (Claude Code / Codex) fills in the content.

**Scope honesty.** v0.5 is the largest roadmap phase (4-5 days realistic). To keep the PR shippable, this plan splits the eight `--from-*` sources into three tiers:

| Tier | Sources | What ships |
|---|---|---|
| **A — full implementation** | `--from-openapi`, `--from-cli` | Generate a complete agent folder from real input. Tested end-to-end. |
| **B — partial implementation** | `--from-nuget` (without decompile), `--from-python` (via `python -c` introspection) | Working but with documented limits. NuGet fetches + unzips the package and extracts the embedded XML docs; no decompiled source. Python introspects a module via subprocess. |
| **C — stubbed, requires C# sidecar** | `--from-dlls`, `--from-com`, `--from-headers`, `--decompile` | Returns `NotYetImplemented` with a clear message pointing at the v0.1 Rust-validation finding: *"v0.5 wants a separate C# NativeAOT sidecar for DLL/NuGet reflection; not in v0.5 scope; tracked for v0.5.1."* |

This matches the v0.1 background-agent verdict (*"v0.5 wants a C# NativeAOT sidecar"*) without blocking the rest of v0.5 on building that sidecar.

**Tech Stack additions:**
- `zip` 2.x — extract `.nupkg` (NuGet packages are zips)
- `quick-xml` 0.36 — parse NuGet XML doc files
- (No new HTTP / async / parsing deps — reuse what v0.2–v0.4 brought in)

---

## Open design decisions (resolved here)

| Decision | Choice | Why | Alternative |
|---|---|---|---|
| **AI delegation for skill commands** | Scaffolding only. `aware skill create` writes a `.md` template with frontmatter + TODO markers; the user's AI tooling (Claude Code etc.) fills in the body. AWARE doesn't bundle an Anthropic SDK or HTTP-call the Claude API | Decalog #4 (no vendor in the loop); AWARE is meant to run UNDER agentic CLIs, not embed them; users already have AI tooling | Direct Claude API integration (introduces vendor lock + key management complexity) |
| **C# NativeAOT sidecar for DLL/NuGet reflection** | Out of scope for v0.5. Stubs ship for `--from-dlls`, `--from-com`, `--from-headers`, `--decompile`. Tracked for v0.5.1. | The v0.1 Rust-validation pass flagged this; building the sidecar correctly needs its own design phase | Implement everything in pure Rust by shelling to `powershell` (fragile on non-Windows; awkward) |
| **OpenAPI parser** | Hand-rolled minimal: parse JSON or YAML into `serde_json::Value`; walk `paths.<path>.<method>` entries to emit commands. Supports OpenAPI 3.0 + 3.1 the same way. No code generation of strongly-typed client; we generate command-level shells only | Avoid pulling a heavy `openapi` crate; we only need to enumerate operations + their summaries | `oas3` / `openapiv3` crates work but add ~30 transitive deps |
| **NuGet fetch** | HTTPS GET to `https://api.nuget.org/v3-flatcontainer/<lowercase-pkg>/<version>/<lowercase-pkg>.<version>.nupkg`. Extract via `zip` crate. Look for `lib/*/<pkg>.xml` for XML docs; skip `lib/*/<pkg>.dll` (decompile is in tier C) | Avoid auth; v3 flatcontainer is the canonical CDN path | NuGet API v3 catalog (more general but more work) |
| **`--from-cli` parsing** | Run `<binary> --help`, parse the output with simple heuristics (look for `Commands:` / `Subcommands:` sections, indented lines). Emit one command per detected subcommand | Most real-world help outputs follow conventions clap/cobra/click all use | Force structured `--help-json` (no binary supports this universally) |
| **Python introspection** | Spawn `python -c "import json; import <module>; print(json.dumps([(n, getattr(<module>, n).__doc__) for n in dir(<module>) if not n.startswith('_')]))"` and parse the result | Subprocess + JSON is the simplest cross-platform option | C-extension Python embedding (huge cost) |
| **License-aware extraction** | For `--from-nuget`, parse the `.nuspec` file (always inside the `.nupkg`) for `<license>` tag. If license is `MIT` / `Apache-2.0` / `BSD-*` / `Unlicense`, allow XML-doc extraction. Otherwise, require `--accept-license` flag | Matches the decalog "no vendor in the loop" + the spec's "license-checked" qualifier | Always allow (footgun); always block (over-cautious) |
| **Generated agent's `provenance` block** | Auto-populated: `generated-by: aware-agent-builder`, `generator-version: <CLI version>`, `source: { type: <source>, path/url: <input>, license: <detected> }`, `generated-at: <ISO timestamp>` | Matches the existing reference agent manifests (Task 3 verified this shape) | Skip provenance (loses traceability) |
| **Output path** | Default: `~/.aware/agents/<agent-id>/`. Override with `--output <agent-id>` (just sets the id; path stays under `agents_dir`). Auto-runs `aware agent install <generated-dir>` workflow internally — validates + writes the agent | One-step UX: build = installed | Two-step (write to temp + manual install) — friction |
| **`aware skill port` source format** | Accepts: `agent-id/skill-name.md` (already-installed AWARE skill), a path to a `.md` file outside `~/.aware/`, or an Anthropic `skill-creator` skill name (just a file copy from a known directory). v0.5 ships the first two; the third is documented as "manual: copy the file in yourself" | Pragmatic; covers the common cases | Full skill-creator delegation (requires AI integration) |
| **`aware skill eval`** | Reads a `<skill>.eval.md` file alongside the skill (created on demand) listing trigger phrases + expected behavior. Runs each phrase through a heuristic match against the skill's frontmatter `description`. Reports pass/fail per phrase | Lightweight; no AI required; matches the `skill-creator` eval pattern | Full LLM-as-judge eval (heavyweight) |
| **Skill modify** | Print the absolute path to the skill file + open it via `$EDITOR` if set | Simplest; integrates with whatever editor / AI tooling the user has | Bundled editor (overkill) |
| **Tests** | Each tier-A/B source has at least one integration test using a fixture (OpenAPI fixture JSON, fixture nupkg, fixture python module). Stubs in tier C have a "returns NotYetImplemented with helpful message" test | Fixture-based; works in CI without external network | Live API tests (flaky) |

---

## Scope discipline

**v0.5 user-facing surfaces** (12 new — many sharing the same `aware build agent` command):
- `aware build agent --from-openapi <url-or-path> [--output <id>]` ✓ (tier A)
- `aware build agent --from-cli <binary> [--output <id>]` ✓ (tier A)
- `aware build agent --from-nuget <pkg>[@version] [--output <id>] [--accept-license]` ✓ (tier B)
- `aware build agent --from-python <module> [--output <id>]` ✓ (tier B)
- `aware build agent --from-dlls <path>` — stubbed (tier C)
- `aware build agent --from-com <progid>` — stubbed (tier C)
- `aware build agent --from-headers <path>` — stubbed (tier C)
- `aware build agent --decompile` flag — stubbed (tier C)
- `aware skill create <agent> <skill-name>` ✓
- `aware skill port <source> <target-agent>` ✓
- `aware skill modify <agent> <skill-name>` ✓
- `aware skill eval <agent> <skill-name>` ✓

**v0.5 internal surfaces:**
- `cli::builder::mod` — `Source` enum, dispatch, common helpers (write_manifest, write_skills, etc.)
- `cli::builder::openapi` — parse spec → commands
- `cli::builder::cli_help` — run `--help` → commands
- `cli::builder::nuget` — fetch nupkg → unzip → extract XML docs + nuspec
- `cli::builder::python` — subprocess introspection
- `cli::builder::stubs` — module containing the four "returns NotYetImplemented + helpful message" stubs
- `cli::skill_builder::mod` — dispatch
- `cli::skill_builder::create` — write skill template
- `cli::skill_builder::port` — copy + adapt frontmatter
- `cli::skill_builder::modify` — open in editor
- `cli::skill_builder::eval` — trigger-corpus matcher

**Out of scope** — nothing left to stub in the roadmap; v0.5 is the LAST phase. The C# sidecar is the only formally-flagged v0.5.1 work.

---

## File Structure

### New files

| Path | Responsibility |
|---|---|
| `cli/src/builder/mod.rs` | Source enum + common helpers (write_manifest, write_skill, generate_provenance). |
| `cli/src/builder/openapi.rs` | OpenAPI 3.x parser → AWARE agent. |
| `cli/src/builder/cli_help.rs` | Run `<binary> --help`, parse, emit. |
| `cli/src/builder/nuget.rs` | Fetch nupkg, extract, parse nuspec + XML docs. |
| `cli/src/builder/python.rs` | Spawn `python -c` to introspect a module. |
| `cli/src/builder/stubs.rs` | dlls / com / headers / decompile NotYetImplemented stubs. |
| `cli/src/skill_builder/mod.rs` | Skill command dispatch. |
| `cli/src/skill_builder/create.rs` | Skill template writer. |
| `cli/src/skill_builder/port.rs` | Skill copier + frontmatter adapter. |
| `cli/src/skill_builder/modify.rs` | $EDITOR launcher. |
| `cli/src/skill_builder/eval.rs` | Trigger-corpus matcher. |
| `cli/tests/build_agent.rs` | Integration tests (one per tier-A/B source). |
| `cli/tests/skill_cmds.rs` | Integration tests for skill create/port/modify/eval. |
| `cli/tests/fixtures/openapi-mini.json` | Tiny OpenAPI 3.0 fixture. |
| `cli/tests/fixtures/fake.nupkg` | Generated at test time by zipping a fixture dir. |

### Modified files

| Path | Change |
|---|---|
| `cli/Cargo.toml` | Add `zip = "2"`, `quick-xml = "0.36"`. |
| `cli/src/main.rs` | Add `mod builder; mod skill_builder;`. |
| `cli/src/commands/build.rs` | Implement the `BuildCommand` dispatch arms (currently `NotYetImplemented`). |
| `cli/src/commands/skill.rs` | Implement the `SkillCommand` dispatch arms. |
| `cli/src/commands/doctor.rs` | No change — doctor already reports installed agents (which would now include builder-generated ones). |
| `cli/README.md` | Bump quickstart with `aware build agent` + `aware skill` examples. |
| `cli/tests/basic.rs` | Update the `unimplemented_subcommand_exits_nonzero_with_message` test target since `build` and `skill` are no longer wholly stubbed — point it at a tier-C stub like `build agent --from-dlls`. |

---

## Tasks

### Task 0: Branch + baseline (done — already on `feat/cli-v05-builders`)

### Task 1: Add zip + quick-xml deps

Append to `cli/Cargo.toml`:

```toml
zip       = "2"
quick-xml = "0.36"
```

Build, regression test, commit: `chore(cli): add zip + quick-xml for v0.5 builders`

### Task 2: `cli::builder` framework + common helpers

`cli/src/builder/mod.rs`:

```rust
//! Agent builder framework.

#![allow(dead_code)]

pub mod openapi;
pub mod cli_help;
pub mod nuget;
pub mod python;
pub mod stubs;

use std::collections::BTreeMap;
use std::path::Path;

use serde::Serialize;

use crate::error::AwareError;

/// Shared shape of every generator's output.
pub struct GeneratedAgent {
    pub id: String,
    pub version: String,
    pub description: String,
    pub commands: BTreeMap<String, GeneratedCommand>,
    pub skills: Vec<GeneratedSkill>,
    pub provenance: Provenance,
    pub stateful: bool,
    pub license: String,
}

pub struct GeneratedCommand {
    pub lifecycle: String,       // "single" | "start" | "stop"
    pub description: String,
    pub inputs_yaml: String,     // raw YAML for inputs block (best-effort)
    pub outputs_yaml: String,
}

pub struct GeneratedSkill {
    pub filename: String,
    pub body: String,
}

#[derive(Serialize)]
pub struct Provenance {
    #[serde(rename = "generated-by")]
    pub generated_by: String,
    #[serde(rename = "generator-version")]
    pub generator_version: String,
    pub source: serde_json::Value,
    #[serde(rename = "generated-at")]
    pub generated_at: String,
}

/// Write the generated agent into `<output_dir>/<agent-id>/`.
pub fn write_agent(agent: &GeneratedAgent, output_dir: &Path) -> Result<std::path::PathBuf, AwareError> {
    let dst = output_dir.join(&agent.id);
    if dst.exists() {
        return Err(AwareError::Conflict(format!("agent {} already exists at {}", agent.id, dst.display())));
    }
    std::fs::create_dir_all(dst.join("skills"))?;
    std::fs::create_dir_all(dst.join("commands"))?;

    // manifest.yaml
    let manifest = build_manifest_yaml(agent)?;
    std::fs::write(dst.join("manifest.yaml"), manifest)?;

    // skills/*.md
    for s in &agent.skills {
        std::fs::write(dst.join("skills").join(&s.filename), &s.body)?;
    }

    // commands/*.md — minimal stubs; humans + AI fill these in via aware skill modify later
    for (name, cmd) in &agent.commands {
        let body = format!(
            "# {name}\n\nLifecycle: {}\n\n{}\n",
            cmd.lifecycle, cmd.description
        );
        std::fs::write(dst.join("commands").join(format!("{name}.md")), body)?;
    }

    Ok(dst)
}

fn build_manifest_yaml(agent: &GeneratedAgent) -> Result<String, AwareError> {
    let mut out = String::new();
    out.push_str(&format!("agent:        {}\n", agent.id));
    out.push_str(&format!("version:      {}\n", agent.version));
    out.push_str(&format!("description: |\n  {}\n", agent.description.replace('\n', "\n  ")));
    out.push_str(&format!("stateful:     {}\n", agent.stateful));
    out.push_str(&format!("license:      {}\n", agent.license));

    // Provenance
    out.push_str("provenance:\n");
    let prov_yaml = serde_yaml::to_string(&agent.provenance)
        .map_err(|e| AwareError::Internal(format!("provenance yaml: {e}")))?;
    for line in prov_yaml.lines() {
        out.push_str("  ");
        out.push_str(line);
        out.push('\n');
    }

    // Transport — default cli
    out.push_str("transport:\n");
    out.push_str(&format!("  cli:\n    binary: aware-{}\n", agent.id));

    // Commands
    out.push_str("commands:\n");
    for (name, cmd) in &agent.commands {
        out.push_str(&format!("  {name}:\n"));
        out.push_str(&format!("    lifecycle: {}\n", cmd.lifecycle));
        out.push_str(&format!("    description: {}\n", cmd.description.replace('\n', " ")));
        if !cmd.inputs_yaml.trim().is_empty() {
            out.push_str("    inputs:\n");
            for line in cmd.inputs_yaml.lines() {
                out.push_str("      ");
                out.push_str(line);
                out.push('\n');
            }
        }
        if !cmd.outputs_yaml.trim().is_empty() {
            out.push_str("    outputs:\n");
            for line in cmd.outputs_yaml.lines() {
                out.push_str("      ");
                out.push_str(line);
                out.push('\n');
            }
        }
    }

    // Skills (sorted for deterministic output)
    if !agent.skills.is_empty() {
        let mut names: Vec<String> = agent.skills.iter().map(|s| s.filename.clone()).collect();
        names.sort();
        out.push_str("skills:\n");
        for n in names {
            out.push_str(&format!("  - {n}\n"));
        }
    }

    Ok(out)
}

pub fn now_iso() -> String {
    chrono::Utc::now().to_rfc3339()
}
```

Tests cover `build_manifest_yaml` round-tripping (parse what we generate via `manifest::loader::load_agent`).

Commit: `feat(cli): builder framework + write_agent helper`

### Task 3: `cli::builder::openapi` (tier A)

Parse OpenAPI 3.x JSON or YAML. For each `paths.<path>.<method>` operation, emit a command named `<operation-id>` (kebab-case fallback to `<method>-<path>` if no operationId).

```rust
//! OpenAPI → AWARE agent.

use std::collections::BTreeMap;

use serde_json::Value;

use crate::builder::{GeneratedAgent, GeneratedCommand, Provenance, now_iso};
use crate::error::AwareError;

pub fn build_from_url_or_path(input: &str, agent_id: Option<&str>) -> Result<GeneratedAgent, AwareError> {
    // Fetch JSON or YAML body
    let body = if input.starts_with("http://") || input.starts_with("https://") {
        let resp = ureq::get(input).call()
            .map_err(|e| AwareError::Network(format!("GET {input}: {e}")))?;
        let mut s = String::new();
        use std::io::Read;
        resp.into_reader().read_to_string(&mut s)
            .map_err(|e| AwareError::Network(format!("read: {e}")))?;
        s
    } else {
        std::fs::read_to_string(input)?
    };

    // Try JSON first, fall back to YAML
    let spec: Value = serde_json::from_str(&body)
        .or_else(|_| serde_yaml::from_str::<Value>(&body))
        .map_err(|e| AwareError::Validation(format!("spec not JSON or YAML: {e}")))?;

    let info = &spec["info"];
    let title = info["title"].as_str().unwrap_or("openapi-agent").to_string();
    let version = info["version"].as_str().unwrap_or("0.1.0").to_string();
    let description = info["description"].as_str().unwrap_or(&title).to_string();
    let license = info["license"]["name"].as_str().unwrap_or("UNKNOWN").to_string();

    let id = agent_id.map(String::from).unwrap_or_else(|| kebab(&title));

    let mut commands = BTreeMap::new();
    if let Some(paths) = spec["paths"].as_object() {
        for (path, methods) in paths {
            if let Some(methods_obj) = methods.as_object() {
                for (method, op) in methods_obj {
                    if !["get", "post", "put", "delete", "patch"].contains(&method.as_str()) {
                        continue;
                    }
                    let op_id = op["operationId"].as_str()
                        .map(String::from)
                        .unwrap_or_else(|| format!("{method}-{}", path.replace('/', "-").trim_matches('-').to_string()));
                    let summary = op["summary"].as_str()
                        .or_else(|| op["description"].as_str())
                        .unwrap_or("OpenAPI operation").to_string();
                    commands.insert(kebab(&op_id), GeneratedCommand {
                        lifecycle: "single".into(),
                        description: format!("{} {} \u{2014} {}", method.to_uppercase(), path, summary),
                        inputs_yaml: String::new(),  // v0.5 doesn't model OpenAPI params yet
                        outputs_yaml: String::new(),
                    });
                }
            }
        }
    }

    let provenance = Provenance {
        generated_by: "aware-agent-builder".into(),
        generator_version: env!("CARGO_PKG_VERSION").into(),
        source: serde_json::json!({ "type": "openapi", "input": input, "license": license }),
        generated_at: now_iso(),
    };

    Ok(GeneratedAgent {
        id,
        version,
        description,
        commands,
        skills: Vec::new(),
        provenance,
        stateful: false,
        license,
    })
}

fn kebab(s: &str) -> String {
    let mut out = String::new();
    let mut prev_upper = false;
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 && !prev_upper { out.push('-'); }
            out.push(c.to_ascii_lowercase());
            prev_upper = true;
        } else if c.is_whitespace() || c == '_' {
            if !out.ends_with('-') && !out.is_empty() { out.push('-'); }
            prev_upper = false;
        } else if c.is_alphanumeric() {
            out.push(c);
            prev_upper = false;
        }
    }
    out.trim_matches('-').to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const PETSTORE_MINI: &str = r#"{
        "openapi": "3.0.0",
        "info": {
            "title": "Petstore",
            "version": "1.0.0",
            "description": "Tiny petstore",
            "license": { "name": "MIT" }
        },
        "paths": {
            "/pets": {
                "get": { "operationId": "listPets", "summary": "List pets" },
                "post": { "operationId": "createPet", "summary": "Create a pet" }
            },
            "/pets/{petId}": {
                "get": { "operationId": "getPet", "summary": "Get pet by id" }
            }
        }
    }"#;

    #[test]
    fn parses_petstore_to_3_commands() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("petstore.json");
        std::fs::write(&path, PETSTORE_MINI).unwrap();

        let agent = build_from_url_or_path(path.to_str().unwrap(), None).unwrap();
        assert_eq!(agent.id, "petstore");
        assert_eq!(agent.commands.len(), 3);
        assert!(agent.commands.contains_key("list-pets"));
        assert!(agent.commands.contains_key("create-pet"));
        assert!(agent.commands.contains_key("get-pet"));
        assert_eq!(agent.license, "MIT");
    }
}
```

Commit: `feat(cli): builder openapi (tier A)`

### Task 4: `cli::builder::cli_help` (tier A)

Spawn `<binary> --help`, parse for "Commands:" / "Subcommands:" sections, emit one command per detected entry.

```rust
pub fn build_from_cli(binary: &str, agent_id: Option<&str>) -> Result<GeneratedAgent, AwareError> {
    let output = std::process::Command::new(binary).arg("--help").output()
        .map_err(|e| AwareError::Network(format!("spawn {binary} --help: {e}")))?;
    if !output.status.success() {
        return Err(AwareError::Network(format!("{binary} --help failed (exit {:?})", output.status.code())));
    }
    let help = String::from_utf8_lossy(&output.stdout);

    // Heuristic: find a line matching /Commands:|Subcommands:/i. Everything after is indented
    // command-name + description lines until a blank line or a new top-level section.
    let mut in_commands = false;
    let mut commands = BTreeMap::new();
    for line in help.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() { continue; }
        if trimmed.to_lowercase().starts_with("commands:") || trimmed.to_lowercase().starts_with("subcommands:") {
            in_commands = true;
            continue;
        }
        if in_commands {
            // Indented? It's a command.
            if line.starts_with(' ') || line.starts_with('\t') {
                let mut parts = trimmed.splitn(2, char::is_whitespace);
                let name = parts.next().unwrap_or("").trim().to_string();
                let desc = parts.next().unwrap_or("").trim().to_string();
                if !name.is_empty() && !name.starts_with('-') {
                    commands.insert(name.clone(), GeneratedCommand {
                        lifecycle: "single".into(),
                        description: if desc.is_empty() { format!("{binary} {name}") } else { desc },
                        inputs_yaml: String::new(),
                        outputs_yaml: String::new(),
                    });
                }
            } else if !trimmed.starts_with('-') {
                // Top-level section header — done with commands
                in_commands = false;
            }
        }
    }

    let id = agent_id.map(String::from).unwrap_or_else(|| binary.to_string());
    let provenance = Provenance {
        generated_by: "aware-agent-builder".into(),
        generator_version: env!("CARGO_PKG_VERSION").into(),
        source: serde_json::json!({ "type": "cli", "binary": binary }),
        generated_at: now_iso(),
    };

    Ok(GeneratedAgent {
        id,
        version: "0.1.0".into(),
        description: format!("Agent wrapping {binary} CLI"),
        commands,
        skills: Vec::new(),
        provenance,
        stateful: false,
        license: "UNKNOWN".into(),
    })
}
```

Test: against the `aware` binary itself (spawn `aware --help`, expect commands like `agent`, `app`, `connect`, `doctor`, `plugins`, `skill`, `build`).

Commit: `feat(cli): builder cli_help (tier A)`

### Task 5: `cli::builder::nuget` (tier B)

Download `https://api.nuget.org/v3-flatcontainer/<lowercase-pkg>/<version>/<lowercase-pkg>.<version>.nupkg`. Unzip with the `zip` crate. Parse the `.nuspec` for metadata. Extract `lib/*/*.xml` files as skills.

```rust
pub fn build_from_nuget(spec: &str, agent_id: Option<&str>, accept_license: bool) -> Result<GeneratedAgent, AwareError> {
    let (pkg, version) = match spec.split_once('@') {
        Some((p, v)) => (p, v.to_string()),
        None => return Err(AwareError::Validation("--from-nuget requires <pkg>@<version>".into())),
    };
    let lower = pkg.to_lowercase();
    let url = format!("https://api.nuget.org/v3-flatcontainer/{lower}/{version}/{lower}.{version}.nupkg");

    let resp = ureq::get(&url).call()
        .map_err(|e| AwareError::Network(format!("GET {url}: {e}")))?;
    let mut bytes = Vec::new();
    use std::io::Read;
    resp.into_reader().read_to_end(&mut bytes)
        .map_err(|e| AwareError::Network(format!("read: {e}")))?;

    // Parse zip
    let reader = std::io::Cursor::new(&bytes);
    let mut zip = zip::ZipArchive::new(reader)
        .map_err(|e| AwareError::Validation(format!("nupkg parse: {e}")))?;

    // Find the .nuspec (top-level file)
    let mut nuspec_text = String::new();
    let mut xml_docs: BTreeMap<String, String> = BTreeMap::new();
    for i in 0..zip.len() {
        let mut entry = zip.by_index(i).map_err(|e| AwareError::Validation(format!("zip entry: {e}")))?;
        let name = entry.name().to_string();
        if name.ends_with(".nuspec") && !name.contains('/') {
            entry.read_to_string(&mut nuspec_text)
                .map_err(|e| AwareError::Validation(format!("nuspec read: {e}")))?;
        } else if name.starts_with("lib/") && name.ends_with(".xml") {
            let mut body = String::new();
            entry.read_to_string(&mut body)
                .map_err(|e| AwareError::Validation(format!("xml read: {e}")))?;
            // Use the basename (e.g. lib/net6.0/Foo.xml → Foo.xml)
            let basename = name.rsplit('/').next().unwrap_or(&name).to_string();
            xml_docs.entry(basename).or_insert(body);
        }
    }

    // Parse nuspec for license
    let license = extract_nuspec_license(&nuspec_text);
    let permissive = ["MIT", "Apache-2.0", "BSD-2-Clause", "BSD-3-Clause", "Unlicense", "ISC", "0BSD"];
    if !permissive.contains(&license.as_str()) && !accept_license {
        return Err(AwareError::PermissionDenied(format!(
            "package {pkg}@{version} has license {license:?}; re-run with --accept-license to proceed"
        )));
    }

    let description = extract_nuspec_description(&nuspec_text).unwrap_or_else(|| pkg.to_string());

    // Generate skills from XML doc files (one skill per XML file, body = wrapped XML)
    let mut skills: Vec<crate::builder::GeneratedSkill> = xml_docs.iter().map(|(name, body)| {
        let skill_name = format!("{}.md", name.trim_end_matches(".xml").to_lowercase());
        let skill_body = format!(
            "---\nname: {agent}-{stem}\ndescription: API reference extracted from {name}\n---\n\n# {stem} API reference\n\n```xml\n{body}\n```\n",
            agent = pkg.to_lowercase(),
            stem = name.trim_end_matches(".xml").to_lowercase(),
            name = name,
            body = body,
        );
        crate::builder::GeneratedSkill { filename: skill_name, body: skill_body }
    }).collect();
    skills.sort_by(|a, b| a.filename.cmp(&b.filename));

    let id = agent_id.map(String::from).unwrap_or_else(|| pkg.to_lowercase());
    let provenance = Provenance {
        generated_by: "aware-agent-builder".into(),
        generator_version: env!("CARGO_PKG_VERSION").into(),
        source: serde_json::json!({ "type": "nuget", "package": pkg, "version": version, "license": license }),
        generated_at: now_iso(),
    };

    Ok(GeneratedAgent {
        id,
        version: version.clone(),
        description,
        commands: BTreeMap::new(),  // v0.5 nuget = docs-only; commands require decompile (tier C)
        skills,
        provenance,
        stateful: false,
        license,
    })
}

fn extract_nuspec_license(nuspec: &str) -> String {
    // Look for <license>VALUE</license> or <license type="expression">MIT</license>
    if let Some(idx) = nuspec.find("<license") {
        if let Some(end) = nuspec[idx..].find("</license>") {
            let chunk = &nuspec[idx..idx + end];
            if let Some(content_start) = chunk.find('>') {
                let content = &chunk[content_start + 1..];
                return content.trim().to_string();
            }
        }
    }
    "UNKNOWN".to_string()
}

fn extract_nuspec_description(nuspec: &str) -> Option<String> {
    let start = nuspec.find("<description>")?;
    let end = nuspec[start..].find("</description>")?;
    Some(nuspec[start + 13..start + end].trim().to_string())
}
```

Test: build a fake `.nupkg` in-memory using the `zip` crate's writer, then test parsing.

Commit: `feat(cli): builder nuget (tier B — docs-only, no decompile)`

### Task 6: `cli::builder::python` (tier B)

Spawn a python process; capture introspection JSON.

```rust
pub fn build_from_python(module: &str, agent_id: Option<&str>) -> Result<GeneratedAgent, AwareError> {
    let script = format!(
        "import json, importlib, inspect; \
         m = importlib.import_module('{module}'); \
         out = []; \
         for n in dir(m): \
             if n.startswith('_'): continue; \
             obj = getattr(m, n); \
             if not callable(obj): continue; \
             doc = (inspect.getdoc(obj) or '').split(chr(10))[0]; \
             out.append((n, doc)); \
         print(json.dumps(out))"
    );
    let output = std::process::Command::new("python")
        .arg("-c").arg(&script).output()
        .map_err(|e| AwareError::Network(format!("spawn python: {e}")))?;
    if !output.status.success() {
        return Err(AwareError::Network(format!("python introspect failed: {}", String::from_utf8_lossy(&output.stderr))));
    }
    let entries: Vec<(String, String)> = serde_json::from_slice(&output.stdout)
        .map_err(|e| AwareError::Validation(format!("python introspect output: {e}")))?;

    let mut commands = BTreeMap::new();
    for (name, doc) in entries {
        let kebab_name = name.replace('_', "-");
        commands.insert(kebab_name.clone(), GeneratedCommand {
            lifecycle: "single".into(),
            description: if doc.is_empty() { format!("{module}.{name}()") } else { doc },
            inputs_yaml: String::new(),
            outputs_yaml: String::new(),
        });
    }

    let id = agent_id.map(String::from).unwrap_or_else(|| module.replace('.', "-"));
    let provenance = Provenance {
        generated_by: "aware-agent-builder".into(),
        generator_version: env!("CARGO_PKG_VERSION").into(),
        source: serde_json::json!({ "type": "python", "module": module }),
        generated_at: now_iso(),
    };

    Ok(GeneratedAgent {
        id,
        version: "0.1.0".into(),
        description: format!("Python module wrapper: {module}"),
        commands,
        skills: Vec::new(),
        provenance,
        stateful: false,
        license: "UNKNOWN".into(),
    })
}
```

Test: needs python on PATH. Use `python -c "print('test')"` to detect; skip the test if absent. Test against `json` stdlib module (always present in any python).

Commit: `feat(cli): builder python (tier B)`

### Task 7: `cli::builder::stubs` (tier C)

A single module that returns clear `NotYetImplemented` errors for the four tier-C sources:

```rust
//! Tier C builder stubs — require a C# NativeAOT sidecar for .NET / Win32 reflection.

use crate::builder::GeneratedAgent;
use crate::error::AwareError;

const MSG: &str = "this source requires the v0.5.1 C# NativeAOT sidecar (tracked separately); \
                   v0.5 ships --from-openapi, --from-cli, --from-nuget, --from-python only";

pub fn build_from_dlls(_path: &str, _agent_id: Option<&str>) -> Result<GeneratedAgent, AwareError> {
    Err(AwareError::NotYetImplemented(MSG))
}

pub fn build_from_com(_progid: &str, _agent_id: Option<&str>) -> Result<GeneratedAgent, AwareError> {
    Err(AwareError::NotYetImplemented(MSG))
}

pub fn build_from_headers(_path: &str, _agent_id: Option<&str>) -> Result<GeneratedAgent, AwareError> {
    Err(AwareError::NotYetImplemented(MSG))
}

pub fn build_with_decompile(_pkg: &str) -> Result<GeneratedAgent, AwareError> {
    Err(AwareError::NotYetImplemented(MSG))
}
```

Commit: `feat(cli): builder stubs for tier-C sources (dlls/com/headers/decompile)`

### Task 8: Wire `aware build agent`

Update `cli/src/commands/build.rs`:

```rust
#[derive(Subcommand, Debug)]
pub enum BuildCommand {
    Agent(BuildAgentArgs),
}

#[derive(Args, Debug)]
pub struct BuildAgentArgs {
    /// OpenAPI spec URL or file path.
    #[arg(long = "from-openapi", conflicts_with_all = ["from_cli", "from_nuget", "from_python", "from_dlls", "from_com", "from_headers"])]
    pub from_openapi: Option<String>,
    /// CLI binary name (must be on PATH).
    #[arg(long = "from-cli")]
    pub from_cli: Option<String>,
    /// NuGet package: <pkg>@<version>.
    #[arg(long = "from-nuget")]
    pub from_nuget: Option<String>,
    /// Python module name.
    #[arg(long = "from-python")]
    pub from_python: Option<String>,
    /// (tier C) DLL glob path.
    #[arg(long = "from-dlls")]
    pub from_dlls: Option<String>,
    /// (tier C) COM ProgID.
    #[arg(long = "from-com")]
    pub from_com: Option<String>,
    /// (tier C) C++ header glob path.
    #[arg(long = "from-headers")]
    pub from_headers: Option<String>,
    /// (tier C) Run decompile pass on DLLs / NuGet.
    #[arg(long)]
    pub decompile: bool,
    /// Accept non-permissive license on --from-nuget.
    #[arg(long = "accept-license")]
    pub accept_license: bool,
    /// Override the generated agent id.
    #[arg(long)]
    pub output: Option<String>,
}

pub fn dispatch(cmd: BuildCommand, ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        BuildCommand::Agent(args) => build_agent(ctx, &args),
    }
}

fn build_agent(ctx: &Context, args: &BuildAgentArgs) -> Result<(), AwareError> {
    use crate::builder;

    let generated = if args.decompile {
        builder::stubs::build_with_decompile(args.from_nuget.as_deref().unwrap_or(""))?
    } else if let Some(s) = &args.from_openapi {
        builder::openapi::build_from_url_or_path(s, args.output.as_deref())?
    } else if let Some(s) = &args.from_cli {
        builder::cli_help::build_from_cli(s, args.output.as_deref())?
    } else if let Some(s) = &args.from_nuget {
        builder::nuget::build_from_nuget(s, args.output.as_deref(), args.accept_license)?
    } else if let Some(s) = &args.from_python {
        builder::python::build_from_python(s, args.output.as_deref())?
    } else if let Some(s) = &args.from_dlls {
        builder::stubs::build_from_dlls(s, args.output.as_deref())?
    } else if let Some(s) = &args.from_com {
        builder::stubs::build_from_com(s, args.output.as_deref())?
    } else if let Some(s) = &args.from_headers {
        builder::stubs::build_from_headers(s, args.output.as_deref())?
    } else {
        return Err(AwareError::Validation("aware build agent: must specify --from-<source>".into()));
    };

    let agents_dir = ctx.paths.agents_dir();
    std::fs::create_dir_all(&agents_dir)?;
    let dst = builder::write_agent(&generated, &agents_dir)?;
    println!("\u{2713} generated {} ({} commands, {} skills) at {}",
        generated.id, generated.commands.len(), generated.skills.len(), dst.display());
    Ok(())
}
```

Integration tests in `cli/tests/build_agent.rs`:
- `--from-openapi` with a fixture file → produces an agent folder
- `--from-cli aware` → produces an agent with the CLI's own subcommands
- `--from-dlls /foo` → returns `NotYetImplemented` with the sidecar message
- (NuGet + python tests skipped in CI; documented in CONTRIBUTING.)

Commit: `feat(cli): wire aware build agent (all 8 sources)`

### Task 9: `cli::skill_builder` framework + create

```rust
//! Skill commands: create / port / modify / eval.

pub mod create;
pub mod port;
pub mod modify;
pub mod eval;

#[derive(Subcommand, Debug)]
pub enum SkillCommand {
    /// Scaffold a new skill in an installed agent.
    Create {
        agent: String,
        #[arg(name = "skill-name")]
        skill_name: String,
    },
    /// Port an existing skill into an agent.
    Port {
        /// Source: <agent>/<skill> or path to .md file.
        source: String,
        #[arg(name = "target-agent")]
        target_agent: String,
    },
    /// Open a skill in $EDITOR.
    Modify {
        agent: String,
        #[arg(name = "skill-name")]
        skill_name: String,
    },
    /// Run a trigger-match eval against the skill.
    Eval {
        agent: String,
        #[arg(name = "skill-name")]
        skill_name: String,
    },
}

pub fn dispatch(cmd: SkillCommand, ctx: &Context) -> Result<(), AwareError> {
    match cmd {
        SkillCommand::Create { agent, skill_name } => create::run(ctx, &agent, &skill_name),
        SkillCommand::Port { source, target_agent } => port::run(ctx, &source, &target_agent),
        SkillCommand::Modify { agent, skill_name } => modify::run(ctx, &agent, &skill_name),
        SkillCommand::Eval { agent, skill_name } => eval::run(ctx, &agent, &skill_name),
    }
}
```

`create.rs`:

```rust
pub fn run(ctx: &Context, agent_id: &str, skill_name: &str) -> Result<(), AwareError> {
    let agent_dir = ctx.paths.agents_dir().join(agent_id);
    if !agent_dir.is_dir() {
        return Err(AwareError::NotFound(format!("agent {agent_id}")));
    }
    let skills_dir = agent_dir.join("skills");
    std::fs::create_dir_all(&skills_dir)?;
    let stem = skill_name.trim_end_matches(".md");
    let path = skills_dir.join(format!("{stem}.md"));
    if path.exists() {
        return Err(AwareError::Conflict(format!("skill {stem}.md already exists")));
    }
    let template = format!(
        "---\nname: {agent_id}-{stem}\ndescription: TODO \u{2014} one paragraph; what task triggers this skill?\n---\n\n# {stem}\n\nTODO: replace this with the skill content. Lead with the rule, then explain.\n\nWhat triggers loading: TODO.\n\nWhy it matters: TODO.\n"
    );
    std::fs::write(&path, template)?;
    println!("\u{2713} created {}", path.display());
    Ok(())
}
```

Tests, commit: `feat(cli): skill_builder create (template scaffolding)`

### Task 10: `skill_builder::port`

Accept `<source-agent>/<skill-name>` OR a plain file path. Copy + adapt frontmatter `name:` to point at target agent.

```rust
pub fn run(ctx: &Context, source: &str, target_agent: &str) -> Result<(), AwareError> {
    // Resolve source path
    let source_path = if source.contains('/') && !std::path::Path::new(source).is_file() {
        // Treat as <agent>/<skill>
        let (sa, sn) = source.split_once('/').unwrap();
        let stem = sn.trim_end_matches(".md");
        ctx.paths.agents_dir().join(sa).join("skills").join(format!("{stem}.md"))
    } else {
        std::path::PathBuf::from(source)
    };
    if !source_path.is_file() {
        return Err(AwareError::NotFound(format!("source skill: {}", source_path.display())));
    }

    let target_dir = ctx.paths.agents_dir().join(target_agent);
    if !target_dir.is_dir() {
        return Err(AwareError::NotFound(format!("agent {target_agent}")));
    }
    let skills_dir = target_dir.join("skills");
    std::fs::create_dir_all(&skills_dir)?;
    let basename = source_path.file_name().ok_or_else(|| AwareError::Validation("source path has no filename".into()))?;
    let target_path = skills_dir.join(basename);
    if target_path.exists() {
        return Err(AwareError::Conflict(format!("target {} already exists", target_path.display())));
    }

    let body = std::fs::read_to_string(&source_path)?;
    // Adapt frontmatter `name:` field
    let stem = source_path.file_stem().unwrap().to_string_lossy().to_string();
    let new_name = format!("name: {target_agent}-{stem}");
    let adapted = adapt_frontmatter_name(&body, &new_name);

    std::fs::write(&target_path, adapted)?;
    println!("\u{2713} ported {} \u{2192} {}", source_path.display(), target_path.display());
    Ok(())
}

fn adapt_frontmatter_name(body: &str, new_name_line: &str) -> String {
    let mut lines: Vec<String> = body.lines().map(String::from).collect();
    let mut in_frontmatter = false;
    for line in lines.iter_mut() {
        if line.trim() == "---" {
            in_frontmatter = !in_frontmatter;
            continue;
        }
        if in_frontmatter && line.trim_start().starts_with("name:") {
            *line = new_name_line.to_string();
        }
    }
    lines.join("\n") + "\n"
}
```

Commit: `feat(cli): skill_builder port`

### Task 11: `skill_builder::modify`

```rust
pub fn run(ctx: &Context, agent_id: &str, skill_name: &str) -> Result<(), AwareError> {
    let stem = skill_name.trim_end_matches(".md");
    let path = ctx.paths.agents_dir().join(agent_id).join("skills").join(format!("{stem}.md"));
    if !path.is_file() {
        return Err(AwareError::NotFound(format!("skill {agent_id}/{stem}")));
    }
    let editor = std::env::var("EDITOR").or_else(|_| std::env::var("VISUAL")).unwrap_or_else(|_|
        if cfg!(windows) { "notepad".to_string() } else { "vi".to_string() }
    );
    println!("Opening {} in {editor}...", path.display());
    let status = std::process::Command::new(&editor).arg(&path).status()
        .map_err(|e| AwareError::Network(format!("spawn {editor}: {e}")))?;
    if !status.success() {
        return Err(AwareError::Network(format!("{editor} exited {:?}", status.code())));
    }
    println!("\u{2713} saved {}", path.display());
    Ok(())
}
```

Tests: skip the actual editor spawn (set $EDITOR=`cmd /c exit 0` on Windows, `true` on Unix); verify the path resolution + missing-skill error.

Commit: `feat(cli): skill_builder modify ($EDITOR launcher)`

### Task 12: `skill_builder::eval`

Looks for `<skill-name>.eval.md` alongside the skill — a YAML frontmatter listing trigger phrases. Matches each phrase against the skill's `description` field via substring + token-overlap heuristic.

```rust
pub fn run(ctx: &Context, agent_id: &str, skill_name: &str) -> Result<(), AwareError> {
    let stem = skill_name.trim_end_matches(".md");
    let skills_dir = ctx.paths.agents_dir().join(agent_id).join("skills");
    let skill_path = skills_dir.join(format!("{stem}.md"));
    let eval_path = skills_dir.join(format!("{stem}.eval.md"));
    if !skill_path.is_file() {
        return Err(AwareError::NotFound(format!("skill {agent_id}/{stem}")));
    }
    if !eval_path.is_file() {
        println!("No eval corpus at {}. Creating template; edit and re-run.", eval_path.display());
        let tpl = "---\ntriggers:\n  - \"example phrase 1\"\n  - \"example phrase 2\"\n---\n\nAdd one trigger phrase per bullet above.\n";
        std::fs::write(&eval_path, tpl)?;
        return Ok(());
    }

    let skill_body = std::fs::read_to_string(&skill_path)?;
    let skill_desc = extract_frontmatter_field(&skill_body, "description").unwrap_or_default();
    let eval_body = std::fs::read_to_string(&eval_path)?;
    let triggers = extract_trigger_list(&eval_body);

    println!("Evaluating {agent_id}/{stem} against {} triggers...", triggers.len());
    let mut hits = 0usize;
    for t in &triggers {
        let pass = matches_trigger(&skill_desc, t);
        let mark = if pass { hits += 1; "\u{2713}" } else { "\u{2717}" };
        println!("  {mark} {t}");
    }
    println!("\n{hits} / {} triggers matched.", triggers.len());
    Ok(())
}

fn extract_frontmatter_field(body: &str, key: &str) -> Option<String> {
    let mut in_fm = false;
    for line in body.lines() {
        if line.trim() == "---" {
            if in_fm { return None; }
            in_fm = true;
            continue;
        }
        if in_fm && line.trim_start().starts_with(&format!("{key}:")) {
            return Some(line.trim_start().trim_start_matches(&format!("{key}:")).trim().to_string());
        }
    }
    None
}

fn extract_trigger_list(eval_body: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut in_triggers = false;
    let mut in_fm = false;
    for line in eval_body.lines() {
        if line.trim() == "---" { in_fm = !in_fm; continue; }
        if !in_fm { continue; }
        if line.trim_start().starts_with("triggers:") { in_triggers = true; continue; }
        if in_triggers {
            if let Some(t) = line.trim().strip_prefix("- ") {
                out.push(t.trim().trim_matches('"').to_string());
            } else if !line.starts_with(' ') {
                in_triggers = false;
            }
        }
    }
    out
}

fn matches_trigger(description: &str, trigger: &str) -> bool {
    let d = description.to_lowercase();
    let t = trigger.to_lowercase();
    // Substring or 50% token overlap
    if d.contains(&t) { return true; }
    let dt: std::collections::HashSet<&str> = d.split_whitespace().collect();
    let tt: Vec<&str> = t.split_whitespace().collect();
    let overlap = tt.iter().filter(|w| dt.contains(*w)).count();
    overlap * 2 >= tt.len()  // 50% threshold
}
```

Commit: `feat(cli): skill_builder eval (trigger corpus matcher)`

### Task 13: Wire `aware skill` + update main.rs

Add `mod skill_builder; mod builder;` to `main.rs`. Replace `cli/src/commands/skill.rs` body to dispatch into `skill_builder`.

Commit: `feat(cli): wire aware skill (create/port/modify/eval)`

### Task 14: Update `tests/basic.rs` smoke test

The current sentinel is `build agent --from-cli something`. Now `build agent --from-cli` is implemented. Switch to `build agent --from-dlls /some/path` (tier C — still stubbed).

Commit: `chore(cli): basic.rs smoke test targets tier-C stub`

### Task 15: README + final verification

Update `cli/README.md`:
- Status → v0.5: all 27 surfaces shipped (or "23 shipped + 4 stubbed for v0.5.1")
- New "Build an agent" section with all 8 source types + tier callouts
- New "Skill commands" section
- Drop the "Implementation order" table (all v0.x phases shipped)

Run `cargo test --all-targets`, `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings`, `cargo build --release`. Manual smoke: `aware build agent --from-openapi cli/tests/fixtures/openapi-mini.json` builds an agent end-to-end.

Commit: `docs(cli): README quickstart for v0.5`

---

## Self-review

**Spec coverage** — v0.5 DoD from cli-roadmap.md:
- `aware build agent --from-nuget Tekla.Structures.Model@2025.0.0` regenerates Tekla agent's API-reference skills → Task 5 (nuget tier B, XML-doc-based skills only; full DLL reflection for command generation deferred to v0.5.1 sidecar)
- `aware skill port` produces output identical/better than manual round-2.6 ports → Task 10
- Eval mode reports trigger accuracy → Task 12
- License-aware extraction blocks decompilation on proprietary packages → Task 5 (license check on nuspec)

**Out of scope acknowledged:** the C# NativeAOT sidecar for DLL / COM / decompile reflection. Tracked for v0.5.1. The v0.5 stubs return `NotYetImplemented` with a clear message pointing at the sidecar.

**Realistic effort:** 4-5 days per roadmap. With this plan + subagent execution, ~3-4 hours of focused dispatching.
