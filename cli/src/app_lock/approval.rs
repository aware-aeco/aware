//! Successor approval records (#628 PR3a): the `approval:` block of a lock
//! whose agent pins were carried forward from an earlier approval.
//!
//! A person's `aware app compile` writes an ORIGINAL approval: a lock with no
//! `approval:` block. A promotion (#628 PR3b — not in this CLI yet) replaces
//! the lock FILE with the promoted plan at the top level and preserves the
//! original approval RECORD: the exact original lock bytes are archived
//! content-addressed under `.aware-approvals/<hex>.lock`, its approval fields
//! are copied verbatim into `approval.original`, and every promotion appends a
//! successor link. The source-hash of a promoted lock is
//! `successor-v1:<hex>`, which an older CLI compares with the raw source hash
//! and so refuses (`E_APP_LOCK_STALE`) instead of running new pins unlabelled.
//!
//! This module only READS that record: [`check_chain`] is the structural
//! consistency of the block (no I/O), [`assess`] verifies it against the
//! archives beside the lock and builds what `app check` and the run record
//! report. A broken chain is `E_APP_LOCK_INVALID`; a missing archive is never a
//! refusal — the record is reported `incomplete` and the run proceeds labelled
//! (repo rule "nothing refuses, it asks").

use super::*;

/// The only `approval.format` this CLI reads.
pub const APPROVAL_FORMAT: u32 = 1;

/// The source-hash prefix of a promoted lock (plan §13).
pub const SUCCESSOR_SOURCE_PREFIX: &str = "successor-v1:";

/// The approval-records directory beside the source (shared with HOLD files).
pub const ARCHIVE_DIR: &str = crate::migration::files::APPROVALS_DIR;

/// The `approval:` block of a promoted lock.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ApprovalChain {
    pub format: u32,
    pub original: OriginalApproval,
    #[serde(default)]
    pub successors: Vec<Successor>,
}

/// The original approval's fields, copied verbatim from the archived lock.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct OriginalApproval {
    /// `sha256:` of the original lock file's exact bytes.
    pub lock_digest: String,
    /// Where those bytes are archived: `.aware-approvals/<hex>.lock`.
    pub archive: String,
    pub compiled_at: String,
    pub compiler_version: String,
    /// The front door that recorded the compile, when it said.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub front_door: Option<String>,
    pub agent_pins: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub agent_digests: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub agent_bundle_pins: BTreeMap<String, String>,
}

/// What one successor link did.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SuccessorKind {
    CarriedForward,
    Reverted,
}

/// One agent's pin in a link's `from` / `to`: the full pin of every agent the
/// lock pins, not only the ones that moved.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ApprovalPin {
    pub version: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_pin: Option<String>,
}

/// Who carried an approval forward. A person approval is a CLAIM the front
/// door recorded: the CLI cannot prove a person clicked (`attested: false`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum CarriedForwardBy {
    #[serde(rename_all = "kebab-case")]
    Person {
        actor: String,
        approval_ref: String,
        front_door: String,
        attested: bool,
        /// `sha256:` of the approval record archived as `.aware-approvals/<hex>.json`.
        approval_record_digest: String,
    },
    #[serde(rename_all = "kebab-case")]
    Policy {
        policy_id: String,
        policy_digest: String,
        policy_approved_by: String,
    },
}

impl CarriedForwardBy {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Person { .. } => "person",
            Self::Policy { .. } => "policy",
        }
    }
}

/// One successor link.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Successor {
    pub seq: u32,
    pub kind: SuccessorKind,
    /// `sha256:` of the exact bytes of the lock this link replaced.
    pub from_lock_digest: String,
    /// [`plan_digest`] of the plan this link approved.
    pub to_plan_digest: String,
    /// `sha256:` of the resulting plan's bytes as promoted — the candidate
    /// lock, before the approval record was attached — archived as
    /// `.aware-approvals/<hex>.lock`.
    pub resulting_lock_digest: String,
    pub from: BTreeMap<String, ApprovalPin>,
    pub to: BTreeMap<String, ApprovalPin>,
    pub carried_forward_by: CarriedForwardBy,
    pub evidence_digest: String,
    /// `.aware-approvals/<hex>.json` of `evidence-digest`.
    pub evidence: String,
    pub front_door: String,
    pub cli_version: String,
    pub promoted_at: String,
}

