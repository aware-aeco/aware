//! The command policy (`<AWARE_HOME>/command-policy.json`): deny rules a host
//! writes for agent commands, per credential slot or verified account, that
//! AWARE checks at every operation dispatch carrying an AWARE-resolved
//! credential. Contract: `10-core/command-policy.md`.
//!
//! A host can gate the reads it starts itself, but a workflow node, a routine
//! or a long-lived watcher is dispatched by AWARE, which loads the credential
//! at each operation — so a per-account "off" can only be enforced here, at the
//! moment of the operation. Nothing in this module knows any provider.
//!
//! The file is read from disk at every check (it is small and size-capped), so
//! a running watcher picks up a new rule on its next event with no restart.
//!
//! Fail-safe: an absent file allows everything (no policy). A file that exists
//! but cannot be read as a policy refuses every credential-bearing dispatch,
//! because AWARE cannot tell which commands the host meant to deny. A single
//! invalid rule is scoped to its own agent when its `agent` is readable.

use std::path::{Path, PathBuf};

use serde_json::{Map, Value as Json, json};
use thiserror::Error;

/// The file, directly under `AWARE_HOME`.
pub(crate) const POLICY_FILE: &str = "command-policy.json";
/// The writers' lock, next to it.
const LOCK_FILE: &str = "command-policy.json.lock";
/// The only schema this CLI reads.
pub(crate) const SCHEMA: &str = "aware.command-policy/v1";
/// The stable refusal code, on every dispatch path.
pub(crate) const DENIED_CODE: &str = "E_COMMAND_DENIED_BY_POLICY";
/// AWARE's own sentence for a refusal — never text from the policy file.
pub(crate) const DENIED_MESSAGE: &str = "This command is turned off for this account by the command policy, so the command was not sent.";
/// The largest policy file read. Larger is unreadable (fail-closed).
const MAX_POLICY_BYTES: u64 = 256 * 1024;
const MAX_ID_CHARS: usize = 128;
const MAX_FIELD_CHARS: usize = 256;
const MAX_NOTE_CHARS: usize = 1024;

const POLICY_FIELDS: &[&str] = &["schema", "rules"];
const RULE_FIELDS: &[&str] = &["id", "agent", "command", "accounts", "except", "note"];
const SELECTOR_FIELDS: &[&str] = &["slot", "binding"];

// ── model ────────────────────────────────────────────────────────────────────

/// One account selector: an exact credential slot, optionally narrowed to the
/// verified account it holds (`binding`, AWARE's `binding_id`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Selector {
    pub(crate) slot: String,
    pub(crate) binding: Option<String>,
}

/// Which accounts a rule denies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Accounts {
    /// `"*"` — every account.
    All,
    /// The listed selectors (never empty).
    Only(Vec<Selector>),
}

/// One deny rule.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Rule {
    pub(crate) id: String,
    pub(crate) agent: String,
    /// An exact command name, or `"*"`.
    pub(crate) command: String,
    pub(crate) accounts: Accounts,
    /// Accounts exempt from this rule; each carries a `binding`.
    pub(crate) except: Vec<Selector>,
    pub(crate) note: Option<String>,
}

/// A whole, valid policy document.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct Policy {
    pub(crate) rules: Vec<Rule>,
}

/// What a dispatch sees: the valid rules, plus whatever could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Loaded {
    /// No file: no policy.
    Absent,
    /// The file is not a readable policy at all; `why` is for `policy show`.
    Unreadable { why: String },
    /// A readable policy. `broken` holds the agents of rules that are invalid
    /// (each with the rule id when that much was readable).
    Readable {
        policy: Policy,
        broken: Vec<(String, Option<String>)>,
    },
}

/// Why a dispatch was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DenyReason {
    /// A rule denies this account.
    Denied,
    /// A rule's answer depends on which account the slot holds, and this
    /// dispatch could not verify it.
    AccountUnverified,
    /// The policy (or a rule for this agent) cannot be read.
    PolicyUnreadable,
}

impl DenyReason {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::Denied => "denied",
            Self::AccountUnverified => "account-unverified",
            Self::PolicyUnreadable => "policy-unreadable",
        }
    }
}

/// A refusal: its reason and the rule that caused it, when one did.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Denial {
    pub(crate) reason: DenyReason,
    pub(crate) rule: Option<String>,
}

