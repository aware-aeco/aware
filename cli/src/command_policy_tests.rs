//! Unit tests for `command_policy`. Compiled only under `cfg(test)`.

use super::*;

const B1: &str = "a1b2c3d4-e5f6-4a7b-8c9d-0e1f2a3b4c5d";
const B2: &str = "22222222-2222-4222-8222-222222222222";

fn sel(slot: &str, binding: Option<&str>) -> Selector {
    Selector {
        slot: slot.into(),
        binding: binding.map(str::to_string),
    }
}

fn rule(id: &str, command: &str, accounts: Accounts, except: Vec<Selector>) -> Rule {
    Rule {
        id: id.into(),
        agent: "svc".into(),
        command: command.into(),
        accounts,
        except,
        note: None,
    }
}

fn readable(rules: Vec<Rule>) -> Loaded {
    Loaded::Readable {
        policy: Policy { rules },
        broken: Vec::new(),
    }
}

fn denied(id: &str) -> Decision {
    Decision::Deny(Denial {
        reason: DenyReason::Denied,
        rule: Some(id.into()),
    })
}

fn unreadable(rule: Option<&str>) -> Decision {
    Decision::Deny(Denial {
        reason: DenyReason::PolicyUnreadable,
        rule: rule.map(str::to_string),
    })
}

const U: Identity<'static> = Identity::Unverified;

// ── evaluation ───────────────────────────────────────────────────────────────

#[test]
fn absent_policy_allows_everything() {
    assert_eq!(
        evaluate(&Loaded::Absent, "svc", "read", "svc", U),
        Decision::Allow
    );
}

#[test]
fn all_accounts_deny_matches_any_slot_and_only_its_command() {
    let p = readable(vec![rule("r", "read", Accounts::All, vec![])]);
    assert_eq!(evaluate(&p, "svc", "read", "svc", U), denied("r"));
    assert_eq!(evaluate(&p, "svc", "read", "svc.work", U), denied("r"));
    assert_eq!(evaluate(&p, "svc", "write", "svc", U), Decision::Allow);
    assert_eq!(evaluate(&p, "other", "read", "svc", U), Decision::Allow);
}

#[test]
fn wildcard_command_covers_every_command_of_the_agent() {
    let p = readable(vec![rule("r", "*", Accounts::All, vec![])]);
    assert_eq!(evaluate(&p, "svc", "anything", "svc", U), denied("r"));
    assert_eq!(evaluate(&p, "other", "anything", "svc", U), Decision::Allow);
}

#[test]
fn slot_deny_matches_only_that_slot_whoever_holds_it() {
    let p = readable(vec![rule(
        "r",
        "read",
        Accounts::Only(vec![sel("svc.work", None)]),
        vec![],
    )]);
    assert_eq!(evaluate(&p, "svc", "read", "svc.work", U), denied("r"));
    // Re-signed to another account: still the slot, still denied.
    assert_eq!(
        evaluate(&p, "svc", "read", "svc.work", Identity::Verified(B2)),
        denied("r")
    );
    assert_eq!(evaluate(&p, "svc", "read", "svc", U), Decision::Allow);
}

#[test]
fn binding_deny_needs_identity_and_then_is_exact() {
    let p = readable(vec![rule(
        "r",
        "read",
        Accounts::Only(vec![sel("svc.work", Some(B1))]),
        vec![],
    )]);
    assert_eq!(
        evaluate(&p, "svc", "read", "svc.work", U),
        Decision::NeedsIdentity
    );
    assert_eq!(
        evaluate(&p, "svc", "read", "svc.work", Identity::Verified(B1)),
        denied("r")
    );
    assert_eq!(
        evaluate(&p, "svc", "read", "svc.work", Identity::Verified(B2)),
        Decision::Allow
    );
    // Another slot is never this binding's.
    assert_eq!(evaluate(&p, "svc", "read", "svc", U), Decision::Allow);
}