/// The archive path (relative to the source directory) of `digest`.
pub fn archive_rel(digest: &str, ext: &str) -> Option<String> {
    crate::agent_store::digest_hex(digest).map(|hex| format!("{ARCHIVE_DIR}/{hex}.{ext}"))
}

/// The full pin map of a lock's top level.
pub fn pins_of(lock: &LockFile) -> BTreeMap<String, ApprovalPin> {
    pin_map(
        &lock.agent_pins,
        &lock.agent_digests,
        &lock.agent_bundle_pins,
    )
}

fn original_pins(original: &OriginalApproval) -> BTreeMap<String, ApprovalPin> {
    pin_map(
        &original.agent_pins,
        &original.agent_digests,
        &original.agent_bundle_pins,
    )
}

fn pin_map(
    pins: &BTreeMap<String, String>,
    digests: &BTreeMap<String, String>,
    bundles: &BTreeMap<String, String>,
) -> BTreeMap<String, ApprovalPin> {
    pins.iter()
        .map(|(id, version)| {
            (
                id.clone(),
                ApprovalPin {
                    version: version.clone(),
                    digest: digests.get(id).cloned(),
                    bundle_pin: bundles.get(id).cloned(),
                },
            )
        })
        .collect()
}

/// The source hash a lock approves, in the raw `sha256:<hex>` form: a
/// promoted lock's `successor-v1:<hex>` names the same source.
pub fn approved_source_hash(lock: &LockFile) -> String {
    match lock.source_hash.strip_prefix(SUCCESSOR_SOURCE_PREFIX) {
        Some(hex) => format!("sha256:{hex}"),
        None => lock.source_hash.clone(),
    }
}

/// Whether `lock` approves the source whose hash is `current`. The prefix is
/// only honoured on a lock whose chain [`check_chain`] accepts — every caller
/// checks that first and reports a prefix without a chain as invalid.
pub fn source_matches(lock: &LockFile, current: &str) -> bool {
    approved_source_hash(lock) == current
}

/// Whether a lock carries anything this module must check.
pub fn has_record(lock: &LockFile) -> bool {
    lock.approval.is_some() || lock.source_hash.starts_with(SUCCESSOR_SOURCE_PREFIX)
}

fn sha(field: &str, digest: &str) -> Result<(), String> {
    crate::agent_store::digest_hex(digest)
        .map(|_| ())
        .ok_or_else(|| format!("{field} {digest:?} is not a sha256 digest"))
}

fn non_empty(field: &str, value: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err(format!("{field} is empty"))
    } else {
        Ok(())
    }
}

fn maps_consistent(
    what: &str,
    pins: &BTreeMap<String, String>,
    digests: &BTreeMap<String, String>,
    bundles: &BTreeMap<String, String>,
) -> Result<(), String> {
    for (id, digest) in digests {
        sha(&format!("{what} agent-digests[{id}]"), digest)?;
        if !pins.contains_key(id) {
            return Err(format!(
                "{what} agent-digests names {id}, which its agent-pins does not pin"
            ));
        }
    }
    for (id, bundle) in bundles {
        sha(&format!("{what} agent-bundle-pins[{id}]"), bundle)?;
        if !pins.contains_key(id) {
            return Err(format!(
                "{what} agent-bundle-pins names {id}, which its agent-pins does not pin"
            ));
        }
        if let Some(digest) = digests.get(id)
            && digest != bundle
        {
            return Err(format!(
                "{what} agent-digests[{id}] is {digest} but agent-bundle-pins[{id}] is {bundle}"
            ));
        }
    }
    Ok(())
}

