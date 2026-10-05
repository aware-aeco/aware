//! The policy store (#628 plan §6, §15): immutable, self-naming, revocable.

use super::*;
use crate::app_lock::candidate::tests::home;

fn record_json(front_door: &str) -> serde_json::Value {
    serde_json::json!({
        "format": 1, "kind": "policy", "actor": "pawel", "front-door": front_door,
        "approval-ref": "first-approval",
        "statement-sha256": crate::app_lock::lock_digest(b"policy text"),
        "at": "2026-10-05T00:00:00Z", "rule": "read-only-patch",
        "scope": { "apps": ["demo"], "agents": ["tekla"], "publishers": ["official-registry"], "bump": "patch" },
    })
}

fn record_bytes(value: &serde_json::Value) -> Vec<u8> {
    value.to_string().into_bytes()
}

#[test]
fn a_recorded_policy_names_itself_by_its_body_and_rerecording_is_a_no_op() {
    let h = home();
    let bytes = record_bytes(&record_json("floless@test"));
    let (loaded, created) = record(&h.paths, &bytes, "floless@test").unwrap();
    assert!(created);
    let id = loaded.policy.policy.clone();
    assert!(is_policy_id(&id), "{id}");
    assert_eq!(
        id,
        derive_id(
            loaded.policy.rule,
            &loaded.policy.scope,
            &loaded.policy.approved_by
        )
        .unwrap()
    );
    assert!(
        !loaded.policy.approved_by.attested,
        "a claim, never attested"
    );
    assert_eq!(
        loaded.policy.approved_by.approval_record_digest,
        crate::app_lock::lock_digest(&bytes)
    );
    assert_eq!(
        loaded.digest,
        crate::app_lock::lock_digest(&std::fs::read(&loaded.path).unwrap())
    );
    let (again, created) = record(&h.paths, &bytes, "floless@test").unwrap();
    assert!(!created);
    assert_eq!(again.policy, loaded.policy);
    assert!(label(&loaded).contains("claimed approval by pawel, recorded by floless@test"));
}

#[test]
fn a_record_that_does_not_fit_is_refused() {
    let h = home();
    let base = record_json("floless@test");
    let edits: [(&str, serde_json::Value); 9] = [
        ("format", 2.into()),
        ("kind", "person".into()),
        ("actor", " ".into()),
        ("statement-sha256", "nope".into()),
        ("rule", "delegated-maintenance".into()),
        ("extra", true.into()),
        (
            "scope",
            serde_json::json!({ "apps": [], "agents": ["tekla"], "publishers": ["official-registry"], "bump": "patch" }),
        ),
        (
            "scope",
            serde_json::json!({ "apps": ["*", "demo"], "agents": ["tekla"], "publishers": ["official-registry"], "bump": "patch" }),
        ),
        (
            "scope",
            serde_json::json!({ "apps": ["demo"], "agents": ["tekla"], "publishers": ["official-registry"], "bump": "minor" }),
        ),
    ];
    for (field, value) in edits {
        let mut json = base.clone();
        json[field] = value;
        let error = record(&h.paths, &record_bytes(&json), "floless@test")
            .unwrap_err()
            .to_string();
        assert!(
            error.contains("E_MIGRATE_APPROVAL_MISMATCH"),
            "{field}: {error}"
        );
    }
    // The front door on the command line must be the one that wrote it.
    let error = record(&h.paths, &record_bytes(&base), "other@1")
        .unwrap_err()
        .to_string();
    assert!(error.contains("E_MIGRATE_APPROVAL_MISMATCH"), "{error}");
    assert!(list(&h.paths).unwrap().is_empty(), "nothing was written");
}

#[test]
fn an_edited_policy_is_invalid_never_obeyed() {
    let h = home();
    let (loaded, _) = record(&h.paths, &record_bytes(&record_json("f@1")), "f@1").unwrap();
    let id = loaded.policy.policy.clone();
    let text = std::fs::read_to_string(&loaded.path).unwrap();
    std::fs::write(&loaded.path, text.replace("- demo", "- '*'")).unwrap();
    let error = load(&h.paths, &id).unwrap_err().to_string();
    assert!(error.contains("E_MIGRATE_POLICY_INVALID"), "{error}");
    assert!(error.contains("changed after it was recorded"), "{error}");
    // Listed as invalid, not hidden; never active.
    let entries = list(&h.paths).unwrap();
    assert_eq!(entries[0].state, "invalid");
    assert!(active(&h.paths).is_empty());
    // Re-recording the original approval does not overwrite the edited file.
    let error = record(&h.paths, &record_bytes(&record_json("f@1")), "f@1")
        .unwrap_err()
        .to_string();
    assert!(error.contains("E_MIGRATE_POLICY_CONFLICT"), "{error}");

    // A file claiming an attested approval is not one this CLI wrote.
    std::fs::write(&loaded.path, &text).unwrap();
    std::fs::write(
        &loaded.path,
        text.replace("attested: false", "attested: true"),
    )
    .unwrap();
    assert!(
        load(&h.paths, &id)
            .unwrap_err()
            .to_string()
            .contains("attested")
    );
}

