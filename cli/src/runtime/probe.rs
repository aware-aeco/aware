//! `aware agent probe` (#617) — run an installed agent's one declared probe,
//! for real, and report a bounded receipt.
//!
//! The probe is the only path in AWARE that runs an agent command with nobody
//! composing it, so everything here is written to a tighter rule than the
//! workflow transports:
//!
//! - **Nothing raw leaves.** No response body, stderr, bridge message, header or
//!   token reaches the output or the log. A failure is a code from
//!   [`ProbeFailure`] plus AWARE's own fixed sentence for it; `details` carries
//!   only codes, HTTP status and counts. Success carries only the declared report
//!   strings, each bounded to 256 characters.
//! - **The manifest cannot choose where a credential goes.** A REST probe's URL
//!   must sit on its declared `rest.origin` before any credential is touched; a
//!   registered integration's credential goes only to that integration's
//!   code-owned `probe_origins`; a custom handle's only to an origin the caller
//!   confirmed with `--allow-origin`. No redirect is followed.
//! - **No fallback between accounts.** `--as A` resolves exactly slot `H.A`;
//!   without it, exactly the manifest's slot. A missing slot is
//!   `E_CREDENTIAL_MISSING`, never another account.
//! - **Bounded.** 64 KiB of stdout / response body, a caller-set deadline, and a
//!   timeout terminates the bridge's whole process tree (a Windows Job Object /
//!   a Unix process group), not just the immediate child.
//! - **Persistent writes, exactly:** one code-only line in `logs/agent-probe.log`
//!   (written by the command layer), plus the credential maintenance the resolver
//!   already performs — an OAuth refresh stored back to the same slot, and a
//!   generation written for a legacy credential.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use serde::Serialize;
use serde_json::{Map, Value as Json, json};

use crate::manifest::agent::Agent;
use crate::manifest::probe::{ProbeDecl, ProbeKind, origin_of, parse_probe};
use crate::runtime::invoker::TransportKind;

/// The `data.schema` of a successful probe.
pub(crate) const SCHEMA: &str = "aware.agent-probe/v1";
/// Cap on a bridge's stdout and on a REST response body.
pub(crate) const OUTPUT_CAP: usize = 64 * 1024;
/// Longest report string, after control characters are removed.
const MAX_REPORT_CHARS: usize = 256;
/// `--timeout-ms` bounds and default.
pub(crate) const MIN_TIMEOUT_MS: u64 = 1_000;
pub(crate) const MAX_TIMEOUT_MS: u64 = 60_000;
pub(crate) const DEFAULT_TIMEOUT_MS: u64 = 15_000;

// ── failure ──────────────────────────────────────────────────────────────────

/// A refused or failed probe: a stable code, and details that hold only codes,
/// statuses and counts.
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ProbeFailure {
    pub(crate) code: &'static str,
    pub(crate) details: Map<String, Json>,
}

impl ProbeFailure {
    fn new(code: &'static str) -> Self {
        Self {
            code,
            details: Map::new(),
        }
    }

    fn reason(code: &'static str, reason: &'static str) -> Self {
        Self::new(code).with("reason", Json::String(reason.into()))
    }

    fn with(mut self, key: &str, value: Json) -> Self {
        self.details.insert(key.into(), value);
        self
    }

    /// AWARE's own fixed sentence for the code — never text from a host,
    /// service, or manifest.
    pub(crate) fn message(&self) -> &'static str {
        match self.code {
            "E_AGENT_NOT_INSTALLED" => "The agent is not installed.",
            "E_AGENT_PLANNED" => "The agent is planned and not runnable yet.",
            "E_PROBE_UNDECLARED" => "The agent declares no connection probe.",
            "E_PROBE_INVALID" => "The agent's probe declaration is not valid.",
            "E_PROBE_ORIGIN_NOT_ALLOWED" => {
                "The probe would send a request to an origin that is not allowed for this credential."
            }
            "E_PROBE_ALIAS_CONFLICT" => {
                "The requested account alias cannot be used for this probe."
            }
            "E_CREDENTIAL_MISSING" => "No usable credential is stored for this account.",
            "E_CREDENTIAL_EXPIRED" => {
                "The stored credential for this account has expired and could not be refreshed."
            }
            "E_CREDENTIAL_CHANGED" => {
                "The stored credential is not the one the caller expected (its generation changed)."
            }
            "E_PROBE_FAILED" => "The service answered the probe with a failure.",
            "E_HOST_UNAVAILABLE" => "The host did not answer the probe.",
            "E_PROBE_OUTPUT_TOO_LARGE" => "The probe's answer exceeded the 64 KiB limit.",
            "E_PROBE_REPORT_MISSING" => {
                "The probe answered, but without the account identity it must report."
            }
            "E_PROBE_TIMEOUT" => "The probe did not finish before the deadline.",
            _ => "The probe failed.",
        }
    }

    /// Process exit status, on the `cli-spec.md` exit-code table.
    pub(crate) fn exit_code(&self) -> i32 {
        match self.code {
            "E_AGENT_NOT_INSTALLED" => 7,
            "E_CREDENTIAL_MISSING" | "E_CREDENTIAL_EXPIRED" | "E_CREDENTIAL_CHANGED" => 6,
            "E_PROBE_FAILED"
            | "E_HOST_UNAVAILABLE"
            | "E_PROBE_OUTPUT_TOO_LARGE"
            | "E_PROBE_REPORT_MISSING"
            | "E_PROBE_TIMEOUT" => 4,
            _ => 3,
        }
    }
}

