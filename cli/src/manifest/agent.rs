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
//! Still deserialized but never read: `homepage`, `engineering`,
//! `EngineeringDecl::pinnable`, every field of `EngineeringPin`, every field
//! of `Provenance` except `generated_by`, and `Requires::filesystem` +
//! `Requires::skills`. They stay because they are what type-check the keys
//! the agent-spec publishes. Dropping them would not narrow what a manifest
//! is allowed to declare — there is no `deny_unknown_fields`, so a key with
//! no field is ignored, not rejected — it would stop checking the shape of
//! these, letting a malformed declaration through unnoticed. Each carries a
//! targeted `#[allow(dead_code)]` rather than a blanket module-level one, so
//! a newly-unread field surfaces as a warning instead of hiding.

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
    /// Runnability of the agent's transport. `available` (default) means the
    /// transport ships / is installable; `planned` means it is not wired yet;
    /// `requires-runtime` means it is runnable only on a CLI at or above
    /// `minimum-cli-version`. Lifecycle validation enforces that requirement at
    /// install, compile, and run instead of allowing an older runtime to fall
    /// through to a transport it cannot execute (#495).
    #[serde(default)]
    pub status: AgentStatus,
    /// The first AWARE CLI version that understands this agent's implementation.
    /// Required when `status: requires-runtime`; otherwise omitted. Kept at agent
    /// level because the runtime implementation is shipped by the CLI, not by an
    /// individual command.
    #[serde(rename = "minimum-cli-version", default)]
    pub minimum_cli_version: Option<String>,
    pub vendor: Option<String>,
    pub license: String,
    #[allow(dead_code)]
    pub homepage: Option<String>,
    #[serde(default)]
    pub keywords: Vec<String>,
    pub provenance: Option<Provenance>,
    pub requires: Option<Requires>,
    /// `engineering:` block — declares pinnable inputs for the engineering
    /// envelope (v0.21). Engineering agents (TSD, IDEA, CSi, etc.) declare
    /// what their downstream apps MUST pin to produce reproducible
    /// calculations. See `10-core/agent-spec.md § Engineering envelope`.
    #[allow(dead_code)]
    #[serde(default)]
    pub engineering: Option<EngineeringDecl>,
    /// Agent-level capability flags (RFC #223). The validator's runtime model-
    /// extraction carve-out is keyed to `runtime-model-extraction: true` declared
    /// HERE *and* a `model-extraction: true` curated command — both, or it's
    /// rejected (`E_APP_RUNTIME_MODEL_FORBIDDEN`). Keeps the exception narrow.
    #[serde(default)]
    pub capabilities: Option<Capabilities>,
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

/// Agent-level capability flags (RFC #223). Currently the single fenced
/// `runtime-model-extraction` flag the validator honors for `vision.extract`.
#[derive(Debug, Deserialize, Clone, Default)]
pub struct Capabilities {
    /// The agent may carry a curated `model-extraction: true` command that calls a
    /// model at run time (the `vision.extract` carve-out). Without this flag, a
    /// `model-extraction` command is rejected `E_APP_RUNTIME_MODEL_FORBIDDEN`.
    #[serde(rename = "runtime-model-extraction", default)]
    pub runtime_model_extraction: bool,
}

/// Whether an agent's transport is runnable today.
#[derive(Debug, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AgentStatus {
    /// Transport binary ships or is installable — the agent can be dispatched to.
    #[default]
    Available,
    /// Declared but not yet runnable (no shipped/installable transport binary).
    /// Apps referencing it fail validation/compile rather than at run time (#161).
    Planned,
    /// The implementation exists, but only in AWARE CLI versions at or above the
    /// manifest's `minimum-cli-version`. Older CLIs that predate this enum value
    /// reject the manifest during deserialization instead of misrouting it (#495).
    RequiresRuntime,
}

impl AgentStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::Planned => "planned",
            Self::RequiresRuntime => "requires-runtime",
        }
    }
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
    pub mcp: Option<Value>,
    pub rest: Option<Value>,
    /// `app` transport — present only on agent manifests synthesized from an
    /// `exposes-as-agent: true` app (see [`crate::manifest::expose`]). Dispatch
    /// runs the named backing app's node chain instead of spawning a binary.
    #[serde(default)]
    pub app: Option<TransportApp>,
    /// `builtin` transport (v0.56, #201) — a `_core` utility the runtime handles
    /// in-process, with no host binary to ship or install. Routed by agent id to a
    /// built-in handler (e.g. `html-report` → the generic HTML renderer). Carrying
    /// it as a present-but-empty block (`builtin: {}`) keeps the agent runnable
    /// (`status: available`) without a `cli`/`rest`/`app`/`mcp` transport.
    #[serde(default)]
    pub builtin: Option<Value>,
}

#[derive(Debug, Deserialize)]
pub struct TransportCli {
    pub binary: String,
}

/// The backing for an app-exposed-as-agent: names the installed app whose
/// composition runs when this synthesized agent is invoked.
#[derive(Debug, Deserialize, Clone)]
pub struct TransportApp {
    #[serde(rename = "backed-by")]
    pub backed_by: String,
}

