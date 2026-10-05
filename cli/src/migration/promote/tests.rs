//! `migrate promote|revert` (#628 plan §6–§7, §11–§15): the writer side of the
//! approval record, checked against the PR3a reader on every path.

use super::*;
use crate::app_lock::candidate::tests::{
    Home, approve, home, update_agent, write_agent, write_app,
};

const READS: &str = "requires: []\nnodes:\n  - { id: a, agent: tool, command: go }\n";
const OVERRIDABLE: &str =
    "requires: []\nnodes:\n  - { id: x, agent: tool, command: exec, mode: read }\n";

fn guard(h: &Home) -> crate::agent_store::RefGuard {
    crate::agent_store::open(&h.paths).unwrap()
}

/// What `aware app migrate prepare` writes: the candidate and its evidence.
/// Returns the candidate digest and the header.
fn prepare(
    h: &Home,
    source: &Path,
    to: Option<BTreeMap<String, PinTarget>>,
) -> (String, crate::app_lock::CandidateHeader) {
    let eval = plan::evaluate(&h.paths, source, to.as_ref(), &guard(h)).unwrap();
    assert!(
        matches!(eval.row.state, State::NeedsPerson | State::AutoUnderPolicy),
        "{:#?}",
        eval.row
    );
    let candidate = eval.candidate.unwrap();
    let evidence = Evidence {
        format: files::EVIDENCE_FORMAT.into(),
        app: eval.row.app.clone(),
        prepared_at: "2026-10-05T00:00:00Z".into(),
        cli_version: env!("CARGO_PKG_VERSION").into(),
        header: candidate.header.clone(),
        row: serde_json::to_value(&eval.row).unwrap(),
    };
    files::write_candidate(
        source.parent().unwrap(),
        &eval.row.app,
        &candidate.bytes,
        &serde_json::to_vec_pretty(&evidence).unwrap(),
    )
    .unwrap();
    (candidate.header.candidate_digest.clone(), candidate.header)
}

fn person_record(
    actor: &str,
    candidate: &str,
    header: &crate::app_lock::CandidateHeader,
) -> serde_json::Value {
    serde_json::json!({
        "format": 1,
        "kind": "person",
        "actor": actor,
        "front-door": "floless@test",
        "approval-ref": "batch-7",
        "candidate-digest": candidate,
        "base-lock-digest": header.base_lock_digest,
        "plan-digest": header.plan_digest,
        "statement-sha256": lock_digest(b"Carry these workflows forward"),
        "at": "2026-10-05T00:00:00Z",
    })
}

fn by_person(
    h: &Home,
    source: &Path,
    candidate: &str,
    record: &serde_json::Value,
    kind: SuccessorKind,
) -> Result<Promoted> {
    promote(
        &h.paths,
        &guard(h),
        source,
        candidate,
        Approver::Person {
            actor: "pawel",
            front_door: "floless@test",
            record: record.to_string().into_bytes(),
        },
        kind,
    )
}

fn lock_of(source: &Path) -> (LockFile, Vec<u8>) {
    let bytes = std::fs::read(source.with_file_name("demo.lock")).unwrap();
    (serde_yaml::from_slice(&bytes).unwrap(), bytes)
}

/// The reader's full verdict on the lock on disk: consistent and complete.
fn reader_accepts(source: &Path) -> approval::ApprovalSummary {
    let (lock, _) = lock_of(source);
    crate::agent_resolution::check_lock_consistency(&lock).unwrap();
    let summary = approval::assess(&lock, source.parent().unwrap()).unwrap();
    assert!(summary.record_complete, "{summary:#?}");
    summary
}

fn archived(source: &Path, digest: &str, ext: &str) -> Vec<u8> {
    std::fs::read(
        source
            .parent()
            .unwrap()
            .join(approval::archive_rel(digest, ext).unwrap()),
    )
    .unwrap()
}

fn approved_demo(body: &str) -> (Home, PathBuf, Vec<u8>) {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    let source = write_app(&h.paths, "demo", body);
    let (_, bytes) = approve(&h.paths, &source);
    (h, source, bytes)
}

fn code(result: Result<Promoted>) -> String {
    match result {
        Ok(p) => panic!("expected a refusal, got {p:#?}"),
        Err(refused) => refused.code,
    }
}

