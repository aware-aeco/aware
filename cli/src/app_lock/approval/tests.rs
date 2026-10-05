//! The reading side of successor approval records (#628 PR3a; plan §8 items
//! 6, 7 and 11 — chain consistency breaks, the legacy read, the record labels).
//! Promoted locks are written by [`super::test_support`]: no promote verb exists.

use super::test_support::{By, Promoted, archive, promote, tamper, write_lock};
use super::*;
use crate::agent_resolution::PinTarget;
use crate::app_lock::candidate::tests::{approve, home, update_agent, write_agent, write_app};

const ONE_TOOL: &str = "requires: []\nnodes:\n  - { id: a, agent: tool, command: go }\n";

fn to(id: &str, digest: &str) -> BTreeMap<String, PinTarget> {
    [(id.to_string(), PinTarget::Digest(digest.to_string()))].into()
}

struct Setup {
    h: crate::app_lock::candidate::tests::Home,
    source: std::path::PathBuf,
    base: LockFile,
    new_digest: String,
}

/// `demo` approved on tool 1.0.0, tool 1.0.1 installed and stored.
fn setup() -> Setup {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    let source = write_app(&h.paths, "demo", ONE_TOOL);
    let (base, _) = approve(&h.paths, &source);
    let new_digest = update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    Setup {
        h,
        source,
        base,
        new_digest,
    }
}

fn promoted_by(s: &Setup, by: By) -> Promoted {
    promote(
        &s.h.paths,
        &s.source,
        to("tool", &s.new_digest),
        by,
        SuccessorKind::CarriedForward,
    )
}

fn load_err(s: &Setup) -> String {
    match load_approved_app_snapshot(&s.source) {
        Ok(_) => panic!("the lock loaded"),
        Err(error) => error.to_string(),
    }
}

fn assert_invalid(s: &Setup, needle: &str) {
    let error = load_err(s);
    assert!(error.contains("[E_APP_LOCK_INVALID]"), "{error}");
    assert!(error.contains(needle), "expected {needle:?} in: {error}");
    // `app check` reports the same lock as invalid, as data.
    let check = crate::agent_resolution::check_app(&s.h.paths, &s.source).unwrap();
    assert_eq!(
        check.lock,
        crate::agent_resolution::LockState::Invalid,
        "{check:?}"
    );
    assert!(!check.approval_current);
}

fn approvals_dir(s: &Setup) -> std::path::PathBuf {
    crate::fs::containing_dir(&s.source).join(ARCHIVE_DIR)
}

fn remove_archive(s: &Setup, digest: &str, ext: &str) {
    let rel = archive_rel(digest, ext).unwrap();
    std::fs::remove_file(crate::fs::containing_dir(&s.source).join(rel)).unwrap();
}

#[test]
fn a_compiled_lock_is_an_original_approval_with_nothing_to_check() {
    let s = setup();
    assert!(s.base.approval.is_none());
    assert!(!s.base.source_hash.starts_with(SUCCESSOR_SOURCE_PREFIX));
    let approved = load_approved_app_snapshot(&s.source).unwrap();
    assert_eq!(approved.approval.origin, Origin::Original);
    assert!(approved.approval.record_complete);
    assert!(approved.approval.label.starts_with("original approval"));
    let text = std::fs::read_to_string(s.source.with_file_name("demo.lock")).unwrap();
    assert!(!text.contains("approval:"), "{text}");
}

