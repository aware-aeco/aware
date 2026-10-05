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
    assert!(owned_txns(&dir, "demo").unwrap().is_empty());
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
        let archives_before = archive_files(source.parent().unwrap());
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
            // Review round 3: a rolled-back promotion leaves no new records.
            assert_eq!(archive_files(dir), archives_before, "{fault:?}");
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
    let (_, before) = lock_of(&source);
    // Codex review round 1: the failed run must be one the CLI can find — a
    // run of this app, under the approval being reverted, that failed.
    let current = last_evidence(&source);
    record_run(&h, "run-ok", Some((1, &current)), "ok", false);
    record_run(&h, "run-under-original", None, "error", true);
    let other = format!("sha256:{}", "9".repeat(64));
    record_run(&h, "run-other-seq-1", Some((1, &other)), "error", true);
    record_run(&h, "run-42", Some((1, &current)), "error", false);
    for (reason, why) in [
        ("because", "the reason must name the failed run"),
        ("run-failed:no-such-run", "an unknown run is no failure"),
        ("run-failed:run-ok", "a run that succeeded is no failure"),
        (
            "run-failed:run-under-original",
            "a failure under another approval",
        ),
        (
            "run-failed:run-other-seq-1",
            "a failure under an earlier successor 1 (before a recompile)",
        ),
        ("run-failed:../escape", "a run id is a plain name"),
    ] {
        assert_eq!(
            code(automatic(&h, &source, &c2, reason)),
            "E_MIGRATE_NOT_ELIGIBLE",
            "{why}"
        );
        assert_eq!(lock_of(&source).1, before, "{why}: the lock moved");
    }
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
    assert_eq!(evidence["row"]["revert"]["failed-run"]["run-id"], "run-42");
    assert_eq!(evidence["row"]["revert"]["failed-run"]["status"], "error");
}