#[test]
fn a_person_promotion_replaces_the_lock_with_a_record_its_reader_accepts() {
    let (h, source, original_bytes) = approved_demo(READS);
    let new_digest = update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    let (candidate, header) = prepare(&h, &source, None);
    let dir = source.parent().unwrap().to_path_buf();
    let candidate_bytes = std::fs::read(files::candidate_path(&dir, "demo")).unwrap();
    let evidence_bytes = std::fs::read(files::evidence_path(&dir, "demo")).unwrap();
    let record = person_record("pawel", &candidate, &header);

    let promoted = by_person(
        &h,
        &source,
        &candidate,
        &record,
        SuccessorKind::CarriedForward,
    )
    .unwrap();
    assert_eq!(promoted.seq, 1);
    assert_eq!(promoted.by_kind, "person");
    assert_eq!(promoted.to["tool"].version, "1.0.1");
    assert_eq!(promoted.from["tool"].version, "1.0.0");

    let (lock, bytes) = lock_of(&source);
    assert_eq!(promoted.lock_digest, lock_digest(&bytes));
    assert!(
        lock.source_hash
            .starts_with(approval::SUCCESSOR_SOURCE_PREFIX)
    );
    assert_eq!(
        lock.agent_digests["tool"], new_digest,
        "the run reads the new pins"
    );
    let summary = reader_accepts(&source);
    assert_eq!(summary.origin, approval::Origin::Successor);
    assert_eq!(
        summary.attested,
        Some(false),
        "a person approval is a claim"
    );
    assert!(
        summary
            .label
            .contains("claimed person approval by pawel, recorded by floless@test"),
        "{}",
        summary.label
    );
    assert!(
        summary
            .label
            .contains("checked by inspection, nothing was run"),
        "{}",
        summary.label
    );

    // Every archive the link cites holds exactly the bytes it names.
    let chain = lock.approval.as_ref().unwrap();
    let link = &chain.successors[0];
    assert_eq!(
        archived(&source, &chain.original.lock_digest, "lock"),
        original_bytes
    );
    assert_eq!(
        archived(&source, &link.resulting_lock_digest, "lock"),
        candidate_bytes
    );
    assert_eq!(
        archived(&source, &link.evidence_digest, "json"),
        evidence_bytes
    );
    let CarriedForwardBy::Person {
        approval_record_digest,
        approval_ref,
        attested,
        ..
    } = &link.carried_forward_by
    else {
        panic!("{link:#?}")
    };
    assert_eq!(approval_ref, "batch-7");
    assert!(!attested);
    assert_eq!(
        archived(&source, approval_record_digest, "json"),
        record.to_string().into_bytes()
    );
    // The consumed candidate is gone; no transaction is left behind.
    assert!(!files::candidate_path(&dir, "demo").exists());
    assert!(!dir.join(approval::ARCHIVE_DIR).join(TXN_DIR).exists());
    // And a run now resolves the new bytes.
    let pins = crate::agent_resolution::PinSet::from_lock(&lock, BTreeMap::new()).unwrap();
    let app = crate::app_lock::read_app_source(&source).unwrap().0;
    let resolved = crate::agent_resolution::resolve_pins(&h.paths, &app, &pins).unwrap();
    assert_eq!(resolved[0].digest, new_digest);
}

#[test]
fn a_second_promotion_appends_seq_2_and_archives_the_lock_it_replaced() {
    let (h, source, _) = approved_demo(READS);
    update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    let (c1, h1) = prepare(&h, &source, None);
    by_person(
        &h,
        &source,
        &c1,
        &person_record("pawel", &c1, &h1),
        SuccessorKind::CarriedForward,
    )
    .unwrap();
    let (_, first_bytes) = lock_of(&source);

    update_agent(&h.paths, "tool", "1.0.2", "mode: read");
    let (c2, h2) = prepare(&h, &source, None);
    assert_eq!(h2.base_lock_digest, lock_digest(&first_bytes));
    let promoted = by_person(
        &h,
        &source,
        &c2,
        &person_record("pawel", &c2, &h2),
        SuccessorKind::CarriedForward,
    )
    .unwrap();
    assert_eq!(promoted.seq, 2);
    let summary = reader_accepts(&source);
    assert_eq!(summary.successors.len(), 2);
    let (lock, _) = lock_of(&source);
    let link = &lock.approval.as_ref().unwrap().successors[1];
    assert_eq!(
        archived(&source, &link.from_lock_digest, "lock"),
        first_bytes
    );
}

