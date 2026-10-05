//! Table-driven tests of every archived record type an approval chain cites
//! (#628 PR3a review round 2): for each record, every field it carries is
//! mutated ONE AT A TIME, the mutated record is archived content-addressed and
//! the chain pointed at it, and the lock must be refused naming that record
//! and that field. The untouched record, re-archived the same way, is the
//! control and must load.
//!
//! A case reports one of three outcomes, so a run against older code shows
//! which mutations used to slip through (`NOT REFUSED`) and which were only
//! caught for another reason (`WRONG REASON`).

use super::test_support::{By, Promoted, archive, promote, tamper, write_lock};
use super::*;
use crate::agent_resolution::PinTarget;
use crate::app_lock::candidate::tests::{approve, home, update_agent, write_agent, write_app};

const ONE_TOOL: &str = "requires: []\nnodes:\n  - { id: a, agent: tool, command: go }\n";

struct Fx {
    _h: crate::app_lock::candidate::tests::Home,
    source: std::path::PathBuf,
    dir: std::path::PathBuf,
    /// The latest promotion.
    p: Promoted,
}

fn sha_of(c: char) -> String {
    format!("sha256:{}", c.to_string().repeat(64))
}

/// `demo` approved on tool 1.0.0, carried forward to 1.0.1 (and, with
/// `two_links`, on to 1.0.2) by `by`.
fn fixture(by: fn() -> By, two_links: bool) -> Fx {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    let source = write_app(&h.paths, "demo", ONE_TOOL);
    approve(&h.paths, &source);
    let mut p = None;
    for version in if two_links {
        &["1.0.1", "1.0.2"][..]
    } else {
        &["1.0.1"][..]
    } {
        let digest = update_agent(&h.paths, "tool", version, "mode: read");
        p = Some(promote(
            &h.paths,
            &source,
            [("tool".to_string(), PinTarget::Digest(digest))].into(),
            by(),
            SuccessorKind::CarriedForward,
        ));
    }
    let dir = crate::fs::containing_dir(&source).to_path_buf();
    Fx {
        _h: h,
        source,
        dir,
        p: p.unwrap(),
    }
}

fn person() -> By {
    By::Person("pawel")
}

fn policy() -> By {
    By::Policy("pol-1")
}

fn chain_of(fx: &Fx) -> ApprovalChain {
    fx.p.lock.approval.clone().unwrap()
}

fn read(fx: &Fx, digest: &str, ext: &str) -> Vec<u8> {
    std::fs::read(fx.dir.join(archive_rel(digest, ext).unwrap())).unwrap()
}

fn lock_from(bytes: &[u8]) -> LockFile {
    serde_yaml::from_slice(bytes).unwrap()
}

fn lock_bytes(fx: &Fx, lock: &LockFile) -> Vec<u8> {
    write_lock(&fx.dir.join("scratch.lock"), lock)
}

/// Load the app and classify the outcome against the expected record + field.
fn outcome(fx: &Fx, record: &str, field: &str) -> Result<(), String> {
    let needle = format!("{record} contradicts the approval record at `{field}`");
    match load_approved_app_snapshot(&fx.source) {
        Ok(_) => Err("NOT REFUSED".into()),
        Err(error) => {
            let error = error.to_string();
            if error.contains("[E_APP_LOCK_INVALID]") && error.contains(&needle) {
                Ok(())
            } else {
                Err(format!("WRONG REASON: {error}"))
            }
        }
    }
}

fn report(table: &str, results: Vec<(&str, Result<(), String>)>) {
    let failed: Vec<String> = results
        .iter()
        .filter_map(|(field, r)| r.as_ref().err().map(|e| format!("  {field}: {e}")))
        .collect();
    assert!(
        failed.is_empty(),
        "{table}: {} of {} cases failed:\n{}",
        failed.len(),
        results.len(),
        failed.join("\n")
    );
}

type LockEdit = fn(&mut LockFile, &ApprovalChain);

// ── the original approval archive ───────────────────────────────────────────

