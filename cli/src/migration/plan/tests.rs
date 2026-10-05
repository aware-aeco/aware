//! `migrate plan` evaluation (#628 plan §7, §11–§14).

use super::*;
use crate::app_lock::candidate::tests::{approve, home, update_agent, write_agent, write_app};

const READS: &str = "requires: []\nnodes:\n  - { id: a, agent: tool, command: go }\n";

fn row_of(h: &crate::app_lock::candidate::tests::Home, source: &Path) -> PlanRow {
    evaluate(&h.paths, source, None).unwrap().row
}

fn codes(row: &PlanRow) -> Vec<&str> {
    row.reasons.iter().map(|r| r.code.as_str()).collect()
}

#[test]
fn nothing_installed_newer_is_up_to_date() {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    let source = write_app(&h.paths, "demo", READS);
    approve(&h.paths, &source);
    let row = row_of(&h, &source);
    assert_eq!(row.state, State::UpToDate, "{row:?}");
    assert!(row.targets.is_empty() && row.reasons.is_empty());
    assert!(!row.no_click_available);
    assert_eq!(row.advisory, None);
    assert_eq!(row.comparison, None);
}

/// §11 Q1 + §12 R1-9: the best case v1 can see — declared read-only, run
/// instructions byte-identical — still needs a person, and says why.
#[test]
fn a_declared_read_with_identical_instructions_still_needs_a_person_in_v1() {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    let source = write_app(&h.paths, "demo", READS);
    approve(&h.paths, &source);
    // 1.0.1 differs only in `version` and `description`, which a run never reads.
    update_agent(&h.paths, "tool", "1.0.1", "mode: read");

    let row = row_of(&h, &source);
    assert_eq!(row.state, State::NeedsPerson, "{row:#?}");
    assert_eq!(row.targets.len(), 1);
    assert_eq!(row.targets[0].bump, "patch");
    assert_eq!(row.targets[0].from.version, "1.0.0");
    assert_eq!(row.targets[0].to.version, "1.0.1");
    assert_eq!(row.effect, Some("declared-read-only"));
    let contract = row.contract.as_ref().unwrap();
    assert!(contract.unchanged, "{contract:#?}");
    let comparison = row.comparison.as_ref().unwrap();
    assert_eq!(
        comparison.status,
        compare::ComparisonStatus::IdenticalInstructions
    );
    assert_eq!(comparison.method, Some(compare::STATIC_INSPECTION));
    assert!(!row.no_click_available);
    assert_eq!(codes(&row), ["no-fixed-state-method"]);
    assert_eq!(row.reasons[0].text, NO_FIXED_STATE_METHOD);
}

#[test]
fn a_mode_overridable_read_is_not_declared_read_only() {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    let source = write_app(
        &h.paths,
        "demo",
        "requires: []\nnodes:\n  - { id: x, agent: tool, command: exec, mode: read }\n",
    );
    approve(&h.paths, &source);
    update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    let row = row_of(&h, &source);
    assert_eq!(row.state, State::NeedsPerson);
    assert_eq!(row.effect, Some("not-declared-read-only"));
    assert_eq!(
        codes(&row),
        ["mode-overridable", "no-fixed-state-method"],
        "{row:#?}"
    );
    assert!(row.reasons[0].text.contains("node x (tool exec)"));
    // The contract of `exec` itself is unchanged: inspection says identical.
    assert_eq!(
        row.comparison.as_ref().unwrap().status,
        compare::ComparisonStatus::IdenticalInstructions
    );
}

#[test]
fn a_changed_contract_is_not_comparable_and_says_what_changed() {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    let source = write_app(&h.paths, "demo", READS);
    approve(&h.paths, &source);
    // The new version declares `go` read too, but changes a run-relevant key.
    update_agent(&h.paths, "tool", "1.0.1", "mode: read\n    timeout-ms: 5");
    let row = row_of(&h, &source);
    assert_eq!(row.state, State::NeedsPerson);
    let contract = row.contract.as_ref().unwrap();
    assert!(!contract.unchanged);
    assert_eq!(contract.diffs[0].commands[0].changes, ["timeout-ms"]);
    let comparison = row.comparison.as_ref().unwrap();
    assert_eq!(comparison.status, compare::ComparisonStatus::NotComparable);
    assert_eq!(comparison.per_agent[0].reason.code, "no-fixed-state");
    assert_eq!(
        codes(&row),
        [
            "contract-changed",
            "not-comparable",
            "no-fixed-state-method"
        ],
        "{row:#?}"
    );
    assert!(
        row.reasons[0].text.contains("commands go"),
        "{:?}",
        row.reasons[0]
    );
}

#[test]
fn a_held_app_reports_held_whatever_else_is_true() {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    let source = write_app(&h.paths, "demo", READS);
    approve(&h.paths, &source);
    let dir = source.parent().unwrap();
    let hold = files::HoldRecord {
        format: 1,
        held_by: "pawel".into(),
        held_at: "t".into(),
        reason: Some("certified".into()),
    };
    files::write_hold(dir, &hold).unwrap();
    assert_eq!(
        row_of(&h, &source).state,
        State::Held,
        "held while up to date"
    );
    update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    let row = row_of(&h, &source);
    assert_eq!(row.state, State::Held);
    assert_eq!(row.hold, Some(hold));
    assert_eq!(row.targets.len(), 1, "the move is still reported");
    files::remove_hold(dir).unwrap();
    assert_eq!(row_of(&h, &source).state, State::NeedsPerson);
}

