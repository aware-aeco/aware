//! One validator per archived record type an approval chain cites (#628 PR3a
//! review round 2). Each checks EVERY field its record carries against what
//! the chain says it must be, in a fixed order, and names the first field
//! that disagrees in a typed [`Contradiction`].
//!
//! The order matters only for which field is named: identity and form first
//! (approval block, source hash, app, version), then the pin maps, then the
//! plan digest last — because the plan digest covers most other fields, a
//! change to any of them would otherwise always be reported as `plan`.
//!
//! What no validator checks, and why:
//! * an original's `nodes`/`schedule`/`engineering` — the chain records no plan
//!   for the original; its exact bytes are bound by `lock-digest` (the file is
//!   content-addressed, so they cannot change without changing its name);
//! * a resulting plan's `compiled-at`/`compiler-version`/`front-door` — "when
//!   and by whom" a candidate was compiled; outside the plan digest by design;
//! * a policy link's `policy-digest` against the policy text — there is no
//!   policy store before #628 PR3b; [`super::check_chain`] checks its form.

use super::*;
use crate::app_lock::candidate::{CANDIDATE_FORMAT, PinMove};
use crate::migration::contract::PinRef;
use crate::migration::files::{EVIDENCE_FORMAT, Evidence};

/// The archived record a [`Contradiction`] is about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordKind {
    Original,
    Replaced,
    ResultingPlan,
    Evidence,
    PersonApproval,
}

impl RecordKind {
    pub fn name(self) -> &'static str {
        match self {
            Self::Original => "original approval archive",
            Self::Replaced => "replaced lock",
            Self::ResultingPlan => "resulting plan",
            Self::Evidence => "evidence",
            Self::PersonApproval => "person approval record",
        }
    }
}

/// An archived record that disagrees with the chain at `field`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contradiction {
    pub record: RecordKind,
    /// The record's field, as it is spelled in the file (`(parse)` when the
    /// file is not that kind of record at all).
    pub field: &'static str,
    pub detail: String,
}

impl std::fmt::Display for Contradiction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "the archived {} contradicts the approval record at `{}`: {}",
            self.record.name(),
            self.field,
            self.detail
        )
    }
}

type Checked = Result<(), Contradiction>;

struct Checker {
    record: RecordKind,
}

impl Checker {
    fn eq<T: PartialEq + std::fmt::Debug>(
        &self,
        field: &'static str,
        found: &T,
        want: &T,
    ) -> Checked {
        if found == want {
            Ok(())
        } else {
            Err(self.fail(field, format!("found {found:?}, the chain needs {want:?}")))
        }
    }

    fn that(&self, field: &'static str, ok: bool, detail: impl Into<String>) -> Checked {
        if ok {
            Ok(())
        } else {
            Err(self.fail(field, detail))
        }
    }

    fn fail(&self, field: &'static str, detail: impl Into<String>) -> Contradiction {
        Contradiction {
            record: self.record,
            field,
            detail: detail.into(),
        }
    }

    fn plan(&self, lock: &LockFile, want: &str) -> Checked {
        let plan = plan_digest(lock).map_err(|e| self.fail("plan", e.to_string()))?;
        self.eq("plan", &plan.as_str(), &want)
    }

    fn pins(
        &self,
        lock: &LockFile,
        pins: &BTreeMap<String, String>,
        digests: &BTreeMap<String, String>,
        bundles: &BTreeMap<String, String>,
    ) -> Checked {
        self.eq("agent-pins", &lock.agent_pins, pins)?;
        self.eq("agent-digests", &lock.agent_digests, digests)?;
        self.eq("agent-bundle-pins", &lock.agent_bundle_pins, bundles)
    }

    /// `source-hash` in the raw `sha256:<hex>` form of an ORIGINAL or a
    /// candidate, naming the source the current lock approves.
    fn raw_source(&self, lock: &LockFile, current: &LockFile) -> Checked {
        self.that(
            "source-hash",
            crate::agent_store::digest_hex(&lock.source_hash).is_some(),
            format!(
                "{} is not a plain sha256 source hash; only a lock carrying an approval record names its source {SUCCESSOR_SOURCE_PREFIX}<hex>",
                lock.source_hash
            ),
        )?;
        self.eq(
            "source-hash",
            &lock.source_hash,
            &approved_source_hash(current),
        )
    }
}