// ── request + receipt ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default)]
pub(crate) struct ProbeOptions {
    pub(crate) alias: Option<String>,
    pub(crate) allow_origin: Option<String>,
    pub(crate) expect_generation: Option<String>,
    pub(crate) timeout: Duration,
}

#[derive(Debug, Clone, Default, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Reported {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) identity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) stable_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) host_version: Option<String>,
}

/// `data` of a successful probe — schema `aware.agent-probe/v1`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProbeReceipt {
    pub(crate) schema: &'static str,
    pub(crate) agent: String,
    pub(crate) version: String,
    pub(crate) manifest_sha256: String,
    pub(crate) command: String,
    pub(crate) transport: &'static str,
    pub(crate) reviewed: bool,
    pub(crate) kind: &'static str,
    pub(crate) alias: Option<String>,
    pub(crate) credential_generation: Option<String>,
    pub(crate) describe: String,
    pub(crate) probed_at: String,
    pub(crate) duration_ms: u128,
    pub(crate) reported: Reported,
}

/// Lowercase hex SHA-256 of the installed manifest's exact bytes — the value a
/// host pairs with `version` to tell whether a stored receipt is still current.
pub(crate) fn manifest_sha256(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(bytes))
}

// ── credential slots ─────────────────────────────────────────────────────────

/// Which credential slot a probe uses, decided BEFORE any credential is read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CredentialPlan {
    /// The probe attaches no credential (a cli probe, or a `no-auth` command).
    None,
    /// A registered OAuth integration: base `integration`, slot alias `alias`.
    Registered {
        integration: String,
        alias: Option<String>,
    },
    /// A custom handle — opaque, dots allowed, never split. `account` is the
    /// exact slot: the handle, or `<handle>.<alias>`.
    Custom {
        handle: String,
        account: String,
        alias: Option<String>,
    },
}

/// The base credential handle a probe would use, if it attaches one — what a
/// host names on its confirmation card. A registered integration's handle is
/// its integration id (the first-dot split); a custom handle is the manifest's
/// `auth.secret`, whole.
pub(crate) fn credential_handle(agent: &Agent, decl: &ProbeDecl) -> Option<String> {
    let auth = probe_auth(agent, decl)?;
    let base = auth
        .secret
        .split_once('.')
        .map_or(auth.secret.as_str(), |(b, _)| b);
    if crate::auth::config::for_integration(base).is_ok() {
        Some(base.to_string())
    } else {
        Some(auth.secret.clone())
    }
}

fn probe_auth<'a>(
    agent: &'a Agent,
    decl: &ProbeDecl,
) -> Option<&'a crate::manifest::agent::AuthScheme> {
    if decl.transport != TransportKind::Rest {
        return None;
    }
    let public = agent
        .commands
        .get(&decl.command)
        .is_some_and(|command| command.no_auth);
    if public { None } else { agent.auth.as_ref() }
}

/// Resolve the slot rule. Pure: reads no credential.
pub(crate) fn plan_credential(
    agent: &Agent,
    decl: &ProbeDecl,
    alias: Option<&str>,
) -> Result<CredentialPlan, ProbeFailure> {
    if let Some(alias) = alias
        && !crate::commands::credential::is_valid_alias(alias)
    {
        return Err(ProbeFailure::reason(
            "E_PROBE_ALIAS_CONFLICT",
            "alias-invalid",
        ));
    }
    let Some(auth) = probe_auth(agent, decl) else {
        if alias.is_some() {
            return Err(ProbeFailure::reason(
                "E_PROBE_ALIAS_CONFLICT",
                "no-credential",
            ));
        }
        return Ok(CredentialPlan::None);
    };
    // The same first-dot split the REST resolver applies (`resolve_rest_credential`):
    // a registered base means the token comes from that integration's OAuth slots.
    let (base, baked) = match auth.secret.split_once('.') {
        Some((base, rest)) => (base, Some(rest)),
        None => (auth.secret.as_str(), None),
    };
    if crate::auth::config::for_integration(base).is_ok() {
        if alias.is_some() && baked.is_some() {
            return Err(ProbeFailure::reason(
                "E_PROBE_ALIAS_CONFLICT",
                "alias-already-in-manifest",
            ));
        }
        let slot_alias = alias.map(str::to_string).or(baked.map(str::to_string));
        if let Some(a) = slot_alias.as_deref()
            && !crate::commands::credential::is_valid_alias(a)
        {
            return Err(ProbeFailure::reason(
                "E_PROBE_ALIAS_CONFLICT",
                "alias-invalid",
            ));
        }
        return Ok(CredentialPlan::Registered {
            integration: base.to_string(),
            alias: slot_alias,
        });
    }
    let account = match alias {
        Some(alias) => format!("{}.{alias}", auth.secret),
        None => auth.secret.clone(),
    };
    if !crate::commands::credential::is_provisionable(&account) {
        return Err(ProbeFailure::reason(
            "E_CREDENTIAL_MISSING",
            "handle-invalid",
        ));
    }
    Ok(CredentialPlan::Custom {
        handle: auth.secret.clone(),
        account,
        alias: alias.map(str::to_string),
    })
}