/// The structural consistency of a lock's approval record (plan §12-§13),
/// reading nothing but the lock. `Err` is the reason it is
/// `E_APP_LOCK_INVALID`. A lock with no record and a plain source hash is `Ok`.
pub fn check_chain(lock: &LockFile) -> Result<(), String> {
    let prefixed = lock.source_hash.strip_prefix(SUCCESSOR_SOURCE_PREFIX);
    let Some(chain) = &lock.approval else {
        return match prefixed {
            Some(_) => Err(format!(
                "source-hash {} names a carried-forward approval but the lock has no approval record",
                lock.source_hash
            )),
            None => Ok(()),
        };
    };
    if chain.format != APPROVAL_FORMAT {
        return Err(format!(
            "approval format {} is not one this CLI reads (it reads format {APPROVAL_FORMAT})",
            chain.format
        ));
    }
    if chain.successors.is_empty() {
        return Err(
            "the approval record has no successor; an original approval is a lock with no approval block"
                .into(),
        );
    }
    let Some(hex) = prefixed else {
        return Err(format!(
            "the lock carries a successor approval record but its source-hash {} is not {SUCCESSOR_SOURCE_PREFIX}<hex>",
            lock.source_hash
        ));
    };
    sha("source-hash", &format!("sha256:{hex}"))?;

    let original = &chain.original;
    sha("approval.original.lock-digest", &original.lock_digest)?;
    if Some(&original.archive) != archive_rel(&original.lock_digest, "lock").as_ref() {
        return Err(format!(
            "approval.original.archive {:?} is not the content-addressed archive of {}",
            original.archive, original.lock_digest
        ));
    }
    maps_consistent(
        "approval.original",
        &original.agent_pins,
        &original.agent_digests,
        &original.agent_bundle_pins,
    )?;
    maps_consistent(
        "the lock's",
        &lock.agent_pins,
        &lock.agent_digests,
        &lock.agent_bundle_pins,
    )?;

    let original_maps = original_pins(original);
    let mut previous_to = &original_maps;
    let mut approved_before: Vec<&BTreeMap<String, ApprovalPin>> = vec![&original_maps];
    for (index, link) in chain.successors.iter().enumerate() {
        let n = index + 1;
        let at = format!("successor {n}");
        if usize::try_from(link.seq).ok() != Some(n) {
            return Err(format!(
                "successor seq {} is out of order: seq must run 1, 2, 3, ... and this is entry {n}",
                link.seq
            ));
        }
        sha(&format!("{at} from-lock-digest"), &link.from_lock_digest)?;
        sha(&format!("{at} to-plan-digest"), &link.to_plan_digest)?;
        sha(
            &format!("{at} resulting-lock-digest"),
            &link.resulting_lock_digest,
        )?;
        sha(&format!("{at} evidence-digest"), &link.evidence_digest)?;
        if Some(&link.evidence) != archive_rel(&link.evidence_digest, "json").as_ref() {
            return Err(format!(
                "{at} evidence {:?} is not the content-addressed archive of {}",
                link.evidence, link.evidence_digest
            ));
        }
        non_empty(&format!("{at} front-door"), &link.front_door)?;
        non_empty(&format!("{at} cli-version"), &link.cli_version)?;
        non_empty(&format!("{at} promoted-at"), &link.promoted_at)?;
        match &link.carried_forward_by {
            CarriedForwardBy::Person {
                actor,
                approval_ref,
                front_door,
                attested,
                approval_record_digest,
            } => {
                non_empty(&format!("{at} actor"), actor)?;
                non_empty(&format!("{at} approval-ref"), approval_ref)?;
                non_empty(&format!("{at} carried-forward-by.front-door"), front_door)?;
                sha(
                    &format!("{at} approval-record-digest"),
                    approval_record_digest,
                )?;
                if *attested {
                    return Err(format!(
                        "{at} claims an attested person approval; this CLI records person approvals as claims (attested: false) and cannot have written it"
                    ));
                }
            }
            CarriedForwardBy::Policy {
                policy_id,
                policy_digest,
                policy_approved_by,
            } => {
                non_empty(&format!("{at} policy-id"), policy_id)?;
                sha(&format!("{at} policy-digest"), policy_digest)?;
                non_empty(&format!("{at} policy-approved-by"), policy_approved_by)?;
            }
        }
        if n == 1 && link.from_lock_digest != original.lock_digest {
            return Err(format!(
                "successor 1 replaced lock {} but the original approval is {}",
                link.from_lock_digest, original.lock_digest
            ));
        }
        if &link.from != previous_to {
            return Err(if n == 1 {
                "successor 1 starts from pins that are not the original approval's".to_string()
            } else {
                format!(
                    "successor {n} starts from pins that are not the ones successor {} carried forward to",
                    n - 1
                )
            });
        }
        if link.kind == SuccessorKind::Reverted && !approved_before.contains(&&link.to) {
            return Err(format!(
                "{at} is a revert to pins that were never approved before it"
            ));
        }
        approved_before.push(&link.to);
        previous_to = &link.to;
    }
    let Some(last) = chain.successors.last() else {
        return Err("the approval record has no successor".into());
    };
    if *previous_to != pins_of(lock) {
        return Err(format!(
            "the lock's agent-pins / agent-digests / agent-bundle-pins are not the pins successor {} carried forward to",
            last.seq
        ));
    }
    let plan = plan_digest(lock).map_err(|e| e.to_string())?;
    if plan != last.to_plan_digest {
        return Err(format!(
            "the lock's plan ({plan}) is not the plan successor {} approved ({})",
            last.seq, last.to_plan_digest
        ));
    }
    Ok(())
}