/// Re-archive the original (edited) and point `approval.original`, link 1 and
/// link 1's evidence at it. Uses a policy link, so no person record also binds
/// the original's digest: only the original's own validator can tell.
fn swap_original(fx: &Fx, edit: LockEdit) {
    let chain = chain_of(fx);
    let mut original = lock_from(&fx.p.replaced);
    edit(&mut original, &chain);
    let digest = archive(&fx.dir, &lock_bytes(fx, &original), "lock");
    // The evidence names the lock link 1 replaced; re-point it too.
    let link = &chain.successors[0];
    let mut evidence: serde_json::Value =
        serde_json::from_slice(&read(fx, &link.evidence_digest, "json")).unwrap();
    evidence["header"]["base-lock-digest"] = serde_json::Value::String(digest.clone());
    let evidence_digest = archive(&fx.dir, evidence.to_string().as_bytes(), "json");
    tamper(&fx.p.lock_path, |l| {
        let c = l.approval.as_mut().unwrap();
        c.original.lock_digest = digest.clone();
        c.original.archive = archive_rel(&digest, "lock").unwrap();
        c.successors[0].from_lock_digest = digest;
        c.successors[0].evidence = archive_rel(&evidence_digest, "json").unwrap();
        c.successors[0].evidence_digest = evidence_digest;
    });
}

#[test]
fn every_field_of_the_original_archive_is_checked() {
    let cases: Vec<(&str, LockEdit)> = vec![
        ("approval", |l, c| l.approval = Some(c.clone())),
        ("source-hash", |l, _| {
            // A promoted-form hash on a lock with no approval block (finding b).
            l.source_hash = l.source_hash.replace("sha256:", SUCCESSOR_SOURCE_PREFIX);
        }),
        ("source-hash", |l, _| l.source_hash = sha_of('4')),
        ("app", |l, _| l.app = "other".into()),
        ("version", |l, _| l.version = "9.9.9".into()),
        ("compiled-at", |l, _| {
            l.compiled_at = "1999-01-01T00:00:00Z".into()
        }),
        ("compiler-version", |l, _| {
            l.compiler_version = "0.0.1".into()
        }),
        ("front-door", |l, _| l.front_door = Some("floless@x".into())),
        ("agent-pins", |l, _| {
            l.agent_pins.insert("tool".into(), "0.9.9".into());
        }),
        ("agent-digests", |l, _| {
            l.agent_digests.insert("tool".into(), sha_of('7'));
        }),
        ("agent-bundle-pins", |l, _| {
            l.agent_bundle_pins.insert("tool".into(), sha_of('8'));
        }),
    ];
    // Control: the untouched original, re-archived through the same path.
    let fx = fixture(policy, false);
    swap_original(&fx, |_, _| {});
    load_approved_app_snapshot(&fx.source).expect("control: untouched original loads");

    let mut results = Vec::new();
    for (field, edit) in cases {
        let fx = fixture(policy, false);
        swap_original(&fx, edit);
        results.push((field, outcome(&fx, "original approval archive", field)));
    }
    report("original approval archive", results);
}

// ── the lock a later link replaced ──────────────────────────────────────────

/// Re-archive the lock successor 2 replaced (edited) and point link 2 (and its
/// evidence, which names that lock) at it.
fn swap_replaced(fx: &Fx, edit: LockEdit) {
    let chain = chain_of(fx);
    let mut replaced = lock_from(&fx.p.replaced);
    edit(&mut replaced, &chain);
    let digest = archive(&fx.dir, &lock_bytes(fx, &replaced), "lock");
    let link = &chain.successors[1];
    let mut evidence: serde_json::Value =
        serde_json::from_slice(&read(fx, &link.evidence_digest, "json")).unwrap();
    evidence["header"]["base-lock-digest"] = serde_json::Value::String(digest.clone());
    let evidence_digest = archive(&fx.dir, evidence.to_string().as_bytes(), "json");
    tamper(&fx.p.lock_path, |l| {
        let link = &mut l.approval.as_mut().unwrap().successors[1];
        link.from_lock_digest = digest;
        link.evidence = archive_rel(&evidence_digest, "json").unwrap();
        link.evidence_digest = evidence_digest;
    });
}