/// The credential to attach, with the generation it carries.
struct LoadedCredential {
    secret: String,
    generation: Option<String>,
}

fn check_expected_generation(
    expected: Option<&str>,
    actual: Option<&str>,
) -> Result<(), ProbeFailure> {
    match expected {
        Some(expected) if actual != Some(expected) => {
            Err(ProbeFailure::new("E_CREDENTIAL_CHANGED"))
        }
        _ => Ok(()),
    }
}

fn usable(secret: &str) -> bool {
    !secret.trim().is_empty() && crate::runtime::invoker::never_sendable_char(secret).is_none()
}

/// Load exactly the planned slot. Blocking (keychain I/O, possibly an OAuth
/// refresh), so callers run it off the reactor.
fn load_credential(
    home: &Path,
    plan: &CredentialPlan,
    expected: Option<&str>,
) -> Result<Option<LoadedCredential>, ProbeFailure> {
    match plan {
        CredentialPlan::None => {
            if expected.is_some() {
                // A generation was expected, but this probe uses no credential at
                // all — nothing can match it, so fail closed.
                return Err(ProbeFailure::reason(
                    "E_CREDENTIAL_MISSING",
                    "no-credential",
                ));
            }
            Ok(None)
        }
        CredentialPlan::Registered { integration, alias } => {
            let stored = crate::auth::keychain::load_token(integration, alias.as_deref(), home)
                .map_err(|_| ProbeFailure::reason("E_CREDENTIAL_MISSING", "unreadable"))?
                .ok_or_else(|| ProbeFailure::new("E_CREDENTIAL_MISSING"))?;
            // Checked BEFORE the refresh, so a probe never refreshes (or sends)
            // a credential for an account the caller did not expect.
            check_expected_generation(expected, stored.generation.as_deref())?;
            let fresh = crate::auth::refresh::ensure_fresh(integration, alias.as_deref(), home)
                .map_err(|_| ProbeFailure::new("E_CREDENTIAL_EXPIRED"))?;
            check_expected_generation(expected, fresh.generation.as_deref())?;
            if !usable(&fresh.access_token) {
                return Err(ProbeFailure::reason("E_CREDENTIAL_MISSING", "unusable"));
            }
            Ok(Some(LoadedCredential {
                secret: fresh.access_token,
                generation: fresh.generation,
            }))
        }
        CredentialPlan::Custom { account, .. } => {
            let (secret, generation, expires_at) =
                match crate::auth::keychain::load_token(account, None, home) {
                    Ok(Some(token)) => (token.access_token, token.generation, token.expires_at),
                    // The transport also accepts a bare JSON string or a `{key: …}`
                    // object; those have no generation to report.
                    _ => {
                        let raw = crate::runtime::invoker::load_secret_value(
                            &home.join("agents"),
                            account,
                        )
                        .as_ref()
                        .and_then(crate::runtime::invoker::secret_as_str)
                        .ok_or_else(|| ProbeFailure::new("E_CREDENTIAL_MISSING"))?;
                        (raw, None, 0)
                    }
                };
            if !usable(&secret) {
                return Err(ProbeFailure::reason("E_CREDENTIAL_MISSING", "unusable"));
            }
            let now = crate::auth::unix_now_secs().unwrap_or(0);
            if expires_at > 0 && expires_at <= now {
                return Err(ProbeFailure::new("E_CREDENTIAL_EXPIRED"));
            }
            check_expected_generation(expected, generation.as_deref())?;
            Ok(Some(LoadedCredential { secret, generation }))
        }
    }
}

/// The origin rule for the credential, checked before the credential is read.
fn check_credential_origin(
    plan: &CredentialPlan,
    declared: &str,
    allow_origin: Option<&str>,
) -> Result<(), ProbeFailure> {
    if let Some(allowed) = allow_origin
        && allowed != declared
    {
        return Err(ProbeFailure::reason(
            "E_PROBE_ORIGIN_NOT_ALLOWED",
            "allow-origin-mismatch",
        ));
    }
    match plan {
        CredentialPlan::None => Ok(()),
        CredentialPlan::Registered { integration, .. } => {
            let config = crate::auth::config::for_integration(integration)
                .map_err(|_| ProbeFailure::reason("E_PROBE_ORIGIN_NOT_ALLOWED", "unregistered"))?;
            if config.probe_origins().contains(&declared) {
                Ok(())
            } else {
                Err(ProbeFailure::reason(
                    "E_PROBE_ORIGIN_NOT_ALLOWED",
                    "origin-not-in-integration-allowlist",
                ))
            }
        }
        CredentialPlan::Custom { .. } => match allow_origin {
            Some(_) => Ok(()),
            None => Err(ProbeFailure::reason(
                "E_PROBE_ORIGIN_NOT_ALLOWED",
                "allow-origin-required",
            )),
        },
    }
}