use super::split_pins as split;

/// The original approval archive: a person's compile of THIS source, exactly
/// as `approval.original` copied it.
pub fn validate_original(
    archived: &LockFile,
    original: &OriginalApproval,
    current: &LockFile,
) -> Checked {
    let c = Checker {
        record: RecordKind::Original,
    };
    c.that(
        "approval",
        archived.approval.is_none(),
        "an original approval carries no approval record",
    )?;
    c.raw_source(archived, current)?;
    c.eq("app", &archived.app, &current.app)?;
    c.eq("version", &archived.version, &current.version)?;
    c.eq("compiled-at", &archived.compiled_at, &original.compiled_at)?;
    c.eq(
        "compiler-version",
        &archived.compiler_version,
        &original.compiler_version,
    )?;
    c.eq("front-door", &archived.front_door, &original.front_door)?;
    c.pins(
        archived,
        &original.agent_pins,
        &original.agent_digests,
        &original.agent_bundle_pins,
    )
}

/// The lock successor `index + 1` (0-based `index` ≥ 1) replaced: the
/// promoted lock successor `index` left — the same record up to that link,
/// the same source, the pins and plan that link approved.
pub fn validate_replaced(
    replaced: &LockFile,
    chain: &ApprovalChain,
    index: usize,
    current: &LockFile,
) -> Checked {
    let c = Checker {
        record: RecordKind::Replaced,
    };
    let prior = &chain.successors[index - 1];
    let Some(record) = &replaced.approval else {
        return Err(c.fail(
            "approval",
            "a lock a successor replaced was itself carried forward, so it carries an approval record",
        ));
    };
    c.eq("approval.format", &record.format, &chain.format)?;
    c.eq("approval.original", &record.original, &chain.original)?;
    c.eq(
        "approval.successors",
        &record.successors.as_slice(),
        &&chain.successors[..index],
    )?;
    c.eq("source-hash", &replaced.source_hash, &current.source_hash)?;
    c.eq("app", &replaced.app, &current.app)?;
    c.eq("version", &replaced.version, &current.version)?;
    let [pins, digests, bundles] = split(&prior.to);
    c.pins(replaced, &pins, &digests, &bundles)?;
    c.plan(replaced, &prior.to_plan_digest)
}

/// A successor's resulting plan: the candidate as promoted — no approval
/// record, the raw source hash, the pins and plan the link approved.
pub fn validate_resulting(result: &LockFile, link: &Successor, current: &LockFile) -> Checked {
    let c = Checker {
        record: RecordKind::ResultingPlan,
    };
    c.that(
        "approval",
        result.approval.is_none(),
        "a resulting plan is the candidate as promoted, before any approval record was attached",
    )?;
    c.raw_source(result, current)?;
    c.eq("app", &result.app, &current.app)?;
    c.eq("version", &result.version, &current.version)?;
    let [pins, digests, bundles] = split(&link.to);
    c.pins(result, &pins, &digests, &bundles)?;
    c.plan(result, &link.to_plan_digest)
}

/// The candidate header's `targets` a link implies: every agent present on
/// both sides whose pin moved.
fn expected_targets(link: &Successor) -> BTreeMap<String, PinMove> {
    let pin_ref = |pin: &ApprovalPin| PinRef {
        version: pin.version.clone(),
        // As `PinSet::from_lock` (which `compile_candidate` records) reads it.
        digest: pin.effective_digest().cloned().unwrap_or_default(),
    };
    link.from
        .iter()
        .filter_map(|(id, from)| {
            let to = link.to.get(id)?;
            (from != to).then(|| {
                (
                    id.clone(),
                    PinMove {
                        from: pin_ref(from),
                        to: pin_ref(to),
                    },
                )
            })
        })
        .collect()
}

