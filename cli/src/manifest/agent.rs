//! Typed deserialization for AWARE agent manifests.
//!
//! Shapes verified against all 7 reference agents:
//! - `20-agents/aeco/engineering/tekla/manifest.yaml`
//! - `20-agents/aeco/construction/trimble-connect/manifest.yaml`
//! - `20-agents/aeco/cross-cutting/microsoft-365/manifest.yaml`
//! - `20-agents/aeco/cross-cutting/google-workspace/manifest.yaml`
//! - `20-agents/_core/aware-agent-builder/manifest.yaml`
//! - `20-agents/_core/aware-skill-builder/manifest.yaml`
//! - `20-agents/_core/html-report/manifest.yaml`
//!
//! Fields consumed so far:
//!
//! - Task 9 (`agent list`): `agent`, `version`, `skills`, `commands`,
//!   `kind()`, `skill_count()`, `command_count()`.
//! - Task 10 (`agent describe`): `display_name`, `description`, `stateful`,
//!   `vendor`, `license`, `transport.cli.binary`, `Command::lifecycle`,
//!   `Command::description`.
//!
//! Remaining unused until later tasks: `homepage`, `keywords`, `provenance`,
//! `requires`, `Command::inputs`, `Command::outputs`, `Transport::mcp`,
//! `Transport::rest`, and all `Provenance`/`Requires` fields.
//! Each gets a targeted `#[allow(dead_code)]`.

use std::collections::BTreeMap;

use serde::Deserialize;
use serde_yaml::Value;

#[derive(Debug, Deserialize)]
pub struct Agent {
    pub agent: String,
    pub version: String,
    /// Optional vendor SDK / product version this agent targets. Distinct
    /// from `version:` (which is the substrate's own semver for the agent).
    /// Surfaced prominently in the substrate report so users can tell at a
    /// glance which Tekla / Revit / etc. release the agent reflects.
    #[serde(rename = "sdk-target", default)]
    pub sdk_target: Option<String>,
    #[serde(rename = "display-name")]
    pub display_name: Option<String>,
    pub description: String,
    pub stateful: bool,
    pub vendor: Option<String>,
    pub license: String,
    #[allow(dead_code)]
    pub homepage: Option<String>,
    #[serde(default)]
    #[allow(dead_code)]
    pub keywords: Vec<String>,
    #[allow(dead_code)]
    pub provenance: Option<Provenance>,
    #[allow(dead_code)]
    pub requires: Option<Requires>,
    /// `engineering:` block — declares pinnable inputs for the engineering
    /// envelope (v0.21). Engineering agents (TSD, IDEA, CSi, etc.) declare
    /// what their downstream apps MUST pin to produce reproducible
    /// calculations. See `10-core/agent-spec.md § Engineering envelope`.
    #[allow(dead_code)]
    #[serde(default)]
    pub engineering: Option<EngineeringDecl>,
    pub transport: Transport,
    /// Declarative auth (v0.39). When present, the REST transport injects the
    /// referenced secret on every call (apiKey header/query, or bearer/oauth2
    /// `Authorization: Bearer`), so a built authenticated API works without the
    /// app author hand-templating `{{ secrets.<id> }}` into each request.
    /// Emitted by `aware build --from-openapi` from the spec's `securitySchemes`.
    #[serde(default)]
    pub auth: Option<AuthScheme>,
    #[serde(default)]
    pub commands: BTreeMap<String, Command>,
    #[serde(default)]
    pub skills: Vec<String>,
}

