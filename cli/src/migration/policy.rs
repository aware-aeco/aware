//! Carry-forward policies (#628 plan §6, revised by §11–§15): a person's
//! standing approval for one narrow kind of update, so that a workflow may be
//! carried forward to it with no click.
//!
//! A policy lives in `AWARE_HOME/migration-policies/<policy-id>.yaml` and is
//! **immutable**: its id is derived from its own body, so a policy whose scope
//! was edited no longer matches its name and is refused as invalid. Revoking
//! one writes `<policy-id>.revoked.yaml` beside it; nothing is ever deleted.
//!
//! v1 knows one rule, `read-only-patch`: a `patch` update of an
//! official-registry tool, for a workflow declared read-only throughout, whose
//! run instructions are byte-identical (or, once one exists, pass an executed
//! fixed-state comparison). The rule, the publishers and the bump are closed
//! sets; the apps and agents a policy covers are named ids or `*`.
//!
//! The person who approved the policy is a CLAIM the front door recorded
//! (`attested: false`), exactly as a person approval of one promotion is: the
//! CLI cannot prove a person clicked (§12, owner decision 1 in
//! pawellisowski/floless.app#1985 — the FloLess click is the trust boundary).

use std::collections::BTreeSet;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use super::Reason;
use super::plan::{PlanRow, State};
use crate::error::AwareError;
use crate::paths::Paths;

/// The only policy format this CLI reads and writes.
pub const POLICY_FORMAT: u32 = 1;
/// The only policy approval record format this CLI reads.
pub const POLICY_RECORD_FORMAT: u32 = 1;
/// A scope entry that covers every app (or every agent).
pub const ANY: &str = "*";
/// The id prefix of every policy.
pub const ID_PREFIX: &str = "pol-";

const REVOKED_SUFFIX: &str = ".revoked.yaml";

/// What a policy allows. Closed: `delegated-maintenance` is not accepted in v1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Rule {
    ReadOnlyPatch,
}

/// Whose packages a policy accepts. Closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Publisher {
    OfficialRegistry,
}

/// Which version moves a policy accepts. Closed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Bump {
    Patch,
}

/// What a policy covers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Scope {
    /// App ids, or `*`.
    pub apps: Vec<String>,
    /// Agent ids, or `*`.
    pub agents: Vec<String>,
    pub publishers: Vec<Publisher>,
    pub bump: Bump,
}

/// Who approved a policy — a claim the front door recorded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct ApprovedBy {
    pub actor: String,
    pub front_door: String,
    pub approval_ref: String,
    /// `sha256:` of the exact plain-English policy text the person was shown.
    pub statement_sha256: String,
    pub at: String,
    /// Always `false`: the CLI cannot prove a person approved it.
    pub attested: bool,
    /// `sha256:` of the front door's approval record file, as given.
    pub approval_record_digest: String,
}

/// A stored policy.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Policy {
    pub policy: String,
    pub format: u32,
    pub rule: Rule,
    pub scope: Scope,
    pub approved_by: ApprovedBy,
}

/// The record a front door writes when a person approves a policy (§12).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct PolicyApprovalRecord {
    pub format: u32,
    pub kind: String,
    pub actor: String,
    pub front_door: String,
    pub approval_ref: String,
    pub statement_sha256: String,
    pub at: String,
    pub rule: Rule,
    pub scope: Scope,
}

/// A revocation, `<policy-id>.revoked.yaml`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Revocation {
    pub policy: String,
    pub revoked_by: String,
    pub front_door: String,
    pub revoked_at: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// A policy read from the store.
#[derive(Debug, Clone)]
pub struct Loaded {
    pub policy: Policy,
    /// `sha256:` of the policy file's exact bytes.
    pub digest: String,
    pub path: PathBuf,
    pub revoked: Option<Revocation>,
}

fn invalid(what: impl std::fmt::Display) -> AwareError {
    AwareError::Validation(format!("[E_MIGRATE_POLICY_INVALID] {what}"))
}

fn policy_path(paths: &Paths, id: &str) -> PathBuf {
    paths.migration_policies_dir().join(format!("{id}.yaml"))
}