#[test]
fn a_person_approval_must_name_this_exact_promotion() {
    let (h, source, _) = approved_demo(READS);
    update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    let (candidate, header) = prepare(&h, &source, None);
    let (_, before) = lock_of(&source);
    let good = person_record("pawel", &candidate, &header);
    let other = format!("sha256:{}", "0".repeat(64));
    let cases: [(&str, serde_json::Value); 8] = [
        ("candidate-digest", other.clone().into()),
        ("base-lock-digest", other.clone().into()),
        ("plan-digest", other.into()),
        ("actor", "someone-else".into()),
        ("front-door", "other@1".into()),
        ("kind", "policy".into()),
        ("approval-ref", "".into()),
        ("statement-sha256", "not a digest".into()),
    ];
    for (field, value) in cases {
        let mut record = good.clone();
        record[field] = value;
        let refused = by_person(
            &h,
            &source,
            &candidate,
            &record,
            SuccessorKind::CarriedForward,
        )
        .unwrap_err();
        assert_eq!(
            refused.code, "E_MIGRATE_APPROVAL_MISMATCH",
            "{field}: {:?}",
            refused.error
        );
        assert_eq!(lock_of(&source).1, before, "{field}: the lock moved");
    }
    // The control: the same record unmodified promotes.
    by_person(
        &h,
        &source,
        &candidate,
        &good,
        SuccessorKind::CarriedForward,
    )
    .unwrap();
}

#[test]
fn a_stale_or_tampered_candidate_is_refused_and_changes_nothing() {
    let (h, source, _) = approved_demo(READS);
    update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    let dir = source.parent().unwrap().to_path_buf();

    // No candidate at all.
    let any = format!("sha256:{}", "a".repeat(64));
    let record = serde_json::json!({});
    assert_eq!(
        code(by_person(
            &h,
            &source,
            &any,
            &record,
            SuccessorKind::CarriedForward
        )),
        "E_MIGRATE_NO_CANDIDATE"
    );

    let (candidate, header) = prepare(&h, &source, None);
    let record = person_record("pawel", &candidate, &header);
    let (_, before) = lock_of(&source);
    // A different candidate digest than the one on disk.
    assert_eq!(
        code(by_person(
            &h,
            &source,
            &any,
            &record,
            SuccessorKind::CarriedForward
        )),
        "E_MIGRATE_CANDIDATE_STALE"
    );
    // Evidence that describes other bytes.
    let evidence_path = files::evidence_path(&dir, "demo");
    let evidence = std::fs::read_to_string(&evidence_path).unwrap();
    std::fs::write(&evidence_path, evidence.replace(&candidate, &any)).unwrap();
    assert_eq!(
        code(by_person(
            &h,
            &source,
            &candidate,
            &record,
            SuccessorKind::CarriedForward
        )),
        "E_MIGRATE_CANDIDATE_TAMPERED"
    );
    std::fs::write(&evidence_path, &evidence).unwrap();
    assert_eq!(lock_of(&source).1, before);

    // A person compiles again after the candidate was prepared.
    approve(&h.paths, &source);
    let (_, recompiled) = lock_of(&source);
    assert_eq!(
        code(by_person(
            &h,
            &source,
            &candidate,
            &record,
            SuccessorKind::CarriedForward
        )),
        "E_MIGRATE_CANDIDATE_STALE"
    );
    assert_eq!(lock_of(&source).1, recompiled);
}

#[test]
fn a_held_app_is_never_promoted() {
    let (h, source, _) = approved_demo(READS);
    update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    let (candidate, header) = prepare(&h, &source, None);
    files::write_hold(
        source.parent().unwrap(),
        &files::HoldRecord {
            format: 1,
            app: "demo".into(),
            held_by: "pawel".into(),
            held_at: "t".into(),
            reason: Some("certified".into()),
        },
    )
    .unwrap();
    let (_, before) = lock_of(&source);
    let record = person_record("pawel", &candidate, &header);
    assert_eq!(
        code(by_person(
            &h,
            &source,
            &candidate,
            &record,
            SuccessorKind::CarriedForward
        )),
        "E_MIGRATE_HELD"
    );
    assert_eq!(lock_of(&source).1, before);
}