impl Denial {
    /// The error an invoke / workflow dispatch returns: structured, so a run
    /// record's `node-error` carries the code, reason and rule as fields, and
    /// exit status 5 (`AwareError::exit_code`).
    pub(crate) fn to_error(&self, agent: &str, command: &str) -> crate::error::AwareError {
        let mut details = std::collections::BTreeMap::new();
        details.insert("reason".to_string(), self.reason.as_str().to_string());
        details.insert("agent".to_string(), agent.to_string());
        details.insert("command".to_string(), command.to_string());
        if let Some(rule) = &self.rule {
            details.insert("rule".to_string(), rule.clone());
        }
        crate::error::AwareError::AgentStructured {
            code: DENIED_CODE.into(),
            phase: "dispatch".into(),
            retryable: false,
            message: format!("{agent}/{command}: {DENIED_MESSAGE}").into(),
            diagnostic_id: self
                .rule
                .clone()
                .unwrap_or_else(|| self.reason.as_str().to_string())
                .into(),
            provider_code: None,
            details: Some(Box::new(crate::error::AgentErrorDetails(details))),
        }
    }
}

/// The policy's answer for one dispatch.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Decision {
    Allow,
    /// The answer depends on the verified account: decide again with it.
    NeedsIdentity,
    Deny(Denial),
}

impl Decision {
    /// For a dispatch that cannot verify the account: `NeedsIdentity` is a
    /// refusal (an exception cannot be confirmed, a binding deny cannot be
    /// ruled out).
    pub(crate) fn unverified(self) -> Result<(), Denial> {
        match self {
            Self::Allow => Ok(()),
            Self::NeedsIdentity => Err(Denial {
                reason: DenyReason::AccountUnverified,
                rule: None,
            }),
            Self::Deny(denial) => Err(denial),
        }
    }
}

/// Which account the dispatch is using, as far as it knows.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Identity<'a> {
    Unverified,
    /// The verified `binding_id` of the account the slot holds now.
    Verified(&'a str),
}

// ── evaluation ───────────────────────────────────────────────────────────────

/// Three-valued match: an unverified account may or may not match a binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tri {
    Yes,
    No,
    Unknown,
}

fn any(values: impl Iterator<Item = Tri>) -> Tri {
    let mut out = Tri::No;
    for value in values {
        match value {
            Tri::Yes => return Tri::Yes,
            Tri::Unknown => out = Tri::Unknown,
            Tri::No => {}
        }
    }
    out
}

fn binding_matches(want: &str, identity: Identity<'_>) -> Tri {
    match identity {
        Identity::Unverified => Tri::Unknown,
        Identity::Verified(have) => {
            if have.eq_ignore_ascii_case(want) {
                Tri::Yes
            } else {
                Tri::No
            }
        }
    }
}

/// A selector in `accounts`: the slot, and the account when it names one.
fn denies(selector: &Selector, slot: &str, identity: Identity<'_>) -> Tri {
    if selector.slot != slot {
        return Tri::No;
    }
    match &selector.binding {
        None => Tri::Yes,
        Some(want) => binding_matches(want, identity),
    }
}

/// A selector in `except`: only ever the slot AND its verified account.
fn exempts(selector: &Selector, slot: &str, identity: Identity<'_>) -> Tri {
    match &selector.binding {
        Some(want) if selector.slot == slot => binding_matches(want, identity),
        _ => Tri::No,
    }
}

/// Decide one dispatch of `agent`/`command` through credential slot `slot`.
pub(crate) fn evaluate(
    loaded: &Loaded,
    agent: &str,
    command: &str,
    slot: &str,
    identity: Identity<'_>,
) -> Decision {
    let (policy, broken) = match loaded {
        Loaded::Absent => return Decision::Allow,
        Loaded::Unreadable { .. } => {
            return Decision::Deny(Denial {
                reason: DenyReason::PolicyUnreadable,
                rule: None,
            });
        }
        Loaded::Readable { policy, broken } => (policy, broken),
    };
    if let Some((_, rule)) = broken.iter().find(|(a, _)| a == agent) {
        return Decision::Deny(Denial {
            reason: DenyReason::PolicyUnreadable,
            rule: rule.clone(),
        });
    }
    let mut needs_identity = false;
    for rule in &policy.rules {
        if rule.agent != agent || (rule.command != "*" && rule.command != command) {
            continue;
        }
        let applies = match &rule.accounts {
            Accounts::All => Tri::Yes,
            Accounts::Only(selectors) => any(selectors.iter().map(|s| denies(s, slot, identity))),
        };
        if applies == Tri::No {
            continue;
        }
        let exempt = any(rule.except.iter().map(|s| exempts(s, slot, identity)));
        match (applies, exempt) {
            (_, Tri::Yes) => {}
            (Tri::Yes, Tri::No) => {
                return Decision::Deny(Denial {
                    reason: DenyReason::Denied,
                    rule: Some(rule.id.clone()),
                });
            }
            _ => needs_identity = true,
        }
    }
    if needs_identity {
        Decision::NeedsIdentity
    } else {
        Decision::Allow
    }
}