fn revocation_path(paths: &Paths, id: &str) -> PathBuf {
    paths
        .migration_policies_dir()
        .join(format!("{id}{REVOKED_SUFFIX}"))
}

fn is_digest(value: &str) -> bool {
    crate::agent_store::digest_hex(value).is_some()
}

/// A policy id: `pol-` and 16 lowercase hex characters.
pub fn is_policy_id(id: &str) -> bool {
    id.strip_prefix(ID_PREFIX).is_some_and(|hex| {
        hex.len() == 16
            && hex
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    })
}

fn check_ids(field: &str, ids: &[String]) -> Result<(), String> {
    if ids.is_empty() {
        return Err(format!(
            "scope.{field} is empty; name the {field} it covers, or `*`"
        ));
    }
    let mut seen = BTreeSet::new();
    for id in ids {
        if id != ANY && !crate::manifest::loader::is_safe_segment(id) {
            return Err(format!("scope.{field} entry {id:?} is not an id or `*`"));
        }
        if !seen.insert(id) {
            return Err(format!("scope.{field} names {id} twice"));
        }
    }
    if ids.len() > 1 && seen.contains(&ANY.to_string()) {
        return Err(format!(
            "scope.{field} mixes `*` with named ids; use one or the other"
        ));
    }
    Ok(())
}

/// The scope rules every stored policy obeys.
pub fn check_scope(scope: &Scope) -> Result<(), String> {
    check_ids("apps", &scope.apps)?;
    check_ids("agents", &scope.agents)?;
    if scope.publishers != [Publisher::OfficialRegistry] {
        return Err(
            "scope.publishers must be exactly [official-registry]: v1 carries forward only official tools"
                .into(),
        );
    }
    Ok(())
}

/// The id a policy body has: `pol-` + the first 16 hex of the sha256 of its
/// canonical JSON (everything but the id itself).
pub fn derive_id(
    rule: Rule,
    scope: &Scope,
    approved_by: &ApprovedBy,
) -> Result<String, AwareError> {
    let body = serde_json::json!({
        "format": POLICY_FORMAT,
        "rule": rule,
        "scope": scope,
        "approved-by": approved_by,
    });
    let canonical = super::contract::canonical_json(body);
    let digest = crate::app_lock::lock_digest(canonical.as_bytes());
    let hex = crate::agent_store::digest_hex(&digest)
        .ok_or_else(|| AwareError::Internal(format!("{digest} is not a digest")))?;
    let short = crate::text::cut_after_chars(hex, 16).unwrap_or(hex);
    Ok(format!("{ID_PREFIX}{short}"))
}

fn non_empty(field: &str, value: &str) -> Result<(), AwareError> {
    if value.trim().is_empty() {
        return Err(AwareError::Validation(format!(
            "[E_MIGRATE_APPROVAL_MISMATCH] the policy approval record's {field} is empty"
        )));
    }
    Ok(())
}