#[test]
fn a_missing_archive_in_the_current_record_refuses_the_next_promotion() {
    let (h, source, _) = approved_demo(READS);
    update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    let (c1, h1) = prepare(&h, &source, None);
    by_person(
        &h,
        &source,
        &c1,
        &person_record("pawel", &c1, &h1),
        SuccessorKind::CarriedForward,
    )
    .unwrap();
    let (lock, _) = lock_of(&source);
    let original = &lock.approval.as_ref().unwrap().original.lock_digest;
    std::fs::remove_file(
        source
            .parent()
            .unwrap()
            .join(approval::archive_rel(original, "lock").unwrap()),
    )
    .unwrap();
    update_agent(&h.paths, "tool", "1.0.2", "mode: read");
    let (c2, h2) = prepare(&h, &source, None);
    let (_, before) = lock_of(&source);
    assert_eq!(
        code(by_person(
            &h,
            &source,
            &c2,
            &person_record("pawel", &c2, &h2),
            SuccessorKind::CarriedForward
        )),
        "E_MIGRATE_ARCHIVE_INVALID"
    );
    assert_eq!(lock_of(&source).1, before);
}

/// §12 R1-7: a crash at any step leaves the old lock (rolled back by the next
/// migrate verb) or the whole promotion (finished by it) — never a lock whose
/// record cannot be shown.
#[test]
fn a_crash_at_any_step_recovers_to_the_old_lock_or_the_whole_promotion() {
    for (fault, lock_moved) in [
        (Fault::Staged, false),
        (Fault::ArchivesPlaced, false),
        (Fault::LockReplaced, true),
    ] {
        let (h, source, original_bytes) = approved_demo(READS);
        update_agent(&h.paths, "tool", "1.0.1", "mode: read");
        let (candidate, header) = prepare(&h, &source, None);
        let record = person_record("pawel", &candidate, &header);
        inject_fault(fault);
        let crashed = by_person(
            &h,
            &source,
            &candidate,
            &record,
            SuccessorKind::CarriedForward,
        )
        .unwrap_err();
        assert!(
            crashed.error.to_string().contains("injected crash"),
            "{fault:?}: {:?}",
            crashed.error
        );
        let dir = source.parent().unwrap();
        assert!(
            !owned_txns(dir, "demo").unwrap().is_empty(),
            "{fault:?}: a crash leaves its transaction"
        );
        let (_, after_crash) = lock_of(&source);
        assert_eq!(after_crash != original_bytes, lock_moved, "{fault:?}");

        let g = guard(&h);
        let held = lock_app(&h.paths, &g, dir, "demo").unwrap();
        let recovered = recover(dir, "demo", &held).unwrap();
        drop(held);
        assert_eq!(recovered.len(), 1, "{fault:?}");
        assert_eq!(
            recovered[0].outcome,
            if lock_moved {
                "finished"
            } else {
                "rolled-back"
            },
            "{fault:?}"
        );
        assert!(owned_txns(dir, "demo").unwrap().is_empty());
        if lock_moved {
            reader_accepts(&source);
        } else {
            assert_eq!(lock_of(&source).1, original_bytes, "{fault:?}");
            // ...and the promotion can simply be run again.
            by_person(
                &h,
                &source,
                &candidate,
                &record,
                SuccessorKind::CarriedForward,
            )
            .unwrap();
            reader_accepts(&source);
        }
    }
}

/// Recovery touches only its own app's transactions.
#[test]
fn recovery_leaves_another_app_s_transaction_alone() {
    let (h, source, _) = approved_demo(READS);
    let dir = source.parent().unwrap();
    let other = txn_root(dir).join(format!("demo2.{}", "b".repeat(32)));
    std::fs::create_dir_all(&other).unwrap();
    let g = guard(&h);
    let held = lock_app(&h.paths, &g, dir, "demo").unwrap();
    assert!(recover(dir, "demo", &held).unwrap().is_empty());
    assert!(other.is_dir());
}