/// Read the policy now and decide. Every dispatch path calls this.
pub(crate) fn decide(
    home: &Path,
    agent: &str,
    command: &str,
    slot: &str,
    identity: Identity<'_>,
) -> Decision {
    evaluate(&load(home), agent, command, slot, identity)
}

// ── reading ──────────────────────────────────────────────────────────────────

pub(crate) fn policy_path(home: &Path) -> PathBuf {
    home.join(POLICY_FILE)
}

/// Read the policy file as a dispatch sees it. Never fails: every problem is a
/// state the evaluation refuses on.
pub(crate) fn load(home: &Path) -> Loaded {
    match read_snapshot(&policy_path(home)) {
        Snapshot::Absent => Loaded::Absent,
        Snapshot::Unreadable(why) => Loaded::Unreadable { why },
        Snapshot::Bytes(bytes) => parse_lenient(&bytes),
    }
}

/// What is at the policy path.
enum Snapshot {
    /// Nothing: the open reported NotFound.
    Absent,
    Unreadable(String),
    Bytes(Vec<u8>),
}

/// Open without following a link and without blocking on a special file.
#[cfg(unix)]
fn open_no_follow(path: &Path) -> std::io::Result<std::fs::File> {
    use std::os::unix::fs::OpenOptionsExt;
    std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(path)
}

/// Open the entry itself (a link or junction is opened as the reparse point,
/// which the handle check then refuses).
#[cfg(windows)]
fn open_no_follow(path: &Path) -> std::io::Result<std::fs::File> {
    use std::os::windows::fs::OpenOptionsExt;
    std::fs::OpenOptions::new()
        .read(true)
        .custom_flags(windows_sys::Win32::Storage::FileSystem::FILE_FLAG_OPEN_REPARSE_POINT)
        .open(path)
}

#[cfg(not(any(unix, windows)))]
fn open_no_follow(path: &Path) -> std::io::Result<std::fs::File> {
    std::fs::File::open(path)
}

/// One bounded snapshot from one handle, opened without following a link; the
/// handle itself must be a plain regular file within the cap. Only a NotFound
/// open is "absent" — a dangling link, a directory, a FIFO, a permission error
/// are all unreadable, never an allow.
fn read_snapshot(path: &Path) -> Snapshot {
    use std::io::Read;
    let file = match open_no_follow(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Snapshot::Absent,
        Err(error) => {
            return Snapshot::Unreadable(format!("cannot open the policy file: {error}"));
        }
    };
    let meta = match file.metadata() {
        Ok(meta) => meta,
        Err(error) => {
            return Snapshot::Unreadable(format!("cannot stat the policy file: {error}"));
        }
    };
    if !meta.is_file() || crate::fs::is_reparse_point(&meta) {
        return Snapshot::Unreadable("the policy path is not a regular file".into());
    }
    if meta.len() > MAX_POLICY_BYTES {
        return Snapshot::Unreadable(format!(
            "the policy file is larger than {MAX_POLICY_BYTES} bytes"
        ));
    }
    let mut bytes = Vec::new();
    if let Err(error) = file.take(MAX_POLICY_BYTES + 1).read_to_end(&mut bytes) {
        return Snapshot::Unreadable(format!("cannot read the policy file: {error}"));
    }
    if bytes.len() as u64 > MAX_POLICY_BYTES {
        return Snapshot::Unreadable(format!(
            "the policy file is larger than {MAX_POLICY_BYTES} bytes"
        ));
    }
    Snapshot::Bytes(bytes)
}