// ── the verb ─────────────────────────────────────────────────────────────────

/// Run `agent_id`'s declared probe against `home` (an AWARE home directory).
///
/// `options.timeout` bounds the WHOLE probe — manifest checks, credential
/// resolution (including an OAuth refresh), the transport call and the
/// `reviewed` lookup — not only the transport. Past it the answer is
/// `E_PROBE_TIMEOUT`.
pub(crate) async fn probe_agent(
    home: &Path,
    agent_id: &str,
    options: &ProbeOptions,
) -> Result<ProbeReceipt, ProbeFailure> {
    let started = Instant::now();
    match tokio::time::timeout(
        options.timeout,
        probe_agent_within(home, agent_id, options, started),
    )
    .await
    {
        Ok(outcome) => outcome,
        Err(_) => Err(ProbeFailure::new("E_PROBE_TIMEOUT")),
    }
}

/// What remains of the probe's deadline, or `E_PROBE_TIMEOUT` once it is spent.
fn remaining(options: &ProbeOptions, started: Instant) -> Result<Duration, ProbeFailure> {
    options
        .timeout
        .checked_sub(started.elapsed())
        .filter(|left| !left.is_zero())
        .ok_or_else(|| ProbeFailure::new("E_PROBE_TIMEOUT"))
}

async fn probe_agent_within(
    home: &Path,
    agent_id: &str,
    options: &ProbeOptions,
    started: Instant,
) -> Result<ProbeReceipt, ProbeFailure> {
    let probed_at = crate::time::now_iso();
    let agents_dir = home.join("agents");
    let manifest_path = crate::manifest::loader::agent_manifest_path(&agents_dir, agent_id)
        .map_err(|_| ProbeFailure::new("E_AGENT_NOT_INSTALLED"))?;
    let bytes =
        std::fs::read(&manifest_path).map_err(|_| ProbeFailure::new("E_AGENT_NOT_INSTALLED"))?;
    let manifest_sha256 = manifest_sha256(&bytes);
    let text = String::from_utf8_lossy(&bytes);
    let agent: Agent = serde_yaml::from_str(text.trim_start_matches('\u{feff}'))
        .map_err(|_| ProbeFailure::reason("E_PROBE_INVALID", "manifest-unreadable"))?;
    if agent.status == crate::manifest::agent::AgentStatus::Planned {
        return Err(ProbeFailure::new("E_AGENT_PLANNED"));
    }
    // The folder an agent is installed under must be the agent the manifest
    // declares, or the receipt (and its `reviewed`) would describe another one.
    if agent.agent != agent_id {
        return Err(ProbeFailure::reason("E_PROBE_INVALID", "agent-id-mismatch"));
    }
    if crate::validate::runtime_requirement_error(&agent, crate::validate::CURRENT_CLI_VERSION)
        .is_some()
    {
        return Err(ProbeFailure::reason(
            "E_PROBE_INVALID",
            "runtime-requirement",
        ));
    }
    let decl = match parse_probe(&agent) {
        Ok(Some(decl)) => decl,
        Ok(None) => return Err(ProbeFailure::new("E_PROBE_UNDECLARED")),
        Err(issue) => {
            return Err(ProbeFailure::new("E_PROBE_INVALID")
                .with("reason", Json::String(issue.reason.into())));
        }
    };
    let plan = plan_credential(&agent, &decl, options.alias.as_deref())?;
    let slot_alias = match &plan {
        CredentialPlan::Registered { alias, .. } | CredentialPlan::Custom { alias, .. } => {
            alias.clone()
        }
        CredentialPlan::None => None,
    };

    let (result, generation) = match decl.transport {
        TransportKind::Cli => {
            if options.allow_origin.is_some() {
                return Err(ProbeFailure::reason(
                    "E_PROBE_ORIGIN_NOT_ALLOWED",
                    "no-origin",
                ));
            }
            load_credential(home, &plan, options.expect_generation.as_deref())?;
            let binary = agent
                .transport
                .cli
                .as_ref()
                .map(|c| c.binary.clone())
                .ok_or_else(|| ProbeFailure::reason("E_PROBE_INVALID", "transport-unsupported"))?;
            let result = probe_cli(home, &binary, &decl, remaining(options, started)?).await?;
            (result, None)
        }
        TransportKind::Rest => probe_rest(home, &agent, &decl, &plan, options, started).await?,
        _ => {
            return Err(ProbeFailure::reason(
                "E_PROBE_INVALID",
                "transport-unsupported",
            ));
        }
    };
    let duration_ms = started.elapsed().as_millis();

    let reported = Reported {
        summary: extract_report(&result, decl.reports.summary.as_deref()),
        identity: extract_report(&result, decl.reports.identity.as_deref()),
        stable_id: extract_report(&result, decl.reports.stable_id.as_deref()),
        host_version: extract_report(&result, decl.reports.host_version.as_deref()),
    };
    if decl.kind == ProbeKind::Account
        && (reported.identity.is_none() || reported.stable_id.is_none())
    {
        return Err(ProbeFailure::new("E_PROBE_REPORT_MISSING"));
    }

    let agent_dir = agents_dir.join(agent_id);
    let (manifest_agent, manifest_version) = (agent.agent.clone(), agent.version.clone());
    // Bounded by what is left of the deadline. A registry that does not answer
    // in time leaves the probe unreviewed (fail closed on trust) rather than
    // failing a connection the probe has already proven.
    let lookup = tokio::task::spawn_blocking(move || {
        bundle_is_reviewed(&agent_dir, &manifest_agent, &manifest_version)
    });
    let reviewed = match remaining(options, started) {
        Ok(left) => matches!(tokio::time::timeout(left, lookup).await, Ok(Ok(true))),
        Err(_) => false,
    };

    Ok(ProbeReceipt {
        schema: SCHEMA,
        agent: agent.agent,
        version: agent.version,
        manifest_sha256,
        command: decl.command,
        transport: decl.transport.as_str(),
        reviewed,
        kind: decl.kind.as_str(),
        alias: slot_alias,
        credential_generation: generation,
        describe: decl.describe,
        probed_at,
        duration_ms,
        reported,
    })
}

