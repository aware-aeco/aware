//! The agent `probe:` block (#617) — one declared, safe read that proves a
//! connection reaches the real host or account.
//!
//! ```yaml
//! probe:
//!   command: model-info        # own effective mode read; never mode-overridable
//!   inputs: {}                 # literal values only — no templating
//!   describe: "Reads the name of the model open in Tekla Structures."
//!   kind: host                 # host | account
//!   rest:                      # rest transport only, and required there
//!     origin: https://openidconnect.googleapis.com
//!   reports:                   # RFC 6901 pointers into the transport's result
//!     summary: /model_name
//!     identity: /host
//!     stable-id: /host_pid
//!     host-version: /host_version
//! ```
//!
//! The grammar is CLOSED: an unknown key anywhere in the block is a refusal, not
//! something to ignore, because a probe is the one place a manifest asks AWARE to
//! run a command with nobody composing it. [`parse_probe`] is the only reader;
//! `validate_agent` (agent validate, both install routes) and `aware agent probe`
//! all go through it, so the three can never disagree about what is valid.
//!
//! What the block cannot do is make itself trusted. Whether a probe is
//! *reviewed* is decided outside the manifest, from the installed bundle's digest
//! against the official registry index (`install::provenance`).

use serde::Deserialize;
use serde_json::Value as Json;
use serde_yaml::Value as Yaml;

use crate::manifest::agent::{Agent, AgentStatus, Lifecycle, Mode};
use crate::runtime::invoker::{TransportKind, dispatch_transport};

/// Longest plain-English `describe` sentence.
pub(crate) const MAX_DESCRIBE_CHARS: usize = 160;
/// Longest RFC 6901 pointer a report may name.
const MAX_POINTER_CHARS: usize = 256;

/// What a probe proves: that a desktop/host process answers, or that a signed-in
/// account answers (and then it must say WHICH account).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, serde::Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum ProbeKind {
    Host,
    Account,
}

impl ProbeKind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            ProbeKind::Host => "host",
            ProbeKind::Account => "account",
        }
    }
}

/// The declared report pointers, each an RFC 6901 pointer into the transport's
/// actual result: the bridge's JSON receipt for `cli`, and `{status, headers,
/// body}` for `rest`.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct ProbeReports {
    pub(crate) summary: Option<String>,
    pub(crate) identity: Option<String>,
    pub(crate) stable_id: Option<String>,
    pub(crate) host_version: Option<String>,
}

/// A probe block that passed the closed grammar and every rule against the
/// command it names.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ProbeDecl {
    pub(crate) command: String,
    /// Literal inputs, already converted to the JSON the transports send.
    pub(crate) inputs: serde_json::Map<String, Json>,
    pub(crate) describe: String,
    pub(crate) kind: ProbeKind,
    /// `rest.origin`, exactly as declared (it must already be in normal form).
    pub(crate) rest_origin: Option<String>,
    pub(crate) reports: ProbeReports,
    /// The transport the command dispatches on — `cli` or `rest`.
    pub(crate) transport: TransportKind,
}

/// Why a probe block is not usable. `reason` is a stable kebab-case code (it is
/// what `aware agent probe` puts in `error.details.reason`); `message` is for a
/// human reading `aware agent validate`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ProbeIssue {
    pub(crate) reason: &'static str,
    pub(crate) message: String,
}