#[derive(Debug, Deserialize)]
pub struct Command {
    pub lifecycle: Lifecycle,
    pub description: String,
    /// Per-command runnability (v0.57, #199). `available` (default) means the
    /// command is dispatchable; `planned` means it is declared but not yet wired
    /// (e.g. a REST agent's command awaiting a multi-step / binary implementation),
    /// so apps that reference it are rejected at validate/compile
    /// (`E_APP_COMMAND_UNAVAILABLE`) instead of failing at run. This is the
    /// command-level counterpart of the agent-level `status:` (#161): an agent can
    /// be partially runnable — some commands wired, some not.
    #[serde(default)]
    pub status: AgentStatus,
    #[serde(default)]
    pub inputs: Value,
    pub outputs: Option<Value>,
    /// REST operation mapping (v0.39, `--from-openapi`): the HTTP method this
    /// command performs. When set, the REST transport executes the command as
    /// `<method> <base><path>` rather than requiring the command name to be a
    /// bare HTTP method. `None` for the generic `http` agent + non-REST agents.
    #[serde(default)]
    pub method: Option<String>,
    /// Opt-in REST response handling. `artifact-stream` spools the successful
    /// response into a run-owned artifact rather than materializing JSON.
    #[serde(default)]
    pub response: Option<String>,
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
    /// Whether the effective read/write mode is *caller-determined* rather than
    /// fixed by the manifest. Set `true` for commands that run caller-supplied
    /// logic whose read/write behavior the agent cannot know — e.g. `exec`,
    /// which compiles and runs an arbitrary C# script. For such a command the
    /// manifest `mode:` is a conservative *default* (so an un-annotated node
    /// still falls under the safety contract), but an explicit node-level
    /// `mode:` in the app overrides it. For all other commands the manifest
    /// `mode:` is authoritative and a conflicting node-level `mode:` is rejected
    /// at validate-time (see `10-core/app-spec.md § Safety contract`). (#165)
    #[serde(rename = "mode-overridable", default)]
    pub mode_overridable: bool,
    /// Marks a command that performs a **runtime model extraction** — it calls a
    /// multimodal model at run time to turn bytes (image/PDF) into schema-bound JSON
    /// (RFC #223, the single fenced exception to decalog #9's no-LLM-in-run-path rule).
    /// This flag is honored ONLY on a `category: curated` command whose agent declares
    /// `capabilities.runtime-model-extraction: true`; anywhere else it is itself a
    /// validation error (`E_APP_RUNTIME_MODEL_FORBIDDEN`), so a reflected or hand-rolled
    /// command can never mint a model-reader. See `validate.rs::check_node_agents`.
    #[serde(rename = "model-extraction", default)]
    pub model_extraction: bool,
}

/// Read/write mode for a command — drives the safety-contract enforcement.
#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Mode {
    Read,
    Write,
}

impl Mode {
    /// Lowercase string form (`"read"` / `"write"`) — matches the YAML
    /// representation and the value recorded in the `.lock`.
    pub fn as_str(self) -> &'static str {
        match self {
            Mode::Read => "read",
            Mode::Write => "write",
        }
    }
}

/// Result of resolving a node's effective read/write mode against the command
/// it invokes — see [`Agent::effective_mode`]. `overridden` records whether an
/// explicit node-level `mode:` took precedence over the manifest because the
/// command is `mode-overridable` (e.g. `exec`), so callers can emit an accurate
/// compile note. (#165)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EffectiveMode {
    pub mode: Mode,
    pub overridden: bool,
}

#[derive(Debug, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Lifecycle {
    Start,
    Stop,
    Single,
}