/// The document's frame: valid JSON, an object, this schema, a `rules` array.
fn frame(bytes: &[u8]) -> Result<Vec<Json>, String> {
    let text =
        std::str::from_utf8(bytes).map_err(|_| "the policy file is not UTF-8".to_string())?;
    let value: Json = serde_json::from_str(text.trim_start_matches('\u{feff}'))
        .map_err(|error| format!("the policy file is not JSON: {error}"))?;
    let Json::Object(map) = value else {
        return Err("the policy is not a JSON object".into());
    };
    if let Some(field) = map.keys().find(|k| !POLICY_FIELDS.contains(&k.as_str())) {
        return Err(format!("unknown policy field {field:?}"));
    }
    match map.get("schema").and_then(Json::as_str) {
        Some(SCHEMA) => {}
        Some(other) => return Err(format!("unsupported policy schema {other:?}")),
        None => return Err("the policy has no schema".into()),
    }
    match map.get("rules") {
        Some(Json::Array(rules)) => Ok(rules.clone()),
        _ => Err("the policy has no rules array".into()),
    }
}

/// What a dispatch reads: a bad rule costs only its own agent, when its agent
/// is readable; otherwise the whole file is unreadable.
fn parse_lenient(bytes: &[u8]) -> Loaded {
    let rules = match frame(bytes) {
        Ok(rules) => rules,
        Err(why) => return Loaded::Unreadable { why },
    };
    let mut policy = Policy::default();
    let mut broken = Vec::new();
    for (index, raw) in rules.iter().enumerate() {
        match parse_rule(raw) {
            // A repeated id is a document no writer would produce, so which
            // rule was meant is unknown: both agents refuse, as a bad rule's
            // agent does.
            Ok(rule) => match policy.rules.iter().find(|r| r.id == rule.id) {
                Some(first) => {
                    broken.push((first.agent.clone(), Some(rule.id.clone())));
                    broken.push((rule.agent.clone(), Some(rule.id.clone())));
                }
                None => policy.rules.push(rule),
            },
            Err(why) => {
                let agent = raw
                    .get("agent")
                    .and_then(Json::as_str)
                    .filter(|a| crate::manifest::loader::is_safe_segment(a));
                let Some(agent) = agent else {
                    return Loaded::Unreadable {
                        why: format!("rule {index} has no readable agent: {why}"),
                    };
                };
                let id = raw
                    .get("id")
                    .and_then(Json::as_str)
                    .filter(|id| valid_id(id))
                    .map(str::to_string);
                broken.push((agent.to_string(), id));
            }
        }
    }
    Loaded::Readable { policy, broken }
}

/// What a writer accepts: every rule valid, ids unique.
pub(crate) fn parse_strict(bytes: &[u8]) -> Result<Policy, PolicyError> {
    let rules = frame(bytes).map_err(PolicyError::Invalid)?;
    let mut policy = Policy::default();
    for (index, raw) in rules.iter().enumerate() {
        let rule =
            parse_rule(raw).map_err(|why| PolicyError::Invalid(format!("rule {index}: {why}")))?;
        if policy.rules.iter().any(|r| r.id == rule.id) {
            return Err(PolicyError::Invalid(format!(
                "rule {index}: duplicate id {:?}",
                rule.id
            )));
        }
        policy.rules.push(rule);
    }
    Ok(policy)
}

fn valid_id(id: &str) -> bool {
    let mut chars = id.chars();
    id.chars().count() <= MAX_ID_CHARS
        && chars
            .next()
            .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        && chars
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '.' | '_' | '-'))
}

/// A slot or command name: printable, no whitespace, bounded.
fn valid_name(text: &str) -> bool {
    !text.is_empty()
        && text.chars().count() <= MAX_FIELD_CHARS
        && !text.chars().any(|c| c.is_control() || c.is_whitespace())
}

pub(crate) fn valid_binding(text: &str) -> bool {
    text.len() == 36
        && text.char_indices().all(|(i, c)| match i {
            8 | 13 | 18 | 23 => c == '-',
            _ => c.is_ascii_hexdigit(),
        })
}

fn text_field<'a>(map: &'a Map<String, Json>, field: &str) -> Result<&'a str, String> {
    map.get(field)
        .and_then(Json::as_str)
        .ok_or_else(|| format!("{field} must be a string"))
}