#[test]
fn a_promoted_lock_loads_runs_the_top_level_pins_and_says_where_it_came_from() {
    let s = setup();
    let p = promoted_by(&s, By::Person("pawel"));
    assert!(p.lock.source_hash.starts_with(SUCCESSOR_SOURCE_PREFIX));
    assert_eq!(
        approved_source_hash(&p.lock),
        approved_source_hash(&s.base),
        "the prefix names the same source"
    );
    check_chain(&p.lock).unwrap();

    let approved = load_approved_app_snapshot(&s.source).unwrap();
    assert_eq!(approved.lock.agent_digests["tool"], s.new_digest);
    let summary = approved.approval;
    assert_eq!(summary.origin, Origin::Successor);
    assert_eq!(summary.seq, Some(1));
    assert_eq!(summary.by_kind, Some("person"));
    assert_eq!(summary.attested, Some(false));
    assert!(summary.record_complete, "{:?}", summary.missing);
    assert_eq!(summary.from["tool"].version, "1.0.0");
    assert_eq!(summary.to["tool"].version, "1.0.1");
    assert_eq!(
        summary.label,
        "approval carried forward from tool 1.0.0 to 1.0.1 — claimed person approval by pawel, \
         recorded by floless@test; declared read-only; the tool's run instructions are \
         byte-identical (checked by inspection, nothing was run)"
    );

    // The run resolves the TOP-LEVEL (promoted) pins.
    let resolved = crate::agent_resolution::resolve_agents(
        &s.h.paths,
        &approved.app,
        &approved.lock,
        crate::agent_resolution::Selection::Default,
    )
    .unwrap();
    let infos: Vec<_> = resolved.infos().iter().collect();
    assert_eq!(infos.len(), 1);
    assert_eq!(infos[0].1.digest, s.new_digest);

    // `app check` reports the origin and the record, approval current.
    let check = crate::agent_resolution::check_app(&s.h.paths, &s.source).unwrap();
    assert!(check.approval_current, "{check:?}");
    assert!(check.source_current);
    assert_eq!(check.approval_origin, Some(Origin::Successor));
    assert_eq!(
        check.approval_record,
        Some(crate::agent_resolution::RecordState::Complete)
    );
    assert_eq!(check.successors.len(), 1);
    assert_eq!(check.successors[0].by_kind, "person");
    assert_eq!(check.successors[0].attested, Some(false));
    let json = serde_json::to_value(&check).unwrap();
    assert_eq!(json["approval-origin"], "successor");
    assert_eq!(json["approval-record"], "complete");
    assert_eq!(json["successors"][0]["kind"], "carried-forward");
    assert_eq!(json["successors"][0]["from"]["tool"]["version"], "1.0.0");
    assert_eq!(json["successors"][0]["to"]["tool"]["version"], "1.0.1");
}

#[test]
fn labels_never_overstate_what_was_checked() {
    let s = setup();
    promoted_by(&s, By::Policy("pol-read-patch"));
    let summary = load_approved_app_snapshot(&s.source).unwrap().approval;
    assert!(
        summary.label.contains(
            "approval carried forward from tool 1.0.0 to 1.0.1 — under policy pol-read-patch (claimed approval by pawel)"
        ),
        "{}",
        summary.label
    );
    assert_eq!(summary.policy_id.as_deref(), Some("pol-read-patch"));
    assert_eq!(summary.attested, None, "a policy link has no person claim");
    for banned in [
        "unchanged behaviour",
        "unchanged behavior",
        "same results",
        "passed 0",
        "0 comparisons",
        "approved by pawel",
    ] {
        assert!(
            !summary.label.contains(banned),
            "{banned}: {}",
            summary.label
        );
    }
}

#[test]
fn a_second_successor_and_a_revert_chain_and_verify_against_their_archives() {
    let s = setup();
    let first = promoted_by(&s, By::Person("pawel"));
    // Revert to the original pins: a second link whose from is link 1's to.
    let original_digest = s.base.agent_digests["tool"].clone();
    let second = promote(
        &s.h.paths,
        &s.source,
        to("tool", &original_digest),
        By::Person("anna"),
        SuccessorKind::Reverted,
    );
    assert_eq!(second.replaced, first.bytes);
    check_chain(&second.lock).unwrap();
    let summary = load_approved_app_snapshot(&s.source).unwrap().approval;
    assert!(summary.record_complete, "{:?}", summary.missing);
    assert_eq!(summary.seq, Some(2));
    assert!(
        summary.label.starts_with(
            "approval reverted from tool 1.0.1 to 1.0.0 — claimed person approval by anna"
        ),
        "{}",
        summary.label
    );
    assert_eq!(summary.successors.len(), 2);

    // A revert to pins that were never approved is not a revert.
    tamper(&second.lock_path, |lock| {
        let chain = lock.approval.as_mut().unwrap();
        let fake = ApprovalPin {
            version: "0.0.1".into(),
            digest: Some(format!("sha256:{}", "1".repeat(64))),
            bundle_pin: None,
        };
        chain.successors[1].to.insert("tool".into(), fake.clone());
        lock.agent_pins.insert("tool".into(), "0.0.1".into());
        lock.agent_digests
            .insert("tool".into(), fake.digest.clone().unwrap());
    });
    assert_invalid(&s, "never approved");
}