/// §15.1 R1: a person's compile of the same app waits for a promotion's lock
/// (and so can never land between its CAS and its rename).
#[test]
fn a_compile_waits_for_the_app_s_promotion_lock() {
    let (h, source, _) = approved_demo(READS);
    let dir = source.parent().unwrap().to_path_buf();
    let g = guard(&h);
    let held = lock_app(&h.paths, &g, &dir, "demo").unwrap();
    let paths = h.paths.clone();
    let compile_source = source.clone();
    let (tx, rx) = std::sync::mpsc::channel();
    let worker = std::thread::spawn(move || {
        let guard = crate::agent_store::open(&paths).unwrap();
        crate::app_lock::compile_to_disk(&compile_source, &paths, &guard).unwrap();
        tx.send(()).unwrap();
    });
    assert!(
        rx.recv_timeout(std::time::Duration::from_millis(400))
            .is_err(),
        "the compile wrote the lock while a promotion held it"
    );
    drop(held);
    rx.recv_timeout(std::time::Duration::from_secs(30))
        .expect("the compile finished once the lock was released");
    worker.join().unwrap();
}

// ── policies ────────────────────────────────────────────────────────────────

/// Install `version` of `tool` as an official-registry install would leave it:
/// with a receipt saying so (the store keeps the receipt with the bytes).
fn update_official(h: &Home, version: &str, mode_line: &str) -> String {
    let dir = write_agent(&h.paths, "tool", version, mode_line);
    crate::install::provenance::write(
        &dir,
        &crate::install::provenance::InstallSource::Registry {
            key: "tool".into(),
            version: version.into(),
            manifest_agent: Some("tool".into()),
            manifest_version: Some(version.into()),
            entry_digest: None,
            installed_digest: None,
            official_source: true,
        },
    );
    crate::agent_store::snapshot(&h.paths, &dir, &guard(h))
        .unwrap()
        .digest
}

fn record_policy(h: &Home, apps: &[&str], agents: &[&str]) -> String {
    let record = serde_json::json!({
        "format": 1, "kind": "policy", "actor": "pawel", "front-door": "floless@test",
        "approval-ref": "first-approval", "statement-sha256": lock_digest(b"policy text"),
        "at": "2026-10-05T00:00:00Z", "rule": "read-only-patch",
        "scope": { "apps": apps, "agents": agents, "publishers": ["official-registry"], "bump": "patch" },
    });
    policy::record(&h.paths, record.to_string().as_bytes(), "floless@test")
        .unwrap()
        .0
        .policy
        .policy
}

fn verified(_: &CandidatePin) -> std::result::Result<(), String> {
    Ok(())
}

fn offline(_: &CandidatePin) -> std::result::Result<(), String> {
    Err("the official registry could not be fetched: offline (test)".into())
}

fn by_policy(
    h: &Home,
    source: &Path,
    candidate: &str,
    id: &str,
    official: &dyn Fn(&CandidatePin) -> std::result::Result<(), String>,
) -> Result<Promoted> {
    promote(
        &h.paths,
        &guard(h),
        source,
        candidate,
        Approver::Policy {
            id,
            front_door: "floless@test",
            official,
        },
        SuccessorKind::CarriedForward,
    )
}

fn row(h: &Home, source: &Path) -> plan::PlanRow {
    plan::evaluate(&h.paths, source, None, &guard(h))
        .unwrap()
        .row
}

/// Owner decision 2 (floless.app#1985) opens the no-click path for a declared
/// read-only patch with identical instructions — and only a person-approved
/// policy walks it.
#[test]
fn a_covering_policy_carries_a_read_only_patch_forward_with_no_click() {
    let (h, source, _) = approved_demo(READS);
    update_official(&h, "1.0.1", "mode: read");
    let before = row(&h, &source);
    assert!(before.no_click_available, "{before:#?}");
    assert_eq!(before.state, State::NeedsPerson, "no policy yet");
    assert!(
        before.reasons.iter().any(|r| r.code == "no-policy"),
        "{before:#?}"
    );

    let id = record_policy(&h, &["demo"], &["tool"]);
    let covered = row(&h, &source);
    assert_eq!(covered.state, State::AutoUnderPolicy, "{covered:#?}");
    assert_eq!(covered.policy.as_deref(), Some(id.as_str()));

    let (candidate, _) = prepare(&h, &source, None);
    let promoted = by_policy(&h, &source, &candidate, &id, &verified).unwrap();
    assert_eq!(promoted.by_kind, "policy");
    assert_eq!(promoted.policy.as_deref(), Some(id.as_str()));
    let summary = reader_accepts(&source);
    assert_eq!(summary.policy_id.as_deref(), Some(id.as_str()));
    assert!(
        summary
            .label
            .contains(&format!("under policy {id} (claimed approval by pawel)")),
        "{}",
        summary.label
    );
    let (lock, _) = lock_of(&source);
    let CarriedForwardBy::Policy { policy_digest, .. } =
        &lock.approval.as_ref().unwrap().successors[0].carried_forward_by
    else {
        panic!()
    };
    assert_eq!(*policy_digest, policy::load(&h.paths, &id).unwrap().digest);
}