// ── Archive verification + reporting ───────────────────────────────────────

/// Where a lock's approval comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Origin {
    /// A person compiled it (`aware app compile`).
    Original,
    /// It was carried forward from an earlier approval.
    Successor,
}

/// `app check`'s `successors[]` row.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct SuccessorRow {
    pub seq: u32,
    pub kind: SuccessorKind,
    /// The agents this link moved, at their old pins.
    pub from: BTreeMap<String, ApprovalPin>,
    /// The same agents at their new pins.
    pub to: BTreeMap<String, ApprovalPin>,
    pub by_kind: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    /// Person approvals only: always `false` — a claim, not a proof.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attested: Option<bool>,
    pub front_door: String,
    pub promoted_at: String,
    pub label: String,
}

/// What `app check` and the run record say about a lock's approval.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "kebab-case")]
pub struct ApprovalSummary {
    pub origin: Origin,
    /// The last successor's seq; absent for an original.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<SuccessorKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub by_kind: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attested: Option<bool>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub from: BTreeMap<String, ApprovalPin>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    pub to: BTreeMap<String, ApprovalPin>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_digest: Option<String>,
    /// Every archive the record cites is present and verifies.
    pub record_complete: bool,
    /// The archives that are missing or cannot be read.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub missing: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub successors: Vec<SuccessorRow>,
    /// The plain-English label.
    pub label: String,
}

/// What a link's archived evidence says, for the label (never for a verdict).
#[derive(Debug, Default, Clone)]
struct EvidenceFacts {
    effect: Option<Recorded<DeclaredEffect>>,
    comparison: Option<Recorded<crate::migration::compare::ComparisonStatus>>,
    comparison_method: Option<String>,
    comparison_runs: Option<u64>,
    comparison_reason: Option<String>,
}

/// A value read from evidence: one this CLI knows, or one it does not — which
/// gets its own words rather than borrowing a known value's.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Recorded<T> {
    Known(T),
    Unknown(String),
}

impl<T: serde::de::DeserializeOwned> Recorded<T> {
    fn read(value: &serde_json::Value) -> Option<Self> {
        let text = value.as_str()?;
        Some(match serde_json::from_value(value.clone()) {
            Ok(known) => Self::Known(known),
            Err(_) => Self::Unknown(text.to_string()),
        })
    }
}

/// The plan row's `effect` (`migration::plan`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum DeclaredEffect {
    DeclaredReadOnly,
    NotDeclaredReadOnly,
}

impl EvidenceFacts {
    fn from_row(row: &serde_json::Value) -> Self {
        let comparison = &row["comparison"];
        Self {
            effect: Recorded::read(&row["effect"]),
            comparison: Recorded::read(&comparison["status"]),
            comparison_method: comparison["method"].as_str().map(str::to_string),
            comparison_runs: comparison["runs"].as_u64(),
            comparison_reason: comparison["reason"]["text"].as_str().map(str::to_string),
        }
    }
}

fn not_recognised(what: &str, value: &str) -> String {
    format!(
        "{what} {value:?}, which this version of AWARE does not recognise, so nothing is claimed about it"
    )
}