#[test]
fn the_replaced_lock_of_a_later_link_must_be_the_lock_the_earlier_link_left() {
    let s = setup();
    promoted_by(&s, By::Person("pawel"));
    let newer = update_agent(&s.h.paths, "tool", "1.0.2", "mode: read");
    let second = promote(
        &s.h.paths,
        &s.source,
        to("tool", &newer),
        By::Person("pawel"),
        SuccessorKind::CarriedForward,
    );
    // Link 2 claims to have replaced a lock that IS archived and DOES hash to
    // its name — the original — but that is not the lock link 1 left.
    tamper(&second.lock_path, |l| {
        let c = chain(l);
        c.successors[1].from_lock_digest = c.original.lock_digest.clone();
    });
    assert_invalid(
        &s,
        "the archived replaced lock contradicts the approval record",
    );
}

// ── every structural break → E_APP_LOCK_INVALID ─────────────────────────────

fn break_case(edit: impl FnOnce(&mut LockFile), needle: &str) {
    let s = setup();
    let p = promoted_by(&s, By::Person("pawel"));
    tamper(&p.lock_path, edit);
    assert_invalid(&s, needle);
}

fn chain(lock: &mut LockFile) -> &mut ApprovalChain {
    lock.approval.as_mut().unwrap()
}

#[test]
fn an_unknown_format_is_invalid() {
    break_case(|l| chain(l).format = 2, "approval format 2");
}

#[test]
fn a_seq_that_does_not_run_from_one_is_invalid() {
    break_case(|l| chain(l).successors[0].seq = 2, "out of order");
}

#[test]
fn a_first_link_that_does_not_start_from_the_original_pins_is_invalid() {
    break_case(
        |l| {
            chain(l)
                .original
                .agent_pins
                .insert("tool".into(), "0.9.0".into());
        },
        "not the original approval's",
    );
}

#[test]
fn a_first_link_that_replaced_another_lock_than_the_original_is_invalid() {
    break_case(
        |l| chain(l).successors[0].from_lock_digest = format!("sha256:{}", "2".repeat(64)),
        "but the original approval is",
    );
}

#[test]
fn a_link_that_does_not_start_where_the_previous_ended_is_invalid() {
    let s = setup();
    promoted_by(&s, By::Person("pawel"));
    let newer = update_agent(&s.h.paths, "tool", "1.0.2", "mode: read");
    let second = promote(
        &s.h.paths,
        &s.source,
        to("tool", &newer),
        By::Person("pawel"),
        SuccessorKind::CarriedForward,
    );
    tamper(&second.lock_path, |l| {
        chain(l).successors[1].from.get_mut("tool").unwrap().version = "1.0.9".into();
    });
    assert_invalid(&s, "not the ones successor 1 carried forward to");
}

#[test]
fn top_level_pins_that_are_not_the_last_link_s_are_invalid() {
    break_case(
        |l| {
            l.agent_pins.insert("tool".into(), "1.0.9".into());
        },
        "are not the pins successor 1 carried forward to",
    );
}

#[test]
fn a_plan_that_is_not_the_one_the_last_link_approved_is_invalid() {
    break_case(
        |l| l.nodes[0].mode = "write".into(),
        "is not the plan successor 1 approved",
    );
}

#[test]
fn a_successor_source_hash_without_a_chain_is_invalid_not_stale() {
    break_case(
        |l| l.approval = None,
        "names a carried-forward approval but the lock has no approval record",
    );
}

#[test]
fn a_chain_with_a_raw_source_hash_is_invalid() {
    break_case(
        |l| l.source_hash = approved_source_hash(l),
        "is not successor-v1:<hex>",
    );
}

#[test]
fn a_chain_with_no_successor_is_invalid() {
    break_case(|l| chain(l).successors.clear(), "has no successor");
}

#[test]
fn an_attested_person_claim_is_invalid() {
    break_case(
        |l| {
            if let CarriedForwardBy::Person { attested, .. } =
                &mut chain(l).successors[0].carried_forward_by
            {
                *attested = true;
            }
        },
        "attested",
    );
}