#[test]
fn a_policy_never_carries_what_it_does_not_cover() {
    // (app body, new version, mode, policy apps, policy agents, official check, expected reason)
    type Check = fn(&CandidatePin) -> std::result::Result<(), String>;
    type Case<'a> = (
        &'a str,
        &'a str,
        &'a str,
        &'a [&'a str],
        &'a [&'a str],
        Check,
        &'a str,
    );
    let cases: [Case; 6] = [
        (
            READS,
            "1.0.1",
            "mode: read",
            &["demo"],
            &["tool"],
            offline,
            "official-unverified",
        ),
        (
            READS,
            "1.1.0",
            "mode: read",
            &["demo"],
            &["tool"],
            verified,
            "policy-not-patch",
        ),
        (
            READS,
            "1.0.1",
            "mode: read",
            &["other"],
            &["tool"],
            verified,
            "policy-app-out-of-scope",
        ),
        (
            READS,
            "1.0.1",
            "mode: read",
            &["*"],
            &["other"],
            verified,
            "policy-agent-out-of-scope",
        ),
        (
            OVERRIDABLE,
            "1.0.1",
            "mode: read",
            &["*"],
            &["*"],
            verified,
            "not-declared-read-only",
        ),
        (
            READS,
            "1.0.1",
            "mode: read\n    timeout-ms: 5",
            &["*"],
            &["*"],
            verified,
            "contract-changed",
        ),
    ];
    for (body, version, mode, apps, agents, official, reason) in cases {
        let (h, source, _) = approved_demo(body);
        update_official(&h, version, mode);
        let id = record_policy(&h, apps, agents);
        // The plan judges on local facts (the install receipt says official);
        // only promotion fetches the registry, so an offline verification is
        // caught there, before anything moves.
        let planned = row(&h, &source).state;
        if reason == "official-unverified" {
            assert_eq!(planned, State::AutoUnderPolicy, "{reason}");
        } else {
            assert_ne!(planned, State::AutoUnderPolicy, "{reason}");
        }
        let (candidate, _) = prepare(&h, &source, None);
        let (_, before) = lock_of(&source);
        let refused = by_policy(&h, &source, &candidate, &id, &official).unwrap_err();
        assert_eq!(
            refused.code, "E_MIGRATE_NOT_ELIGIBLE",
            "{reason}: {:?}",
            refused.error
        );
        let codes: Vec<String> = refused.details["reasons"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| r["code"].as_str().unwrap().to_string())
            .collect();
        assert!(codes.contains(&reason.to_string()), "{reason}: {codes:?}");
        assert_eq!(lock_of(&source).1, before, "{reason}: the lock moved");
    }
}

#[test]
fn a_package_without_an_official_receipt_is_not_covered() {
    let (h, source, _) = approved_demo(READS);
    update_agent(&h.paths, "tool", "1.0.1", "mode: read"); // no receipt
    let id = record_policy(&h, &["*"], &["*"]);
    let r = row(&h, &source);
    assert_eq!(r.state, State::NeedsPerson);
    let (candidate, _) = prepare(&h, &source, None);
    let refused = by_policy(&h, &source, &candidate, &id, &verified).unwrap_err();
    assert!(
        refused.details.to_string().contains("policy-not-official"),
        "{:?}",
        refused.details
    );
}

