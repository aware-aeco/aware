use super::*;

fn agent(extra: &str) -> Agent {
    serde_yaml::from_str(&format!(
        "agent: verbot\nversion: 1.1.0\ndescription: x\nstateful: false\nlicense: MIT\n\
         transport:\n  cli:\n    binary: aware-tekla\n{extra}\
         commands:\n  go: {{ lifecycle: single, description: x }}\n"
    ))
    .unwrap()
}

fn declared(extra: &str) -> Declaration {
    parse(&agent(extra))
}

fn range(min: Option<u32>, max: Option<u32>) -> Declaration {
    Declaration::Declared(Range { min, max })
}

#[test]
fn a_manifest_without_the_key_makes_no_claim() {
    assert_eq!(declared(""), Declaration::Absent);
}

#[test]
fn valid_declarations_parse_with_either_bound_or_both() {
    assert_eq!(
        declared("    bridge-protocol: { min: 2 }\n"),
        range(Some(2), None)
    );
    assert_eq!(
        declared("    bridge-protocol: { max: 3 }\n"),
        range(None, Some(3))
    );
    assert_eq!(
        declared("    bridge-protocol: { min: 1, max: 1 }\n"),
        range(Some(1), Some(1))
    );
}

#[test]
fn every_malformed_shape_is_invalid_with_its_own_reason() {
    for (yaml, needle) in [
        // present-but-null is NOT absent
        ("    bridge-protocol:\n", "must be a mapping"),
        ("    bridge-protocol: null\n", "must be a mapping"),
        ("    bridge-protocol: 2\n", "must be a mapping"),
        ("    bridge-protocol: [1, 2]\n", "must be a mapping"),
        ("    bridge-protocol: {}\n", "min`, `max` or both"),
        ("    bridge-protocol: { min: 0 }\n", "1 or higher"),
        ("    bridge-protocol: { min: -1 }\n", "1 or higher"),
        ("    bridge-protocol: { min: 1.5 }\n", "whole number"),
        ("    bridge-protocol: { min: \"2\" }\n", "whole number"),
        ("    bridge-protocol: { min: 4294967296 }\n", "whole number"),
        ("    bridge-protocol: { min: 3, max: 2 }\n", "higher than"),
        ("    bridge-protocol: { exact: 2 }\n", "unknown key `exact`"),
    ] {
        match declared(yaml) {
            Declaration::Invalid(reason) => {
                assert!(reason.contains(needle), "{yaml}: {reason}");
            }
            other => panic!("{yaml}: expected Invalid, got {other:?}"),
        }
    }
}

#[test]
fn the_pure_verdict_matrix_has_inclusive_bounds() {
    let min2 = Range {
        min: Some(2),
        max: None,
    };
    let max2 = Range {
        min: None,
        max: Some(2),
    };
    let both = Range {
        min: Some(2),
        max: Some(3),
    };
    assert_eq!(bridge_fit(min2, 1), Status::BridgeTooOld);
    assert_eq!(bridge_fit(min2, 2), Status::Fits);
    assert_eq!(bridge_fit(min2, 9), Status::Fits);
    assert_eq!(bridge_fit(max2, 2), Status::Fits);
    assert_eq!(bridge_fit(max2, 3), Status::BridgeTooNew);
    assert_eq!(bridge_fit(both, 1), Status::BridgeTooOld);
    assert_eq!(bridge_fit(both, 3), Status::Fits);
    assert_eq!(bridge_fit(both, 4), Status::BridgeTooNew);
}

fn plant(dir: &Path, flat: bool, version: Option<&str>, protocol: Option<&str>) {
    let exe = if flat {
        dir.join("aware-tekla.exe")
    } else {
        let sub = dir.join("aware-tekla");
        std::fs::create_dir_all(&sub).unwrap();
        sub.join("aware-tekla.exe")
    };
    std::fs::write(exe, b"x").unwrap();
    if let Some(v) = version {
        std::fs::write(dir.join("aware-tekla.version"), v).unwrap();
    }
    if let Some(p) = protocol {
        std::fs::write(dir.join("aware-tekla.protocol"), p).unwrap();
    }
}

fn finding(extra: &str, dir: &Path) -> Option<Finding> {
    finding_for(&agent(extra), dir)
}

#[test]
fn a_stamp_is_read_from_either_executable_layout() {
    for flat in [true, false] {
        let dir = tempfile::tempdir().unwrap();
        plant(dir.path(), flat, Some("0.156.0"), Some("2\n"));
        let f = finding("    bridge-protocol: { min: 2 }\n", dir.path()).unwrap();
        assert_eq!(f.status, Status::Fits, "flat={flat}");
        assert_eq!(f.installed_protocol, Some(2));
        assert!(!f.assumed);
    }
}

#[test]
fn the_baseline_is_assumed_only_before_the_first_stamping_release() {
    let dir = tempfile::tempdir().unwrap();
    plant(dir.path(), true, Some("0.155.0"), None);
    let f = finding("    bridge-protocol: { min: 2 }\n", dir.path()).unwrap();
    assert_eq!(
        (f.status, f.installed_protocol, f.assumed),
        (Status::BridgeTooOld, Some(1), true)
    );

    // 0.156.0 stamps, so a missing stamp there is NOT a claim.
    let dir = tempfile::tempdir().unwrap();
    plant(dir.path(), true, Some("0.156.0"), None);
    let f = finding("    bridge-protocol: { min: 2 }\n", dir.path()).unwrap();
    assert_eq!((f.status, f.installed_protocol), (Status::Unknown, None));
    assert!(!f.needs_attention(), "unknown has nothing to act on");

    // No stamps at all (an interrupted install): unknown, never a stale claim.
    let dir = tempfile::tempdir().unwrap();
    plant(dir.path(), true, None, None);
    let f = finding("    bridge-protocol: { min: 2 }\n", dir.path()).unwrap();
    assert_eq!(f.status, Status::Unknown);
}