#[test]
fn exception_exempts_only_the_verified_binding_in_its_slot() {
    let p = readable(vec![rule(
        "off",
        "read",
        Accounts::All,
        vec![sel("svc.work", Some(B1))],
    )]);
    // Unverified: cannot be exempted yet.
    assert_eq!(
        evaluate(&p, "svc", "read", "svc.work", U),
        Decision::NeedsIdentity
    );
    assert_eq!(
        evaluate(&p, "svc", "read", "svc.work", Identity::Verified(B1)),
        Decision::Allow
    );
    // Upper-case binding from a caller compares equal.
    assert_eq!(
        evaluate(
            &p,
            "svc",
            "read",
            "svc.work",
            Identity::Verified(&B1.to_ascii_uppercase())
        ),
        Decision::Allow
    );
    // The slot re-signed to someone else: the opt-in does not carry over.
    assert_eq!(
        evaluate(&p, "svc", "read", "svc.work", Identity::Verified(B2)),
        denied("off")
    );
    // The opted-in account under another slot is a different binding: denied,
    // and no identity is needed to know it (no exception names that slot).
    assert_eq!(evaluate(&p, "svc", "read", "svc", U), denied("off"));
}

#[test]
fn needs_identity_is_a_refusal_where_identity_cannot_be_read() {
    assert_eq!(
        Decision::NeedsIdentity.unverified(),
        Err(Denial {
            reason: DenyReason::AccountUnverified,
            rule: None
        })
    );
    assert_eq!(Decision::Allow.unverified(), Ok(()));
}

#[test]
fn any_definite_deny_wins_over_an_undecided_rule() {
    let p = readable(vec![
        rule(
            "maybe",
            "read",
            Accounts::Only(vec![sel("svc", Some(B1))]),
            vec![],
        ),
        rule("sure", "read", Accounts::All, vec![]),
    ]);
    assert_eq!(evaluate(&p, "svc", "read", "svc", U), denied("sure"));
}

#[test]
fn unreadable_policy_refuses_every_dispatch() {
    let p = Loaded::Unreadable { why: "x".into() };
    assert_eq!(
        evaluate(&p, "anything", "read", "slot", U),
        unreadable(None)
    );
}

#[test]
fn a_broken_rule_costs_only_its_own_agent() {
    let p = Loaded::Readable {
        policy: Policy::default(),
        broken: vec![("svc".into(), Some("bad".into()))],
    };
    assert_eq!(
        evaluate(&p, "svc", "read", "svc", U),
        unreadable(Some("bad"))
    );
    assert_eq!(evaluate(&p, "other", "read", "other", U), Decision::Allow);
}

// ── reading ──────────────────────────────────────────────────────────────────

fn write(home: &Path, text: &str) {
    std::fs::write(home.join(POLICY_FILE), text).unwrap();
}