/// `reviewed` — anchored outside the manifest: the installed bundle's digest
/// equals the `bundle-digest` a freshly fetched official registry index records
/// for this agent@version. Anything else (a local install, an edited file, an
/// offline machine, a custom registry) is `false`.
pub(crate) fn bundle_is_reviewed(agent_dir: &Path, agent: &str, version: &str) -> bool {
    if !crate::install::provenance::claims_official(agent_dir) {
        return false;
    }
    let index = crate::registry::fetch::fetch_fresh_official_index().ok();
    crate::install::provenance::assess_against_index(agent_dir, agent, version, index.as_ref())
        .verified
}

/// One declared report: a string or number at `pointer`, control characters
/// removed, at most 256 characters. Anything else — absent, another type, empty,
/// too long — is no report.
pub(crate) fn extract_report(result: &Json, pointer: Option<&str>) -> Option<String> {
    let value = result.pointer(pointer?)?;
    let text = match value {
        Json::String(s) => s.clone(),
        Json::Number(n) => n.to_string(),
        _ => return None,
    };
    let clean: String = text.chars().filter(|c| !c.is_control()).collect();
    let clean = clean.trim().to_string();
    if clean.is_empty() || clean.chars().count() > MAX_REPORT_CHARS {
        return None;
    }
    Some(clean)
}

// ── cli transport ────────────────────────────────────────────────────────────

async fn probe_cli(
    home: &Path,
    binary: &str,
    decl: &ProbeDecl,
    timeout: Duration,
) -> Result<Json, ProbeFailure> {
    let program = crate::runtime::invoker::resolve_cli_binary(binary, &home.join("bridges"));
    let stdin = serde_json::to_vec(&Json::Object(decl.inputs.clone()))
        .map_err(|_| ProbeFailure::reason("E_PROBE_INVALID", "inputs-not-literal"))?;
    let output = run_bounded(
        &program,
        &[decl.command.as_str(), "--json-stdin"],
        stdin,
        timeout,
        OUTPUT_CAP,
    )
    .await
    .map_err(|failure| match failure {
        BoundedFailure::Spawn(std::io::ErrorKind::NotFound) => {
            ProbeFailure::reason("E_HOST_UNAVAILABLE", "bridge-missing")
        }
        BoundedFailure::Spawn(_) | BoundedFailure::Supervise => {
            ProbeFailure::reason("E_HOST_UNAVAILABLE", "bridge-spawn-failed")
        }
        BoundedFailure::Timeout => ProbeFailure::new("E_PROBE_TIMEOUT"),
        BoundedFailure::StdoutTooLarge => ProbeFailure::new("E_PROBE_OUTPUT_TOO_LARGE"),
    })?;
    let receipt: Option<Json> = serde_json::from_slice(&output.stdout).ok();
    let failed = !output.status.success() || receipt.as_ref().is_some_and(receipt_reports_failure);
    if failed {
        return Err(host_failure(
            receipt.as_ref(),
            &output.stderr,
            output.status.code(),
        ));
    }
    receipt.ok_or_else(|| ProbeFailure::reason("E_HOST_UNAVAILABLE", "receipt-not-json"))
}

fn receipt_reports_failure(receipt: &Json) -> bool {
    receipt.get("ok").and_then(Json::as_bool) == Some(false)
        || matches!(
            receipt.get("status").and_then(Json::as_str),
            Some("err" | "error")
        )
}

/// A bridge's structured failure code: kebab-case, at most 64 characters. Any
/// other text a bridge emits is a message, and messages never leave.
fn structured_code(value: Option<&Json>) -> Option<String> {
    let code = value?.as_str()?;
    let well_formed = !code.is_empty()
        && code.len() <= 64
        && code
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
        && code.starts_with(|c: char| c.is_ascii_lowercase());
    well_formed.then(|| code.to_string())
}