#[test]
fn an_archive_path_that_is_not_content_addressed_is_invalid() {
    break_case(
        |l| chain(l).original.archive = "../../elsewhere.lock".into(),
        "is not the content-addressed archive",
    );
    break_case(
        |l| chain(l).successors[0].evidence = ".aware-approvals/x.json".into(),
        "is not the content-addressed archive",
    );
}

// ── archives: tampered → invalid; missing → incomplete, never a refusal ─────

#[test]
fn a_changed_archive_is_invalid() {
    let s = setup();
    let p = promoted_by(&s, By::Person("pawel"));
    let chain = p.lock.approval.as_ref().unwrap();
    let path = crate::fs::containing_dir(&s.source).join(&chain.original.archive);
    let mut bytes = std::fs::read(&path).unwrap();
    bytes.extend_from_slice(b"# edited\n");
    std::fs::write(&path, bytes).unwrap();
    assert_invalid(&s, "does not hash to");
}

#[test]
fn an_original_record_that_is_not_the_archived_original_is_invalid() {
    let s = setup();
    let p = promoted_by(&s, By::Person("pawel"));
    tamper(&p.lock_path, |l| {
        // Point the original (and link 1's from) at another archived lock —
        // the resulting plan — whose bytes do hash to the name.
        let c = chain(l);
        let other = c.successors[0].resulting_lock_digest.clone();
        c.original.lock_digest = other.clone();
        c.original.archive = archive_rel(&other, "lock").unwrap();
        c.successors[0].from_lock_digest = other;
    });
    assert_invalid(
        &s,
        "original approval archive contradicts the approval record",
    );
}

#[test]
fn a_resulting_plan_archive_that_is_not_the_approved_plan_is_invalid() {
    let s = setup();
    let p = promoted_by(&s, By::Person("pawel"));
    tamper(&p.lock_path, |l| {
        let c = chain(l);
        c.successors[0].resulting_lock_digest = c.original.lock_digest.clone();
    });
    assert_invalid(
        &s,
        "the archived resulting plan contradicts the approval record",
    );
}

#[test]
fn a_person_approval_record_for_another_plan_is_invalid() {
    let s = setup();
    let p = promoted_by(&s, By::Person("pawel"));
    let dir = crate::fs::containing_dir(&s.source).to_path_buf();
    let chain0 = &p.lock.approval.as_ref().unwrap().successors[0];
    let record = serde_json::json!({
        "format": 1, "kind": "person", "actor": "pawel", "front-door": "floless@test",
        "approval-ref": "batch-1",
        "candidate-digest": chain0.resulting_lock_digest,
        "base-lock-digest": chain0.from_lock_digest,
        "plan-digest": format!("sha256:{}", "3".repeat(64)),
        "statement-sha256": format!("sha256:{}", "9".repeat(64)),
        "at": "2026-10-05T00:00:00Z",
    });
    let digest = archive(&dir, record.to_string().as_bytes(), "json");
    tamper(&p.lock_path, |l| {
        if let CarriedForwardBy::Person {
            approval_record_digest,
            ..
        } = &mut chain(l).successors[0].carried_forward_by
        {
            *approval_record_digest = digest;
        }
    });
    assert_invalid(&s, "contradicts the approval record at `plan-digest`");
}

#[test]
fn a_missing_original_archive_is_incomplete_and_the_run_proceeds_labelled() {
    let s = setup();
    let p = promoted_by(&s, By::Person("pawel"));
    let chain = p.lock.approval.clone().unwrap();
    remove_archive(&s, &chain.original.lock_digest, "lock");

    let approved = load_approved_app_snapshot(&s.source).expect("never a refusal");
    let summary = approved.approval;
    assert!(!summary.record_complete);
    assert_eq!(summary.missing.len(), 1, "{:?}", summary.missing);
    assert!(summary.missing[0].contains(&chain.original.archive));
    assert!(
        summary
            .label
            .ends_with("the original approval record is missing, provenance cannot be shown"),
        "{}",
        summary.label
    );
    let check = crate::agent_resolution::check_app(&s.h.paths, &s.source).unwrap();
    assert!(check.approval_current, "incomplete is not a refusal");
    assert_eq!(
        check.approval_record,
        Some(crate::agent_resolution::RecordState::Incomplete)
    );
    assert_eq!(check.approval_record_missing, summary.missing);
}