/// A successor's evidence: `aware.migration-evidence/v1` written for THIS
/// app, base lock, source, candidate, plan and pin moves. Returns it parsed,
/// for the label's facts.
pub fn validate_evidence(
    bytes: &[u8],
    link: &Successor,
    current: &LockFile,
) -> Result<Evidence, Contradiction> {
    let c = Checker {
        record: RecordKind::Evidence,
    };
    let evidence: Evidence = serde_json::from_slice(bytes)
        .map_err(|e| c.fail("(parse)", format!("not migration evidence: {e}")))?;
    c.eq("format", &evidence.format.as_str(), &EVIDENCE_FORMAT)?;
    c.eq("app", &evidence.app, &current.app)?;
    c.that(
        "prepared-at",
        !evidence.prepared_at.trim().is_empty(),
        "empty",
    )?;
    c.that(
        "cli-version",
        !evidence.cli_version.trim().is_empty(),
        "empty",
    )?;
    c.that(
        "row",
        evidence.row.is_object(),
        "the plan row is not an object",
    )?;
    let h = &evidence.header;
    c.eq("header.format", &h.format.as_str(), &CANDIDATE_FORMAT)?;
    c.eq("header.app", &h.app, &current.app)?;
    c.eq(
        "header.base-lock-digest",
        &h.base_lock_digest,
        &link.from_lock_digest,
    )?;
    c.eq(
        "header.base-source-hash",
        &h.base_source_hash,
        &approved_source_hash(current),
    )?;
    c.eq(
        "header.candidate-digest",
        &h.candidate_digest,
        &link.resulting_lock_digest,
    )?;
    c.eq("header.plan-digest", &h.plan_digest, &link.to_plan_digest)?;
    c.eq("header.targets", &h.targets, &expected_targets(link))?;
    Ok(evidence)
}

/// The person approval record a front door writes (plan §12).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct PersonApprovalRecord {
    pub format: u32,
    pub kind: String,
    pub actor: String,
    pub front_door: String,
    pub approval_ref: String,
    pub candidate_digest: String,
    pub base_lock_digest: String,
    pub plan_digest: String,
    pub statement_sha256: String,
    pub at: String,
}

/// The only person approval record format this CLI reads.
pub const PERSON_RECORD_FORMAT: u32 = 1;

/// A person approval record: format 1, `kind: person`, naming this link's
/// claim (actor, approval-ref, front door) and binding it to this link's
/// candidate, base lock and plan, with the digest of the statement shown.
pub fn validate_person_record(bytes: &[u8], link: &Successor) -> Checked {
    let c = Checker {
        record: RecordKind::PersonApproval,
    };
    let CarriedForwardBy::Person {
        actor,
        approval_ref,
        front_door,
        ..
    } = &link.carried_forward_by
    else {
        return Err(c.fail("kind", "the link was not carried forward by a person"));
    };
    let record: PersonApprovalRecord = serde_json::from_slice(bytes)
        .map_err(|e| c.fail("(parse)", format!("not a person approval record: {e}")))?;
    c.eq("format", &record.format, &PERSON_RECORD_FORMAT)?;
    c.eq("kind", &record.kind.as_str(), &"person")?;
    c.eq("actor", &record.actor, actor)?;
    c.eq("approval-ref", &record.approval_ref, approval_ref)?;
    c.eq("front-door", &record.front_door, front_door)?;
    c.eq(
        "candidate-digest",
        &record.candidate_digest,
        &link.resulting_lock_digest,
    )?;
    c.eq(
        "base-lock-digest",
        &record.base_lock_digest,
        &link.from_lock_digest,
    )?;
    c.eq("plan-digest", &record.plan_digest, &link.to_plan_digest)?;
    c.that(
        "statement-sha256",
        crate::agent_store::digest_hex(&record.statement_sha256).is_some(),
        format!("{:?} is not a sha256 digest", record.statement_sha256),
    )?;
    c.that("at", !record.at.trim().is_empty(), "empty")
}