/// Record the policy a person approved, from the front door's approval
/// `record` (exact file bytes). `front_door` is the `--front-door` the caller
/// passed; the record must name the same one. Returns the stored policy and
/// whether it was newly written (re-recording the same approval is a no-op).
pub fn record(
    paths: &Paths,
    record_bytes: &[u8],
    front_door: &str,
) -> Result<(Loaded, bool), AwareError> {
    let record: PolicyApprovalRecord = serde_json::from_slice(record_bytes).map_err(|e| {
        AwareError::Validation(format!(
            "[E_MIGRATE_APPROVAL_MISMATCH] the policy approval record is not one this CLI reads: {e}"
        ))
    })?;
    if record.format != POLICY_RECORD_FORMAT {
        return Err(AwareError::Validation(format!(
            "[E_MIGRATE_APPROVAL_MISMATCH] policy approval record format {} is not one this CLI reads (it reads {POLICY_RECORD_FORMAT})",
            record.format
        )));
    }
    if record.kind != "policy" {
        return Err(AwareError::Validation(format!(
            "[E_MIGRATE_APPROVAL_MISMATCH] the approval record is of kind {:?}, not a policy approval",
            record.kind
        )));
    }
    non_empty("actor", &record.actor)?;
    non_empty("approval-ref", &record.approval_ref)?;
    non_empty("at", &record.at)?;
    if record.front_door != front_door {
        return Err(AwareError::Validation(format!(
            "[E_MIGRATE_APPROVAL_MISMATCH] the approval record was written by front door {:?}, but --front-door is {front_door:?}",
            record.front_door
        )));
    }
    if !is_digest(&record.statement_sha256) {
        return Err(AwareError::Validation(format!(
            "[E_MIGRATE_APPROVAL_MISMATCH] statement-sha256 {:?} is not a sha256 digest",
            record.statement_sha256
        )));
    }
    check_scope(&record.scope)
        .map_err(|e| AwareError::Validation(format!("[E_MIGRATE_APPROVAL_MISMATCH] {e}")))?;
    let approved_by = ApprovedBy {
        actor: record.actor.trim().to_string(),
        front_door: record.front_door.clone(),
        approval_ref: record.approval_ref.clone(),
        statement_sha256: record.statement_sha256.clone(),
        at: record.at.clone(),
        attested: false,
        approval_record_digest: crate::app_lock::lock_digest(record_bytes),
    };
    let id = derive_id(record.rule, &record.scope, &approved_by)?;
    let policy = Policy {
        policy: id.clone(),
        format: POLICY_FORMAT,
        rule: record.rule,
        scope: record.scope,
        approved_by,
    };
    let yaml = serde_yaml::to_string(&policy)
        .map_err(|e| AwareError::Internal(format!("serialize policy: {e}")))?;
    let bytes = format!(
        "# carry-forward policy {id} — immutable; revoke with `aware app migrate policy revoke {id}`\n{yaml}"
    )
    .into_bytes();
    let dir = paths.migration_policies_dir();
    std::fs::create_dir_all(&dir)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", dir.display())))?;
    let path = policy_path(paths, &id);
    let created = match std::fs::read(&path) {
        Ok(existing) if existing == bytes => false,
        Ok(_) => {
            return Err(AwareError::Validation(format!(
                "[E_MIGRATE_POLICY_CONFLICT] {} already exists with different contents; a policy is immutable and its id names its body, so this file was changed by hand — nothing was written",
                path.display()
            )));
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            match crate::app_lock::replace_atomically(&path, &bytes)
                .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", path.display())))?
            {
                crate::fs::Replaced::Durable => {}
                crate::fs::Replaced::NotDurable(error) => eprintln!(
                    "\u{26a0} {} was written, but making it durable failed ({error})",
                    path.display()
                ),
            }
            true
        }
        Err(error) => {
            return Err(
                std::io::Error::new(error.kind(), format!("{}: {error}", path.display())).into(),
            );
        }
    };
    Ok((load(paths, &id)?, created))
}

/// Read policy `id`: it must exist, parse, name itself, and carry the id its
/// own body derives (an edited policy is invalid, never silently obeyed).
pub fn load(paths: &Paths, id: &str) -> Result<Loaded, AwareError> {
    if !is_policy_id(id) {
        return Err(AwareError::Validation(format!(
            "[E_MIGRATE_POLICY_NOT_FOUND] {id:?} is not a policy id (pol- and 16 hex characters)"
        )));
    }
    let path = policy_path(paths, id);
    let bytes = match std::fs::read(&path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Err(AwareError::Validation(format!(
                "[E_MIGRATE_POLICY_NOT_FOUND] no policy {id} ({})",
                path.display()
            )));
        }
        Err(error) => {
            return Err(
                std::io::Error::new(error.kind(), format!("{}: {error}", path.display())).into(),
            );
        }
    };
    let policy: Policy =
        serde_yaml::from_slice(&bytes).map_err(|e| invalid(format!("{}: {e}", path.display())))?;
    if policy.policy != id {
        return Err(invalid(format!(
            "{} names policy {}, not {id}",
            path.display(),
            policy.policy
        )));
    }
    if policy.format != POLICY_FORMAT {
        return Err(invalid(format!(
            "{}: format {} is not one this CLI reads",
            path.display(),
            policy.format
        )));
    }
    check_scope(&policy.scope).map_err(|e| invalid(format!("{}: {e}", path.display())))?;
    if policy.approved_by.attested {
        return Err(invalid(format!(
            "{}: it claims an attested approval; this CLI records approvals as claims (attested: false) and cannot have written it",
            path.display()
        )));
    }
    let derived = derive_id(policy.rule, &policy.scope, &policy.approved_by)?;
    if derived != id {
        return Err(invalid(format!(
            "{}: its contents derive the id {derived}, not {id} — the policy was changed after it was recorded",
            path.display()
        )));
    }
    Ok(Loaded {
        digest: crate::app_lock::lock_digest(&bytes),
        revoked: read_revocation(paths, id),
        policy,
        path,
    })
}