#[test]
fn load_reads_absent_valid_and_broken_files() {
    let home = tempfile::tempdir().unwrap();
    assert_eq!(load(home.path()), Loaded::Absent);

    write(
        home.path(),
        &format!(
            r#"{{"schema":"{SCHEMA}","rules":[{{"id":"r","agent":"svc","command":"read","accounts":"*","except":[{{"slot":"svc","binding":"{B1}"}}],"note":"n"}}]}}"#
        ),
    );
    let Loaded::Readable { policy, broken } = load(home.path()) else {
        panic!("not readable")
    };
    assert!(broken.is_empty());
    assert_eq!(policy.rules.len(), 1);
    assert_eq!(policy.rules[0].except, vec![sel("svc", Some(B1))]);

    for bad in [
        "not json",
        "[]",
        r#"{"schema":"aware.command-policy/v2","rules":[]}"#,
        r#"{"rules":[]}"#,
        &format!(r#"{{"schema":"{SCHEMA}"}}"#),
        &format!(r#"{{"schema":"{SCHEMA}","rules":[],"extra":1}}"#),
        // A rule with no readable agent widens to the whole file.
        &format!(
            r#"{{"schema":"{SCHEMA}","rules":[{{"id":"r","command":"read","accounts":"*"}}]}}"#
        ),
    ] {
        write(home.path(), bad);
        assert!(
            matches!(load(home.path()), Loaded::Unreadable { .. }),
            "{bad}"
        );
    }
}

#[test]
fn an_invalid_rule_with_a_readable_agent_is_scoped_to_that_agent() {
    let home = tempfile::tempdir().unwrap();
    for bad_rule in [
        r#"{"id":"bad","agent":"svc","command":"read"}"#,
        r#"{"id":"bad","agent":"svc","command":"read","accounts":[]}"#,
        r#"{"id":"bad","agent":"svc","command":"read","accounts":"*","except":[{"slot":"svc"}]}"#,
        r#"{"id":"bad","agent":"svc","command":"read","accounts":"*","surprise":true}"#,
        r#"{"id":"bad","agent":"svc","command":7,"accounts":"*"}"#,
        r#"{"id":"bad","agent":"svc","command":"read","accounts":[{"slot":"svc","binding":"nope"}]}"#,
    ] {
        write(
            home.path(),
            &format!(
                r#"{{"schema":"{SCHEMA}","rules":[{bad_rule},{{"id":"ok","agent":"other","command":"x","accounts":"*"}}]}}"#
            ),
        );
        let loaded = load(home.path());
        assert_eq!(
            evaluate(&loaded, "svc", "read", "svc", U),
            unreadable(Some("bad")),
            "{bad_rule}"
        );
        assert_eq!(
            evaluate(&loaded, "svc2", "read", "svc2", U),
            Decision::Allow
        );
        assert_eq!(evaluate(&loaded, "other", "x", "other", U), denied("ok"));
    }
}

#[test]
fn an_oversized_or_non_file_policy_is_unreadable() {
    let home = tempfile::tempdir().unwrap();
    write(home.path(), &" ".repeat(MAX_POLICY_BYTES as usize + 1));
    assert!(matches!(load(home.path()), Loaded::Unreadable { .. }));
    std::fs::remove_file(home.path().join(POLICY_FILE)).unwrap();
    std::fs::create_dir(home.path().join(POLICY_FILE)).unwrap();
    assert!(matches!(load(home.path()), Loaded::Unreadable { .. }));
}

#[test]
fn strict_parse_rejects_what_lenient_parse_scopes() {
    let doc = format!(
        r#"{{"schema":"{SCHEMA}","rules":[{{"id":"r","agent":"svc","command":"read","accounts":"*","except":[{{"slot":"svc"}}]}}]}}"#
    );
    assert!(matches!(
        parse_strict(doc.as_bytes()),
        Err(PolicyError::Invalid(_))
    ));
    let dup = format!(
        r#"{{"schema":"{SCHEMA}","rules":[{{"id":"r","agent":"a","command":"x","accounts":"*"}},{{"id":"r","agent":"b","command":"x","accounts":"*"}}]}}"#
    );
    assert!(matches!(
        parse_strict(dup.as_bytes()),
        Err(PolicyError::Invalid(_))
    ));
}

// ── writing ──────────────────────────────────────────────────────────────────

#[test]
fn upsert_and_remove_round_trip_through_the_file() {
    let home = tempfile::tempdir().unwrap();
    let r = rule("r", "read", Accounts::All, vec![sel("svc", Some(B1))]);
    assert!(!upsert(home.path(), r.clone()).unwrap());
    assert_eq!(
        evaluate(
            &load(home.path()),
            "svc",
            "read",
            "svc",
            Identity::Verified(B2)
        ),
        denied("r")
    );
    let narrower = rule(
        "r",
        "read",
        Accounts::Only(vec![sel("svc.x", None)]),
        vec![],
    );
    assert!(upsert(home.path(), narrower.clone()).unwrap());
    let Loaded::Readable { policy, .. } = load(home.path()) else {
        panic!("not readable")
    };
    assert_eq!(policy.rules, vec![narrower]);
    assert!(remove(home.path(), "r").unwrap());
    assert!(!remove(home.path(), "r").unwrap());
    assert_eq!(
        evaluate(&load(home.path()), "svc", "read", "svc.x", U),
        Decision::Allow
    );
    // No temp file left behind next to the policy.
    let stray: Vec<_> = std::fs::read_dir(home.path())
        .unwrap()
        .flatten()
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .filter(|n| n != POLICY_FILE && n != LOCK_FILE)
        .collect();
    assert!(stray.is_empty(), "{stray:?}");
}

#[test]
fn updates_refuse_to_merge_into_an_unreadable_file_but_set_replaces_it() {
    let home = tempfile::tempdir().unwrap();
    write(home.path(), "{ broken");
    let r = rule("r", "read", Accounts::All, vec![]);
    assert!(matches!(
        upsert(home.path(), r.clone()),
        Err(PolicyError::Unreadable(_))
    ));
    assert!(matches!(
        remove(home.path(), "r"),
        Err(PolicyError::Unreadable(_))
    ));
    // Still the broken bytes: nothing was silently dropped.
    assert_eq!(
        std::fs::read_to_string(home.path().join(POLICY_FILE)).unwrap(),
        "{ broken"
    );
    set(home.path(), &Policy { rules: vec![r] }).unwrap();
    assert_eq!(
        evaluate(&load(home.path()), "svc", "read", "svc", U),
        denied("r")
    );
}

#[test]
fn upsert_rejects_an_exception_without_a_binding() {
    let home = tempfile::tempdir().unwrap();
    let r = rule("r", "read", Accounts::All, vec![sel("svc", None)]);
    assert!(matches!(
        upsert(home.path(), r),
        Err(PolicyError::Invalid(_))
    ));
    assert_eq!(load(home.path()), Loaded::Absent);
}

#[test]
fn a_write_that_would_exceed_the_cap_is_refused_and_keeps_the_old_policy() {
    let home = tempfile::tempdir().unwrap();
    let kept = Policy {
        rules: vec![rule("kept", "read", Accounts::All, vec![])],
    };
    set(home.path(), &kept).unwrap();
    let before = std::fs::read(home.path().join(POLICY_FILE)).unwrap();
    // Each rule is ~100 bytes compact and more pretty-printed: 3000 of them is
    // well under the cap as a compact `set` document would be counted by some
    // callers, and over it as written.
    let many = Policy {
        rules: (0..3000)
            .map(|i| rule(&format!("r{i}"), "read", Accounts::All, vec![]))
            .collect(),
    };
    assert!(matches!(
        set(home.path(), &many),
        Err(PolicyError::Invalid(_))
    ));
    assert_eq!(
        std::fs::read(home.path().join(POLICY_FILE)).unwrap(),
        before
    );
    // Growth through `deny` is held to the same cap.
    let big_note = "n".repeat(MAX_NOTE_CHARS);
    let mut error = None;
    for i in 0..400 {
        let mut r = rule(&format!("g{i}"), "read", Accounts::All, vec![]);
        r.note = Some(big_note.clone());
        if let Err(e) = upsert(home.path(), r) {
            error = Some(e);
            break;
        }
    }
    assert!(matches!(error, Some(PolicyError::Invalid(_))));
    assert!(matches!(load(home.path()), Loaded::Readable { .. }));
}

#[cfg(unix)]
#[test]
fn a_link_or_fifo_at_the_policy_path_is_unreadable_and_never_blocks() {
    let home = tempfile::tempdir().unwrap();
    let path = home.path().join(POLICY_FILE);
    // A link to a valid, empty policy: still refused (never followed).
    let target = home.path().join("elsewhere.json");
    std::fs::write(&target, format!(r#"{{"schema":"{SCHEMA}","rules":[]}}"#)).unwrap();
    std::os::unix::fs::symlink(&target, &path).unwrap();
    assert!(matches!(load(home.path()), Loaded::Unreadable { .. }));
    // A dangling link is not "absent".
    std::fs::remove_file(&target).unwrap();
    assert!(matches!(load(home.path()), Loaded::Unreadable { .. }));
    std::fs::remove_file(&path).unwrap();
    // A FIFO with no writer: refused at once, not a hang.
    let c_path = std::ffi::CString::new(path.to_string_lossy().as_bytes()).unwrap();
    // SAFETY: `c_path` is a valid NUL-terminated path that outlives the call.
    assert_eq!(unsafe { libc::mkfifo(c_path.as_ptr(), 0o600) }, 0);
    assert!(matches!(load(home.path()), Loaded::Unreadable { .. }));
}

#[cfg(windows)]
#[test]
fn a_link_at_the_policy_path_is_unreadable_where_links_can_be_made() {
    let home = tempfile::tempdir().unwrap();
    let path = home.path().join(POLICY_FILE);
    let target = home.path().join("elsewhere.json");
    std::fs::write(&target, format!(r#"{{"schema":"{SCHEMA}","rules":[]}}"#)).unwrap();
    // Creating a symlink needs Developer Mode or the privilege; skip without.
    if std::os::windows::fs::symlink_file(&target, &path).is_err() {
        eprintln!("[skip] cannot create a file symlink here");
        return;
    }
    assert!(matches!(load(home.path()), Loaded::Unreadable { .. }));
    std::fs::remove_file(&target).unwrap();
    assert!(matches!(load(home.path()), Loaded::Unreadable { .. }));
}