/// Write the trace of a run of `demo`: under successor `seq` (or the original
/// approval), ending `status`, optionally with a node error.
fn record_run(h: &Home, run_id: &str, under: Option<(u32, &str)>, status: &str, node_error: bool) {
    use crate::runtime::provenance::RunEvent;
    let approval = match under {
        Some((seq, evidence)) => {
            serde_json::json!({ "origin": "successor", "seq": seq, "evidence-digest": evidence })
        }
        None => serde_json::json!({ "origin": "original" }),
    };
    let mut events = vec![RunEvent::RunStart {
        ts: "t".into(),
        run_id: run_id.into(),
        app: "demo".into(),
        instance: "default".into(),
        config: serde_json::json!({ "approval": approval }),
    }];
    if node_error {
        events.push(RunEvent::NodeError {
            ts: "t".into(),
            run_id: run_id.into(),
            node: "a".into(),
            error: "boom".into(),
            structured: None,
        });
    }
    events.push(RunEvent::RunEnd {
        ts: "t".into(),
        run_id: run_id.into(),
        status: status.into(),
    });
    let path =
        crate::runtime::provenance::log_path_for(&h.paths.logs_dir(), "demo", "default", run_id);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    let body: String = events
        .iter()
        .map(|e| {
            serde_json::to_string(e).unwrap()
                + "
"
        })
        .collect();
    std::fs::write(path, body).unwrap();
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
    // Even after a genuinely failed run under it, a person's approval is not
    // undone automatically.
    let current = last_evidence(&source);
    record_run(&h, "run-1", Some((1, &current)), "error", true);
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

/// A content-addressed archive already in place must hash to its name; one
/// that does not is a changed record, refused before the lock moves.
#[test]
fn a_changed_archive_already_in_place_refuses_the_promotion() {
    let (h, source, _) = approved_demo(READS);
    update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    let (candidate, header) = prepare(&h, &source, None);
    let dir = source.parent().unwrap();
    let squatter = dir.join(approval::archive_rel(&candidate, "lock").unwrap());
    std::fs::create_dir_all(squatter.parent().unwrap()).unwrap();
    std::fs::write(&squatter, b"not the candidate").unwrap();
    let (_, before) = lock_of(&source);
    let archives_before = archive_files(dir);
    let record = person_record("pawel", &candidate, &header);
    let refused = by_person(
        &h,
        &source,
        &candidate,
        &record,
        SuccessorKind::CarriedForward,
    )
    .unwrap_err();
    assert_eq!(
        refused.code, "E_MIGRATE_ARCHIVE_INVALID",
        "{:?}",
        refused.error
    );
    assert_eq!(lock_of(&source).1, before);
    assert!(owned_txns(dir, "demo").unwrap().is_empty(), "rolled back");
    assert_eq!(
        archive_files(dir),
        archives_before,
        "nothing else was written"
    );
}

/// §15.1 R3: a policy promotion holds the policies lock shared, so it waits
/// while a revocation holds it exclusive.
#[test]
fn a_policy_promotion_waits_for_a_revocation_in_flight() {
    let (h, source, _) = approved_demo(READS);
    update_official(&h, "1.0.1", "mode: read");
    let id = record_policy(&h, &["demo"], &["tool"]);
    let (candidate, _) = prepare(&h, &source, None);
    let exclusive = policy::lock_policies(&h.paths, true).unwrap();
    let paths = h.paths.clone();
    let worker_source = source.clone();
    let (tx, rx) = std::sync::mpsc::channel();
    let worker = std::thread::spawn(move || {
        let guard = crate::agent_store::open(&paths).unwrap();
        let result = promote(
            &paths,
            &guard,
            &worker_source,
            &candidate,
            Approver::Policy {
                id: &id,
                front_door: "floless@test",
                official: &verified,
            },
            SuccessorKind::CarriedForward,
        );
        tx.send(result.map(|p| p.seq).map_err(|r| r.code)).unwrap();
    });
    assert!(
        rx.recv_timeout(std::time::Duration::from_millis(400))
            .is_err(),
        "the policy promotion ran while a revocation held the policies lock"
    );
    drop(exclusive);
    let outcome = rx.recv_timeout(std::time::Duration::from_secs(30)).unwrap();
    worker.join().unwrap();
    assert_eq!(outcome, Ok(1));
}

/// The negative control of the writer's self-check: a chain the reader would
/// refuse is never written — the lock stays, the transaction rolls back.
#[test]
fn a_chain_its_own_reader_would_refuse_is_never_written() {
    let (h, source, _) = approved_demo(READS);
    update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    let (candidate, header) = prepare(&h, &source, None);
    let (_, before) = lock_of(&source);
    let archives_before = archive_files(source.parent().unwrap());
    corrupt_next_chain();
    let refused = by_person(
        &h,
        &source,
        &candidate,
        &person_record("pawel", &candidate, &header),
        SuccessorKind::CarriedForward,
    )
    .unwrap_err();
    assert!(
        refused
            .error
            .to_string()
            .contains("would not pass its own reader"),
        "{:?}",
        refused.error
    );
    assert_eq!(lock_of(&source).1, before);
    assert!(
        owned_txns(source.parent().unwrap(), "demo")
            .unwrap()
            .is_empty()
    );
    assert_eq!(
        archive_files(source.parent().unwrap()),
        archives_before,
        "a refused promotion leaves no new records"
    );
}

/// The archive files in `.aware-approvals/` (not the transaction area).
fn archive_files(dir: &Path) -> std::collections::BTreeSet<String> {
    std::fs::read_dir(dir.join(approval::ARCHIVE_DIR))
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().is_file())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect()
}

/// The evidence digest of the last successor of the lock on disk.
fn last_evidence(source: &Path) -> String {
    lock_of(source)
        .0
        .approval
        .unwrap()
        .successors
        .last()
        .unwrap()
        .evidence_digest
        .clone()
}