impl ProbeIssue {
    fn new(reason: &'static str, message: impl Into<String>) -> Self {
        Self {
            reason,
            message: message.into(),
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawProbe {
    command: String,
    #[serde(default)]
    inputs: Option<Yaml>,
    describe: String,
    kind: ProbeKind,
    #[serde(default)]
    rest: Option<RawRest>,
    #[serde(default)]
    reports: Option<RawReports>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawRest {
    origin: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RawReports {
    #[serde(default)]
    summary: Option<String>,
    #[serde(default)]
    identity: Option<String>,
    #[serde(rename = "stable-id", default)]
    stable_id: Option<String>,
    #[serde(rename = "host-version", default)]
    host_version: Option<String>,
}

/// Parse and validate an agent's `probe:` block.
///
/// `Ok(None)` — the agent declares no probe. `Err` — it declares one that is not
/// usable, with the first rule it broke.
pub(crate) fn parse_probe(agent: &Agent) -> Result<Option<ProbeDecl>, ProbeIssue> {
    let Some(raw) = agent.probe.as_ref() else {
        return Ok(None);
    };
    if raw.is_null() {
        return Ok(None);
    }
    let raw: RawProbe = serde_yaml::from_value(raw.clone()).map_err(|e| {
        ProbeIssue::new(
            "malformed",
            format!("probe block does not match the closed probe grammar: {e}"),
        )
    })?;

    // ── command ──────────────────────────────────────────────────────────────
    let Some(command) = agent.commands.get(&raw.command) else {
        return Err(ProbeIssue::new(
            "command-unknown",
            format!(
                "probe names command {:?}, which the agent does not declare",
                raw.command
            ),
        ));
    };
    if command.status != AgentStatus::Available {
        return Err(ProbeIssue::new(
            "command-not-available",
            format!(
                "probe command {:?} is {}, not runnable",
                raw.command,
                command.status.as_str()
            ),
        ));
    }
    if command.lifecycle != Lifecycle::Single {
        return Err(ProbeIssue::new(
            "command-not-single",
            format!(
                "probe command {:?} has lifecycle {}; a probe must be one request/response call (lifecycle: single)",
                raw.command,
                command.lifecycle.as_str()
            ),
        ));
    }
    if command.mode_overridable {
        return Err(ProbeIssue::new(
            "command-mode-overridable",
            format!(
                "probe command {:?} is mode-overridable — its read/write behavior is decided by the caller's input, so it proves nothing about what a probe does",
                raw.command
            ),
        ));
    }
    if agent.mode_of(&raw.command, command) != Mode::Read {
        return Err(ProbeIssue::new(
            "command-not-read",
            format!(
                "probe command {:?} is write-mode; a probe may only read",
                raw.command
            ),
        ));
    }
    if command.model_extraction {
        return Err(ProbeIssue::new(
            "command-model-extraction",
            format!(
                "probe command {:?} calls a model at run time; a probe must be a deterministic read",
                raw.command
            ),
        ));
    }
    if command.response.is_some() {
        return Err(ProbeIssue::new(
            "command-streams-artifact",
            format!(
                "probe command {:?} streams its response to an artifact; a probe reads one bounded response",
                raw.command
            ),
        ));
    }

    // ── transport + rest.origin ──────────────────────────────────────────────
    let transport = match dispatch_transport(&agent.transport) {
        Some(kind @ (TransportKind::Cli | TransportKind::Rest)) => kind,
        Some(other) => {
            return Err(ProbeIssue::new(
                "transport-unsupported",
                format!(
                    "agent dispatches on the `{}` transport; a probe is supported on `cli` and `rest` only",
                    other.as_str()
                ),
            ));
        }
        None => {
            return Err(ProbeIssue::new(
                "transport-unsupported",
                "agent has no dispatchable transport",
            ));
        }
    };
    let rest_origin = match (transport, raw.rest) {
        (TransportKind::Rest, None) => {
            return Err(ProbeIssue::new(
                "rest-origin-required",
                "a probe on a rest agent must declare `rest: { origin: https://<host> }`",
            ));
        }
        (TransportKind::Rest, Some(rest)) => {
            if !is_normal_origin(&rest.origin) {
                return Err(ProbeIssue::new(
                    "rest-origin-invalid",
                    format!(
                        "probe rest.origin {:?} is not an exact origin — write it as `https://<host>`: https, no port, no userinfo, no path, no trailing slash",
                        rest.origin
                    ),
                ));
            }
            if command.method.is_none() {
                return Err(ProbeIssue::new(
                    "rest-method-missing",
                    format!(
                        "probe command {:?} declares no `method:`/`path:`, so its request URL cannot be fixed by the manifest",
                        raw.command
                    ),
                ));
            }
            // The origin is checked again on the URL actually assembled at probe
            // time. Checking the template here means a manifest whose path and
            // declared origin disagree is refused at validate/install, not first
            // discovered when someone presses "check it works".
            let base = agent
                .transport
                .rest
                .as_ref()
                .and_then(|r| r.get("base"))
                .and_then(|b| b.as_str());
            let template =
                crate::runtime::invoker::resolve_url(base, command.path.as_deref().unwrap_or(""));
            let filled = fill_path_placeholders(&template);
            if origin_of(&filled).as_deref() != Some(rest.origin.as_str()) {
                return Err(ProbeIssue::new(
                    "rest-origin-mismatch",
                    format!(
                        "probe command {:?} resolves to {template:?}, whose origin is not the declared rest.origin {:?}",
                        raw.command, rest.origin
                    ),
                ));
            }
            Some(rest.origin)
        }
        (_, Some(_)) => {
            return Err(ProbeIssue::new(
                "rest-origin-forbidden",
                "`rest:` is only meaningful for a probe on a rest agent",
            ));
        }
        (_, None) => None,
    };

    // ── inputs ───────────────────────────────────────────────────────────────
    let inputs = parse_inputs(raw.inputs, &command.inputs, &raw.command)?;

    // ── describe ─────────────────────────────────────────────────────────────
    let describe = raw.describe.trim().to_string();
    if describe.is_empty()
        || describe.chars().count() > MAX_DESCRIBE_CHARS
        || describe.chars().any(char::is_control)
    {
        return Err(ProbeIssue::new(
            "describe-invalid",
            format!(
                "probe describe must be one plain-English sentence of 1..={MAX_DESCRIBE_CHARS} characters with no control characters"
            ),
        ));
    }

    // ── reports ──────────────────────────────────────────────────────────────
    let reports = match raw.reports {
        None => ProbeReports::default(),
        Some(r) => ProbeReports {
            summary: r.summary,
            identity: r.identity,
            stable_id: r.stable_id,
            host_version: r.host_version,
        },
    };
    for (name, pointer) in [
        ("summary", &reports.summary),
        ("identity", &reports.identity),
        ("stable-id", &reports.stable_id),
        ("host-version", &reports.host_version),
    ] {
        if let Some(pointer) = pointer
            && !is_json_pointer(pointer)
        {
            return Err(ProbeIssue::new(
                "pointer-invalid",
                format!("probe reports.{name} {pointer:?} is not an RFC 6901 JSON pointer"),
            ));
        }
    }
    if raw.kind == ProbeKind::Account && (reports.identity.is_none() || reports.stable_id.is_none())
    {
        return Err(ProbeIssue::new(
            "account-reports-missing",
            "an account probe must declare reports.identity and reports.stable-id — without them no account is identified",
        ));
    }

    Ok(Some(ProbeDecl {
        command: raw.command,
        inputs,
        describe,
        kind: raw.kind,
        rest_origin,
        reports,
        transport,
    }))
}

/// The probe's literal inputs, checked against the command's declared inputs.
fn parse_inputs(
    raw: Option<Yaml>,
    declared: &Yaml,
    command: &str,
) -> Result<serde_json::Map<String, Json>, ProbeIssue> {
    let provided = match raw {
        None | Some(Yaml::Null) => serde_json::Map::new(),
        Some(value @ Yaml::Mapping(_)) => match serde_json::to_value(value) {
            Ok(Json::Object(map)) => map,
            _ => {
                return Err(ProbeIssue::new(
                    "inputs-not-literal",
                    "probe inputs must be a mapping of input names to literal values",
                ));
            }
        },
        Some(_) => {
            return Err(ProbeIssue::new(
                "inputs-not-literal",
                "probe inputs must be a mapping of input names to literal values",
            ));
        }
    };
    for (name, value) in &provided {
        if contains_template(value) {
            return Err(ProbeIssue::new(
                "inputs-not-literal",
                format!(
                    "probe input {name:?} carries a template expression; probe inputs are literal values only"
                ),
            ));
        }
    }
    let declared_map = declared.as_mapping();
    for (name, value) in &provided {
        let spec = declared_map.and_then(|m| m.get(Yaml::String(name.clone())));
        let Some(spec) = spec else {
            return Err(ProbeIssue::new(
                "inputs-undeclared",
                format!("probe input {name:?} is not an input of command {command:?}"),
            ));
        };
        if !literal_matches_type(value, spec) {
            return Err(ProbeIssue::new(
                "inputs-type-mismatch",
                format!(
                    "probe input {name:?} does not match the type command {command:?} declares"
                ),
            ));
        }
    }
    if let Some(map) = declared_map {
        for (name, spec) in map {
            let Some(name) = name.as_str() else { continue };
            let required = spec
                .get("required")
                .and_then(Yaml::as_bool)
                .unwrap_or(false);
            if required && !provided.contains_key(name) {
                return Err(ProbeIssue::new(
                    "inputs-required-missing",
                    format!(
                        "command {command:?} requires input {name:?}, which the probe does not supply"
                    ),
                ));
            }
        }
    }
    Ok(provided)
}

fn contains_template(value: &Json) -> bool {
    match value {
        Json::String(s) => s.contains("{{") || s.contains("{%"),
        Json::Array(items) => items.iter().any(contains_template),
        Json::Object(map) => map.values().any(contains_template),
        _ => false,
    }
}

/// A light type check for the declared input shapes the manifests use (`type:
/// string` mappings, or the `name: string` shorthand). Unknown/complex types
/// accept any literal — the command's own validation is the real authority.
fn literal_matches_type(value: &Json, spec: &Yaml) -> bool {
    let declared = match spec {
        Yaml::String(s) => Some(s.as_str()),
        Yaml::Mapping(_) => spec.get("type").and_then(Yaml::as_str),
        _ => None,
    };
    match declared {
        Some("string") => value.is_string(),
        Some("integer" | "int") => value.is_i64() || value.is_u64(),
        Some("number") => value.is_number(),
        Some("boolean" | "bool") => value.is_boolean(),
        Some("array") => value.is_array(),
        Some("object") => value.is_object(),
        Some("enum") => {
            let allowed = spec.get("values").and_then(Yaml::as_sequence);
            match (allowed, value.as_str()) {
                (Some(values), Some(v)) => values.iter().any(|a| a.as_str() == Some(v)),
                _ => false,
            }
        }
        _ => true,
    }
}

/// RFC 6901: empty (the whole document) or `/`-prefixed segments in which `~` is
/// only ever `~0` or `~1`.
pub(crate) fn is_json_pointer(pointer: &str) -> bool {
    if pointer.chars().count() > MAX_POINTER_CHARS {
        return false;
    }
    if pointer.is_empty() {
        return true;
    }
    if !pointer.starts_with('/') || pointer.chars().any(char::is_control) {
        return false;
    }
    let bytes = pointer.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'~' {
            if !matches!(bytes.get(i + 1), Some(b'0' | b'1')) {
                return false;
            }
            i += 2;
        } else {
            i += 1;
        }
    }
    true
}

/// Replace `{name}` path placeholders with a fixed segment so a URL template can
/// be parsed for its origin. Placeholders never sit in the authority of a
/// declared path, and if one did the parse below would simply fail to match.
fn fill_path_placeholders(template: &str) -> String {
    let mut out = String::with_capacity(template.len());
    let mut in_placeholder = false;
    for c in template.chars() {
        match (in_placeholder, c) {
            (false, '{') => {
                in_placeholder = true;
                out.push('x');
            }
            (true, '}') => in_placeholder = false,
            (true, _) => {}
            (false, c) => out.push(c),
        }
    }
    out
}

/// The exact origin of a URL in the one form a probe accepts — `https://<host>`,
/// lowercased, with no userinfo and no explicit port — or `None` when the URL is
/// not such a URL at all.
///
/// Test builds additionally accept `http://127.0.0.1:<port>`, so the origin pin,
/// the redirect refusal and the token non-disclosure can be proven against a real
/// local HTTP fixture. Production builds never do.
pub(crate) fn origin_of(url: &str) -> Option<String> {
    let parsed = url::Url::parse(url).ok()?;
    if !parsed.username().is_empty() || parsed.password().is_some() {
        return None;
    }
    let host = parsed.host_str()?.to_ascii_lowercase();
    match parsed.scheme() {
        "https" if parsed.port().is_none() => Some(format!("https://{host}")),
        #[cfg(test)]
        "http" if host == "127.0.0.1" => Some(match parsed.port() {
            Some(port) => format!("http://127.0.0.1:{port}"),
            None => "http://127.0.0.1".to_string(),
        }),
        _ => None,
    }
}

/// Whether `origin` is already exactly the normal form [`origin_of`] produces,
/// with nothing after the authority.
pub(crate) fn is_normal_origin(origin: &str) -> bool {
    let Ok(parsed) = url::Url::parse(origin) else {
        return false;
    };
    parsed.path() == "/"
        && parsed.query().is_none()
        && parsed.fragment().is_none()
        && !origin.ends_with('/')
        && origin_of(origin).as_deref() == Some(origin)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CLI_AGENT: &str = r#"
agent: hostx
version: 1.0.0
description: x
stateful: false
license: MIT
transport: { cli: { binary: aware-hostx } }
commands:
  model-info:
    lifecycle: single
    description: Reads the open model.
  exec:
    lifecycle: single
    description: Runs a script.
    mode: write
    mode-overridable: true
  watch:
    lifecycle: start
    description: Streams.
  insert:
    lifecycle: single
    description: Writes.
  later:
    lifecycle: single
    status: planned
    description: Not yet.
  greet:
    lifecycle: single
    description: Takes a name.
    inputs:
      name:
        type: string
        required: true
      count:
        type: integer
"#;

    const REST_AGENT: &str = r#"
agent: svc
version: 1.0.0
description: x
stateful: false
license: MIT
transport: { rest: { base: "https://gmail.googleapis.com/gmail/v1/" } }
auth: { scheme: oauth2, secret: google-workspace }
commands:
  account.userinfo:
    lifecycle: single
    description: Reads who is signed in.
    method: GET
    path: https://openidconnect.googleapis.com/v1/userinfo
  labels:
    lifecycle: single
    description: Lists labels.
    method: GET
    path: users/me/labels
  bare:
    lifecycle: single
    description: No method.
"#;

    fn with_probe(base: &str, probe: &str) -> Agent {
        let yaml = format!("{base}probe:\n{probe}");
        serde_yaml::from_str(&yaml).unwrap_or_else(|e| panic!("{e}\n{yaml}"))
    }

    fn reason(base: &str, probe: &str) -> &'static str {
        parse_probe(&with_probe(base, probe))
            .expect_err("probe must be refused")
            .reason
    }

    #[test]
    fn an_absent_probe_is_undeclared_not_invalid() {
        let a: Agent = serde_yaml::from_str(CLI_AGENT).unwrap();
        assert_eq!(parse_probe(&a), Ok(None));
    }

    #[test]
    fn a_valid_host_probe_parses_with_its_reports() {
        let a = with_probe(
            CLI_AGENT,
            "  command: model-info\n  describe: Reads the model name.\n  kind: host\n  reports:\n    summary: /model_name\n    stable-id: /host_pid\n",
        );
        let p = parse_probe(&a).unwrap().unwrap();
        assert_eq!(p.command, "model-info");
        assert_eq!(p.kind, ProbeKind::Host);
        assert_eq!(p.transport, TransportKind::Cli);
        assert_eq!(p.reports.summary.as_deref(), Some("/model_name"));
        assert_eq!(p.reports.stable_id.as_deref(), Some("/host_pid"));
        assert!(p.rest_origin.is_none());
        assert!(p.inputs.is_empty());
    }

    #[test]
    fn an_unknown_key_anywhere_is_refused() {
        for probe in [
            "  command: model-info\n  describe: d\n  kind: host\n  extra: 1\n",
            "  command: model-info\n  describe: d\n  kind: host\n  reports: { summary: /a, owner: /b }\n",
        ] {
            assert_eq!(reason(CLI_AGENT, probe), "malformed", "{probe}");
        }
        assert_eq!(
            reason(
                REST_AGENT,
                "  command: account.userinfo\n  describe: d\n  kind: host\n  rest: { origin: \"https://openidconnect.googleapis.com\", port: 1 }\n"
            ),
            "malformed"
        );
    }

    #[test]
    fn the_kind_is_a_closed_set() {
        assert_eq!(
            reason(
                CLI_AGENT,
                "  command: model-info\n  describe: d\n  kind: service\n"
            ),
            "malformed"
        );
    }

    #[test]
    fn each_command_rule_refuses_with_its_own_reason() {
        let cases = [
            ("nope", "command-unknown"),
            ("exec", "command-mode-overridable"),
            ("watch", "command-not-single"),
            ("insert", "command-not-read"),
            ("later", "command-not-available"),
        ];
        for (command, expected) in cases {
            let probe = format!("  command: {command}\n  describe: d\n  kind: host\n");
            assert_eq!(reason(CLI_AGENT, &probe), expected, "{command}");
        }
    }

    #[test]
    fn mode_overridable_is_refused_even_when_its_manifest_mode_is_read() {
        let base = CLI_AGENT.replace(
            "    mode: write\n    mode-overridable",
            "    mode: read\n    mode-overridable",
        );
        assert_eq!(
            reason(&base, "  command: exec\n  describe: d\n  kind: host\n"),
            "command-mode-overridable"
        );
    }

    #[test]
    fn inputs_must_be_declared_typed_literals_and_cover_required_ones() {
        let ok = with_probe(
            CLI_AGENT,
            "  command: greet\n  inputs: { name: ada, count: 2 }\n  describe: d\n  kind: host\n",
        );
        assert!(parse_probe(&ok).unwrap().is_some());
        let cases = [
            ("{ count: 2 }", "inputs-required-missing"),
            ("{ name: ada, other: 1 }", "inputs-undeclared"),
            ("{ name: 3 }", "inputs-type-mismatch"),
            ("{ name: \"{{ secrets.x }}\" }", "inputs-not-literal"),
            ("[a]", "inputs-not-literal"),
        ];
        for (inputs, expected) in cases {
            let probe =
                format!("  command: greet\n  inputs: {inputs}\n  describe: d\n  kind: host\n");
            assert_eq!(reason(CLI_AGENT, &probe), expected, "{inputs}");
        }
    }

    #[test]
    fn describe_is_bounded_plain_text() {
        let long = "x".repeat(MAX_DESCRIBE_CHARS + 1);
        for describe in ["\"  \"", long.as_str(), "\"a\\tb\""] {
            let probe = format!("  command: model-info\n  describe: {describe}\n  kind: host\n");
            assert_eq!(reason(CLI_AGENT, &probe), "describe-invalid", "{describe}");
        }
    }

    #[test]
    fn pointers_must_be_rfc6901() {
        for pointer in ["model_name", "/a~2b", "/a~"] {
            let probe = format!(
                "  command: model-info\n  describe: d\n  kind: host\n  reports: {{ summary: \"{pointer}\" }}\n"
            );
            assert_eq!(reason(CLI_AGENT, &probe), "pointer-invalid", "{pointer}");
        }
        assert!(is_json_pointer(""));
        assert!(is_json_pointer("/body/a~1b~0c"));
    }

    #[test]
    fn an_account_probe_must_identify_the_account() {
        for reports in [
            "{ identity: /body/email }",
            "{ stable-id: /body/sub }",
            "{}",
        ] {
            let probe = format!(
                "  command: account.userinfo\n  describe: d\n  kind: account\n  rest: {{ origin: \"https://openidconnect.googleapis.com\" }}\n  reports: {reports}\n"
            );
            assert_eq!(
                reason(REST_AGENT, &probe),
                "account-reports-missing",
                "{reports}"
            );
        }
    }

    #[test]
    fn a_rest_probe_pins_an_exact_https_origin_matching_its_url() {
        let ok = with_probe(
            REST_AGENT,
            "  command: account.userinfo\n  describe: d\n  kind: account\n  rest: { origin: \"https://openidconnect.googleapis.com\" }\n  reports: { identity: /body/email, stable-id: /body/sub }\n",
        );
        let p = parse_probe(&ok).unwrap().unwrap();
        assert_eq!(p.transport, TransportKind::Rest);
        assert_eq!(
            p.rest_origin.as_deref(),
            Some("https://openidconnect.googleapis.com")
        );

        let refuse = |command: &str, rest: &str| {
            let probe = format!("  command: {command}\n  describe: d\n  kind: host\n{rest}");
            reason(REST_AGENT, &probe)
        };
        assert_eq!(refuse("account.userinfo", ""), "rest-origin-required");
        for bad in [
            "https://openidconnect.googleapis.com/",
            "https://openidconnect.googleapis.com:443",
            "https://openidconnect.googleapis.com:8443",
            "https://user@openidconnect.googleapis.com",
            "http://openidconnect.googleapis.com",
            "https://openidconnect.googleapis.com/v1",
            "https://OpenIDConnect.googleapis.com",
        ] {
            assert_eq!(
                refuse(
                    "account.userinfo",
                    &format!("  rest: {{ origin: \"{bad}\" }}\n")
                ),
                "rest-origin-invalid",
                "{bad}"
            );
        }
        // The labels command lives on the Gmail base, not the declared origin.
        assert_eq!(
            refuse(
                "labels",
                "  rest: { origin: \"https://openidconnect.googleapis.com\" }\n"
            ),
            "rest-origin-mismatch"
        );
        assert_eq!(
            refuse(
                "bare",
                "  rest: { origin: \"https://gmail.googleapis.com\" }\n"
            ),
            "rest-method-missing"
        );
        assert_eq!(
            reason(
                CLI_AGENT,
                "  command: model-info\n  describe: d\n  kind: host\n  rest: { origin: \"https://a.example\" }\n"
            ),
            "rest-origin-forbidden"
        );
    }

    #[test]
    fn a_builtin_agent_cannot_declare_a_probe() {
        let base = CLI_AGENT.replace("{ cli: { binary: aware-hostx } }", "{ builtin: {} }");
        assert_eq!(
            reason(
                &base,
                "  command: model-info\n  describe: d\n  kind: host\n"
            ),
            "transport-unsupported"
        );
    }

    #[test]
    fn origin_of_accepts_only_https_without_port_or_userinfo() {
        assert_eq!(
            origin_of("https://OpenIDConnect.googleapis.com/v1/userinfo?x=1").as_deref(),
            Some("https://openidconnect.googleapis.com")
        );
        assert_eq!(origin_of("https://a.example:444/"), None);
        assert_eq!(origin_of("https://u:p@a.example/"), None);
        assert_eq!(origin_of("http://a.example/"), None);
        assert_eq!(origin_of("file:///etc/passwd"), None);
    }
}