/// A policy's revocation, if any. A revocation file that cannot be read still
/// revokes (fail safe): it is reported with an unknown revoker.
fn read_revocation(paths: &Paths, id: &str) -> Option<Revocation> {
    let path = revocation_path(paths, id);
    match std::fs::read(&path) {
        Ok(bytes) => Some(
            serde_yaml::from_slice::<Revocation>(&bytes)
                .ok()
                .filter(|r| r.policy == id)
                .unwrap_or_else(|| Revocation {
                    policy: id.to_string(),
                    revoked_by: "(unknown — the revocation file could not be read)".into(),
                    front_door: String::new(),
                    revoked_at: String::new(),
                    reason: Some(format!(
                        "{} exists but is not a revocation of {id}; it still revokes",
                        path.display()
                    )),
                }),
        ),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => None,
        Err(error) => Some(Revocation {
            policy: id.to_string(),
            revoked_by: "(unknown — the revocation file could not be read)".into(),
            front_door: String::new(),
            revoked_at: String::new(),
            reason: Some(format!("{}: {error}", path.display())),
        }),
    }
}

/// The policies lock (§15.1 R3): a policy promotion holds it SHARED from
/// loading its policy until the app's lock has moved; a revocation takes it
/// EXCLUSIVE, so it lands before or after a promotion, never in between.
/// Last in the lock order (store lock, app promotion lock, this).
pub fn lock_policies(paths: &Paths, exclusive: bool) -> Result<std::fs::File, AwareError> {
    use fs2::FileExt;
    let dir = paths.migration_locks_dir();
    std::fs::create_dir_all(&dir)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", dir.display())))?;
    let path = dir.join("policies.flock");
    let file = std::fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(&path)
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", path.display())))?;
    let locked = if exclusive {
        file.lock_exclusive()
    } else {
        file.lock_shared()
    };
    locked.map_err(|e| std::io::Error::new(e.kind(), format!("take {}: {e}", path.display())))?;
    Ok(file)
}

/// Revoke policy `id`. Idempotent: an existing revocation is returned as is
/// (`false` = it was already revoked). A policy file that no longer reads as a
/// valid policy can still be revoked — revoking is always the safe direction.
pub fn revoke(
    paths: &Paths,
    id: &str,
    actor: &str,
    front_door: &str,
    reason: Option<String>,
) -> Result<(Revocation, bool), AwareError> {
    if !is_policy_id(id) || !policy_path(paths, id).is_file() {
        return Err(AwareError::Validation(format!(
            "[E_MIGRATE_POLICY_NOT_FOUND] no policy {id}"
        )));
    }
    let _policies = lock_policies(paths, true)?;
    if let Some(existing) = read_revocation(paths, id) {
        return Ok((existing, false));
    }
    let revocation = Revocation {
        policy: id.to_string(),
        revoked_by: actor.to_string(),
        front_door: front_door.to_string(),
        revoked_at: chrono::Utc::now().to_rfc3339(),
        reason,
    };
    let yaml = serde_yaml::to_string(&revocation)
        .map_err(|e| AwareError::Internal(format!("serialize revocation: {e}")))?;
    let path = revocation_path(paths, id);
    match crate::app_lock::replace_atomically(&path, yaml.as_bytes())
        .map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", path.display())))?
    {
        crate::fs::Replaced::Durable => {}
        crate::fs::Replaced::NotDurable(error) => eprintln!(
            "\u{26a0} {} was written, but making it durable failed ({error})",
            path.display()
        ),
    }
    Ok((revocation, true))
}

/// One row of `migrate policy list`.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListEntry {
    pub policy: String,
    /// `active` | `revoked` | `invalid`.
    pub state: &'static str,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<Rule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Scope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_by: Option<ApprovedBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revoked: Option<Revocation>,
    /// Why an `invalid` policy cannot be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub problem: Option<String>,
    /// The plain-English label.
    pub label: String,
}