#[test]
fn a_garbled_stamp_is_unknown() {
    for bad in ["", "abc", "0", "-1", "1.5", "2 3"] {
        let dir = tempfile::tempdir().unwrap();
        plant(dir.path(), true, Some("0.156.0"), Some(bad));
        let f = finding("    bridge-protocol: { min: 2 }\n", dir.path()).unwrap();
        assert_eq!(f.status, Status::Unknown, "stamp {bad:?}");
    }
}

fn agent_with_binary(binary: &str) -> Agent {
    serde_yaml::from_str(&format!(
        "agent: verbot\nversion: 1.1.0\ndescription: x\nstateful: false\nlicense: MIT\n\
         transport:\n  cli:\n    binary: {binary}\n    bridge-protocol: {{ min: 2 }}\n\
         commands:\n  go: {{ lifecycle: single, description: x }}\n"
    ))
    .unwrap()
}

#[test]
fn the_verdict_describes_what_dispatch_would_run() {
    let dir = tempfile::tempdir().unwrap();
    plant(dir.path(), true, Some("0.156.0"), Some("1"));
    // Dispatch matches the binary name exactly, so `aware-tekla.exe` goes to PATH.
    assert_eq!(
        finding_for(&agent_with_binary("aware-tekla.exe"), dir.path())
            .unwrap()
            .status,
        Status::Unknown
    );
    // A binary that is not a managed bridge at all.
    assert_eq!(
        finding_for(&agent_with_binary("some-tool"), dir.path())
            .unwrap()
            .status,
        Status::Unknown
    );
    // Managed copy absent (a PATH-only legacy copy would be what runs): unknown.
    let empty = tempfile::tempdir().unwrap();
    assert_eq!(
        finding("    bridge-protocol: { min: 2 }\n", empty.path())
            .unwrap()
            .status,
        Status::Unknown
    );
    // The exact name with a managed copy IS judged.
    assert_eq!(
        finding_for(&agent_with_binary("aware-tekla"), dir.path())
            .unwrap()
            .status,
        Status::BridgeTooOld
    );
}

#[test]
fn an_invalid_declaration_asks_for_attention_but_never_blocks() {
    let dir = tempfile::tempdir().unwrap();
    plant(dir.path(), true, Some("0.156.0"), Some("1"));
    let f = finding("    bridge-protocol:\n", dir.path()).unwrap();
    assert_eq!(f.status, Status::DeclarationInvalid);
    assert!(f.needs_attention());
    assert!(
        f.detail.contains("no bridge check was made"),
        "{}",
        f.detail
    );
    assert_eq!(f.declared, None);
}

#[test]
fn the_choices_name_the_real_commands_and_always_allow_carrying_on() {
    let dir = tempfile::tempdir().unwrap();
    plant(dir.path(), true, Some("0.156.0"), Some("1"));
    let old = finding("    bridge-protocol: { min: 2 }\n", dir.path()).unwrap();
    assert_eq!(old.status, Status::BridgeTooOld);
    assert!(old.choices[0].contains("aware sidecar install tekla"));
    assert!(old.choices.last().unwrap().contains("Carry on anyway"));
    assert!(
        warning("verbot", &old)
            .starts_with("\u{26a0} agent verbot: verbot 1.1.0 needs protocol 2 or newer")
    );

    plant(dir.path(), true, Some("0.156.0"), Some("3"));
    let new = finding("    bridge-protocol: { max: 2 }\n", dir.path()).unwrap();
    assert_eq!(new.status, Status::BridgeTooNew);
    assert!(new.choices[0].contains("aware agent update verbot"));
    assert!(new.choices.last().unwrap().contains("Carry on anyway"));

    let fits = finding("    bridge-protocol: { min: 1, max: 3 }\n", dir.path()).unwrap();
    assert_eq!(fits.status, Status::Fits);
    assert!(fits.choices.is_empty() && !fits.needs_attention());
}

#[test]
fn no_cli_transport_or_no_key_means_no_finding() {
    let dir = tempfile::tempdir().unwrap();
    plant(dir.path(), true, Some("0.156.0"), Some("1"));
    assert!(finding("", dir.path()).is_none());
    let builtin: Agent = serde_yaml::from_str(
        "agent: verbot\nversion: 1.1.0\ndescription: x\nstateful: false\nlicense: MIT\n\
         transport:\n  builtin: {}\n\
         commands:\n  go: { lifecycle: single, description: x }\n",
    )
    .unwrap();
    assert!(finding_for(&builtin, dir.path()).is_none());
}

#[test]
fn validate_agent_rejects_a_malformed_declaration_and_accepts_the_rest() {
    let bad = agent("    bridge-protocol: { min: 3, max: 2 }\n");
    let issues = crate::validate::validate_agent(&bad);
    assert!(
        issues
            .iter()
            .any(|i| i.code == "E_AGENT_BRIDGE_PROTOCOL_INVALID"),
        "{issues:?}"
    );
    for ok in ["", "    bridge-protocol: { min: 1 }\n"] {
        let issues = crate::validate::validate_agent(&agent(ok));
        assert!(
            issues
                .iter()
                .all(|i| i.code != "E_AGENT_BRIDGE_PROTOCOL_INVALID"),
            "{ok}: {issues:?}"
        );
    }
}