fn host_failure(receipt: Option<&Json>, stderr: &[u8], exit: Option<i32>) -> ProbeFailure {
    let mut failure = ProbeFailure::new("E_HOST_UNAVAILABLE");
    let stderr_json: Option<Json> = serde_json::from_slice(trim_ascii(stderr)).ok();
    let code = structured_code(receipt.and_then(|r| r.get("code")))
        .or_else(|| structured_code(stderr_json.as_ref().and_then(|e| e.get("code"))));
    if let Some(code) = code {
        failure = failure.with("hostCode", Json::String(code));
    }
    if let Some(count) = receipt
        .and_then(|r| r.get("instance_count"))
        .and_then(Json::as_u64)
        .filter(|count| *count <= 10_000)
    {
        failure = failure.with("instanceCount", json!(count));
    }
    if let Some(exit) = exit {
        failure = failure.with("exitCode", json!(exit));
    }
    failure
}

fn trim_ascii(bytes: &[u8]) -> &[u8] {
    let start = bytes
        .iter()
        .position(|b| !b.is_ascii_whitespace())
        .unwrap_or(bytes.len());
    let end = bytes
        .iter()
        .rposition(|b| !b.is_ascii_whitespace())
        .map_or(start, |i| i + 1);
    &bytes[start..end.max(start)]
}

// ── rest transport ───────────────────────────────────────────────────────────

async fn probe_rest(
    home: &Path,
    agent: &Agent,
    decl: &ProbeDecl,
    plan: &CredentialPlan,
    options: &ProbeOptions,
    started: Instant,
) -> Result<(Json, Option<String>), ProbeFailure> {
    let declared = decl
        .rest_origin
        .clone()
        .ok_or_else(|| ProbeFailure::reason("E_PROBE_INVALID", "rest-origin-required"))?;
    // Built from the manifest that was hashed and validated, never a second read.
    let (method, url, mut headers, mut query, body) =
        crate::runtime::invoker::build_operation_request_for(
            agent,
            &decl.command,
            &Json::Object(decl.inputs.clone()),
        )
        .map_err(|_| ProbeFailure::reason("E_PROBE_INVALID", "request-unbuildable"))?;
    // 1. The URL actually assembled must sit on the declared origin.
    if origin_of(&url).as_deref() != Some(declared.as_str()) {
        return Err(ProbeFailure::reason(
            "E_PROBE_ORIGIN_NOT_ALLOWED",
            "url-origin-mismatch",
        ));
    }
    // 2. The declared origin must be one this credential may go to.
    check_credential_origin(plan, &declared, options.allow_origin.as_deref())?;
    // 3. Only now is the credential read — exactly the planned slot.
    let plan_owned = plan.clone();
    let home_owned = home.to_path_buf();
    let expected = options.expect_generation.clone();
    let credential = tokio::task::spawn_blocking(move || {
        load_credential(&home_owned, &plan_owned, expected.as_deref())
    })
    .await
    .map_err(|_| ProbeFailure::reason("E_CREDENTIAL_MISSING", "unreadable"))??;
    let generation = credential.as_ref().and_then(|c| c.generation.clone());
    if let (Some(credential), Some(auth)) = (credential.as_ref(), agent.auth.as_ref()) {
        let slot = crate::runtime::invoker::inject_auth(
            auth,
            &credential.secret,
            &mut headers,
            &mut query,
        );
        // The stored credential MUST be what authenticates the request: a receipt
        // naming its generation is a claim that this credential reached that
        // account. If nothing was attached (an unknown scheme, or a probe input
        // already filling the slot), the call would prove some other token — or
        // none — so it is refused before anything is sent.
        if slot.is_none() {
            return Err(ProbeFailure::reason("E_PROBE_INVALID", "auth-not-attached"));
        }
        if slot == Some(crate::runtime::invoker::AuthSlot::Header)
            && crate::runtime::invoker::unsendable_in_header_char(&credential.secret).is_some()
        {
            return Err(ProbeFailure::reason("E_CREDENTIAL_MISSING", "unusable"));
        }
    }
    let request_body = body
        .filter(|b| matches!(method.as_str(), "POST" | "PUT" | "PATCH" | "DELETE") && !b.is_null());
    let remaining = remaining(options, started)?;
    let request = RestRequest {
        method,
        url,
        headers,
        query,
        body: request_body,
    };
    let blocking = tokio::task::spawn_blocking(move || send_bounded(request, remaining));
    let shaped = match tokio::time::timeout(remaining + Duration::from_secs(2), blocking).await {
        Ok(Ok(result)) => result?,
        Ok(Err(_)) => return Err(ProbeFailure::reason("E_PROBE_FAILED", "transport")),
        Err(_) => return Err(ProbeFailure::new("E_PROBE_TIMEOUT")),
    };
    Ok((shaped, generation))
}

struct RestRequest {
    method: String,
    url: String,
    headers: Vec<(String, String)>,
    query: Vec<(String, String)>,
    body: Option<Json>,
}