/// Every policy file in the store, sorted by id. An unreadable one is listed
/// `invalid` with its problem, never hidden.
pub fn list(paths: &Paths) -> Result<Vec<ListEntry>, AwareError> {
    let dir = paths.migration_policies_dir();
    let entries = match std::fs::read_dir(&dir) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => {
            return Err(
                std::io::Error::new(error.kind(), format!("{}: {error}", dir.display())).into(),
            );
        }
    };
    let mut ids = BTreeSet::new();
    for entry in entries {
        let entry =
            entry.map_err(|e| std::io::Error::new(e.kind(), format!("{}: {e}", dir.display())))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.ends_with(REVOKED_SUFFIX) {
            continue;
        }
        if let Some(id) = name.strip_suffix(".yaml") {
            ids.insert(id.to_string());
        }
    }
    Ok(ids
        .into_iter()
        .map(|id| match load(paths, &id) {
            Ok(loaded) => {
                let state = if loaded.revoked.is_some() {
                    "revoked"
                } else {
                    "active"
                };
                ListEntry {
                    label: label(&loaded),
                    policy: id,
                    state,
                    path: loaded.path.display().to_string(),
                    digest: Some(loaded.digest),
                    rule: Some(loaded.policy.rule),
                    scope: Some(loaded.policy.scope),
                    approved_by: Some(loaded.policy.approved_by),
                    revoked: loaded.revoked,
                    problem: None,
                }
            }
            Err(error) => ListEntry {
                label: format!("policy {id} cannot be used: {error}"),
                path: policy_path(paths, &id).display().to_string(),
                policy: id,
                state: "invalid",
                digest: None,
                rule: None,
                scope: None,
                approved_by: None,
                revoked: None,
                problem: Some(error.to_string()),
            },
        })
        .collect())
}

/// "a", "a and b", "a, b and c".
fn english_list(ids: &[String]) -> String {
    match ids {
        [] => String::new(),
        [only] => only.clone(),
        [rest @ .., last] => format!("{} and {last}", rest.join(", ")),
    }
}

/// The scope's ids as a noun phrase: "any official tool", "the official tool
/// tekla", "the official tools tekla and verbot" (`adjective` may be empty).
fn ids_phrase(ids: &[String], adjective: &str, noun: &str) -> String {
    let adjective = if adjective.is_empty() {
        String::new()
    } else {
        format!("{adjective} ")
    };
    if ids.iter().any(|id| id == ANY) {
        format!("any {adjective}{noun}")
    } else {
        let plural = if ids.len() == 1 { "" } else { "s" };
        format!("the {adjective}{noun}{plural} {}", english_list(ids))
    }
}

/// The word a publisher set reads as ("official"). Exhaustive, so a new
/// publisher cannot be worded by accident.
fn publishers_word(publishers: &[Publisher]) -> String {
    let words: BTreeSet<&str> = publishers
        .iter()
        .map(|p| match p {
            Publisher::OfficialRegistry => "official",
        })
        .collect();
    words.into_iter().collect::<Vec<_>>().join(" or ")
}

/// "policy pol-…: carries forward declared read-only patch updates of the
/// official tool tekla, for the workflows a and b — claimed approval by pawel,
/// recorded by floless@1.2.3". A pure function of the stored policy.
pub fn label(loaded: &Loaded) -> String {
    let p = &loaded.policy;
    let rule = match p.rule {
        Rule::ReadOnlyPatch => "declared read-only",
    };
    let bump = match p.scope.bump {
        Bump::Patch => "patch",
    };
    let mut text = format!(
        "policy {}: carries forward {rule} {bump} updates of {}, for {} — claimed approval by {}, recorded by {}",
        p.policy,
        ids_phrase(
            &p.scope.agents,
            &publishers_word(&p.scope.publishers),
            "tool"
        ),
        ids_phrase(&p.scope.apps, "", "workflow"),
        p.approved_by.actor,
        p.approved_by.front_door
    );
    if let Some(revoked) = &loaded.revoked {
        text.push_str(&format!("; revoked by {}", revoked.revoked_by));
    }
    text
}