#[test]
fn a_revoked_or_edited_policy_carries_nothing() {
    let (h, source, _) = approved_demo(READS);
    update_official(&h, "1.0.1", "mode: read");
    let id = record_policy(&h, &["demo"], &["tool"]);
    let (candidate, _) = prepare(&h, &source, None);
    policy::revoke(&h.paths, &id, "pawel", "floless@test", Some("done".into())).unwrap();
    assert_eq!(row(&h, &source).state, State::NeedsPerson);
    assert_eq!(
        code(by_policy(&h, &source, &candidate, &id, &verified)),
        "E_MIGRATE_POLICY_REVOKED"
    );

    let id2 = record_policy(&h, &["*"], &["tool"]);
    let path = policy::file_of(&h.paths, &id2);
    let text = std::fs::read_to_string(&path).unwrap();
    std::fs::write(&path, text.replace("- tool", "- other")).unwrap();
    assert_eq!(
        code(by_policy(&h, &source, &candidate, &id2, &verified)),
        "E_MIGRATE_POLICY_INVALID"
    );
    assert_eq!(
        code(by_policy(
            &h,
            &source,
            &candidate,
            "pol-0000000000000000",
            &verified
        )),
        "E_MIGRATE_POLICY_NOT_FOUND"
    );
}

/// §15.1 R3: a revocation waits for a policy promotion in flight.
#[test]
fn a_revocation_waits_for_a_policy_promotion_in_flight() {
    let (h, _, _) = approved_demo(READS);
    let id = record_policy(&h, &["*"], &["*"]);
    let shared = policy::lock_policies(&h.paths, false).unwrap();
    let paths = h.paths.clone();
    let (tx, rx) = std::sync::mpsc::channel();
    let worker = std::thread::spawn(move || {
        policy::revoke(&paths, &id, "pawel", "floless@test", None).unwrap();
        tx.send(()).unwrap();
    });
    assert!(
        rx.recv_timeout(std::time::Duration::from_millis(400))
            .is_err()
    );
    drop(shared);
    rx.recv_timeout(std::time::Duration::from_secs(30)).unwrap();
    worker.join().unwrap();
}

// ── revert ──────────────────────────────────────────────────────────────────

fn back_to(digest: &str) -> Option<BTreeMap<String, PinTarget>> {
    Some([("tool".to_string(), PinTarget::Digest(digest.to_string()))].into())
}

#[test]
fn a_person_revert_goes_back_only_to_pins_approved_before() {
    let (h, source, _) = approved_demo(READS);
    let (original, _) = lock_of(&source);
    let old = original.agent_digests["tool"].clone();
    update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    let (c1, h1) = prepare(&h, &source, None);
    by_person(
        &h,
        &source,
        &c1,
        &person_record("pawel", &c1, &h1),
        SuccessorKind::CarriedForward,
    )
    .unwrap();

    // 1.0.2 was never approved: a revert to it is refused.
    let never = update_agent(&h.paths, "tool", "1.0.2", "mode: read");
    let (c, hd) = prepare(&h, &source, back_to(&never));
    assert_eq!(
        code(by_person(
            &h,
            &source,
            &c,
            &person_record("pawel", &c, &hd),
            SuccessorKind::Reverted
        )),
        "E_MIGRATE_REVERT_UNAPPROVED"
    );

    // Back to the original's bytes.
    let (c2, h2) = prepare(&h, &source, back_to(&old));
    let reverted = by_person(
        &h,
        &source,
        &c2,
        &person_record("pawel", &c2, &h2),
        SuccessorKind::Reverted,
    )
    .unwrap();
    assert_eq!(reverted.kind, SuccessorKind::Reverted);
    assert_eq!(reverted.seq, 2);
    let summary = reader_accepts(&source);
    assert!(
        summary
            .label
            .starts_with("approval reverted from tool 1.0.1 to 1.0.0"),
        "{}",
        summary.label
    );
    assert!(
        !summary.label.to_lowercase().contains("undo"),
        "{}",
        summary.label
    );
    assert_eq!(lock_of(&source).0.agent_digests["tool"], old);
}

#[test]
fn an_original_approval_has_nothing_to_revert() {
    let (h, source, _) = approved_demo(READS);
    let new = update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    let (c, hd) = prepare(&h, &source, back_to(&new));
    assert_eq!(
        code(by_person(
            &h,
            &source,
            &c,
            &person_record("pawel", &c, &hd),
            SuccessorKind::Reverted
        )),
        "E_MIGRATE_REVERT_UNAPPROVED"
    );
}

fn automatic(h: &Home, source: &Path, candidate: &str, reason: &str) -> Result<Promoted> {
    promote(
        &h.paths,
        &guard(h),
        source,
        candidate,
        Approver::Automatic {
            reason,
            front_door: "floless@test",
        },
        SuccessorKind::Reverted,
    )
}