#[test]
fn every_missing_archive_is_named() {
    let s = setup();
    let p = promoted_by(&s, By::Person("pawel"));
    let link = p.lock.approval.as_ref().unwrap().successors[0].clone();
    remove_archive(&s, &link.evidence_digest, "json");
    remove_archive(&s, &link.resulting_lock_digest, "lock");
    if let CarriedForwardBy::Person {
        approval_record_digest,
        ..
    } = &link.carried_forward_by
    {
        remove_archive(&s, approval_record_digest, "json");
    }
    let summary = load_approved_app_snapshot(&s.source).unwrap().approval;
    assert!(!summary.record_complete);
    assert_eq!(summary.missing.len(), 3, "{:?}", summary.missing);
    assert!(
        summary
            .label
            .contains("part of the approval record is missing (3 items)"),
        "{}",
        summary.label
    );
    // Without the evidence nothing is claimed about effect or comparison.
    assert!(!summary.label.contains("read-only"), "{}", summary.label);
    // Everything gone: still runs.
    std::fs::remove_dir_all(approvals_dir(&s)).unwrap();
    assert!(load_approved_app_snapshot(&s.source).is_ok());
}

// ── source drift, compile, plan, legacy ─────────────────────────────────────

#[test]
fn a_promoted_lock_whose_source_changed_is_stale() {
    let s = setup();
    promoted_by(&s, By::Person("pawel"));
    write_app(
        &s.h.paths,
        "demo",
        "requires: []\nnodes:\n  - { id: b, agent: tool, command: go }\n",
    );
    let error = load_err(&s);
    assert!(error.contains("[E_APP_LOCK_STALE]"), "{error}");
}

#[test]
fn compile_writes_a_fresh_original_and_records_the_front_door_outside_the_plan() {
    let s = setup();
    let p = promoted_by(&s, By::Person("pawel"));
    let (_, lock) = compile_to_disk_for(&s.source, &s.h.paths, Some("floless@9.9.9")).unwrap();
    assert!(lock.approval.is_none());
    assert!(lock.source_hash.starts_with("sha256:"));
    assert_eq!(lock.front_door.as_deref(), Some("floless@9.9.9"));
    let text = std::fs::read_to_string(&p.lock_path).unwrap();
    assert!(!text.contains("approval:") && !text.contains(SUCCESSOR_SOURCE_PREFIX));
    assert!(text.contains("front-door: floless@9.9.9"), "{text}");
    // Who asked is not part of what runs.
    let mut without = lock;
    let with = plan_digest(&without).unwrap();
    without.front_door = None;
    assert_eq!(plan_digest(&without).unwrap(), with);
    assert_eq!(
        load_approved_app_snapshot(&s.source)
            .unwrap()
            .approval
            .origin,
        Origin::Original
    );
}

#[test]
fn a_promoted_lock_has_the_plan_digest_of_the_candidate_it_promoted() {
    let s = setup();
    let p = promoted_by(&s, By::Person("pawel"));
    let link = &p.lock.approval.as_ref().unwrap().successors[0];
    assert_eq!(plan_digest(&p.lock).unwrap(), link.to_plan_digest);
}

#[test]
fn migrate_plans_a_promoted_lock_from_its_top_level_pins() {
    let s = setup();
    promoted_by(&s, By::Person("pawel"));
    let eval = crate::migration::plan::evaluate(&s.h.paths, &s.source, None).unwrap();
    assert_eq!(
        eval.row.state,
        crate::migration::plan::State::UpToDate,
        "{:?}",
        eval.row
    );
    let newest = update_agent(&s.h.paths, "tool", "1.0.2", "mode: read");
    let eval = crate::migration::plan::evaluate(&s.h.paths, &s.source, None).unwrap();
    assert_eq!(eval.row.targets.len(), 1, "{:?}", eval.row);
    let json = serde_json::to_value(&eval.row).unwrap();
    assert_eq!(json["targets"][0]["from"]["version"], "1.0.1", "{json}");
    assert_eq!(json["targets"][0]["from"]["digest"], s.new_digest);
    assert_eq!(json["targets"][0]["to"]["digest"], newest);
    let candidate = eval.candidate.unwrap();
    assert_eq!(
        candidate.header.base_source_hash,
        approved_source_hash(&s.base)
    );

    // A tampered chain is reported, not planned against.
    tamper(&s.source.with_file_name("demo.lock"), |l| {
        chain(l).format = 7;
    });
    let eval = crate::migration::plan::evaluate(&s.h.paths, &s.source, None).unwrap();
    let json = serde_json::to_value(&eval.row).unwrap();
    assert_eq!(json["state"], "blocked", "{json}");
    assert_eq!(json["reasons"][0]["code"], "approval-invalid");
}