/// Review round 2: a workflow that runs a backing workflow with a pending
/// update is never carried forward under a policy, as the plan says.
#[test]
fn a_caller_of_a_moving_backing_app_is_not_carried_forward_under_a_policy() {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    let backing = write_app(
        &h.paths,
        "inner",
        "exposes-as-agent: true\nexposed-commands:\n  ask:\n    lifecycle: single\n    outputs:\n      type: single\n\
         requires: []\nnodes:\n  - { id: a, agent: tool, command: go }\n",
    );
    approve(&h.paths, &backing);
    let wrapper = h.paths.agents_dir().join("inner-wrapper");
    std::fs::create_dir_all(&wrapper).unwrap();
    std::fs::write(
        wrapper.join("manifest.yaml"),
        "agent: inner-wrapper\nversion: 1.0.0\ndescription: x\nstateful: false\nlicense: app-exposed\n\
         transport:\n  app:\n    backed-by: inner\n\
         commands:\n  ask:\n    lifecycle: single\n    mode: read\n    description: x\n",
    )
    .unwrap();
    let source = write_app(
        &h.paths,
        "demo",
        "requires: []\nnodes:\n  - { id: a, agent: tool, command: go }\n  - { id: c, agent: inner-wrapper, command: ask }\n",
    );
    approve(&h.paths, &source);
    update_official(&h, "1.0.1", "mode: read");
    let id = record_policy(&h, &["*"], &["*"]);
    // On its own the caller's update is no-click eligible and covered...
    let own = row(&h, &source);
    assert!(own.no_click_available, "{own:#?}");
    assert_eq!(own.state, State::AutoUnderPolicy, "{own:#?}");
    // ...but the plan, which sees the backing app's pending move, gives one
    // consistent answer: a person decides (review round 5).
    let rows = plan::plan_rows(
        &h.paths,
        &[
            ("demo".into(), source.clone()),
            ("inner".into(), backing.clone()),
        ],
        None,
        &guard(&h),
    );
    let caller = rows.iter().find(|r| r.app == "demo").unwrap();
    assert_eq!(caller.state, State::NeedsPerson, "{caller:#?}");
    assert!(!caller.no_click_available, "{caller:#?}");
    assert_eq!(caller.policy, None);
    let (candidate, _) = prepare(&h, &source, None);
    let (_, before) = lock_of(&source);
    let refused = by_policy(&h, &source, &candidate, &id, &verified).unwrap_err();
    assert_eq!(
        refused.code, "E_MIGRATE_NOT_ELIGIBLE",
        "{:?}",
        refused.error
    );
    assert!(
        refused.details.to_string().contains("backing-app-moved"),
        "{:?}",
        refused.details
    );
    assert_eq!(lock_of(&source).1, before);
}

/// Review round 2 (§15.1 R3): a policy promotion holds the policies lock
/// from its verdict until the lock has moved — a revocation started in
/// between waits for it.
#[test]
fn a_revocation_cannot_land_between_a_policy_verdict_and_the_lock_moving() {
    let (h, source, _) = approved_demo(READS);
    update_official(&h, "1.0.1", "mode: read");
    let id = record_policy(&h, &["demo"], &["tool"]);
    let (candidate, _) = prepare(&h, &source, None);
    let (at_verdict, verdict_reached) = std::sync::mpsc::channel::<()>();
    let (go, wait_for_go) = std::sync::mpsc::channel::<()>();
    let paths = h.paths.clone();
    let worker_source = source.clone();
    let worker_id = id.clone();
    let promoter = std::thread::spawn(move || {
        on_policy_verdict(Box::new(move || {
            at_verdict.send(()).unwrap();
            wait_for_go.recv().unwrap();
        }));
        let guard = crate::agent_store::open(&paths).unwrap();
        promote(
            &paths,
            &guard,
            &worker_source,
            &candidate,
            Approver::Policy {
                id: &worker_id,
                front_door: "floless@test",
                official: &verified,
            },
            SuccessorKind::CarriedForward,
        )
        .map(|p| p.seq)
        .map_err(|r| r.code)
    });
    verdict_reached
        .recv_timeout(std::time::Duration::from_secs(30))
        .unwrap();
    let paths = h.paths.clone();
    let (revoked, revoke_done) = std::sync::mpsc::channel();
    let revoker = std::thread::spawn(move || {
        policy::revoke(&paths, &id, "pawel", "floless@test", None).unwrap();
        revoked.send(()).unwrap();
    });
    assert!(
        revoke_done
            .recv_timeout(std::time::Duration::from_millis(400))
            .is_err(),
        "a revocation landed between the policy verdict and the lock moving"
    );
    go.send(()).unwrap();
    assert_eq!(promoter.join().unwrap(), Ok(1));
    revoke_done
        .recv_timeout(std::time::Duration::from_secs(30))
        .unwrap();
    revoker.join().unwrap();
    reader_accepts(&source);
}