#[test]
fn revoking_is_idempotent_and_an_unreadable_revocation_still_revokes() {
    let h = home();
    let (loaded, _) = record(&h.paths, &record_bytes(&record_json("f@1")), "f@1").unwrap();
    let id = loaded.policy.policy.clone();
    assert_eq!(active(&h.paths).len(), 1);
    let (first, now) = revoke(&h.paths, &id, "pawel", "f@1", Some("retired".into())).unwrap();
    assert!(now);
    let (again, now) = revoke(&h.paths, &id, "someone", "f@2", None).unwrap();
    assert!(!now);
    assert_eq!(again, first, "the first revocation stands");
    assert!(active(&h.paths).is_empty());
    assert_eq!(list(&h.paths).unwrap()[0].state, "revoked");

    std::fs::write(revocation_path(&h.paths, &id), "{ garbled").unwrap();
    let still = load(&h.paths, &id).unwrap().revoked.expect("still revoked");
    assert!(still.revoked_by.contains("unknown"), "{still:?}");
    assert!(active(&h.paths).is_empty());

    let error = revoke(&h.paths, "pol-0123456789abcdef", "p", "f", None)
        .unwrap_err()
        .to_string();
    assert!(error.contains("E_MIGRATE_POLICY_NOT_FOUND"), "{error}");
}

fn row_for(app: &str) -> PlanRow {
    use super::super::plan::TargetRow;
    use crate::migration::contract::PinRef;
    let mut row = PlanRow::unevaluated(
        app,
        std::path::Path::new("x.flo"),
        &AwareError::Internal("x".into()),
    );
    row.state = State::AutoUnderPolicy;
    row.reasons.clear();
    row.targets.push(TargetRow {
        agent: "tekla".into(),
        from: PinRef {
            version: "0.1.5".into(),
            digest: "sha256:a".into(),
        },
        to: PinRef {
            version: "0.1.6".into(),
            digest: "sha256:b".into(),
        },
        bump: "patch",
        publisher: "official-registry",
    });
    row.effect = Some("declared-read-only");
    row.contract = Some(super::super::plan::ContractRow {
        unchanged: true,
        diff_digest: "sha256:d".into(),
        probe_changed: false,
        diffs: Vec::new(),
    });
    row.comparison = Some(super::super::compare::Comparison {
        status: super::super::compare::ComparisonStatus::IdenticalInstructions,
        method: Some(super::super::compare::STATIC_INSPECTION),
        runs: 0,
        reason: Reason::new("identical-instructions", "x"),
        per_agent: Vec::new(),
    });
    row.no_click_available = true;
    row
}

/// Each conjunct of the rule flips the verdict on its own (plan §8: "policy
/// eligibility truth table (each conjunct alone flips; advisory never changes
/// verdict)").
#[test]
fn every_condition_of_a_policy_flips_the_verdict_alone() {
    let h = home();
    let (loaded, _) = record(&h.paths, &record_bytes(&record_json("f@1")), "f@1").unwrap();
    assert!(
        ineligibility(&loaded, &row_for("demo")).is_empty(),
        "the control is covered"
    );

    let mut advisory = row_for("demo");
    advisory.advisory = Some(serde_json::json!({ "severity": "critical" }));
    assert!(
        ineligibility(&loaded, &advisory).is_empty(),
        "urgency is never approval"
    );

    type Edit = fn(&mut PlanRow);
    let edits: [(&str, Edit); 10] = [
        ("policy-app-out-of-scope", |r| r.app = "other".into()),
        ("policy-agent-out-of-scope", |r| {
            r.targets[0].agent = "gw".into()
        }),
        ("policy-not-patch", |r| r.targets[0].bump = "minor"),
        ("policy-not-official", |r| r.targets[0].publisher = "local"),
        ("not-declared-read-only", |r| {
            r.effect = Some("not-declared-read-only")
        }),
        ("contract-changed", |r| {
            r.contract.as_mut().unwrap().unchanged = false
        }),
        ("comparison-not-accepted", |r| {
            r.comparison.as_mut().unwrap().status =
                super::super::compare::ComparisonStatus::NotComparable;
        }),
        ("backing-app-moved", |r| r.backing_app_moved = true),
        ("held", |r| r.state = State::Held),
        ("blocked", |r| r.state = State::Blocked),
    ];
    for (code, edit) in edits {
        let mut row = row_for("demo");
        edit(&mut row);
        let reasons = ineligibility(&loaded, &row);
        assert!(
            reasons.iter().any(|r| r.code == code),
            "{code}: {reasons:?}"
        );
    }
    revoke(&h.paths, &loaded.policy.policy, "p", "f", None).unwrap();
    let revoked = load(&h.paths, &loaded.policy.policy).unwrap();
    assert!(
        ineligibility(&revoked, &row_for("demo"))
            .iter()
            .any(|r| r.code == "policy-revoked")
    );
}