/// The words for a recorded effect. Exhaustive: every value has its own.
fn effect_words(effect: &Option<Recorded<DeclaredEffect>>) -> Option<String> {
    match effect {
        None => None,
        Some(Recorded::Known(DeclaredEffect::DeclaredReadOnly)) => {
            Some("declared read-only".into())
        }
        Some(Recorded::Known(DeclaredEffect::NotDeclaredReadOnly)) => {
            Some("not declared read-only, so it may write".into())
        }
        Some(Recorded::Unknown(value)) => Some(not_recognised("its effect is recorded as", value)),
    }
}

fn plural(n: u64, one: &str) -> String {
    format!("{n} {one}{}", if n == 1 { "" } else { "s" })
}

/// The words for a recorded comparison. Exhaustive: every status has its own
/// — a comparison that found different results is never worded as one that
/// did not run. `who` names who carried it forward regardless.
fn comparison_words(fact: &EvidenceFacts, verb: &str, who: &str) -> Option<String> {
    use crate::migration::compare::ComparisonStatus;
    let method = fact
        .comparison_method
        .as_deref()
        .unwrap_or("method not recorded");
    match &fact.comparison {
        None => None,
        Some(Recorded::Known(ComparisonStatus::Pass)) => Some(match fact.comparison_runs {
            Some(runs) if runs > 0 => format!(
                "passed {} on fixed state ({method})",
                plural(runs, "comparison")
            ),
            _ => "a comparison is recorded as passed but no run is recorded, so no comparison is claimed".into(),
        }),
        Some(Recorded::Known(ComparisonStatus::Fail)) => Some(format!(
            "the old and new versions gave different results when compared ({method}, {}) — {verb} anyway {who}",
            match fact.comparison_runs {
                Some(runs) => plural(runs, "run"),
                None => "runs not recorded".into(),
            }
        )),
        Some(Recorded::Known(ComparisonStatus::IdenticalInstructions)) => Some(
            "the tool's run instructions are byte-identical (checked by inspection, nothing was run)"
                .into(),
        ),
        Some(Recorded::Known(ComparisonStatus::NotComparable)) => Some(format!(
            "the results could not be compared ({})",
            fact.comparison_reason
                .as_deref()
                .unwrap_or("no reason recorded")
        )),
        Some(Recorded::Unknown(value)) => {
            Some(not_recognised("a comparison is recorded as", value))
        }
    }
}

enum Archive {
    Present(Vec<u8>),
    Missing(String),
}

fn read_archive(dir: &Path, digest: &str, ext: &str, what: &str) -> Result<Archive, String> {
    let rel =
        archive_rel(digest, ext).ok_or_else(|| format!("{what}: {digest:?} is not a digest"))?;
    let path = dir.join(&rel);
    match std::fs::read(&path) {
        Ok(bytes) => {
            if lock_digest(&bytes) != digest {
                return Err(format!(
                    "{what} {rel} does not hash to {digest}: the archived record was changed"
                ));
            }
            Ok(Archive::Present(bytes))
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            Ok(Archive::Missing(format!("{what} {rel}")))
        }
        Err(error) => Ok(Archive::Missing(format!(
            "{what} {rel} (cannot be read: {error})"
        ))),
    }
}

fn parse_archived_lock(bytes: &[u8], what: &str) -> Result<LockFile, String> {
    serde_yaml::from_slice(bytes).map_err(|e| format!("{what} is not a lock: {e}"))
}