#[test]
fn an_automatic_revert_undoes_only_a_policy_s_move_back_to_an_approved_plan() {
    let (h, source, _) = approved_demo(READS);
    let old = lock_of(&source).0.agent_digests["tool"].clone();
    update_official(&h, "1.0.1", "mode: read");
    let id = record_policy(&h, &["demo"], &["tool"]);
    let (c1, _) = prepare(&h, &source, None);
    by_policy(&h, &source, &c1, &id, &verified).unwrap();

    let (c2, _) = prepare(&h, &source, back_to(&old));
    assert_eq!(
        code(automatic(&h, &source, &c2, "because")),
        "E_MIGRATE_NOT_ELIGIBLE",
        "the reason must name the failed run"
    );
    let reverted = automatic(&h, &source, &c2, "run-failed:run-42").unwrap();
    assert_eq!(
        reverted.by_kind, "policy",
        "recorded under the policy that moved it"
    );
    assert_eq!(reverted.policy.as_deref(), Some(id.as_str()));
    reader_accepts(&source);
    let (lock, _) = lock_of(&source);
    let link = &lock.approval.as_ref().unwrap().successors[1];
    let evidence: serde_json::Value =
        serde_json::from_slice(&archived(&source, &link.evidence_digest, "json")).unwrap();
    assert_eq!(evidence["row"]["revert"]["reason"], "run-failed:run-42");
    assert_eq!(evidence["row"]["revert"]["automatic"], true);
}

#[test]
fn an_automatic_revert_never_undoes_a_person_s_approval() {
    let (h, source, _) = approved_demo(READS);
    let old = lock_of(&source).0.agent_digests["tool"].clone();
    update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    let (c1, h1) = prepare(&h, &source, None);
    by_person(
        &h,
        &source,
        &c1,
        &person_record("pawel", &c1, &h1),
        SuccessorKind::CarriedForward,
    )
    .unwrap();
    let (c2, _) = prepare(&h, &source, back_to(&old));
    let (_, before) = lock_of(&source);
    assert_eq!(
        code(automatic(&h, &source, &c2, "run-failed:run-1")),
        "E_MIGRATE_NOT_ELIGIBLE"
    );
    assert_eq!(lock_of(&source).1, before);
}

/// §15.1 R2: the same pins recompiled to a different plan (a newer compiler)
/// are not the plan a person approved.
#[test]
fn an_automatic_revert_needs_the_approved_plan_not_only_its_pins() {
    let (h, source, _) = approved_demo(READS);
    let old = lock_of(&source).0.agent_digests["tool"].clone();
    update_official(&h, "1.0.1", "mode: read");
    let id = record_policy(&h, &["demo"], &["tool"]);
    let (c1, _) = prepare(&h, &source, None);
    by_policy(&h, &source, &c1, &id, &verified).unwrap();
    let (c2, h2) = prepare(&h, &source, back_to(&old));
    let (lock, _) = lock_of(&source);
    let chain = lock.approval.as_ref().unwrap();
    let to = approved_states(chain)[0].clone();
    let dir = source.parent().unwrap();
    // The control: the real plan digest is accepted…
    automatic_authority(
        "demo",
        dir,
        Some(chain),
        &to,
        &h2.plan_digest,
        "run-failed:r",
    )
    .unwrap();
    // …a different plan at the same pins is not.
    let other = format!("sha256:{}", "1".repeat(64));
    let refused =
        automatic_authority("demo", dir, Some(chain), &to, &other, "run-failed:r").unwrap_err();
    assert_eq!(refused.code, "E_MIGRATE_NOT_ELIGIBLE");
    let _ = c2;
}

#[test]
fn the_ai_session_marker_is_reported_by_name() {
    let _clear = crate::test_env::EnvVarGuard::scope(&[
        ("CLAUDECODE", None),
        ("CODEX_SANDBOX", None),
        ("AWARE_AI_SESSION", None),
    ]);
    assert_eq!(ai_session_marker(), None);
    drop(_clear);
    let _set = crate::test_env::EnvVarGuard::scope(&[
        ("CLAUDECODE", None),
        ("CODEX_SANDBOX", Some(std::ffi::OsStr::new("seatbelt"))),
        ("AWARE_AI_SESSION", None),
    ]);
    assert_eq!(ai_session_marker(), Some("CODEX_SANDBOX"));
}