/// Review round 2: two apps sharing a source directory share
/// `.aware-approvals/.txn`; one promotion's cleanup never removes the
/// directory another is about to use.
#[test]
fn a_promotion_leaves_the_shared_transaction_directory_in_place() {
    let (h, source, _) = approved_demo(READS);
    update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    let (candidate, header) = prepare(&h, &source, None);
    by_person(
        &h,
        &source,
        &candidate,
        &person_record("pawel", &candidate, &header),
        SuccessorKind::CarriedForward,
    )
    .unwrap();
    assert!(txn_root(source.parent().unwrap()).is_dir());
}

/// Review round 4: the archived evidence row words the approval labels, so a
/// row edited after `prepare` (here: a write workflow claimed read-only) is
/// refused, never archived.
#[test]
fn an_evidence_row_that_no_longer_matches_the_candidate_is_refused() {
    let (h, source, _) = approved_demo(OVERRIDABLE);
    update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    let (candidate, header) = prepare(&h, &source, None);
    let dir = source.parent().unwrap();
    let path = files::evidence_path(dir, "demo");
    let mut evidence: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    assert_eq!(evidence["row"]["effect"], "not-declared-read-only");
    evidence["row"]["effect"] = "declared-read-only".into();
    std::fs::write(&path, serde_json::to_vec_pretty(&evidence).unwrap()).unwrap();
    let (_, before) = lock_of(&source);
    let refused = by_person(
        &h,
        &source,
        &candidate,
        &person_record("pawel", &candidate, &header),
        SuccessorKind::CarriedForward,
    )
    .unwrap_err();
    assert_eq!(
        refused.code, "E_MIGRATE_CANDIDATE_TAMPERED",
        "{:?}",
        refused.error
    );
    assert_eq!(lock_of(&source).1, before);
}

/// Review round 4: every migrate verb that takes the app's lock recovers a
/// crashed promotion first (`lock_and_recover`, used by prepare, discard and
/// hold) — leaving neither the transaction nor its new archives behind.
#[test]
fn taking_the_lock_for_another_verb_recovers_a_crashed_promotion() {
    let (h, source, _) = approved_demo(READS);
    update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    let (candidate, header) = prepare(&h, &source, None);
    let dir = source.parent().unwrap();
    let archives_before = archive_files(dir);
    inject_fault(Fault::ArchivesPlaced);
    by_person(
        &h,
        &source,
        &candidate,
        &person_record("pawel", &candidate, &header),
        SuccessorKind::CarriedForward,
    )
    .unwrap_err();
    assert!(!owned_txns(dir, "demo").unwrap().is_empty());
    assert_ne!(archive_files(dir), archives_before);
    let g = guard(&h);
    let (_lock, recovered) = lock_and_recover(&h.paths, &g, dir, "demo").unwrap();
    assert_eq!(recovered.len(), 1);
    assert_eq!(recovered[0].outcome, "rolled-back");
    assert!(owned_txns(dir, "demo").unwrap().is_empty());
    assert_eq!(archive_files(dir), archives_before);
}
