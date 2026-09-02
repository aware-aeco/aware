//! Agent builder framework — shared types + write_agent helper.
//!
//! Per-source builders land in tasks 3-7 (openapi / cli_help / nuget / python / stubs).

pub mod cli_help;
pub mod coverage;
pub mod npm;
pub mod nuget;
pub mod openapi;
pub mod python;
pub mod roslyn;
pub mod ruby;
pub mod stubs;
pub mod yard;

use std::collections::BTreeMap;
use std::path::Path;

use serde::Serialize;

use crate::error::AwareError;

#[derive(Debug)]
pub struct GeneratedAgent {
    pub id: String,
    /// Substrate semver. Starts at "0.1.0" for newly-generated agents.
    pub version: String,
    /// Optional vendor SDK / package version this agent reflects. Distinct
    /// from `version` (which tracks substrate revisions of the agent).
    pub sdk_target: Option<String>,
    pub description: String,
    pub commands: BTreeMap<String, GeneratedCommand>,
    pub skills: Vec<GeneratedSkill>,
    pub provenance: Provenance,
    pub stateful: bool,
    pub license: String,
    /// When set, the agent uses the `rest` transport (HTTP API) with an
    /// optional base URL and declarative auth, instead of the default `cli`
    /// transport (a wrapped binary). Set by the OpenAPI builder; `None` for
    /// SDK/CLI-derived agents.
    pub rest: Option<RestBlock>,
}

/// REST transport + declarative auth for a generated HTTP-API agent.
#[derive(Debug, Default)]
pub struct RestBlock {
    /// Base URL (from the OpenAPI `servers`); commands pass relative paths.
    pub base: Option<String>,
    /// Declarative auth derived from the spec's `securitySchemes`.
    pub auth: Option<AuthBlock>,
}

/// Declarative auth emitted into the manifest's `auth:` block. The credential
/// `secret` is also added to `requires.secrets`.
#[derive(Debug)]
pub struct AuthBlock {
    /// `api-key` | `bearer` | `oauth2`.
    pub scheme: String,
    /// For `api-key`: `header` | `query`.
    pub location: Option<String>,
    /// For `api-key`: the header / query-param name.
    pub name: Option<String>,
    /// Credential handle (also added to `requires.secrets`).
    pub secret: String,
}

#[derive(Debug, Default)]
pub struct GeneratedCommand {
    pub lifecycle: String,
    pub description: String,
    pub inputs_yaml: String,
    pub outputs_yaml: String,
    /// REST operation mapping (OpenAPI builder): HTTP method + path template,
    /// emitted into the manifest so the REST transport can execute the command.
    /// `None` for SDK/CLI-derived commands.
    pub method: Option<String>,
    pub path: Option<String>,
    /// Explicit read/write mode (e.g. `write` for mutating HTTP methods), so the
    /// safety contract applies regardless of the command name. `None` lets the
    /// loader infer mode from the name convention.
    pub mode: Option<String>,
    /// Public endpoint that does not use the agent's declared auth (OpenAPI
    /// operation with empty effective security). Emitted as `no-auth: true`.
    pub no_auth: bool,
}

#[derive(Debug)]
pub struct GeneratedSkill {
    pub filename: String,
    pub body: String,
}

#[derive(Debug, Serialize)]
pub struct Provenance {
    #[serde(rename = "generated-by")]
    pub generated_by: String,
    #[serde(rename = "generator-version")]
    pub generator_version: String,
    pub source: serde_json::Value,
    #[serde(rename = "generated-at")]
    pub generated_at: String,
}

// Byte-identical to the copy that used to live in `runtime::provenance`. Both
// are now re-exports of `crate::time::now_iso`; existing imports
// (`use crate::builder::{..., now_iso, ...}` in `sidecar.rs`) still resolve.
pub use crate::time::now_iso;