#[test]
fn every_field_of_a_replaced_lock_is_checked() {
    let cases: Vec<(&str, LockEdit)> = vec![
        ("approval", |l, _| l.approval = None),
        ("approval.original", |l, _| {
            l.approval.as_mut().unwrap().original.compiled_at = "1999-01-01T00:00:00Z".into();
        }),
        ("approval.successors", |l, _| {
            l.approval.as_mut().unwrap().successors[0].promoted_at = "1999".into();
        }),
        ("approval.format", |l, _| {
            l.approval.as_mut().unwrap().format = 2;
        }),
        ("source-hash", |l, _| {
            l.source_hash = approved_source_hash(l)
        }),
        ("source-hash", |l, _| {
            l.source_hash = format!("{SUCCESSOR_SOURCE_PREFIX}{}", "4".repeat(64))
        }),
        ("app", |l, _| l.app = "other".into()),
        ("version", |l, _| l.version = "9.9.9".into()),
        ("agent-pins", |l, _| {
            l.agent_pins.insert("tool".into(), "0.9.9".into());
        }),
        ("agent-digests", |l, _| {
            l.agent_digests.insert("tool".into(), sha_of('7'));
        }),
        ("agent-bundle-pins", |l, _| {
            l.agent_bundle_pins.insert("tool".into(), sha_of('8'));
        }),
        ("plan", |l, _| l.nodes[0].mode = "write".into()),
    ];
    let fx = fixture(policy, true);
    swap_replaced(&fx, |_, _| {});
    load_approved_app_snapshot(&fx.source).expect("control: untouched replaced lock loads");

    let mut results = Vec::new();
    for (field, edit) in cases {
        let fx = fixture(policy, true);
        swap_replaced(&fx, edit);
        results.push((field, outcome(&fx, "replaced lock", field)));
    }
    report("replaced lock", results);
}

// ── a link's resulting plan ─────────────────────────────────────────────────

/// Re-archive successor 1's resulting plan (edited) and point the link at it.
/// A policy link, and evidence re-written to name the new digest, so only the
/// resulting plan's own validator can tell.
fn swap_resulting(fx: &Fx, edit: LockEdit) {
    let chain = chain_of(fx);
    let link = &chain.successors[0];
    let mut result = lock_from(&read(fx, &link.resulting_lock_digest, "lock"));
    edit(&mut result, &chain);
    let digest = archive(&fx.dir, &lock_bytes(fx, &result), "lock");
    let mut evidence: serde_json::Value =
        serde_json::from_slice(&read(fx, &link.evidence_digest, "json")).unwrap();
    evidence["header"]["candidate-digest"] = serde_json::Value::String(digest.clone());
    let evidence_digest = archive(&fx.dir, evidence.to_string().as_bytes(), "json");
    tamper(&fx.p.lock_path, |l| {
        let link = &mut l.approval.as_mut().unwrap().successors[0];
        link.resulting_lock_digest = digest;
        link.evidence = archive_rel(&evidence_digest, "json").unwrap();
        link.evidence_digest = evidence_digest;
    });
}

#[test]
fn every_field_of_a_resulting_plan_is_checked() {
    let cases: Vec<(&str, LockEdit)> = vec![
        ("approval", |l, c| l.approval = Some(c.clone())),
        ("source-hash", |l, _| {
            l.source_hash = l.source_hash.replace("sha256:", SUCCESSOR_SOURCE_PREFIX);
        }),
        ("source-hash", |l, _| l.source_hash = sha_of('4')),
        ("app", |l, _| l.app = "other".into()),
        ("version", |l, _| l.version = "9.9.9".into()),
        ("agent-pins", |l, _| {
            l.agent_pins.insert("tool".into(), "0.9.9".into());
        }),
        ("agent-digests", |l, _| {
            l.agent_digests.insert("tool".into(), sha_of('7'));
        }),
        ("agent-bundle-pins", |l, _| {
            l.agent_bundle_pins.insert("tool".into(), sha_of('8'));
        }),
        ("plan", |l, _| l.nodes[0].mode = "write".into()),
    ];
    let fx = fixture(policy, false);
    swap_resulting(&fx, |_, _| {});
    load_approved_app_snapshot(&fx.source).expect("control: untouched resulting plan loads");

    let mut results = Vec::new();
    for (field, edit) in cases {
        let fx = fixture(policy, false);
        swap_resulting(&fx, edit);
        results.push((field, outcome(&fx, "resulting plan", field)));
    }
    report("resulting plan", results);
}

// ── evidence ────────────────────────────────────────────────────────────────

type JsonEdit = fn(&mut serde_json::Value);

fn swap_json(fx: &Fx, bytes: &[u8], evidence: bool) {
    let digest = archive(&fx.dir, bytes, "json");
    tamper(&fx.p.lock_path, |l| {
        let link = &mut l.approval.as_mut().unwrap().successors[0];
        if evidence {
            link.evidence = archive_rel(&digest, "json").unwrap();
            link.evidence_digest = digest;
        } else if let CarriedForwardBy::Person {
            approval_record_digest,
            ..
        } = &mut link.carried_forward_by
        {
            *approval_record_digest = digest;
        }
    });
}