/// Declarative authentication for a REST-transport agent.
#[derive(Debug, Deserialize, Clone)]
pub struct AuthScheme {
    /// `api-key` | `bearer` | `oauth2`. (`oauth2` is treated as bearer at the
    /// transport: the provisioned secret is sent as `Authorization: Bearer`.)
    pub scheme: String,
    /// For `api-key`: where the key goes — `header` (default) or `query`.
    #[serde(rename = "in", default)]
    pub location: Option<String>,
    /// For `api-key`: the header or query-param name (e.g. `apikey`, `X-API-Key`).
    #[serde(default)]
    pub name: Option<String>,
    /// Credential handle — matches a `requires.secrets` entry and the
    /// keychain account / `~/.aware/credentials/<secret>.json` file.
    pub secret: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct EngineeringDecl {
    #[serde(default)]
    pub pinnable: Vec<EngineeringPin>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct EngineeringPin {
    pub id: String,
    pub description: String,
    #[serde(default)]
    pub required: bool,
    pub example: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Provenance {
    #[serde(rename = "generated-by")]
    pub generated_by: Option<String>,
    #[serde(rename = "generator-version")]
    pub generator_version: Option<String>,
    pub source: Option<Value>,
    #[serde(rename = "refined-by", default)]
    pub refined_by: Vec<String>,
    #[serde(rename = "generated-at")]
    pub generated_at: Option<String>,
    /// Present in some ported agents (e.g. microsoft-365, google-workspace)
    /// to record the FloLess production path the skills were ported from.
    #[serde(rename = "ported-from")]
    pub ported_from: Option<String>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize, Default)]
pub struct Requires {
    #[serde(default)]
    pub filesystem: Vec<Value>,
    #[serde(default)]
    pub network: Vec<String>,
    #[serde(default)]
    pub software: Vec<String>,
    #[serde(default)]
    pub secrets: Vec<String>,
    /// Present in `aware-skill-builder` to declare a dependency on an
    /// Anthropic built-in skill (e.g. `anthropic:skill-creator`).
    #[serde(default)]
    pub skills: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Transport {
    pub cli: Option<TransportCli>,
    #[allow(dead_code)]
    pub mcp: Option<Value>,
    #[allow(dead_code)]
    pub rest: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct TransportCli {
    pub binary: String,
}

#[derive(Debug, Deserialize)]
pub struct Command {
    pub lifecycle: Lifecycle,
    pub description: String,
    #[serde(default)]
    #[allow(dead_code)]
    pub inputs: Value,
    #[allow(dead_code)]
    pub outputs: Option<Value>,
    /// REST operation mapping (v0.39, `--from-openapi`): the HTTP method this
    /// command performs. When set, the REST transport executes the command as
    /// `<method> <base><path>` rather than requiring the command name to be a
    /// bare HTTP method. `None` for the generic `http` agent + non-REST agents.
    #[serde(default)]
    pub method: Option<String>,
    /// REST operation path template (e.g. `/pets/{petId}`). `{name}` segments
    /// are filled from inputs whose schema declares `in: path`.
    #[serde(default)]
    pub path: Option<String>,
    /// This command is a public endpoint that does NOT use the agent's declared
    /// `auth:` — set by `--from-openapi` for operations whose effective security
    /// is empty (e.g. login/health). The REST transport skips auth injection
    /// (and its missing-credential check) for such commands.
    #[serde(rename = "no-auth", default)]
    pub no_auth: bool,
    /// Explicit per-command category. If `None`, the agent-level provenance
    /// is used to infer the default (see `Agent::default_category`).
    #[serde(default)]
    pub category: Option<Category>,
    /// Explicit read/write mode. If `None`, inferred from the command name
    /// per the convention in `10-core/app-spec.md § Safety contract`.
    #[serde(default)]
    pub mode: Option<Mode>,
}

/// Read/write mode for a command — drives the safety-contract enforcement.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Read,
    Write,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Lifecycle {
    Start,
    Stop,
    Single,
}

/// Whether a command is a hand-curated workflow verb (`Curated`) or an
/// auto-generated leaf-level API method (`Reflected`).
///
/// Per `10-core/agent-spec.md § Commands § Curated vs reflected commands`,
/// curated commands have typed `inputs:`/`outputs:`, examples, and a skill
/// that says when to use them. Reflected commands are auto-generated from a
/// vendor SDK / OpenAPI / decompile — wide coverage as an escape hatch but no
/// curation contract.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Curated,
    Reflected,
}

impl Agent {
    pub fn skill_count(&self) -> usize {
        self.skills.len()
    }

    pub fn command_count(&self) -> usize {
        self.commands.len()
    }

    /// Default category for commands that don't declare one explicitly.
    ///
    /// Inference rule (per spec): if the manifest has a `provenance.generated-by`
    /// block (i.e. the agent was machine-generated from a vendor source),
    /// commands without explicit category default to `Reflected`. Otherwise
    /// they default to `Curated`.
    pub fn default_category(&self) -> Category {
        let machine_generated = self
            .provenance
            .as_ref()
            .and_then(|p| p.generated_by.as_deref())
            .is_some();
        if machine_generated {
            Category::Reflected
        } else {
            Category::Curated
        }
    }

    /// Resolve the effective category for a command — explicit if present,
    /// otherwise the agent's default.
    pub fn category_of(&self, cmd: &Command) -> Category {
        cmd.category.unwrap_or_else(|| self.default_category())
    }

    /// Number of commands resolving to `Curated`.
    pub fn curated_count(&self) -> usize {
        self.commands
            .values()
            .filter(|c| self.category_of(c) == Category::Curated)
            .count()
    }

    /// Number of commands resolving to `Reflected`.
    pub fn reflected_count(&self) -> usize {
        self.commands.len().saturating_sub(self.curated_count())
    }

    /// Resolve the effective mode for a command — explicit if present,
    /// otherwise inferred from the command name per the convention in
    /// `10-core/app-spec.md § Safety contract`.
    ///
    /// A command is `Write` if its name matches any of:
    /// `*.create`, `*.update`, `*.delete`, `*.bump`, `*.stamp`, `*.reload-all`,
    /// `*.bulk-write`, `*.insert`, `*.save`, `*.publish`. Otherwise `Read`.
    pub fn mode_of(&self, name: &str, cmd: &Command) -> Mode {
        if let Some(m) = cmd.mode {
            return m;
        }
        if is_write_by_convention(name) {
            Mode::Write
        } else {
            Mode::Read
        }
    }

    /// Human-readable kind label for the `agent list` table: lowercased
    /// `display-name`, falling back to `agent` id, truncated to 17 chars.
    pub fn kind(&self) -> String {
        let raw = self
            .display_name
            .as_ref()
            .map(|d| d.to_lowercase())
            .unwrap_or_else(|| self.agent.clone());
        if raw.chars().count() > 17 {
            let mut s: String = raw.chars().take(14).collect();
            s.push_str("...");
            s
        } else {
            raw
        }
    }
}

/// Names matching this convention default to `Mode::Write`.
///
/// See `10-core/app-spec.md § Safety contract (write-mode nodes)`.
fn is_write_by_convention(name: &str) -> bool {
    const SUFFIXES: &[&str] = &[
        ".create",
        ".update",
        ".delete",
        ".bump",
        ".stamp",
        ".reload-all",
        ".bulk-write",
        ".insert",
        ".save",
        ".publish",
        ".export-pdfs",
        ".export",
    ];
    SUFFIXES.iter().any(|s| name.ends_with(s))
        // The Tekla curated agent's write commands don't match the dotted
        // namespace convention because they predate it.
        || name == "insert"
        || name == "save-attributes"
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEKLA_MIN: &str = r#"
agent: tekla
version: 2025.0.1
display-name: Tekla Structures
description: |
  Watches the active Tekla model.
stateful: true
license: Apache-2.0
transport:
  cli:
    binary: aware-tekla
commands:
  watch:
    lifecycle: start
    description: Subscribe.
    outputs:
      type: stream
  insert:
    lifecycle: single
    description: Create.
skills:
  - drawing-identity.md
  - event-threading.md
"#;

    #[test]
    fn parses_minimal_manifest() {
        let a: Agent = serde_yaml::from_str(TEKLA_MIN).unwrap();
        assert_eq!(a.agent, "tekla");
        assert_eq!(a.version, "2025.0.1");
        assert!(a.stateful);
        assert_eq!(a.license, "Apache-2.0");
        assert_eq!(a.skill_count(), 2);
        assert_eq!(a.command_count(), 2);
        assert_eq!(a.transport.cli.unwrap().binary, "aware-tekla");
    }

    #[test]
    fn kind_uses_lowercased_display_name() {
        let a: Agent = serde_yaml::from_str(TEKLA_MIN).unwrap();
        assert_eq!(a.kind(), "tekla structures");
    }

    #[test]
    fn kind_truncates_long_names() {
        let yaml = TEKLA_MIN.replace("Tekla Structures", "Some Extremely Long Name For The Agent");
        let a: Agent = serde_yaml::from_str(&yaml).unwrap();
        let k = a.kind();
        assert!(k.chars().count() <= 17, "kind too long: {k:?}");
        assert!(k.ends_with("..."));
    }

    #[test]
    fn kind_falls_back_to_agent_id_when_no_display_name() {
        let yaml = r#"
agent: tekla
version: 1.0
description: x
stateful: false
license: MIT
transport: { cli: { binary: x } }
commands: {}
"#;
        let a: Agent = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(a.kind(), "tekla");
    }

    #[test]
    fn missing_required_field_errors() {
        let bad = "agent: tekla\nversion: 1.0";
        assert!(serde_yaml::from_str::<Agent>(bad).is_err());
    }

    #[test]
    fn parses_real_tekla_manifest() {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("20-agents/aeco/engineering/tekla/manifest.yaml");
        let text = std::fs::read_to_string(&path).unwrap();
        let a: Agent = serde_yaml::from_str(&text).unwrap();
        assert_eq!(a.agent, "tekla");
        // tekla is the gold-standard curated agent — currently 33 skills and
        // 23 commands. (Count was stale at 22; the manifest has since grown.)
        assert_eq!(a.skill_count(), 33);
        assert_eq!(a.command_count(), 23);
        assert!(a.stateful);
    }

    #[test]
    fn tekla_commands_are_explicitly_curated() {
        // The Tekla curated agent is the gold-standard category: curated agent.
        // All 3 commands declare `category: curated` explicitly.
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("20-agents/aeco/engineering/tekla/manifest.yaml");
        let text = std::fs::read_to_string(&path).unwrap();
        let a: Agent = serde_yaml::from_str(&text).unwrap();
        // All tekla commands are explicitly `category: curated` (23 total).
        assert_eq!(a.curated_count(), 23);
        assert_eq!(a.reflected_count(), 0);
        for (_name, cmd) in &a.commands {
            assert_eq!(cmd.category, Some(Category::Curated));
        }
    }

    #[test]
    fn revit_manifest_mixes_curated_and_reflected() {
        // The Revit-2026 manifest is machine-generated (provenance has
        // generated-by), so the inference rule makes unmarked commands
        // default to Reflected. The first wave of curated workflow verbs
        // explicitly carry `category: curated`.
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("20-agents/aeco/architecture/revit-2026/manifest.yaml");
        let text = std::fs::read_to_string(&path).unwrap();
        let a: Agent = serde_yaml::from_str(&text).unwrap();
        assert!(
            a.curated_count() >= 10,
            "expected ≥10 curated, got {}",
            a.curated_count()
        );
        assert!(
            a.reflected_count() > 7000,
            "expected ≫7000 reflected, got {}",
            a.reflected_count()
        );
        assert!(matches!(a.default_category(), Category::Reflected));
    }

    #[test]
    fn category_inference_falls_back_to_curated_without_provenance() {
        // No provenance block in the minimal Tekla fixture → default is Curated.
        let a: Agent = serde_yaml::from_str(TEKLA_MIN).unwrap();
        assert!(matches!(a.default_category(), Category::Curated));
        // Both commands have no explicit category → both resolve to Curated.
        assert_eq!(a.curated_count(), 2);
        assert_eq!(a.reflected_count(), 0);
    }

    #[test]
    fn category_inference_falls_back_to_reflected_with_provenance() {
        let yaml = r#"
agent: ex
version: 1.0
description: x
stateful: false
license: MIT
provenance:
  generated-by: aware-agent-builder
  generator-version: 0.8.0
transport: { cli: { binary: x } }
commands:
  ex-foo:
    lifecycle: single
    description: x
"#;
        let a: Agent = serde_yaml::from_str(yaml).unwrap();
        assert!(matches!(a.default_category(), Category::Reflected));
        assert_eq!(a.reflected_count(), 1);
        assert_eq!(a.curated_count(), 0);
    }
}