/// Slugify a reflected symbol name into the kebab-case id the substrate uses for
/// agent ids and command names.
///
/// The contract, stated once so it stops drifting: **every** ASCII uppercase
/// letter opens a new segment (`HTTPServer` → `h-t-t-p-server`), and **every** run
/// of characters outside `[A-Za-z0-9]` collapses to a single `-` (`Tekla::Model` →
/// `tekla-model`). Leading and trailing separators are trimmed.
///
/// `builder::npm`, `builder::ruby` and `builder::yard` each carried a private copy
/// of this. Two were byte-identical and the third was the same function rewritten
/// with a `prev_was_sep` flag standing in for the `out.is_empty() ||
/// out.ends_with('-')` test — indistinguishable from outside, and three places for
/// the next id-scheme fix to have to land.
///
/// Not to be confused with [`openapi::kebab`], which is a genuinely different
/// slugifier — see its doc comment.
pub fn kebab_ascii(s: &str) -> String {
    let mut out = String::new();
    for ch in s.chars() {
        if ch.is_ascii_alphanumeric() {
            if ch.is_ascii_uppercase() && !out.is_empty() && !out.ends_with('-') {
                out.push('-');
            }
            out.push(ch.to_ascii_lowercase());
        } else if !out.is_empty() && !out.ends_with('-') {
            out.push('-');
        }
    }
    out.trim_matches('-').to_string()
}

/// Write the generated agent into `<output_dir>/<agent-id>/`. Returns the new agent's root path.
pub fn write_agent(
    agent: &GeneratedAgent,
    output_dir: &Path,
) -> Result<std::path::PathBuf, AwareError> {
    let dst = output_dir.join(&agent.id);
    if dst.exists() {
        return Err(AwareError::Conflict(format!(
            "agent {} already exists at {}",
            agent.id,
            dst.display()
        )));
    }
    std::fs::create_dir_all(dst.join("skills"))?;
    std::fs::create_dir_all(dst.join("commands"))?;

    let manifest = build_manifest_yaml(agent)?;
    std::fs::write(dst.join("manifest.yaml"), manifest)?;

    for s in &agent.skills {
        std::fs::write(dst.join("skills").join(&s.filename), &s.body)?;
    }

    for (name, cmd) in &agent.commands {
        let mut body = format!(
            "# {name}\n\nLifecycle: {}\n\n{}\n",
            cmd.lifecycle, cmd.description
        );
        if !cmd.inputs_yaml.trim().is_empty() {
            body.push_str(&format!("\n## Inputs\n\n```yaml\n{}```\n", cmd.inputs_yaml));
        }
        if !cmd.outputs_yaml.trim().is_empty() {
            body.push_str(&format!(
                "\n## Output\n\n```yaml\n{}```\n",
                cmd.outputs_yaml
            ));
        }
        std::fs::write(dst.join("commands").join(format!("{name}.md")), body)?;
    }

    Ok(dst)
}