#[test]
fn every_field_of_the_evidence_is_checked() {
    let cases: Vec<(&str, JsonEdit)> = vec![
        ("(parse)", |e| {
            *e = serde_json::Value::String("not evidence".into())
        }),
        ("(parse)", |e| {
            e.as_object_mut().unwrap().remove("header");
        }),
        ("format", |e| {
            e["format"] = "aware.migration-evidence/v2".into()
        }),
        ("app", |e| e["app"] = "other".into()),
        ("prepared-at", |e| e["prepared-at"] = "".into()),
        ("cli-version", |e| e["cli-version"] = "".into()),
        ("row", |e| e["row"] = "not a row".into()),
        ("header.format", |e| {
            e["header"]["format"] = "aware.migration-candidate/v2".into()
        }),
        ("header.app", |e| e["header"]["app"] = "other".into()),
        ("header.base-lock-digest", |e| {
            e["header"]["base-lock-digest"] = sha_of('5').into()
        }),
        ("header.base-source-hash", |e| {
            e["header"]["base-source-hash"] = sha_of('6').into()
        }),
        ("header.candidate-digest", |e| {
            e["header"]["candidate-digest"] = sha_of('7').into()
        }),
        ("header.plan-digest", |e| {
            e["header"]["plan-digest"] = sha_of('8').into()
        }),
        ("header.targets", |e| {
            e["header"]["targets"]["tool"]["to"]["version"] = "9.9.9".into()
        }),
        ("header.targets", |e| {
            e["header"]["targets"] = serde_json::json!({});
        }),
    ];
    let original = |fx: &Fx| -> serde_json::Value {
        let link = &chain_of(fx).successors[0];
        serde_json::from_slice(&read(fx, &link.evidence_digest, "json")).unwrap()
    };
    let fx = fixture(person, false);
    let control = original(&fx);
    swap_json(
        &fx,
        serde_json::to_string_pretty(&control).unwrap().as_bytes(),
        true,
    );
    load_approved_app_snapshot(&fx.source).expect("control: untouched evidence loads");

    let mut results = Vec::new();
    for (field, edit) in cases {
        let fx = fixture(person, false);
        let mut evidence = original(&fx);
        edit(&mut evidence);
        swap_json(&fx, evidence.to_string().as_bytes(), true);
        results.push((field, outcome(&fx, "evidence", field)));
    }
    let fx = fixture(person, false);
    swap_json(&fx, b"\xff not even text", true);
    results.push(("(parse)", outcome(&fx, "evidence", "(parse)")));
    report("evidence", results);
}

// ── the person approval record ──────────────────────────────────────────────

#[test]
fn every_field_of_the_person_approval_record_is_checked() {
    let cases: Vec<(&str, JsonEdit)> = vec![
        ("(parse)", |r| {
            r.as_object_mut().unwrap().remove("format");
        }),
        ("(parse)", |r| {
            r.as_object_mut().unwrap().remove("statement-sha256");
        }),
        ("(parse)", |r| *r = serde_json::json!([1, 2, 3])),
        ("format", |r| r["format"] = 2.into()),
        ("kind", |r| r["kind"] = "policy".into()),
        ("actor", |r| r["actor"] = "someone-else".into()),
        ("approval-ref", |r| r["approval-ref"] = "batch-9".into()),
        ("front-door", |r| r["front-door"] = "other@1".into()),
        ("candidate-digest", |r| {
            r["candidate-digest"] = sha_of('5').into()
        }),
        ("base-lock-digest", |r| {
            r["base-lock-digest"] = sha_of('6').into()
        }),
        ("plan-digest", |r| r["plan-digest"] = sha_of('7').into()),
        ("statement-sha256", |r| {
            r["statement-sha256"] = "sha256:xyz".into()
        }),
        ("at", |r| r["at"] = "".into()),
    ];
    let original = |fx: &Fx| -> serde_json::Value {
        let link = &chain_of(fx).successors[0];
        let CarriedForwardBy::Person {
            approval_record_digest,
            ..
        } = &link.carried_forward_by
        else {
            unreachable!("person fixture")
        };
        serde_json::from_slice(&read(fx, approval_record_digest, "json")).unwrap()
    };
    let fx = fixture(person, false);
    let control = original(&fx);
    swap_json(
        &fx,
        serde_json::to_string_pretty(&control).unwrap().as_bytes(),
        false,
    );
    load_approved_app_snapshot(&fx.source).expect("control: untouched record loads");

    let mut results = Vec::new();
    for (field, edit) in cases {
        let fx = fixture(person, false);
        let mut record = original(&fx);
        edit(&mut record);
        swap_json(&fx, record.to_string().as_bytes(), false);
        results.push((field, outcome(&fx, "person approval record", field)));
    }
    let fx = fixture(person, false);
    swap_json(&fx, b"not json", false);
    results.push(("(parse)", outcome(&fx, "person approval record", "(parse)")));
    report("person approval record", results);
}