/// Verify `lock`'s approval record against the archives in `source_dir` and
/// describe it. `Err`: the record contradicts an archive it cites (tampered)
/// — `E_APP_LOCK_INVALID`. Callers run [`check_chain`] first.
pub fn assess(lock: &LockFile, source_dir: &Path) -> Result<ApprovalSummary, String> {
    let Some(chain) = &lock.approval else {
        return Ok(ApprovalSummary {
            origin: Origin::Original,
            seq: None,
            kind: None,
            by_kind: None,
            actor: None,
            policy_id: None,
            attested: None,
            from: BTreeMap::new(),
            to: BTreeMap::new(),
            evidence_digest: None,
            record_complete: true,
            missing: Vec::new(),
            successors: Vec::new(),
            label: format!(
                "original approval, compiled {} by aware {}",
                lock.compiled_at, lock.compiler_version
            ),
        });
    };
    let mut missing = Vec::new();
    let original = &chain.original;
    let mut original_missing = false;
    match read_archive(
        source_dir,
        &original.lock_digest,
        "lock",
        "original approval archive",
    )? {
        Archive::Missing(item) => {
            original_missing = true;
            missing.push(item);
        }
        Archive::Present(bytes) => {
            let archived = parse_archived_lock(&bytes, "the original approval archive")?;
            records::validate_original(&archived, original, lock).map_err(|c| c.to_string())?;
        }
    }

    let mut facts = Vec::new();
    for (index, link) in chain.successors.iter().enumerate() {
        let n = link.seq;
        if index > 0 {
            match read_archive(
                source_dir,
                &link.from_lock_digest,
                "lock",
                &format!("successor {n}: replaced lock"),
            )? {
                Archive::Missing(item) => missing.push(item),
                Archive::Present(bytes) => {
                    let replaced =
                        parse_archived_lock(&bytes, &format!("successor {n}'s replaced lock"))?;
                    records::validate_replaced(&replaced, chain, index, lock)
                        .map_err(|c| format!("successor {n}: {c}"))?;
                }
            }
        }
        match read_archive(
            source_dir,
            &link.resulting_lock_digest,
            "lock",
            &format!("successor {n}: resulting plan"),
        )? {
            Archive::Missing(item) => missing.push(item),
            Archive::Present(bytes) => {
                let result =
                    parse_archived_lock(&bytes, &format!("successor {n}'s resulting plan"))?;
                records::validate_resulting(&result, link, lock)
                    .map_err(|c| format!("successor {n}: {c}"))?;
            }
        }
        let mut fact = EvidenceFacts::default();
        match read_archive(
            source_dir,
            &link.evidence_digest,
            "json",
            &format!("successor {n}: evidence"),
        )? {
            Archive::Missing(item) => missing.push(item),
            Archive::Present(bytes) => {
                // The labels' effect and comparison wording comes from this
                // file, so it must be evidence FOR this link.
                let evidence = records::validate_evidence(&bytes, link, lock)
                    .map_err(|c| format!("successor {n}: {c}"))?;
                fact = EvidenceFacts::from_row(&evidence.row);
            }
        }
        facts.push(fact);
        if let CarriedForwardBy::Person {
            approval_record_digest,
            ..
        } = &link.carried_forward_by
        {
            match read_archive(
                source_dir,
                approval_record_digest,
                "json",
                &format!("successor {n}: person approval record"),
            )? {
                Archive::Missing(item) => missing.push(item),
                Archive::Present(bytes) => records::validate_person_record(&bytes, link)
                    .map_err(|c| format!("successor {n}: {c}"))?,
            }
        }
    }

    let Some(last) = chain.successors.last() else {
        return Err("the approval record has no successor".into());
    };
    let last_fact = facts.last().cloned().unwrap_or_default();
    let (from, to) = moved(&last.from, &last.to);
    let complete = missing.is_empty();
    let mut label = link_label(last, Some(&last_fact));
    if original_missing {
        label.push_str("; the original approval record is missing, provenance cannot be shown");
    } else if !complete {
        label.push_str(&format!(
            "; part of the approval record is missing ({} item{}), provenance cannot be fully shown",
            missing.len(),
            if missing.len() == 1 { "" } else { "s" }
        ));
    }
    let successors = chain
        .successors
        .iter()
        .zip(&facts)
        .map(|(link, fact)| {
            let (from, to) = moved(&link.from, &link.to);
            let (actor, policy_id, attested) = by_fields(&link.carried_forward_by);
            SuccessorRow {
                seq: link.seq,
                kind: link.kind,
                from,
                to,
                by_kind: link.carried_forward_by.kind(),
                actor,
                policy_id,
                attested,
                front_door: link.front_door.clone(),
                promoted_at: link.promoted_at.clone(),
                label: link_label(link, Some(fact)),
            }
        })
        .collect();
    let (actor, policy_id, attested) = by_fields(&last.carried_forward_by);
    Ok(ApprovalSummary {
        origin: Origin::Successor,
        seq: Some(last.seq),
        kind: Some(last.kind),
        by_kind: Some(last.carried_forward_by.kind()),
        actor,
        policy_id,
        attested,
        from,
        to,
        evidence_digest: Some(last.evidence_digest.clone()),
        record_complete: complete,
        missing,
        successors,
        label,
    })
}