fn build_manifest_yaml(agent: &GeneratedAgent) -> Result<String, AwareError> {
    let mut out = String::new();
    out.push_str(&format!("agent:        {}\n", agent.id));
    out.push_str(&format!("version:      {}\n", agent.version));
    if let Some(sdk) = &agent.sdk_target {
        out.push_str(&format!("sdk-target:   {sdk}\n"));
    }
    out.push_str(&format!(
        "description: |\n  {}\n",
        agent.description.replace('\n', "\n  ")
    ));
    out.push_str(&format!("stateful:     {}\n", agent.stateful));
    out.push_str(&format!("license:      {}\n", agent.license));

    out.push_str("provenance:\n");
    let prov_yaml = serde_yaml::to_string(&agent.provenance)
        .map_err(|e| AwareError::Internal(format!("provenance yaml: {e}")))?;
    for line in prov_yaml.lines() {
        if line.trim().is_empty() {
            continue;
        }
        out.push_str("  ");
        out.push_str(line);
        out.push('\n');
    }

    out.push_str("transport:\n");
    match &agent.rest {
        Some(rest) => {
            out.push_str("  rest:");
            match &rest.base {
                Some(base) => out.push_str(&format!("\n    base: {}\n", quote_yaml_scalar(base))),
                None => out.push_str(" {}\n"),
            }
        }
        None => {
            out.push_str(&format!("  cli:\n    binary: aware-{}\n", agent.id));
        }
    }

    // Declarative auth + the credential it requires (REST agents only).
    if let Some(auth) = agent.rest.as_ref().and_then(|r| r.auth.as_ref()) {
        out.push_str("auth:\n");
        out.push_str(&format!("  scheme: {}\n", auth.scheme));
        if let Some(loc) = &auth.location {
            out.push_str(&format!("  in: {loc}\n"));
        }
        if let Some(name) = &auth.name {
            out.push_str(&format!("  name: {}\n", quote_yaml_scalar(name)));
        }
        out.push_str(&format!("  secret: {}\n", quote_yaml_scalar(&auth.secret)));
        out.push_str("requires:\n  secrets:\n");
        out.push_str(&format!("    - {}\n", quote_yaml_scalar(&auth.secret)));
    }

    if !agent.commands.is_empty() {
        out.push_str("commands:\n");
        for (name, cmd) in &agent.commands {
            out.push_str(&format!("  {name}:\n"));
            out.push_str(&format!("    lifecycle: {}\n", cmd.lifecycle));
            // Single-line description (YAML-safe)
            let desc_one_line = cmd.description.replace('\n', " ");
            out.push_str(&format!(
                "    description: {}\n",
                quote_yaml_scalar(&desc_one_line)
            ));
            if let Some(mode) = &cmd.mode {
                out.push_str(&format!("    mode: {}\n", quote_yaml_scalar(mode)));
            }
            if let Some(method) = &cmd.method {
                out.push_str(&format!("    method: {method}\n"));
            }
            if let Some(path) = &cmd.path {
                out.push_str(&format!("    path: {}\n", quote_yaml_scalar(path)));
            }
            if cmd.no_auth {
                out.push_str("    no-auth: true\n");
            }
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
    } else {
        // serde_yaml chokes on empty map — write `commands: {}` explicitly
        out.push_str("commands: {}\n");
    }

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

/// Quote a YAML scalar unless it survives being emitted bare — i.e. unless a
/// reader gets back the same `String` that went in.
///
/// Everything this quotes comes from a vendor artifact (an OpenAPI `summary`, a
/// JSDoc blurb, a `securitySchemes` parameter name), so the input is arbitrary
/// text and the emitted manifest has to stay loadable whatever it holds.
///
/// This used to be a hand-kept list of leading indicators (`- ? * & ! | > @ \` %
/// [`) plus "contains `:`/`#`/`"`". A list is exactly the wrong shape for this
/// job — the failure mode is a character nobody thought of, and five were
/// missing. Each of these round-tripped WRONG through the old rule, and the
/// first four make the whole generated manifest unloadable rather than
/// mis-loading one field:
///
/// * `}`, `,`, `]` leading — "did not find expected node content";
/// * `'` leading — YAML starts scanning a single-quoted scalar and runs off the
///   end of the document;
/// * an embedded newline — ends the scalar mid-value;
/// * `{` leading — silently reads as a flow MAPPING, so `{@link Foo} does x`
///   (ordinary JSDoc, which `builder::npm` and `builder::yard` copy verbatim
///   into a command description) becomes a map and the value is gone;
/// * leading / trailing whitespace — silently stripped;
/// * `true` / `null` / `~` / `0` — resolve to a bool / null / number, so the
///   loader rejects the manifest with "invalid type: boolean, expected a
///   string".
///
/// So rather than extend the list, ask the question the list was approximating:
/// parse the bare form and check it reads back as this exact string. That
/// covers every indicator and every non-string resolution at once, and it
/// tracks the YAML reader instead of drifting from it.
///
/// The three explicit checks in front of it are the ones a bare-document parse
/// cannot answer, because they are about the scalar's surroundings inside the
/// manifest rather than the scalar alone: a `:` or `#` is what ENDS a plain
/// scalar in the `key: value` position these land in, and a control character
/// is never wanted regardless.
pub(crate) fn quote_yaml_scalar(s: &str) -> String {
    if plain_scalar_is_safe(s) {
        s.to_string()
    } else {
        format!("\"{}\"", escape_double_quoted(s))
    }
}

/// Escape `s` for a YAML **double-quoted** scalar, which is the one YAML style
/// that interprets `\` escapes — so everything it would interpret has to be
/// written as an escape rather than emitted raw.
///
/// Escaping only `\` and `"` (which is all this did) is not enough, and the gap
/// is worse than it looks because a double-quoted scalar may span lines: a raw
/// newline inside the quotes is FOLDED to a space, so `"a\nb"` read back as
/// `a b` — a silent corruption, where the same value emitted bare at least
/// failed loudly. A raw `\r` folded the same way, and a raw NUL or ESC made the
/// document unparseable ("control characters are not allowed").
///
/// Control characters reach here rather than being rejected: they are exactly
/// what `plain_scalar_is_safe` refuses, so the quoted branch is the only branch
/// that can carry them and it has to carry them losslessly.
fn escape_double_quoted(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            // Every remaining control character is C0, DEL or C1 — all <= 0x9F,
            // so YAML's 8-bit `\xNN` escape covers the set exhaustively.
            c if c.is_control() => out.push_str(&format!("\\x{:02x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

/// Whether `s` can be emitted with no quotes and be read back as itself.
fn plain_scalar_is_safe(s: &str) -> bool {
    // Position-dependent, so the round-trip below cannot see them: `: ` opens a
    // mapping and ` #` opens a comment where these scalars actually sit, and a
    // control character (a newline above all) terminates the scalar. `"` is
    // cheaper to reject here than to reason about.
    if s.contains(':') || s.contains('#') || s.contains('"') || s.chars().any(char::is_control) {
        return false;
    }
    // The scalar alone: does YAML read the bare form back as this same string?
    // An empty scalar reads as null, a leading indicator errors or changes the
    // node's type, surrounding whitespace is stripped, and `true`/`0`/`null`
    // resolve to a non-string — all of which fail this and get quoted.
    serde_yaml::from_str::<serde_yaml::Value>(s)
        .ok()
        .and_then(|v| v.as_str().map(|parsed| parsed == s))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kebab_ascii_handles_namespaces_and_camelcase() {
        assert_eq!(kebab_ascii("Sketchup::Animation"), "sketchup-animation");
        assert_eq!(kebab_ascii("nextFrame"), "next-frame");
        assert_eq!(kebab_ascii("ArcCurve"), "arc-curve");
    }

    /// The contract the three former copies shared, pinned so the next edit has to
    /// admit it is a behaviour change: every capital opens a segment, every
    /// non-alphanumeric run collapses to one `-`, and the ends are trimmed.
    #[test]
    fn kebab_ascii_splits_on_every_capital_and_collapses_separators() {
        assert_eq!(kebab_ascii("HTTPServer"), "h-t-t-p-server");
        assert_eq!(kebab_ascii("  spaced__out  "), "spaced-out");
        assert_eq!(kebab_ascii("already-kebab"), "already-kebab");
        assert_eq!(kebab_ascii("v2Point3"), "v2-point3");
        assert_eq!(kebab_ascii("!!!"), "");
    }

    fn sample_agent() -> GeneratedAgent {
        let mut cmds = BTreeMap::new();
        cmds.insert(
            "do-thing".into(),
            GeneratedCommand {
                lifecycle: "single".into(),
                description: "Does a thing.".into(),
                inputs_yaml: String::new(),
                outputs_yaml: String::new(),
                ..Default::default()
            },
        );
        GeneratedAgent {
            id: "test-agent".into(),
            version: "0.1.0".into(),
            sdk_target: Some("1.2.3".into()),
            description: "Test agent generated for unit testing.".into(),
            commands: cmds,
            skills: vec![GeneratedSkill {
                filename: "intro.md".into(),
                body: "# intro\n\nIntro content.\n".into(),
            }],
            provenance: Provenance {
                generated_by: "aware-agent-builder".into(),
                generator_version: "test".into(),
                source: serde_json::json!({"type": "test"}),
                generated_at: "2026-05-16T00:00:00Z".into(),
            },
            stateful: false,
            license: "MIT".into(),
            rest: None,
        }
    }

    #[test]
    fn write_agent_creates_full_folder() {
        let tmp = tempfile::tempdir().unwrap();
        let agent = sample_agent();
        let dst = write_agent(&agent, tmp.path()).unwrap();
        assert!(dst.join("manifest.yaml").is_file());
        assert!(dst.join("skills/intro.md").is_file());
        assert!(dst.join("commands/do-thing.md").is_file());
    }

    #[test]
    fn write_agent_rejects_existing_folder() {
        let tmp = tempfile::tempdir().unwrap();
        let agent = sample_agent();
        write_agent(&agent, tmp.path()).unwrap();
        let err = write_agent(&agent, tmp.path()).unwrap_err();
        assert!(matches!(err, AwareError::Conflict(_)));
    }

    #[test]
    fn generated_manifest_parses_via_loader() {
        let tmp = tempfile::tempdir().unwrap();
        let agent = sample_agent();
        let dst = write_agent(&agent, tmp.path()).unwrap();
        // Round-trip: load the generated manifest via the existing loader
        let loaded = crate::manifest::loader::load_agent(&dst.join("manifest.yaml")).unwrap();
        assert_eq!(loaded.agent, "test-agent");
        assert_eq!(loaded.version, "0.1.0");
        assert_eq!(loaded.commands.len(), 1);
        assert_eq!(loaded.skills.len(), 1);
    }

    /// Build a manifest for one command whose fields the caller sets, then read it
    /// back through the real loader. Every assertion below travels through
    /// `serde_yaml` rather than through a value the test just wrote, so a broken
    /// emitter shows up as a parse failure or a changed value, not as a tautology.
    fn round_trip(agent: &GeneratedAgent) -> crate::manifest::agent::Agent {
        let tmp = tempfile::tempdir().unwrap();
        let dst = write_agent(agent, tmp.path()).unwrap();
        crate::manifest::loader::load_agent(&dst.join("manifest.yaml")).unwrap_or_else(|e| {
            panic!(
                "generated manifest did not load: {e}\n--- manifest ---\n{}",
                build_manifest_yaml(agent).unwrap()
            )
        })
    }

    fn agent_with(command: GeneratedCommand) -> GeneratedAgent {
        let mut a = sample_agent();
        a.commands = BTreeMap::from([("op".to_string(), command)]);
        a
    }

    fn cmd(description: &str) -> GeneratedCommand {
        GeneratedCommand {
            lifecycle: "single".into(),
            description: description.into(),
            ..Default::default()
        }
    }

    /// Command descriptions are vendor text copied verbatim (an OpenAPI `summary`,
    /// a JSDoc blurb), so the emitter has to keep the manifest loadable whatever
    /// they open with. Each of these read back as something other than itself
    /// before `quote_yaml_scalar` stopped guessing from a list of indicators —
    /// four of them by failing to parse at all, taking the whole manifest with
    /// them.
    #[test]
    fn a_description_survives_whatever_a_vendor_docstring_opens_with() {
        let hostile = [
            "{@link Foo} returns a thing", // JSDoc — read back as a flow mapping
            "}unbalanced",                 // parse error
            "]unbalanced",                 // parse error
            ",leading comma",              // parse error
            "'tis deprecated",             // opens a single-quoted scalar, runs off the end
            "\"quoted\" per the vendor",
            "  padded on both sides  ", // whitespace silently stripped
            "true",                     // resolves to a bool, which the loader rejects
            "null",
            "~",
            "0",
            "-1 means unbounded",
            "*ptr is not owned",
            "% of total",
            "#1 in the list",
            "returns: a thing",
        ];
        for text in hostile {
            let loaded = round_trip(&agent_with(cmd(text)));
            assert_eq!(
                loaded.commands["op"].description, text,
                "description {text:?} did not survive the manifest round-trip"
            );
        }
    }

    /// Quoting is only half the job: the quoted form has to read back as the same
    /// string. A YAML double-quoted scalar may span lines, so a raw newline inside
    /// the quotes FOLDS to a space — quoting a multi-line value without escaping it
    /// turns a loud parse failure into a silent one-space-instead-of-a-newline
    /// corruption. `sidecar.rs` sends plug-in-supplied names and defaults through
    /// this same function, and nothing upstream of it strips control characters.
    #[test]
    fn a_quoted_scalar_reads_back_as_the_string_that_went_in() {
        for s in [
            "line one\nline two", // folded to a space when not escaped
            "carriage\rreturn",   // likewise
            "tab\tseparated",
            "nul\u{0}byte",   // unescaped: "control characters are not allowed"
            "esc\u{1b}[0m",   // likewise
            "back\\slash: x", // the `\` must not be eaten as an escape introducer
            "already \\n two chars",
            "quote\"inside: x",
        ] {
            let doc = format!("v: {}\n", quote_yaml_scalar(s));
            let parsed: serde_yaml::Value = serde_yaml::from_str(&doc)
                .unwrap_or_else(|e| panic!("{s:?} emitted an unparseable scalar: {e}"));
            assert_eq!(
                parsed["v"].as_str(),
                Some(s),
                "{s:?} did not survive quoting; emitted {doc:?}"
            );
        }
    }

    /// The other direction, without which "quote everything" would pass the test
    /// above: ordinary prose stays bare, so the generated manifests a human reads
    /// are not smothered in escapes.
    #[test]
    fn ordinary_prose_is_left_unquoted() {
        let yaml = build_manifest_yaml(&agent_with(cmd("Lists every project"))).unwrap();
        assert!(
            yaml.contains("description: Lists every project\n"),
            "expected a bare scalar, got:\n{yaml}"
        );
    }

    /// `sdk-target` is the vendor release an agent reflects, and it is optional —
    /// the key must be absent (not empty) when the builder did not learn one,
    /// since `sdk-target:` with no value deserializes as null and not as `None`.
    #[test]
    fn an_absent_sdk_target_omits_the_key_rather_than_emitting_an_empty_one() {
        let mut agent = sample_agent();
        assert_eq!(round_trip(&agent).sdk_target.as_deref(), Some("1.2.3"));
        agent.sdk_target = None;
        let yaml = build_manifest_yaml(&agent).unwrap();
        assert!(!yaml.contains("sdk-target"), "got:\n{yaml}");
        assert_eq!(round_trip(&agent).sdk_target, None);
    }

    /// A REST agent carries its base URL, its declarative auth, AND the credential
    /// handle mirrored into `requires.secrets` — the transport reads the first two
    /// and `aware connect` / `credential status` read the third, so an emitter that
    /// dropped either half would leave a built agent that cannot authenticate.
    #[test]
    fn a_rest_agent_emits_its_base_auth_and_the_secret_it_requires() {
        let mut agent = agent_with(cmd("Lists things"));
        agent.rest = Some(RestBlock {
            base: Some("https://api.example.com/v1".into()),
            auth: Some(AuthBlock {
                scheme: "api-key".into(),
                location: Some("header".into()),
                name: Some("X-API-Key".into()),
                secret: "example-api".into(),
            }),
        });
        let loaded = round_trip(&agent);

        let rest = loaded.transport.rest.expect("rest transport");
        assert_eq!(rest["base"].as_str(), Some("https://api.example.com/v1"));
        assert!(loaded.transport.cli.is_none(), "rest agent got a cli block");

        let auth = loaded.auth.expect("auth block");
        assert_eq!(auth.scheme, "api-key");
        assert_eq!(auth.location.as_deref(), Some("header"));
        assert_eq!(auth.name.as_deref(), Some("X-API-Key"));
        assert_eq!(auth.secret, "example-api");

        assert_eq!(
            loaded.requires.expect("requires").secrets,
            vec!["example-api".to_string()],
            "the auth handle must also be declared as a required secret"
        );
    }

    /// A spec with no `servers` yields a based-less REST block. It still has to be
    /// a present, empty mapping: `rest:` alone is null, which reads as "no rest
    /// transport" and makes the agent undispatchable.
    #[test]
    fn a_rest_agent_without_a_base_still_declares_the_transport() {
        let mut agent = agent_with(cmd("Lists things"));
        agent.rest = Some(RestBlock {
            base: None,
            auth: None,
        });
        let loaded = round_trip(&agent);
        let rest = loaded.transport.rest.expect("rest transport");
        assert!(rest.is_mapping(), "expected a mapping, got {rest:?}");
        assert!(rest["base"].is_null(), "unexpected base: {rest:?}");
        assert!(loaded.auth.is_none());
        // No auth means no credential to require.
        assert!(loaded.requires.is_none_or(|r| r.secrets.is_empty()));
    }

    /// The default (SDK/CLI-derived) path: the transport names the wrapper binary
    /// by convention, and nothing an authenticated REST agent needs is emitted.
    #[test]
    fn a_non_rest_agent_gets_a_cli_transport_named_after_its_id() {
        let loaded = round_trip(&agent_with(cmd("Does a thing")));
        assert_eq!(
            loaded.transport.cli.expect("cli transport").binary,
            "aware-test-agent"
        );
        assert!(loaded.transport.rest.is_none());
        assert!(loaded.auth.is_none());
    }

    /// The OpenAPI operation mapping. `mode` is what puts a mutating endpoint under
    /// the safety contract regardless of what the operation is called, and `no-auth`
    /// is what stops the transport demanding a credential for a public endpoint —
    /// both are silent-wrong-answer fields if the emitter drops them.
    #[test]
    fn an_operation_command_carries_its_method_path_mode_and_no_auth() {
        let mut c = cmd("Creates a thing");
        c.method = Some("post".into());
        c.path = Some("/v1/things/{id}".into());
        c.mode = Some("write".into());
        c.no_auth = true;
        let loaded = round_trip(&agent_with(c));

        let op = &loaded.commands["op"];
        assert_eq!(op.method.as_deref(), Some("post"));
        assert_eq!(op.path.as_deref(), Some("/v1/things/{id}"));
        assert_eq!(op.mode, Some(crate::manifest::agent::Mode::Write));
        assert!(op.no_auth);
    }

    /// Those same fields are absent — not null, not defaulted-in — for a command
    /// that has no operation mapping, so a CLI agent never looks like a half-built
    /// REST one.
    #[test]
    fn a_command_without_an_operation_mapping_omits_those_keys() {
        let loaded = round_trip(&agent_with(cmd("Does a thing")));
        let op = &loaded.commands["op"];
        assert_eq!(op.method, None);
        assert_eq!(op.path, None);
        assert_eq!(op.mode, None);
        assert!(!op.no_auth);
    }

    /// `inputs_yaml` / `outputs_yaml` arrive as a YAML fragment at column zero and
    /// are re-indented under the command. Nesting is the whole content of a schema,
    /// so a re-indent that flattens or over-indents one level silently changes what
    /// the command declares.
    #[test]
    fn input_and_output_schemas_keep_their_nesting_when_re_indented() {
        let mut c = cmd("Lists things");
        c.inputs_yaml = "limit:\n  type: integer\n  in: query\nid:\n  type: string\n".into();
        c.outputs_yaml = "body:\n  type: object\n".into();
        let loaded = round_trip(&agent_with(c));

        let op = &loaded.commands["op"];
        assert_eq!(op.inputs["limit"]["type"].as_str(), Some("integer"));
        assert_eq!(op.inputs["limit"]["in"].as_str(), Some("query"));
        assert_eq!(op.inputs["id"]["type"].as_str(), Some("string"));
        let outputs = op.outputs.as_ref().expect("outputs");
        assert_eq!(outputs["body"]["type"].as_str(), Some("object"));
    }

    /// The agent description is emitted as a block scalar and every continuation
    /// line has to be indented into it. Under-indent one and the second line reads
    /// as a sibling key.
    #[test]
    fn a_multi_line_description_stays_one_scalar() {
        let mut agent = sample_agent();
        agent.description = "First line.\nSecond line.\nThird: line.".into();
        let loaded = round_trip(&agent);
        assert_eq!(
            loaded.description.trim_end(),
            "First line.\nSecond line.\nThird: line."
        );
    }

    /// Skills are listed sorted, not in the order the builder happened to collect
    /// them, so regenerating an agent from an unchanged source produces an
    /// unchanged manifest.
    #[test]
    fn skills_are_listed_in_sorted_order() {
        let mut agent = sample_agent();
        agent.skills = ["z.md", "a.md", "m.md"]
            .map(|f| GeneratedSkill {
                filename: f.into(),
                body: format!("# {f}\n"),
            })
            .into();
        let loaded = round_trip(&agent);
        assert_eq!(loaded.skills, vec!["a.md", "m.md", "z.md"]);
    }
}