// ── the link fields themselves (no archive behind them) ─────────────────────

type ChainEdit = fn(&mut ApprovalChain);

#[test]
fn every_link_field_is_checked_structurally() {
    let person_cases: Vec<(&str, ChainEdit)> = vec![
        ("original.lock-digest", |c| {
            c.original.lock_digest = "sha256:x".into()
        }),
        ("original.archive", |c| {
            c.original.archive = "elsewhere.lock".into()
        }),
        ("from-lock-digest", |c| {
            c.successors[0].from_lock_digest = "x".into()
        }),
        ("to-plan-digest", |c| {
            c.successors[0].to_plan_digest = "x".into()
        }),
        ("resulting-lock-digest", |c| {
            c.successors[0].resulting_lock_digest = "x".into()
        }),
        ("evidence-digest", |c| {
            c.successors[0].evidence_digest = "x".into()
        }),
        ("evidence", |c| c.successors[0].evidence = "x.json".into()),
        ("front-door", |c| c.successors[0].front_door = " ".into()),
        ("cli-version", |c| c.successors[0].cli_version = "".into()),
        ("promoted-at", |c| c.successors[0].promoted_at = "".into()),
        ("actor", |c| set_person(c, |a, _, _, _, _| *a = "".into())),
        ("approval-ref", |c| {
            set_person(c, |_, r, _, _, _| *r = "".into())
        }),
        ("carried-forward-by.front-door", |c| {
            set_person(c, |_, _, f, _, _| *f = "".into())
        }),
        ("attested", |c| set_person(c, |_, _, _, t, _| *t = true)),
        ("approval-record-digest", |c| {
            set_person(c, |_, _, _, _, d| *d = "x".into())
        }),
    ];
    let policy_cases: Vec<(&str, ChainEdit)> = vec![
        ("policy-id", |c| set_policy(c, |i, _, _| *i = "".into())),
        ("policy-digest", |c| {
            set_policy(c, |_, d, _| *d = "x".into())
        }),
        ("policy-approved-by", |c| {
            set_policy(c, |_, _, b| *b = "".into())
        }),
    ];
    let mut failed = Vec::new();
    for (by, cases) in [(person as fn() -> By, person_cases), (policy, policy_cases)] {
        for (field, edit) in cases {
            let fx = fixture(by, false);
            let mut lock = fx.p.lock_from_disk();
            edit(lock.approval.as_mut().unwrap());
            match check_chain(&lock) {
                Err(reason) if reason.contains(field) => {}
                other => failed.push(format!("  {field}: {other:?}")),
            }
        }
    }
    assert!(failed.is_empty(), "link fields:\n{}", failed.join("\n"));
}

fn set_person(
    c: &mut ApprovalChain,
    f: fn(&mut String, &mut String, &mut String, &mut bool, &mut String),
) {
    if let CarriedForwardBy::Person {
        actor,
        approval_ref,
        front_door,
        attested,
        approval_record_digest,
    } = &mut c.successors[0].carried_forward_by
    {
        f(
            actor,
            approval_ref,
            front_door,
            attested,
            approval_record_digest,
        );
    }
}

fn set_policy(c: &mut ApprovalChain, f: fn(&mut String, &mut String, &mut String)) {
    if let CarriedForwardBy::Policy {
        policy_id,
        policy_digest,
        policy_approved_by,
    } = &mut c.successors[0].carried_forward_by
    {
        f(policy_id, policy_digest, policy_approved_by);
    }
}

impl Promoted {
    fn lock_from_disk(&self) -> LockFile {
        lock_from(&std::fs::read(&self.lock_path).unwrap())
    }
}