/// One bounded request: no redirects, a total deadline, 64 KiB of body, and an
/// HTTP status >= 300 is a failure carrying only that status.
fn send_bounded(request: RestRequest, timeout: Duration) -> Result<Json, ProbeFailure> {
    use std::io::Read;
    let started = Instant::now();
    let agent = ureq::AgentBuilder::new()
        .redirects(0)
        .resolver(crate::http_body::BoundedDnsResolver::new(
            timeout.min(Duration::from_secs(10)),
        ))
        .timeout(timeout)
        .build();
    let mut req = agent.request(&request.method, &request.url);
    for (name, value) in &request.headers {
        req = req.set(name, value);
    }
    for (name, value) in &request.query {
        req = req.query(name, value);
    }
    let outcome = match request.body {
        Some(Json::String(text)) => req.send_string(&text),
        Some(other) => {
            if !request
                .headers
                .iter()
                .any(|(k, _)| k.eq_ignore_ascii_case("content-type"))
            {
                req = req.set("Content-Type", "application/json");
            }
            req.send_string(&other.to_string())
        }
        None => req.call(),
    };
    let timed_out = |started: Instant| started.elapsed() + Duration::from_millis(50) >= timeout;
    let response = match outcome {
        Ok(response) => response,
        Err(ureq::Error::Status(_, response)) => response,
        Err(ureq::Error::Transport(_)) => {
            return Err(if timed_out(started) {
                ProbeFailure::new("E_PROBE_TIMEOUT")
            } else {
                ProbeFailure::reason("E_PROBE_FAILED", "transport")
            });
        }
    };
    let status = response.status();
    if status >= 300 {
        return Err(ProbeFailure::new("E_PROBE_FAILED").with("status", json!(status)));
    }
    let mut headers = Map::new();
    for name in response.headers_names() {
        if let Some(value) = response.header(&name) {
            headers.insert(name.to_ascii_lowercase(), Json::String(value.to_string()));
        }
    }
    let mut bytes = Vec::new();
    if response
        .into_reader()
        .take(OUTPUT_CAP as u64 + 1)
        .read_to_end(&mut bytes)
        .is_err()
    {
        return Err(if timed_out(started) {
            ProbeFailure::new("E_PROBE_TIMEOUT")
        } else {
            ProbeFailure::reason("E_PROBE_FAILED", "read")
        });
    }
    if bytes.len() > OUTPUT_CAP {
        return Err(ProbeFailure::new("E_PROBE_OUTPUT_TOO_LARGE"));
    }
    let body = serde_json::from_slice::<Json>(&bytes)
        .unwrap_or_else(|_| Json::String(String::from_utf8_lossy(&bytes).into_owned()));
    Ok(json!({ "status": status, "headers": headers, "body": body }))
}

// ── bounded, tree-killing child supervision ──────────────────────────────────

#[derive(Debug)]
pub(crate) struct BoundedOutput {
    pub(crate) status: std::process::ExitStatus,
    pub(crate) stdout: Vec<u8>,
    pub(crate) stderr: Vec<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum BoundedFailure {
    Spawn(std::io::ErrorKind),
    Supervise,
    Timeout,
    StdoutTooLarge,
}

/// The whole process tree a probe spawned: a Job Object on Windows (the child
/// starts suspended and is assigned before it runs, so nothing it starts can
/// escape), a process group on Unix.
struct ProcessTree {
    #[cfg(windows)]
    job: Option<win32job::Job>,
    #[cfg(unix)]
    group: Option<u32>,
}

impl ProcessTree {
    #[cfg(windows)]
    fn attach(child: &tokio::process::Child) -> Result<Self, ()> {
        let job = crate::commands::model_reader_host::provider_job(child).map_err(|_| ())?;
        if crate::commands::model_reader_host::resume_provider(child).is_err() {
            return Err(());
        }
        Ok(Self { job: Some(job) })
    }

    #[cfg(unix)]
    fn attach(child: &tokio::process::Child) -> Result<Self, ()> {
        Ok(Self { group: child.id() })
    }

    #[cfg(not(any(windows, unix)))]
    fn attach(_child: &tokio::process::Child) -> Result<Self, ()> {
        Ok(Self {})
    }

    /// Kill every process in the tree and wait until they are gone.
    async fn terminate(mut self) {
        #[cfg(windows)]
        if let Some(job) = self.job.take() {
            let _ = crate::commands::model_reader_host::terminate_provider_job(job).await;
        }
        #[cfg(unix)]
        {
            let _ = crate::commands::model_reader_host::terminate_provider_group(self.group.take())
                .await;
        }
    }