fn covers(ids: &[String], id: &str) -> bool {
    ids.iter().any(|entry| entry == ANY || entry == id)
}

/// Why `policy` does NOT cover carrying `row` forward, judged on the facts a
/// plan row records (empty = it covers it). `migrate promote --policy` adds a
/// fresh official-registry check of every new package on top.
pub fn ineligibility(loaded: &Loaded, row: &PlanRow) -> Vec<Reason> {
    let p = &loaded.policy;
    let mut out = Vec::new();
    if let Some(revoked) = &loaded.revoked {
        out.push(Reason::new(
            "policy-revoked",
            format!(
                "Policy {} was revoked by {}, so it carries nothing forward.",
                p.policy, revoked.revoked_by
            ),
        ));
    }
    match row.state {
        State::Held => out.push(Reason::new(
            "held",
            "A person put this workflow on hold, so no policy carries it forward.",
        )),
        State::Blocked => out.push(Reason::new(
            "blocked",
            "This update cannot be carried forward until a person fixes the workflow (see the other reasons).",
        )),
        State::UpToDate if row.targets.is_empty() => out.push(Reason::new(
            "nothing-to-carry",
            "No tool this workflow uses would move, so there is nothing to carry forward.",
        )),
        _ => {}
    }
    if !covers(&p.scope.apps, &row.app) {
        out.push(Reason::new(
            "policy-app-out-of-scope",
            format!(
                "Policy {} does not cover the workflow {}.",
                p.policy, row.app
            ),
        ));
    }
    for target in &row.targets {
        if !covers(&p.scope.agents, &target.agent) {
            out.push(Reason::new(
                "policy-agent-out-of-scope",
                format!(
                    "Policy {} does not cover the tool {}.",
                    p.policy, target.agent
                ),
            ));
        }
        if target.bump != "patch" {
            out.push(Reason::new(
                "policy-not-patch",
                format!(
                    "{} {} -> {} is a {} change; a policy carries forward patch updates only.",
                    target.agent, target.from.version, target.to.version, target.bump
                ),
            ));
        }
        if target.publisher != "official-registry" {
            out.push(Reason::new(
                "policy-not-official",
                format!(
                    "{} {} was not installed from the official registry (its install receipt says {}); a policy carries forward official tools only.",
                    target.agent, target.to.version, target.publisher
                ),
            ));
        }
    }
    if !row.targets.is_empty() {
        if row.effect != Some("declared-read-only") {
            out.push(Reason::new(
                "not-declared-read-only",
                "The workflow is not declared read-only throughout, so a policy cannot carry it forward.",
            ));
        }
        if !row.contract.as_ref().is_some_and(|c| c.unchanged) {
            out.push(Reason::new(
                "contract-changed",
                "The tools' run instructions changed, so a policy cannot carry it forward.",
            ));
        }
        if !row
            .comparison
            .as_ref()
            .is_some_and(super::compare::Comparison::accepted_for_policy)
        {
            out.push(Reason::new(
                "comparison-not-accepted",
                "The update has no accepted fixed-state comparison (identical instructions or an executed pass), so a policy cannot carry it forward.",
            ));
        }
    }
    if row.backing_app_moved {
        out.push(Reason::new(
            "backing-app-moved",
            "A workflow other workflows run as a tool, or a move of one, is never carried forward under a policy.",
        ));
    }
    out
}

/// The active policies, sorted by id. Unreadable ones are skipped here (`list`
/// reports them); a revoked one is never active.
pub fn active(paths: &Paths) -> Vec<Loaded> {
    list(paths)
        .unwrap_or_default()
        .into_iter()
        .filter(|entry| entry.state == "active")
        .filter_map(|entry| load(paths, &entry.policy).ok())
        .collect()
}

/// The first active policy that covers `row`, if any.
pub fn covering(paths: &Paths, row: &PlanRow) -> Option<Loaded> {
    active(paths)
        .into_iter()
        .find(|loaded| ineligibility(loaded, row).is_empty())
}

/// The file of policy `id` (tests tamper with it).
#[cfg(test)]
pub(crate) fn file_of(paths: &Paths, id: &str) -> PathBuf {
    policy_path(paths, id)
}

#[cfg(test)]
mod tests;