/// The `LockFile` and approval gate of the RELEASED 0.149 CLI, frozen. A
/// promoted lock must be refused by it (`E_APP_LOCK_STALE`, "compile again")
/// — never run on its new pins unlabelled (plan §13).
mod frozen_0_149 {
    use serde::Deserialize;
    use std::collections::BTreeMap;

    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    pub struct LockFile {
        #[serde(rename = "source-hash")]
        pub source_hash: String,
        #[serde(rename = "compiled-at")]
        pub compiled_at: String,
        #[serde(rename = "compiler-version")]
        pub compiler_version: String,
        pub app: String,
        pub version: String,
        #[serde(rename = "agent-pins")]
        pub agent_pins: BTreeMap<String, String>,
        #[serde(rename = "agent-bundle-pins", default)]
        pub agent_bundle_pins: BTreeMap<String, String>,
        #[serde(rename = "agent-digests", default)]
        pub agent_digests: BTreeMap<String, String>,
        pub nodes: Vec<CompiledNode>,
        pub schedule: Option<serde_yaml::Value>,
        pub engineering: Option<serde_yaml::Value>,
    }

    #[derive(Debug, Deserialize)]
    #[allow(dead_code)]
    pub struct CompiledNode {
        pub id: String,
        pub kind: String,
        pub agent: Option<String>,
        pub command: Option<String>,
        pub mode: String,
        pub safety: Option<serde_yaml::Value>,
        pub inputs: Option<serde_yaml::Value>,
        #[serde(rename = "output-schema")]
        pub output_schema: Option<serde_yaml::Value>,
        #[serde(default)]
        pub notes: Vec<serde_yaml::Value>,
        #[serde(rename = "runtime-model", default)]
        pub runtime_model: bool,
        #[serde(rename = "model-pin", default)]
        pub model_pin: Option<String>,
    }

    /// 0.149's `load_approved_app_snapshot` after reading the source: parse,
    /// then compare the raw hash.
    pub fn gate(lock_text: &str, current_hash: &str) -> Result<LockFile, String> {
        let lock: LockFile = serde_yaml::from_str(lock_text)
            .map_err(|error| format!("[E_APP_LOCK_INVALID] {error}"))?;
        if lock.source_hash != current_hash {
            return Err(format!(
                "[E_APP_LOCK_STALE] approved {}, current {current_hash}",
                lock.source_hash
            ));
        }
        Ok(lock)
    }
}

#[test]
fn the_released_0_149_gate_refuses_a_promoted_lock_as_stale() {
    let s = setup();
    let (_, current) = read_app_source(&s.source).unwrap();
    // Control: 0.149 accepts this CLI's ORIGINAL lock.
    let original = std::fs::read_to_string(s.source.with_file_name("demo.lock")).unwrap();
    assert!(frozen_0_149::gate(&original, &current).is_ok());

    let p = promoted_by(&s, By::Person("pawel"));
    let text = String::from_utf8(p.bytes).unwrap();
    // It still parses (no unknown-field refusal) — and is refused as stale.
    assert!(serde_yaml::from_str::<frozen_0_149::LockFile>(&text).is_ok());
    let error = frozen_0_149::gate(&text, &current).unwrap_err();
    assert!(error.starts_with("[E_APP_LOCK_STALE]"), "{error}");
}

#[test]
fn an_approval_block_survives_a_round_trip_unchanged() {
    let s = setup();
    let p = promoted_by(&s, By::Policy("pol-1"));
    let again: LockFile = serde_yaml::from_slice(&p.bytes).unwrap();
    assert_eq!(again.approval, p.lock.approval);
    let rewritten = write_lock(&p.lock_path, &again);
    assert_eq!(rewritten, p.bytes);
    let text = String::from_utf8(p.bytes).unwrap();
    for key in [
        "approval:",
        "format: 1",
        "original:",
        "lock-digest:",
        "successors:",
        "kind: carried-forward",
        "from-lock-digest:",
        "to-plan-digest:",
        "resulting-lock-digest:",
        "carried-forward-by:",
        "kind: policy",
        "policy-id: pol-1",
        "evidence-digest:",
        "promoted-at:",
    ] {
        assert!(text.contains(key), "{key} missing:\n{text}");
    }
}