    /// After a normal exit, remove anything the child left behind (a lingering
    /// grandchild would otherwise hold the output pipes open).
    async fn release(mut self) {
        // Dropping the Job closes its last handle; KILL_ON_JOB_CLOSE ends every
        // process still in it without waiting on a signal that may never come.
        #[cfg(windows)]
        drop(self.job.take());
        #[cfg(unix)]
        {
            let _ = crate::commands::model_reader_host::terminate_provider_group(self.group.take())
                .await;
        }
    }
}

/// Run `program args…`, feeding `stdin`, with a deadline and a stdout cap. On
/// timeout or overflow the whole process tree is terminated.
pub(crate) async fn run_bounded(
    program: &Path,
    args: &[&str],
    stdin: Vec<u8>,
    timeout: Duration,
    cap: usize,
) -> Result<BoundedOutput, BoundedFailure> {
    use std::process::Stdio;
    use tokio::io::AsyncWriteExt;

    let mut command = tokio::process::Command::new(program);
    crate::private_rest_header::scrub_tokio_child(&mut command);
    command
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        command.as_std_mut().process_group(0);
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command
            .as_std_mut()
            .creation_flags(crate::commands::model_reader_host::provider_creation_flags());
    }
    let mut child = command
        .spawn()
        .map_err(|error| BoundedFailure::Spawn(error.kind()))?;
    let tree = match ProcessTree::attach(&child) {
        Ok(tree) => tree,
        Err(()) => {
            let _ = child.start_kill();
            let _ = child.wait().await;
            return Err(BoundedFailure::Supervise);
        }
    };
    let (Some(stdout), Some(stderr)) = (child.stdout.take(), child.stderr.take()) else {
        tree.terminate().await;
        let _ = child.wait().await;
        return Err(BoundedFailure::Supervise);
    };
    if let Some(mut pipe) = child.stdin.take() {
        tokio::spawn(async move {
            let _ = pipe.write_all(&stdin).await;
            let _ = pipe.shutdown().await;
        });
    }
    let mut stdout_task = tokio::spawn(read_capped(stdout, cap));
    let stderr_task = tokio::spawn(read_keep_first(stderr, cap));
    let deadline = tokio::time::sleep(timeout);
    tokio::pin!(deadline);

    let mut stdout_result: Option<Vec<u8>> = None;
    let status = loop {
        tokio::select! {
            status = child.wait() => break status,
            read = &mut stdout_task, if stdout_result.is_none() => {
                match read {
                    Ok(Captured::Complete(bytes)) => stdout_result = Some(bytes),
                    Ok(Captured::Overflow) => {
                        tree.terminate().await;
                        let _ = child.wait().await;
                        return Err(BoundedFailure::StdoutTooLarge);
                    }
                    Err(_) => stdout_result = Some(Vec::new()),
                }
            }
            () = &mut deadline => {
                tree.terminate().await;
                let _ = child.start_kill();
                let _ = child.wait().await;
                return Err(BoundedFailure::Timeout);
            }
        }
    };
    let status = match status {
        Ok(status) => status,
        Err(_) => {
            tree.terminate().await;
            return Err(BoundedFailure::Supervise);
        }
    };
    tree.release().await;
    let stdout = match stdout_result {
        Some(bytes) => bytes,
        None => match tokio::time::timeout(Duration::from_secs(5), stdout_task).await {
            Ok(Ok(Captured::Complete(bytes))) => bytes,
            Ok(Ok(Captured::Overflow)) => return Err(BoundedFailure::StdoutTooLarge),
            _ => Vec::new(),
        },
    };
    let stderr = match tokio::time::timeout(Duration::from_secs(2), stderr_task).await {
        Ok(Ok(bytes)) => bytes,
        _ => Vec::new(),
    };
    Ok(BoundedOutput {
        status,
        stdout,
        stderr,
    })
}

enum Captured {
    Complete(Vec<u8>),
    Overflow,
}

/// Read to EOF, or stop the moment more than `cap` bytes arrived.
async fn read_capped<R: tokio::io::AsyncRead + Unpin>(mut reader: R, cap: usize) -> Captured {
    use tokio::io::AsyncReadExt;
    let mut out = Vec::new();
    let mut chunk = [0u8; 8192];
    loop {
        match reader.read(&mut chunk).await {
            Ok(0) | Err(_) => return Captured::Complete(out),
            Ok(n) => {
                out.extend_from_slice(&chunk[..n]);
                if out.len() > cap {
                    return Captured::Overflow;
                }
            }
        }
    }
}

/// Keep the first `cap` bytes and keep draining, so a chatty child can never
/// block on a full pipe.
async fn read_keep_first<R: tokio::io::AsyncRead + Unpin>(mut reader: R, cap: usize) -> Vec<u8> {
    use tokio::io::AsyncReadExt;
    let mut out = Vec::new();
    let mut chunk = [0u8; 8192];
    loop {
        match reader.read(&mut chunk).await {
            Ok(0) | Err(_) => return out,
            Ok(n) => {
                let room = cap.saturating_sub(out.len());
                out.extend_from_slice(&chunk[..n.min(room)]);
            }
        }
    }
}

/// Append the code-only log line for one probe: when, which agent, and the
/// outcome code. Best-effort — the log must never change a probe's result.
pub(crate) fn append_log_line(logs_dir: &Path, agent_id: &str, outcome: &str) {
    use std::io::Write;
    let agent = if crate::manifest::loader::is_safe_segment(agent_id) {
        agent_id
    } else {
        "-"
    };
    let line = format!("{} agent-probe {agent} {outcome}\n", crate::time::now_iso());
    if std::fs::create_dir_all(logs_dir).is_err() {
        return;
    }
    let path: PathBuf = logs_dir.join("agent-probe.log");
    if let Ok(mut file) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
    {
        let _ = file.write_all(line.as_bytes());
    }
}

#[cfg(test)]
#[path = "probe_tests.rs"]
mod tests;