fn by_fields(by: &CarriedForwardBy) -> (Option<String>, Option<String>, Option<bool>) {
    match by {
        CarriedForwardBy::Person {
            actor, attested, ..
        } => (Some(actor.clone()), None, Some(*attested)),
        CarriedForwardBy::Policy {
            policy_id,
            policy_approved_by,
            ..
        } => (
            Some(policy_approved_by.clone()),
            Some(policy_id.clone()),
            None,
        ),
    }
}

/// The agents whose pin differs between `from` and `to`, at both ends.
type PinMaps = (BTreeMap<String, ApprovalPin>, BTreeMap<String, ApprovalPin>);
fn moved(from: &BTreeMap<String, ApprovalPin>, to: &BTreeMap<String, ApprovalPin>) -> PinMaps {
    let ids: BTreeSet<&String> = from.keys().chain(to.keys()).collect();
    let mut a = BTreeMap::new();
    let mut b = BTreeMap::new();
    for id in ids {
        if from.get(id) != to.get(id) {
            if let Some(pin) = from.get(id) {
                a.insert(id.clone(), pin.clone());
            }
            if let Some(pin) = to.get(id) {
                b.insert(id.clone(), pin.clone());
            }
        }
    }
    (a, b)
}

fn short(pin: &ApprovalPin) -> String {
    match pin
        .digest
        .as_deref()
        .and_then(crate::agent_store::digest_hex)
    {
        Some(hex) => format!("{} (sha256:{}…)", pin.version, &hex[..12]),
        None => pin.version.clone(),
    }
}

/// "tekla 0.1.5 to 0.1.6" for each moved agent; the digest is shown only when
/// the version alone does not tell the two pins apart.
fn moves_text(link: &Successor) -> String {
    let (from, to) = moved(&link.from, &link.to);
    let ids: BTreeSet<&String> = from.keys().chain(to.keys()).collect();
    let parts: Vec<String> = ids
        .into_iter()
        .filter_map(|id| match (from.get(id), to.get(id)) {
            (Some(a), Some(b)) if a.version == b.version => {
                Some(format!("{id} {} to {}", short(a), short(b)))
            }
            (Some(a), Some(b)) => Some(format!("{id} {} to {}", a.version, b.version)),
            (Some(a), None) => Some(format!("{id} {} to no pin", a.version)),
            (None, Some(b)) => Some(format!("{id} no pin to {}", b.version)),
            // `ids` comes from the two maps, so one side is always present.
            (None, None) => None,
        })
        .collect();
    parts.join(" and ")
}

/// The plain-English label of one link (plan §11/§12 honesty rules): a person
/// approval is a CLAIM, the effect is DECLARED, and an inspection is never
/// called a comparison that passed.
fn link_label(link: &Successor, fact: Option<&EvidenceFacts>) -> String {
    let verb = match link.kind {
        SuccessorKind::CarriedForward => "carried forward",
        SuccessorKind::Reverted => "reverted",
    };
    let moves = moves_text(link);
    let mut label = if moves.is_empty() {
        format!("approval {verb} (no tool pin changed)")
    } else {
        format!("approval {verb} from {moves}")
    };
    let (by, who) = match &link.carried_forward_by {
        CarriedForwardBy::Person {
            actor, front_door, ..
        } => (
            format!("claimed person approval by {actor}, recorded by {front_door}"),
            format!("by claimed person approval by {actor}"),
        ),
        CarriedForwardBy::Policy {
            policy_id,
            policy_approved_by,
            ..
        } => (
            format!("under policy {policy_id} (claimed approval by {policy_approved_by})"),
            format!("under policy {policy_id}"),
        ),
    };
    label.push_str(" — ");
    label.push_str(&by);
    if let Some(fact) = fact {
        for words in [
            effect_words(&fact.effect),
            comparison_words(fact, verb, &who),
        ]
        .into_iter()
        .flatten()
        {
            label.push_str("; ");
            label.push_str(&words);
        }
    }
    label
}

pub mod records;

#[cfg(test)]
pub(crate) mod test_support;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod record_tables;