// ── mutation follow-ups: each archive comparison has its own witness ────────

#[test]
fn an_original_record_whose_pins_differ_from_the_archived_original_is_invalid() {
    let s = setup();
    let p = promoted_by(&s, By::Person("pawel"));
    tamper(&p.lock_path, |l| {
        // Change the original's pins AND link 1's from, so the chain is
        // structurally whole and only the archive can tell.
        let c = chain(l);
        c.original.agent_pins.insert("tool".into(), "0.9.9".into());
        c.successors[0].from.get_mut("tool").unwrap().version = "0.9.9".into();
    });
    assert_invalid(
        &s,
        "original approval archive contradicts the approval record",
    );
}

#[test]
fn a_resulting_plan_archive_that_carries_an_approval_record_is_invalid() {
    let s = setup();
    let p = promoted_by(&s, By::Person("pawel"));
    // Same plan, same pins - but it is a promoted lock, not the plan as promoted.
    let dir = crate::fs::containing_dir(&s.source).to_path_buf();
    let promoted = archive(&dir, &p.bytes, "lock");
    tamper(&p.lock_path, |l| {
        chain(l).successors[0].resulting_lock_digest = promoted;
    });
    assert_invalid(
        &s,
        "the archived resulting plan contradicts the approval record",
    );
}

#[test]
fn a_replaced_lock_with_another_original_record_is_invalid() {
    let s = setup();
    let first = promoted_by(&s, By::Person("pawel"));
    let newer = update_agent(&s.h.paths, "tool", "1.0.2", "mode: read");
    let second = promote(
        &s.h.paths,
        &s.source,
        to("tool", &newer),
        By::Person("pawel"),
        SuccessorKind::CarriedForward,
    );
    // The lock link 2 replaced, but with its original's compile time changed:
    // same successors, same plan, a different original record.
    let mut replaced: LockFile = serde_yaml::from_slice(&first.bytes).unwrap();
    replaced.approval.as_mut().unwrap().original.compiled_at = "1999-01-01T00:00:00Z".into();
    let dir = crate::fs::containing_dir(&s.source).to_path_buf();
    let other = dir.join("other.lock");
    let bytes = write_lock(&other, &replaced);
    let digest = archive(&dir, &bytes, "lock");
    tamper(&second.lock_path, |l| {
        chain(l).successors[1].from_lock_digest = digest;
    });
    assert_invalid(
        &s,
        "the archived replaced lock contradicts the approval record",
    );
}

// ── review round 1 ──────────────────────────────────────────────────────────

/// Re-archive the original with `edit` applied and point the chain at it, so
/// only the changed field can tell the archive apart from the real original.
fn swap_original(s: &Setup, p: &Promoted, edit: impl FnOnce(&mut LockFile)) {
    let mut original: LockFile = serde_yaml::from_slice(&p.replaced).unwrap();
    edit(&mut original);
    let dir = crate::fs::containing_dir(&s.source).to_path_buf();
    let bytes = write_lock(&dir.join("scratch.lock"), &original);
    let digest = archive(&dir, &bytes, "lock");
    tamper(&p.lock_path, |l| {
        let c = chain(l);
        c.original.lock_digest = digest.clone();
        c.original.archive = archive_rel(&digest, "lock").unwrap();
        c.successors[0].from_lock_digest = digest;
    });
}

#[test]
fn an_original_approval_of_another_source_is_invalid() {
    let s = setup();
    // A policy link: no person record whose base-lock-digest would catch the swap.
    let p = promoted_by(&s, By::Policy("pol-1"));
    swap_original(&s, &p, |o| {
        o.source_hash = format!("sha256:{}", "4".repeat(64));
    });
    assert_invalid(
        &s,
        "original approval archive contradicts the approval record at `source-hash`",
    );
}