fn parse_selector(raw: &Json, need_binding: bool) -> Result<Selector, String> {
    let Json::Object(map) = raw else {
        return Err("an account selector must be an object".into());
    };
    if let Some(field) = map.keys().find(|k| !SELECTOR_FIELDS.contains(&k.as_str())) {
        return Err(format!("unknown selector field {field:?}"));
    }
    let slot = text_field(map, "slot")?;
    if !valid_name(slot) {
        return Err("slot is not a valid credential slot".into());
    }
    let binding = match map.get("binding") {
        None => None,
        Some(Json::String(binding)) if valid_binding(binding) => Some(binding.to_ascii_lowercase()),
        Some(_) => return Err("binding must be a binding id (a UUID)".into()),
    };
    if need_binding && binding.is_none() {
        return Err(
            "an except selector must name a binding: a slot alone can be re-signed to another account"
                .into(),
        );
    }
    Ok(Selector {
        slot: slot.to_string(),
        binding,
    })
}

fn parse_rule(raw: &Json) -> Result<Rule, String> {
    let Json::Object(map) = raw else {
        return Err("a rule must be an object".into());
    };
    if let Some(field) = map.keys().find(|k| !RULE_FIELDS.contains(&k.as_str())) {
        return Err(format!("unknown rule field {field:?}"));
    }
    let id = text_field(map, "id")?;
    if !valid_id(id) {
        return Err("id must be 1-128 of a-z 0-9 . _ - starting with a letter or digit".into());
    }
    let agent = text_field(map, "agent")?;
    if !crate::manifest::loader::is_safe_segment(agent) || !valid_name(agent) {
        return Err("agent is not a valid agent id".into());
    }
    let command = text_field(map, "command")?;
    if !valid_name(command) {
        return Err("command is not a valid command name".into());
    }
    let accounts = match map.get("accounts") {
        Some(Json::String(all)) if all == "*" => Accounts::All,
        Some(Json::Array(list)) if !list.is_empty() => Accounts::Only(
            list.iter()
                .map(|s| parse_selector(s, false))
                .collect::<Result<_, _>>()?,
        ),
        _ => return Err("accounts must be \"*\" or a non-empty list of selectors".into()),
    };
    let except = match map.get("except") {
        None => Vec::new(),
        Some(Json::Array(list)) => list
            .iter()
            .map(|s| parse_selector(s, true))
            .collect::<Result<_, _>>()?,
        Some(_) => return Err("except must be a list of selectors".into()),
    };
    let note = match map.get("note") {
        None => None,
        Some(Json::String(note)) if note.chars().count() <= MAX_NOTE_CHARS => Some(note.clone()),
        Some(_) => {
            return Err(format!(
                "note must be a string of at most {MAX_NOTE_CHARS} characters"
            ));
        }
    };
    Ok(Rule {
        id: id.to_string(),
        agent: agent.to_string(),
        command: command.to_string(),
        accounts,
        except,
        note,
    })
}

// ── JSON form ────────────────────────────────────────────────────────────────

fn selector_json(selector: &Selector) -> Json {
    let mut map = Map::new();
    map.insert("slot".into(), Json::String(selector.slot.clone()));
    if let Some(binding) = &selector.binding {
        map.insert("binding".into(), Json::String(binding.clone()));
    }
    Json::Object(map)
}

pub(crate) fn rule_json(rule: &Rule) -> Json {
    let mut map = Map::new();
    map.insert("id".into(), Json::String(rule.id.clone()));
    map.insert("agent".into(), Json::String(rule.agent.clone()));
    map.insert("command".into(), Json::String(rule.command.clone()));
    map.insert(
        "accounts".into(),
        match &rule.accounts {
            Accounts::All => Json::String("*".into()),
            Accounts::Only(list) => Json::Array(list.iter().map(selector_json).collect()),
        },
    );
    if !rule.except.is_empty() {
        map.insert(
            "except".into(),
            Json::Array(rule.except.iter().map(selector_json).collect()),
        );
    }
    if let Some(note) = &rule.note {
        map.insert("note".into(), Json::String(note.clone()));
    }
    Json::Object(map)
}

pub(crate) fn policy_json(policy: &Policy) -> Json {
    json!({
        "schema": SCHEMA,
        "rules": policy.rules.iter().map(rule_json).collect::<Vec<_>>(),
    })
}

// ── writing ──────────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub(crate) enum PolicyError {
    #[error("[E_COMMAND_POLICY_INVALID] {0}")]
    Invalid(String),
    #[error(
        "[E_COMMAND_POLICY_UNREADABLE] the existing policy file cannot be read ({0}); replace it whole with `aware policy set`"
    )]
    Unreadable(String),
    #[error("command policy file: {0}")]
    Io(#[from] std::io::Error),
}