impl Lifecycle {
    /// The manifest spelling of this variant — the exact string the
    /// `rename_all = "lowercase"` deserializer above accepts, so a value read
    /// from a manifest round-trips back to the same word.
    ///
    /// `manifest::expose` and `registry::catalog` each carried a private,
    /// byte-identical copy of this match. Both write the word back out at a
    /// boundary an author reads — the synthesized agent manifest and the
    /// registry catalog — so a rename in one copy alone would have silently
    /// desynchronised the two surfaces from each other and from the parser.
    pub fn as_str(self) -> &'static str {
        match self {
            Lifecycle::Start => "start",
            Lifecycle::Stop => "stop",
            Lifecycle::Single => "single",
        }
    }
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

    /// Resolve the effective read/write mode for a *node* invoking `cmd`.
    ///
    /// For a `mode-overridable` command — one whose behavior is caller-determined
    /// because it runs caller-supplied logic (e.g. `exec`) — an explicit
    /// node-level `mode:` wins over the manifest, and `overridden` is `true`.
    /// For every other command the manifest [`mode_of`](Self::mode_of) is
    /// authoritative; a node-level `mode:` does not change the result here (the
    /// validator rejects a *conflicting* declaration on a non-overridable
    /// command rather than silently dropping it). (#165)
    pub fn effective_mode(
        &self,
        name: &str,
        cmd: &Command,
        node_mode: Option<Mode>,
    ) -> EffectiveMode {
        match (cmd.mode_overridable, node_mode) {
            (true, Some(m)) => EffectiveMode {
                mode: m,
                overridden: true,
            },
            _ => EffectiveMode {
                mode: self.mode_of(name, cmd),
                overridden: false,
            },
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
        // Cut at 14 only once past 17, so a 15–17 char name shows in full rather
        // than losing three characters to gain an ellipsis.
        match crate::text::cut_after_chars(&raw, 17) {
            None => raw,
            Some(_) => {
                let mut s = crate::text::cut_after_chars(&raw, 14)
                    .unwrap_or(&raw)
                    .to_string();
                s.push_str("...");
                s
            }
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

    /// Parse one of the repo's real, shipped agent manifests. The paths are
    /// relative to the repo root, which is `cli/`'s parent.
    ///
    /// Four tests in this module spelled this walk out inline, byte for byte,
    /// before it was factored out; five call it now. Reading production manifests
    /// rather than fixtures is deliberate — several of the assertions in this file
    /// are claims about what the substrate actually ships, and a fixture cannot
    /// carry those.
    fn real_manifest(rel: &str) -> Agent {
        let path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join(rel);
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
        serde_yaml::from_str(&text)
            .unwrap_or_else(|e| panic!("cannot parse {}: {e}", path.display()))
    }

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
        assert_eq!(a.status, AgentStatus::Available);
        assert!(a.minimum_cli_version.is_none());
        assert_eq!(a.transport.cli.unwrap().binary, "aware-tekla");
    }

    #[test]
    fn parses_runtime_gated_manifest() {
        let yaml = TEKLA_MIN.replace(
            "stateful: true\n",
            "stateful: true\nstatus: requires-runtime\nminimum-cli-version: 0.136.0\n",
        );
        let a: Agent = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(a.status, AgentStatus::RequiresRuntime);
        assert_eq!(a.minimum_cli_version.as_deref(), Some("0.136.0"));
    }

    // `legacy_v0_135_status_enum_rejects_requires_runtime` stood here. It
    // declared its OWN two-variant `V0_135AgentStatus` inside the test body and
    // asserted that serde rejects `requires-runtime` against it. Nothing in that
    // test reached production code: the only thing it imported from `super` was
    // the `Deserialize` derive, so it was a test of serde's unknown-variant
    // handling, not of this module. Give the real `AgentStatus` an
    // `#[serde(alias = "requires-runtime")] Available` — the most complete
    // destruction of the property its name claims — and it still passed, because
    // the enum it checked was the one it had just written two lines up.
    //
    // The claim it was reaching for is about a SHIPPED v0.135.0 binary, which no
    // test in this tree can exercise; its own comment conceded that ("The release
    // test can additionally exercise the tagged binary"). The half that IS
    // testable here — that `AgentStatus` has no catch-all, so an older CLI meeting
    // a status it does not know refuses the manifest instead of misrouting it —
    // is asserted against the production type by
    // `an_unknown_status_value_is_refused_rather_than_absorbed` below.

    /// `AgentStatus` deliberately carries no `#[serde(other)]` arm: a CLI that
    /// predates a status value must fail to parse the manifest, not absorb the
    /// unknown word into a variant and then route the agent through a transport it
    /// cannot execute (#495). That is the whole mechanism by which
    /// `status: requires-runtime` is safe to publish to runtimes older than the
    /// one that introduced it.
    ///
    /// Asserted on `Agent` — the production type a real manifest is read into —
    /// rather than on a lookalike declared in the test.
    #[test]
    fn an_unknown_status_value_is_refused_rather_than_absorbed() {
        let yaml = TEKLA_MIN.replace(
            "stateful: true\n",
            "stateful: true\nstatus: supersedes-everything\n",
        );
        let err = serde_yaml::from_str::<Agent>(&yaml)
            .expect_err("an unrecognised status must not parse");
        assert!(err.to_string().contains("unknown variant"), "{err}");
        // The same manifest without the unknown value parses, so the error above
        // is the status and not some unrelated defect in `TEKLA_MIN`.
        serde_yaml::from_str::<Agent>(TEKLA_MIN).expect("the base fixture must parse");
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
        let a = real_manifest("20-agents/aeco/engineering/tekla/manifest.yaml");
        assert_eq!(a.agent, "tekla");
        // tekla is the gold-standard curated agent — currently 33 skills and
        // 26 commands. (Grew from 23 when the `bake-scene` verb landed in #235,
        // and from 24 when #520 declared `list-instances` and `close`, which the
        // bridge had always dispatched with nothing publishing them.)
        assert_eq!(a.skill_count(), 33);
        assert_eq!(a.command_count(), 26);
        assert!(a.stateful);
    }

    #[test]
    fn parses_real_trimble_connect_manifest_is_rest_with_auth() {
        // #196: trimble-connect must be a REST agent — no `cli` transport (which
        // the runtime selects first, routing every call to a never-built
        // `aware-trimble-connect` binary), a declared `auth:` block so the OAuth
        // token is attached, and read commands carrying `method`/`path` so the
        // REST transport can form an authenticated request.
        let a = real_manifest("20-agents/aeco/construction/trimble-connect/manifest.yaml");

        // REST-only: no CLI transport to mis-route to.
        assert!(
            a.transport.cli.is_none(),
            "must not declare a cli transport"
        );
        assert!(a.transport.rest.is_some(), "must declare a rest transport");

        // Declarative OAuth2 auth wired to the connect credential.
        let auth = a.auth.as_ref().expect("must declare an auth block");
        assert_eq!(auth.scheme, "oauth2");
        assert_eq!(auth.secret, "trimble-connect");

        // Read commands carry method + path; path placeholders use the input name
        // (`{folder-id}`) the REST transport substitutes from `in: path` inputs.
        let projects = a.commands.get("list-projects").expect("list-projects");
        assert_eq!(projects.method.as_deref(), Some("GET"));
        assert_eq!(projects.path.as_deref(), Some("/projects"));

        let folders = a.commands.get("list-folders").expect("list-folders");
        assert_eq!(folders.method.as_deref(), Some("GET"));
        assert_eq!(folders.path.as_deref(), Some("/folders/{folder-id}/items"));
    }

    #[test]
    fn tekla_commands_are_explicitly_curated() {
        // The Tekla curated agent is the gold-standard category: curated agent.
        // All 3 commands declare `category: curated` explicitly.
        let a = real_manifest("20-agents/aeco/engineering/tekla/manifest.yaml");
        // All tekla commands are explicitly `category: curated` (26 total).
        assert_eq!(a.curated_count(), 26);
        assert_eq!(a.reflected_count(), 0);
        for cmd in a.commands.values() {
            assert_eq!(cmd.category, Some(Category::Curated));
        }
    }

    #[test]
    fn revit_manifest_mixes_curated_and_reflected() {
        // The Revit-2026 manifest is machine-generated (provenance has
        // generated-by), so the inference rule makes unmarked commands
        // default to Reflected. The first wave of curated workflow verbs
        // explicitly carry `category: curated`.
        let a = real_manifest("20-agents/aeco/architecture/revit-2026/manifest.yaml");
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

    // ----------------------------------------------------------------------
    // The safety contract: how a node's read/write mode is resolved.
    //
    // `mode_of` / `is_write_by_convention` / `effective_mode` are what decide
    // whether `validate_app_safety` demands a `safety:` block from a node
    // (`E_APP_WRITE_WITHOUT_SAFETY`) — the structural guard the 2026-05-17
    // persona audit made a precondition for live-model writes
    // (`10-core/app-spec.md § Safety contract (write-mode nodes)`).
    //
    // Before the tests below, the convention was covered at exactly one point. Every
    // safety test in `validate.rs` that depends on a POSITIVE convention match
    // depends on the bare legacy name `insert`
    // (`safety_check_rejects_write_node_without_safety_block`,
    // `safety_check_passes_when_write_node_has_safety_block`); every other one
    // declares an explicit `mode:`, which short-circuits before the convention is
    // consulted at all. (`safety_check_skips_read_mode_nodes` does reach it, with the
    // command `list` — but it needs the answer to be Read, so no suffix deletion can
    // make it fail.) Not one of the twelve DOTTED suffixes was pinned anywhere: with
    // `".bump"` deleted from `SUFFIXES`, `cargo test -- --skip manifest::agent::tests`
    // is 2070 passed, 0 failed, and the same holds for `".delete"`. The asymmetry is
    // structural rather than an oversight — loosening the list only ever REMOVES a
    // safety error, and every integration test over an example app asserts that the
    // app is valid, so only a refusal assertion can catch it.
    //
    // `SUFFIXES` is a fn-local const with exactly one reader, `is_write_by_convention`,
    // itself called only from `mode_of` — so these tests and the `validate.rs` gate
    // test added alongside them are the whole of its coverage.
    // ----------------------------------------------------------------------

    /// Build a `Command` from YAML rather than field-by-field. `Command` is
    /// `Deserialize`-only, and going through the parser keeps these assertions
    /// pointed at `mode_of` instead of at a struct the test just filled in.
    ///
    /// One hazard that comes with the parser: `Command` declares no
    /// `deny_unknown_fields`, so a misspelled key here (`mode-overridible:`) is
    /// ignored rather than rejected and yields a fixture that is not the one the
    /// caller named. Callers whose behaviour turns on a key assert that the key
    /// landed — see `effective_mode_records_whether_the_node_overrode_the_manifest`.
    fn command(extra: &str) -> Command {
        serde_yaml::from_str(&format!("lifecycle: single\ndescription: x\n{extra}"))
            .unwrap_or_else(|e| panic!("bad command fixture {extra:?}: {e}"))
    }

    /// Any `Agent` will do — `mode_of` reads only its arguments — so reuse the
    /// minimal fixture rather than inventing a second one per test.
    fn tekla_min() -> Agent {
        serde_yaml::from_str(TEKLA_MIN).expect("TEKLA_MIN must parse")
    }

    /// Seven commands the substrate ships **today** that declare no `mode:` of
    /// their own, each resting on a different entry of `is_write_by_convention`
    /// for its write-mode classification — and therefore for the `safety:` block
    /// every node calling them must carry.
    ///
    /// Read from the production manifests, not fixtures, because the claim is
    /// about what is shipped: delete `".bump"` from `SUFFIXES` and
    /// `revit-2026 revision.bump` becomes a read, so an app may add a revision
    /// letter to a user's model — `Document.AddRevision`, locking the prior
    /// revision — with no transaction group, no snapshot and no worksharing
    /// pre-flight. Its own description says it is "WRITE-mode against the Revit
    /// model — requires `safety:` block"; nothing but this convention makes that
    /// claim true.
    ///
    /// The `mode.is_none()` pre-check on each row is load-bearing rather than
    /// decoration. Were a manifest to annotate one of these commands explicitly
    /// later, the `mode_of` assertion would still pass — through the explicit
    /// branch, proving nothing about the convention. The pre-check turns that
    /// into a red test naming the row to re-ground on a still-unannotated
    /// command.
    #[test]
    fn shipped_write_commands_with_no_declared_mode_are_inferred_from_their_names() {
        // (manifest, [(command, the convention entry it rests on)])
        const GROUNDED: &[(&str, &[(&str, &str)])] = &[
            (
                "20-agents/aeco/architecture/revit-2026/manifest.yaml",
                &[
                    ("revision.bump", ".bump"),
                    ("sheet.stamp", ".stamp"),
                    ("link.reload-all", ".reload-all"),
                    ("parameter.bulk-write", ".bulk-write"),
                ],
            ),
            (
                "20-agents/aeco/engineering/tekla/manifest.yaml",
                // The bare legacy name — the Tekla curated verbs predate the
                // dotted namespace, so `save-attributes` is matched by exact
                // string and by nothing else.
                &[("save-attributes", "save-attributes")],
            ),
            (
                "20-agents/aeco/construction/navisworks/manifest.yaml",
                &[("clash.export", ".export")],
            ),
            (
                "20-agents/aeco/architecture/autocad-2026/manifest.yaml",
                &[("layout.export-pdfs", ".export-pdfs")],
            ),
        ];

        // The doc comment above says "Seven commands"; this is what keeps that true.
        // The table is a compile-time literal, so it cannot be empty at run time —
        // but the `mode.is_none()` guard below tells a maintainer to re-ground a row,
        // and a re-grounding that empties a `rows` slice would pass vacuously for that
        // manifest with nothing noticing.
        assert_eq!(
            GROUNDED.iter().map(|(_, rows)| rows.len()).sum::<usize>(),
            7,
            "the grounded table changed size — update the count in this test's doc \
             comment too, and check no manifest was left with zero rows"
        );
        for (rel, rows) in GROUNDED {
            assert!(!rows.is_empty(), "{rel} has no rows left to ground");
            // One parse per manifest — `revit-2026` is 1.1 MB of YAML.
            let agent = real_manifest(rel);
            for (name, entry) in *rows {
                let cmd = agent
                    .commands
                    .get(*name)
                    .unwrap_or_else(|| panic!("{rel} no longer declares {name}"));
                assert!(
                    cmd.mode.is_none(),
                    "{rel}:{name} now declares an explicit `mode:`. That is a fine \
                     change to the manifest — belt and braces — but it means this row \
                     no longer grounds the convention entry {entry:?}, which would \
                     then be pinned only synthetically. Re-ground the row on a command \
                     that still declares no `mode:`, or drop it; do not revert the \
                     manifest."
                );
                assert_eq!(
                    agent.mode_of(name, cmd),
                    Mode::Write,
                    "{rel}:{name} must be write-mode by the {entry:?} convention"
                );
            }
        }
    }

    /// `10-core/app-spec.md § Safety contract` publishes the convention to app
    /// authors, naming eight suffixes. Exactly four of those eight — `.create`,
    /// `.update`, `.delete`, `.insert` — have no shipped un-annotated command to
    /// ground them in the test above: every shipped `.create`, `.update` and
    /// `.delete` command declares `mode: write` outright, and no shipped command
    /// ends in `.insert` at all. So this reads the promise out of the spec and
    /// holds the implementation to it. Two artifacts, one claim: deleting a suffix
    /// from `SUFFIXES` while the spec still advertises it goes red here.
    ///
    /// Not a restatement of `SUFFIXES` — the spec is the primary source and the
    /// list is extracted from it, so this test cannot be satisfied by editing the
    /// code it checks. Note the gap it leaves. The implementation carries four
    /// entries the spec does not name at all (`.save`, `.publish`, `.export`,
    /// `.export-pdfs`); `.export` and `.export-pdfs` are grounded on real
    /// manifests above, but `.save` and `.publish` are pinned by nothing, because
    /// the one shipped `.save` command declares `mode: write` outright and no
    /// shipped command ends in `.publish`. Widening the spec to name the four, or
    /// narrowing the code to the eight, is a decision about the contract and not
    /// a test's to make.
    #[test]
    fn every_suffix_the_app_spec_publishes_is_inferred_as_a_write() {
        let spec = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("10-core/app-spec.md");
        let text = std::fs::read_to_string(&spec)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", spec.display()));

        // The bullet that states the rule, located by its prose rather than by line
        // number so an edit elsewhere in the spec cannot silently move it.
        const MARKER: &str = "the command's name conventionally implies a write";
        let lines: Vec<&str> = text.lines().collect();
        let first = lines
            .iter()
            .position(|l| l.contains(MARKER))
            .unwrap_or_else(|| {
                panic!(
                    "{} no longer contains {MARKER:?} — the safety contract was reworded, so \
                     re-ground this test on the new wording rather than deleting it",
                    spec.display()
                )
            });

        // The WHOLE bullet, not just the line the marker fell on. Reading one line
        // looks equivalent and is not: the list is long enough that ordinary markdown
        // reflow would push its tail onto a continuation line, and a suffix the spec
        // publishes there would then be skipped in silence — spec promising a write,
        // code resolving a read, test green. Demonstrated on this branch with
        // `*.purge`, `*.truncate` added on a second line of the same bullet.
        //
        // A markdown bullet continues while the following lines are indented
        // continuations: non-empty, and not the start of a new bullet, heading or
        // fence.
        let bullet: String = std::iter::once(lines[first])
            .chain(
                lines[first + 1..]
                    .iter()
                    .take_while(|l| {
                        let t = l.trim_start();
                        !t.is_empty()
                            && !t.starts_with('-')
                            && !t.starts_with('#')
                            && !t.starts_with("```")
                    })
                    .copied(),
            )
            .collect::<Vec<_>>()
            .join(" ");

        // Every `` `*.<something>` `` token in that bullet.
        let published: Vec<&str> = bullet
            .split('`')
            .filter(|tok| tok.starts_with("*."))
            .map(|tok| &tok[1..]) // `*.create` -> `.create`
            .collect();

        // Separate the two things that can go wrong, because they want different
        // remedies. First: did extraction work at all? A reword that drops the
        // backticks, or moves the list into a sub-list, yields nothing — and an empty
        // list leaves the loop below iterating over nothing and the test green, which
        // is the exact vacuity this file is being audited for.
        assert!(
            !published.is_empty(),
            "extracted no `*.<suffix>` tokens from the spec's contract bullet, so the \
             loop below would assert nothing. The bullet's formatting changed — fix the \
             extraction, do not delete the test. Bullet: {bullet}"
        );

        // Second: the spec and the code have to name the same things, and drift in
        // EITHER direction is a finding. A floor on the COUNT catches neither, and the
        // gap is reachable rather than theoretical: swap `*.insert` for `*.export` in
        // the bullet — a plausible edit, since `.export` is implemented and no shipped
        // command ends in `.insert` — and the count stays eight, this test's own claim
        // stays true, and `.insert` quietly loses the only coverage it has anywhere in
        // the repo. Drop `".insert"` from `SUFFIXES` after that and the whole suite is
        // green while any `*.insert` command resolves read and stops needing a
        // `safety:` block. Measured on this branch before the check below was added.
        //
        // So the SET is asserted. This is still not a restatement of `SUFFIXES` — it
        // restates the spec's published commitment, and the loop below restates the
        // code's, so each artifact checks the other.
        for want in [
            ".create",
            ".update",
            ".delete",
            ".bump",
            ".stamp",
            ".reload-all",
            ".bulk-write",
            ".insert",
        ] {
            assert!(
                published.contains(&want),
                "app-spec.md § Safety contract no longer names {want} among the suffixes \
                 that imply a write (found {published:?}). The spec narrowed while the \
                 code still implements {want} — that may well be the right call, but \
                 decide it in both places: drop the entry from `SUFFIXES` and from this \
                 list, or put it back in the spec. Bullet: {bullet}"
            );
        }

        let agent = tekla_min();
        for suffix in &published {
            let name = format!("thing{suffix}");
            assert_eq!(
                agent.mode_of(&name, &command("")),
                Mode::Write,
                "app-spec.md promises {suffix} implies a write, but {name} resolved read"
            );
        }
    }

    /// The four entries `SUFFIXES` carries that `10-core/app-spec.md § Safety
    /// contract` does not publish. The spec-parity test above covers the eight it
    /// names; nothing covered these, so dropping one was invisible.
    ///
    /// This is a pin on a security-relevant list, and yes, it mirrors the constant —
    /// which is exactly why it is written as the consequence rather than as the list.
    /// `.export` and `.export-pdfs` are additionally grounded on shipped
    /// un-annotated commands by
    /// `shipped_write_commands_with_no_declared_mode_are_inferred_from_their_names`;
    /// `.save` and `.publish` have no shipped un-annotated command to ground them (the
    /// one shipped `.save`, `navisworks federation.save`, declares `mode: write`
    /// outright, and no shipped command ends in `.publish` at all), so this is their
    /// only coverage and a silent deletion of `".save"` would make the next
    /// `model.save` command a read.
    ///
    /// Whether the right resolution is to widen the spec to name these four or to
    /// narrow the code to the spec's eight is a decision about the published contract,
    /// not a test's to make. Until someone makes it, today's behaviour is what app
    /// authors get, so today's behaviour is what is pinned.
    #[test]
    fn the_suffixes_the_code_adds_beyond_the_spec_are_pinned_too() {
        let agent = tekla_min();
        for suffix in [".save", ".publish", ".export", ".export-pdfs"] {
            let name = format!("thing{suffix}");
            assert_eq!(
                agent.mode_of(&name, &command("")),
                Mode::Write,
                "{name} resolved read: `is_write_by_convention` no longer carries \
                 {suffix:?}. If that removal was deliberate, app-spec.md § Safety \
                 contract and this test both want updating — a command named {name} \
                 now needs no `safety:` block."
            );
        }
    }

    /// The convention matches a whole trailing name, never a substring. Each
    /// negative below is a name a real agent could plausibly carry, and each one
    /// is what a specific loosening of `is_write_by_convention` would get wrong:
    /// `ends_with` → `contains` lets `thing.deleted` through, and the two exact
    /// legacy comparisons → `starts_with` lets `insert-many` through.
    ///
    /// The positive controls are in the same test on purpose: a `mode_of` that
    /// had regressed to answering `Read` unconditionally would satisfy every
    /// negative here and nothing else in this test file would notice.
    #[test]
    fn the_convention_matches_a_whole_trailing_name_not_a_substring() {
        let agent = tekla_min();
        let cmd = command("");
        let mode = |name: &str| agent.mode_of(name, &cmd);

        for name in [
            "thing.deleted", // suffix must END the name
            "delete.thing",  // ...and be at the end, not the start
            "insert-many",   // the legacy names match exactly, not by prefix
            "save-attributes-v2",
            "thing.exports",
            // A bare `create` is NOT inferred as a write: the only two names
            // matched without a dot are the two legacy Tekla verbs. This is why
            // `10-core/app-spec.md` has the manifest `mode:` be authoritative —
            // an agent author who names a write command without the dotted
            // namespace has to declare it, and `validate.rs`'s
            // `safety_check_honors_explicit_mode_write_field` covers that path.
            "create",
            "sheet.list",
        ] {
            assert_eq!(
                mode(name),
                Mode::Read,
                "{name} must not be inferred a write"
            );
        }

        // Each positive names the arm of `is_write_by_convention` that owns it, so a
        // failure points at the right half of the function.
        for (name, arm) in [
            ("thing.delete", "the `.delete` entry in SUFFIXES"),
            ("insert", "the legacy exact-match `name == \"insert\"`"),
            (
                "save-attributes",
                "the legacy exact-match `name == \"save-attributes\"`",
            ),
        ] {
            assert_eq!(
                mode(name),
                Mode::Write,
                "{name} must be inferred a write via {arm}; a read here drops the \
                 `safety:` requirement from every node calling it"
            );
        }
    }

    /// `effective_mode`'s truth table over (`mode-overridable`, node-level
    /// `mode:`), including the `overridden` flag that `app_lock::compile_node`
    /// reads to decide whether the lock carries the "using author-declared mode"
    /// compile note.
    ///
    /// Three of the four rows were already covered end-to-end and are kept here as
    /// a single readable table rather than as new coverage: the overridable rows via
    /// `validate.rs`'s `safety_check_passes_read_override_on_mode_overridable_exec`
    /// and `..._still_requires_safety_on_unannotated_mode_overridable_exec`, and
    /// `overridden == true` via `app_lock.rs`'s
    /// `compile_honors_mode_read_on_mode_overridable_command`, which asserts the note
    /// that is emitted only under `if resolved.overridden`. What is new is the
    /// `(overridable, Some(Write))` row — the spec's "(A node declaring `mode: write`
    /// likewise wins.)" (`10-core/app-spec.md:817`) — and the assertion on
    /// `EffectiveMode` as a value rather than on a note's text downstream of it.
    ///
    /// The `fixed` + node-`read` row is defence in depth, not a reachable path: an app
    /// shaped like it is refused by `validate_app_agents` with
    /// `E_APP_NODE_MODE_NOT_OVERRIDABLE` on every lock-producing path
    /// (`validate.rs:1381`, covered by
    /// `agents_check_rejects_conflicting_node_mode_on_non_overridable_command`), so it
    /// never reaches here in production. It is pinned because `validate_app_safety`
    /// and `app_lock::compile_node` ask THIS function what the mode is, and a
    /// loosening here would silently make their answer depend on a check living
    /// somewhere else.
    #[test]
    fn effective_mode_records_whether_the_node_overrode_the_manifest() {
        let agent = tekla_min();
        let overridable = command("mode: write\nmode-overridable: true\n");
        let fixed = command("mode: write\n");
        let read_by_name = command("");
        // `Command` ignores unknown keys, so a typo in the fixture above would
        // silently produce a NON-overridable command and quietly turn the first three
        // rows into copies of the fourth. Check the fixtures are what they claim.
        assert!(
            overridable.mode_overridable,
            "fixture key did not land: the `overridable` command is not mode-overridable"
        );
        assert!(
            !fixed.mode_overridable && fixed.mode == Some(Mode::Write),
            "fixture `fixed` must be a non-overridable write"
        );
        assert!(
            read_by_name.mode.is_none(),
            "fixture `read_by_name` must declare no mode, so the convention decides"
        );

        // The command name cannot affect the rows below — both fixtures they use
        // declare an explicit `mode:`, which short-circuits the convention — so it
        // is spelled `exec` after the canonical caller-determined command the spec
        // uses to introduce `mode-overridable`.
        let cases = [
            // (command, node-level mode, expected mode, expected `overridden`)
            (&overridable, Some(Mode::Read), Mode::Read, true),
            (&overridable, Some(Mode::Write), Mode::Write, true),
            (&overridable, None, Mode::Write, false),
            (&fixed, Some(Mode::Read), Mode::Write, false),
        ];
        for (cmd, node_mode, mode, overridden) in cases {
            assert_eq!(
                agent.effective_mode("exec", cmd, node_mode),
                EffectiveMode { mode, overridden },
                "node_mode {node_mode:?} on mode-overridable={}",
                cmd.mode_overridable
            );
        }

        // And with no manifest `mode:` at all the convention still feeds through
        // `effective_mode`, never only `mode_of` — `validate_app_safety` and
        // `app_lock::compile_node` both call `effective_mode`, so a convention match
        // that stopped reaching this arm would un-gate every un-annotated write
        // command.
        assert_eq!(
            agent.effective_mode("sheet.stamp", &read_by_name, None),
            EffectiveMode {
                mode: Mode::Write,
                overridden: false
            },
            "an un-annotated `.stamp` command must reach `effective_mode` as a write \
             (is_write_by_convention via mode_of); a read here means every node calling \
             revit-2026 sheet.stamp stops needing a `safety:` block"
        );
    }

    /// Each of these three enums has a hand-written `as_str` that writes a word back
    /// out at a boundary somebody reads or re-parses: `Lifecycle` into the synthesized
    /// agent manifest (`manifest::expose:101`) and the published registry catalog
    /// (`registry::catalog:254`); `Mode` into the `.lock` a node's mode is recorded in
    /// (`app_lock:570`), the catalog (`registry::catalog:257`) and a validator message
    /// (`validate:1391`); `AgentStatus` into `agent describe --json`
    /// (`commands::agent:1186` and `:1198`) and the catalog (`registry::catalog:216`),
    /// where `describe_from_catalog` then `match`es the word against string literals
    /// (`commands::agent:2322`). Not `app show` — `commands::app:1288` matches a
    /// `RunEvent::RunEnd` status, which is an unrelated type that happens to share the
    /// field name.
    ///
    /// Six of the eight variant rows were already pinned incidentally, and saying
    /// otherwise would be wrong: both `Mode` words via `app_lock.rs`'s
    /// `mode_axis_agents`, which feeds `mode: read` / `mode: write` through serde and
    /// asserts the lock string `Mode::as_str` produced; all three `AgentStatus` words
    /// via `registry/catalog.rs`'s `status_str` (a thin delegation to `as_str`), whose
    /// tests feed `status: planned` / `requires-runtime` and assert the published
    /// word; and `Lifecycle::Single` via the catalog's JSON golden. What was unpinned
    /// is `Lifecycle::Start` and `Lifecycle::Stop` — `expose.rs` uses those words only
    /// as inputs and nothing asserted the emitted word — even though
    /// `Lifecycle::as_str`'s own doc comment states the round-trip property outright
    /// ("a value read from a manifest round-trips back to the same word") and names
    /// the two surfaces that would desynchronise.
    ///
    /// So the standing value of this test is less the eight rows than the two gates
    /// under them — the exhaustiveness-checked variant walk and the published-word
    /// table it is length-checked against. Those incidental pins are per-variant and
    /// per-surface; none of them notices a variant that is ADDED, and none of them
    /// catches a rename that moves the serde word and the `as_str` arm together.
    ///
    /// This is not a serde round-trip: the value under test is the `&'static str`
    /// a `match` produced, and serde is the independent oracle it is checked
    /// against.
    ///
    /// The variants are walked through the successor chains below rather than
    /// listed in an array literal, so a variant added to one of these enums
    /// cannot slip past this test — see [`variants`] for why that distinction is
    /// load-bearing (Codex review, PR #610).
    #[test]
    fn each_enum_writes_back_the_word_its_own_parser_reads() {
        check_published_words(
            variants(Lifecycle::Start, next_lifecycle),
            Lifecycle::as_str,
            &[
                (Lifecycle::Start, "start"),
                (Lifecycle::Stop, "stop"),
                (Lifecycle::Single, "single"),
            ],
            "Lifecycle",
        );
        check_published_words(
            variants(Mode::Read, next_mode),
            Mode::as_str,
            &[(Mode::Read, "read"), (Mode::Write, "write")],
            "Mode",
        );
        check_published_words(
            variants(AgentStatus::Available, next_agent_status),
            AgentStatus::as_str,
            &[
                (AgentStatus::Available, "available"),
                (AgentStatus::Planned, "planned"),
                (AgentStatus::RequiresRuntime, "requires-runtime"),
            ],
            "AgentStatus",
        );
    }

    /// Hold one enum's `as_str` to three properties at once, for every variant the
    /// exhaustiveness-gated walk finds.
    ///
    /// 1. **The word is the published literal.** Serde is an independent oracle for a
    ///    one-sided typo but not for a rename that moves the serde word and the
    ///    `as_str` arm together: that round-trips perfectly and still breaks every
    ///    consumer that `match`es the literal — `commands::agent:2322`,
    ///    `registry::catalog:111`. These words are a wire vocabulary (the published
    ///    registry catalog, the `.lock`, `agent describe --json`), so they are spelled
    ///    out here the way a golden test spells out a wire format.
    /// 2. **The word round-trips.** `as_str`'s output fed back through the derive must
    ///    return the same variant, which is what `Lifecycle::as_str`'s own doc comment
    ///    promises.
    /// 3. **The table is complete.** The length check against the walk is what stops a
    ///    newly added variant from being given an `as_str` arm and no published word:
    ///    the walk cannot miss it (the successor `match` is exhaustive, so the module
    ///    stops compiling), and the table cannot then stay short.
    fn check_published_words<T>(
        walked: Vec<T>,
        as_str: fn(T) -> &'static str,
        expected: &[(T, &str)],
        ty: &str,
    ) where
        T: Copy + PartialEq + std::fmt::Debug + serde::de::DeserializeOwned,
    {
        assert_eq!(
            walked.len(),
            expected.len(),
            "{ty}: the variant walk found {:?} but the published-word table lists {} \
             entries — a variant was added without deciding the word it writes to the \
             catalog, the `.lock` and `--json`",
            walked,
            expected.len()
        );
        for v in walked {
            let (_, want) = expected
                .iter()
                .find(|(e, _)| *e == v)
                .unwrap_or_else(|| panic!("{ty}::{v:?} has no entry in the published-word table"));
            let got = as_str(v);
            assert_eq!(
                got, *want,
                "{ty}::{v:?} writes {got:?}, but {want:?} is the published word that \
                 consumers match as a string literal"
            );
            assert_eq!(
                serde_yaml::from_str::<T>(got).unwrap_or_else(|e| panic!(
                    "{ty}::as_str produced {got:?}, which its own parser rejects: {e}"
                )),
                v,
                "{ty}::{v:?} writes {got:?}, which parses back as a different variant"
            );
        }
    }

    /// Collect an enum's variants by walking a successor function from its first
    /// variant.
    ///
    /// The point is the *compile-time* gate the successor functions below carry,
    /// which an array literal cannot. `[Lifecycle::Start, Lifecycle::Stop,
    /// Lifecycle::Single]` — which is what stood here — stays perfectly valid
    /// when a fourth variant is added, so the new variant's `as_str` would never
    /// be checked and
    /// `each_enum_writes_back_the_word_its_own_parser_reads` would stay green
    /// while doing none of its job (Codex review, PR #610). Each `next_*` below
    /// is an exhaustive `match`, so the same addition stops this module compiling
    /// until the variant is wired into the chain the test walks.
    ///
    /// One residual hole, named rather than hidden: the compile error can also be
    /// resolved by giving the new variant its own `=> None` arm beside the
    /// existing terminator, which satisfies the compiler and orphans the variant
    /// from the walk. Each chain is therefore written in declaration order with
    /// the last-declared variant as the sole terminator, so appending a variant
    /// makes the correct edit the obvious one — but no `match` can force it.
    ///
    /// The cycle check is what keeps a mis-wired chain a red test rather than a
    /// hung one: `successors` on a chain that loops back never terminates, and
    /// libtest has no per-test timeout.
    fn variants<T>(first: T, next: fn(T) -> Option<T>) -> Vec<T>
    where
        T: Copy + PartialEq + std::fmt::Debug,
    {
        let mut out = vec![first];
        let mut cur = first;
        while let Some(v) = next(cur) {
            assert!(
                !out.contains(&v),
                "the successor chain cycles at {v:?} after {out:?}; it must visit each \
                 variant once and terminate on the last"
            );
            out.push(v);
            cur = v;
        }
        out
    }

    fn next_lifecycle(v: Lifecycle) -> Option<Lifecycle> {
        match v {
            Lifecycle::Start => Some(Lifecycle::Stop),
            Lifecycle::Stop => Some(Lifecycle::Single),
            Lifecycle::Single => None,
        }
    }

    fn next_mode(v: Mode) -> Option<Mode> {
        match v {
            Mode::Read => Some(Mode::Write),
            Mode::Write => None,
        }
    }

    fn next_agent_status(v: AgentStatus) -> Option<AgentStatus> {
        match v {
            AgentStatus::Available => Some(AgentStatus::Planned),
            AgentStatus::Planned => Some(AgentStatus::RequiresRuntime),
            AgentStatus::RequiresRuntime => None,
        }
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