/// Archive `bytes` as successor 1's evidence and point the link at it.
fn swap_evidence(s: &Setup, p: &Promoted, bytes: &[u8]) {
    let dir = crate::fs::containing_dir(&s.source).to_path_buf();
    let digest = archive(&dir, bytes, "json");
    tamper(&p.lock_path, |l| {
        let link = &mut chain(l).successors[0];
        link.evidence = archive_rel(&digest, "json").unwrap();
        link.evidence_digest = digest;
    });
}

fn current_evidence(s: &Setup, p: &Promoted) -> serde_json::Value {
    let link = &p.lock.approval.as_ref().unwrap().successors[0];
    let path = crate::fs::containing_dir(&s.source).join(&link.evidence);
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}

#[test]
fn evidence_for_another_candidate_is_invalid_and_never_labels_the_link() {
    for (field, value) in [
        ("candidate-digest", format!("sha256:{}", "5".repeat(64))),
        ("plan-digest", format!("sha256:{}", "6".repeat(64))),
    ] {
        let s = setup();
        let p = promoted_by(&s, By::Person("pawel"));
        let mut evidence = current_evidence(&s, &p);
        evidence["header"][field] = serde_json::Value::String(value);
        swap_evidence(&s, &p, evidence.to_string().as_bytes());
        assert_invalid(&s, "the archived evidence contradicts the approval record");
    }
    let s = setup();
    let p = promoted_by(&s, By::Person("pawel"));
    let mut evidence = current_evidence(&s, &p);
    evidence["app"] = serde_json::Value::String("another-app".into());
    swap_evidence(&s, &p, evidence.to_string().as_bytes());
    assert_invalid(&s, "the archived evidence contradicts the approval record");
}

#[test]
fn evidence_that_is_not_evidence_is_invalid_not_silently_complete() {
    let s = setup();
    let p = promoted_by(&s, By::Person("pawel"));
    swap_evidence(&s, &p, b"this is not json");
    assert_invalid(&s, "evidence");
    let s = setup();
    let p = promoted_by(&s, By::Person("pawel"));
    swap_evidence(&s, &p, br#"{"row": {"effect": "declared-read-only"}}"#);
    assert_invalid(&s, "evidence");
}

#[test]
fn migrate_never_plans_from_an_approval_its_archives_contradict() {
    let s = setup();
    let p = promoted_by(&s, By::Person("pawel"));
    update_agent(&s.h.paths, "tool", "1.0.2", "mode: read");
    // Complete record: planned as usual, no warning about the record.
    let eval = crate::migration::plan::evaluate(&s.h.paths, &s.source, None).unwrap();
    let json = serde_json::to_value(&eval.row).unwrap();
    assert_eq!(json["state"], "needs-person", "{json}");
    assert!(
        !json["warnings"].to_string().contains("approval-record"),
        "{json}"
    );

    // A missing archive: still planned (never a refusal), and said so.
    let original = p.lock.approval.as_ref().unwrap().original.clone();
    let path = crate::fs::containing_dir(&s.source).join(&original.archive);
    let bytes = std::fs::read(&path).unwrap();
    std::fs::remove_file(&path).unwrap();
    let eval = crate::migration::plan::evaluate(&s.h.paths, &s.source, None).unwrap();
    let json = serde_json::to_value(&eval.row).unwrap();
    assert_eq!(json["state"], "needs-person", "{json}");
    assert!(eval.candidate.is_some());
    let warning = json["warnings"]
        .as_array()
        .unwrap()
        .iter()
        .find(|w| w["code"] == "approval-record-incomplete")
        .unwrap_or_else(|| panic!("no incomplete-record warning: {json}"));
    assert!(
        warning["text"]
            .as_str()
            .unwrap()
            .contains(&original.archive),
        "{warning}"
    );

    // An archive that contradicts the record: blocked, exactly as run/check refuse it.
    let mut changed = bytes;
    changed.extend_from_slice(b"# edited\n");
    std::fs::write(&path, changed).unwrap();
    let eval = crate::migration::plan::evaluate(&s.h.paths, &s.source, None).unwrap();
    let json = serde_json::to_value(&eval.row).unwrap();
    assert_eq!(json["state"], "blocked", "{json}");
    assert_eq!(json["reasons"][0]["code"], "approval-invalid", "{json}");
    assert!(eval.candidate.is_none());
}