impl From<PolicyError> for crate::error::AwareError {
    fn from(error: PolicyError) -> Self {
        match error {
            PolicyError::Io(io) => Self::Io(io),
            other => Self::Validation(other.to_string()),
        }
    }
}

/// Hold the writers' lock for the duration of `f`.
fn with_lock<T>(home: &Path, f: impl FnOnce() -> Result<T, PolicyError>) -> Result<T, PolicyError> {
    use fs2::FileExt;
    std::fs::create_dir_all(home)?;
    let lock = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(home.join(LOCK_FILE))?;
    lock.lock_exclusive()?;
    let out = f();
    let _ = FileExt::unlock(&lock);
    out
}

/// Replace the file with `policy`, atomically (a reader sees the old file or
/// the new one, never a mix). The caller holds the lock.
fn write_locked(home: &Path, policy: &Policy) -> Result<(), PolicyError> {
    use std::io::Write;
    let mut text = serde_json::to_string_pretty(&policy_json(policy))
        .map_err(|error| PolicyError::Invalid(error.to_string()))?;
    text.push('\n');
    // Never write a file a dispatch would read as unreadable: that would turn
    // a successful write into "refuse every credential-bearing command".
    if text.len() as u64 > MAX_POLICY_BYTES {
        return Err(PolicyError::Invalid(format!(
            "the policy would be {} bytes written, over the {MAX_POLICY_BYTES}-byte limit",
            text.len()
        )));
    }
    let mut temp = tempfile::NamedTempFile::new_in(home)?;
    temp.write_all(text.as_bytes())?;
    temp.as_file().sync_all()?;
    let temp = temp.into_temp_path();
    let path = policy_path(home);
    let replaced = crate::fs::replace_file(&temp, &path)?;
    // The temp path is gone (renamed); keeping it from deleting the new file.
    let _ = temp.keep();
    // The new policy is in effect; only its durability is in doubt, so this is
    // a warning, never "the write failed" (see `crate::fs::Replaced`).
    if let crate::fs::Replaced::NotDurable(error) = replaced {
        eprintln!(
            "\u{26a0} {} was replaced and is in effect, but making the change durable failed \
             ({error}); a power loss before the system flushes could bring back the previous policy",
            path.display()
        );
    }
    Ok(())
}

/// The policy as a writer sees it: absent is empty, unreadable is an error
/// (merging into it would drop the rules that could not be read).
fn read_for_update(home: &Path) -> Result<Policy, PolicyError> {
    let bytes = match read_snapshot(&policy_path(home)) {
        Snapshot::Absent => return Ok(Policy::default()),
        Snapshot::Unreadable(why) => return Err(PolicyError::Unreadable(why)),
        Snapshot::Bytes(bytes) => bytes,
    };
    parse_strict(&bytes).map_err(|error| match error {
        PolicyError::Invalid(why) => PolicyError::Unreadable(why),
        other => other,
    })
}

/// `aware policy set`: replace the whole policy.
pub(crate) fn set(home: &Path, policy: &Policy) -> Result<(), PolicyError> {
    with_lock(home, || write_locked(home, policy))
}

/// `aware policy deny`: insert the rule, or replace the one with its id.
/// Returns whether a rule was replaced.
pub(crate) fn upsert(home: &Path, rule: Rule) -> Result<bool, PolicyError> {
    // Validate through the same parser a dispatch uses.
    parse_rule(&rule_json(&rule)).map_err(PolicyError::Invalid)?;
    with_lock(home, || {
        let mut policy = read_for_update(home)?;
        let replaced = match policy.rules.iter_mut().find(|r| r.id == rule.id) {
            Some(existing) => {
                *existing = rule;
                true
            }
            None => {
                policy.rules.push(rule);
                false
            }
        };
        write_locked(home, &policy)?;
        Ok(replaced)
    })
}

/// `aware policy remove`: drop the rule with `id`. Returns whether one existed.
pub(crate) fn remove(home: &Path, id: &str) -> Result<bool, PolicyError> {
    with_lock(home, || {
        let mut policy = read_for_update(home)?;
        let before = policy.rules.len();
        policy.rules.retain(|r| r.id != id);
        let removed = policy.rules.len() != before;
        if removed {
            write_locked(home, &policy)?;
        }
        Ok(removed)
    })
}

#[cfg(test)]
#[path = "command_policy_tests.rs"]
mod tests;