#[test]
fn a_candidate_a_person_must_fix_is_blocked() {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    let source = write_app(&h.paths, "demo", READS);
    approve(&h.paths, &source);
    update_agent(&h.paths, "tool", "1.0.1", "mode: write");
    let row = row_of(&h, &source);
    assert_eq!(row.state, State::Blocked, "{row:#?}");
    assert_eq!(codes(&row)[0], "needs-source-edit");
    assert!(!codes(&row).contains(&"no-fixed-state-method"));
}

#[test]
fn an_unapproved_or_changed_workflow_is_blocked_with_the_person_path() {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    let source = write_app(&h.paths, "demo", READS);
    let row = row_of(&h, &source);
    assert_eq!(
        (row.state, codes(&row)),
        (State::Blocked, vec!["no-approval"])
    );
    approve(&h.paths, &source);
    write_app(
        &h.paths,
        "demo",
        "requires: []\nnodes:\n  - { id: b, agent: tool, command: go }\n",
    );
    let row = row_of(&h, &source);
    assert_eq!(
        (row.state, codes(&row)),
        (State::Blocked, vec!["source-changed"])
    );
    assert!(row.reasons[0].text.contains("compile it again"));
}

#[test]
fn an_explicit_target_moves_only_what_it_names() {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    write_agent(&h.paths, "other", "1.0.0", "mode: read");
    let source = write_app(
        &h.paths,
        "demo",
        "requires: []\nnodes:\n  - { id: a, agent: tool, command: go }\n  - { id: b, agent: other, command: go }\n",
    );
    approve(&h.paths, &source);
    update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    update_agent(&h.paths, "other", "1.0.1", "mode: read");
    assert_eq!(
        row_of(&h, &source).targets.len(),
        2,
        "default: every installed move"
    );
    let only: BTreeMap<String, PinTarget> =
        [("tool".to_string(), PinTarget::Version("1.0.1".into()))].into();
    let row = evaluate(&h.paths, &source, Some(&only)).unwrap().row;
    let moved: Vec<&str> = row.targets.iter().map(|t| t.agent.as_str()).collect();
    assert_eq!(moved, ["tool"]);
    // A target for an agent this app does not use is simply not this app's.
    let unused: BTreeMap<String, PinTarget> =
        [("ghost".to_string(), PinTarget::Version("1.0.0".into()))].into();
    assert_eq!(
        evaluate(&h.paths, &source, Some(&unused))
            .unwrap()
            .row
            .state,
        State::UpToDate
    );
}

#[test]
fn a_backing_app_never_moves_and_its_callers_are_named() {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    let inner = write_app(
        &h.paths,
        "inner",
        "exposes-as-agent: true\nexposed-commands:\n  ask:\n    lifecycle: single\n    outputs:\n      type: single\n\
         requires: []\nnodes:\n  - { id: a, agent: tool, command: go }\n",
    );
    approve(&h.paths, &inner);
    let outer = write_app(
        &h.paths,
        "outer",
        "requires: []\nnodes:\n  - { id: c, agent: inner, command: ask }\n",
    );
    update_agent(&h.paths, "tool", "1.0.1", "mode: read");

    let row = row_of(&h, &inner);
    assert_eq!(row.state, State::NeedsPerson, "{row:#?}");
    assert!(row.backing_app_moved);
    assert_eq!(row.callers, ["outer"]);
    assert!(codes(&row).contains(&"backing-app-moved"));
    assert!(!row.no_click_available);

    let rows = plan_rows(
        &h.paths,
        &[
            ("inner".into(), inner.clone()),
            ("outer".into(), outer.clone()),
        ],
        None,
    );
    let caller = rows.iter().find(|r| r.app == "outer").unwrap();
    assert!(
        caller
            .reasons
            .iter()
            .any(|r| r.code == "backing-app-moved" && r.text.contains("inner")),
        "{caller:#?}"
    );
}

#[test]
fn only_an_accepted_comparison_could_ever_skip_the_person() {
    // `advisory` is not an input to the decision at all.
    assert_eq!(decide(false, false, false, false), State::UpToDate);
    assert_eq!(decide(false, true, false, false), State::NeedsPerson);
    assert_eq!(decide(false, true, true, false), State::Blocked);
    assert_eq!(decide(true, true, false, true), State::Held);
    assert_eq!(decide(false, true, false, true), State::AutoUnderPolicy);
}

#[test]
fn a_stored_candidate_is_fresh_only_against_the_current_approval() {
    let h = home();
    write_agent(&h.paths, "tool", "1.0.0", "mode: read");
    let source = write_app(&h.paths, "demo", READS);
    approve(&h.paths, &source);
    update_agent(&h.paths, "tool", "1.0.1", "mode: read");
    let eval_now = evaluate(&h.paths, &source, None).unwrap();
    assert!(!eval_now.row.candidate.present);
    let candidate = eval_now.candidate.unwrap();
    let evidence = serde_json::to_vec(&files::Evidence {
        format: files::EVIDENCE_FORMAT.into(),
        app: "demo".into(),
        prepared_at: "t".into(),
        cli_version: "v".into(),
        header: candidate.header.clone(),
        row: serde_json::Value::Null,
    })
    .unwrap();
    let dir = source.parent().unwrap();
    files::write_candidate(dir, "demo", &candidate.bytes, &evidence).unwrap();
    let row = row_of(&h, &source);
    assert!(
        row.candidate.present && row.candidate.fresh,
        "{:?}",
        row.candidate
    );

    // A person recompiles: the old candidate starts from a lock that is gone.
    approve(&h.paths, &source);
    let row = row_of(&h, &source);
    assert!(row.candidate.present && !row.candidate.fresh);
}
