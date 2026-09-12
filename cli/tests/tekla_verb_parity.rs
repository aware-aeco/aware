//! Gate: the tekla manifest advertises exactly the verbs the `aware-tekla` bridge can dispatch.
//!
//! The manifest declared 24 commands; the bridge's `switch (verb)` dispatched 7. The other 19 —
//! `insert`, `part-list`, `report-create`, `uda-get`, `ifc-export`, `clash-check` and the rest —
//! were declared as ordinary runnable commands, so `aware app validate` and `aware app compile`
//! accepted a node using one and the run then died at the bridge with
//! `aware-tekla: unknown verb 'insert'`. An app author (or the terminal AI composing a workflow)
//! reads `manifest.yaml` as the contract, and that contract listed a native connection insert, UDA
//! get/set, NC/IFC export, drawing issue and clash check that do not exist — with the first signal
//! arriving after compile and the Run gate had both said yes (#520).
//!
//! The same drift, at one command, is #176 (`watch` declared, not implemented) and #161
//! (`html-report` declared a binary it did not ship). The marker that closes it already existed:
//! per-command `status: planned` (#199, `cli/src/validate.rs`), which turns such a node into
//! `E_APP_COMMAND_UNAVAILABLE` at validate/install time. It was simply unused on the agent that
//! needed it most.
//!
//! Marking the 19 fixes today. This fixes tomorrow, in BOTH directions, because each direction is
//! a different defect and neither implies the other:
//!
//!   * A **declared-but-undispatched** command without `status: planned` is the drift above — a
//!     promise the bridge cannot keep, failing at run.
//!   * A **dispatched** command marked `status: planned` (or absent from the manifest) is the
//!     inverse: working functionality nobody can reach. `list-instances` and `close` were in that
//!     state — dispatched since the bridge was written, published nowhere, so the only supported
//!     way to enumerate running hosts or shut one down was invisible to `aware agent describe` and
//!     to every app author.
//!
//! So a verb landing in the bridge without its marker being lifted, and a command being marked
//! planned while it works, both fail here.
//!
//! ## Fail closed, not open
//!
//! The gate's whole value rests on [`dispatched_verbs`] reading the real dispatch set, and the
//! dangerous failure is a parser that silently returns too FEW verbs: the planned-marker half
//! would then be satisfied vacuously by commands that do work. So it is an error rather than an
//! empty set whenever the shape it expects is absent, and [`dispatch_anchors`] additionally
//! asserts the verbs that cannot plausibly leave the bridge are present. The over-matching
//! direction is covered by the decoy cases in this file's own unit tests: `Program.cs` has four
//! other `switch` statements over strings (a watch filter, the argv loop, …) and an earlier
//! whole-file scan for `case "…":` collected those too.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use predicates::prelude::*;
use serde_yaml::Value;

/// Verbs that must be dispatched for the bridge to be the bridge at all. Not the full set, on
/// purpose — pinning that would make implementing a planned verb fail this gate, which is the
/// opposite of the point. These three are the load-bearing ones: `exec` and `bake-scene` are what
/// every hand-written Tekla workflow runs through today, and `send-status` is the connection
/// smoke-test every other verb's documentation points at.
const DISPATCH_ANCHORS: [&str; 3] = ["send-status", "exec", "bake-scene"];

fn repository_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("cli crate must live directly below the repository root")
        .to_path_buf()
}

fn tekla_manifest_path() -> PathBuf {
    repository_root().join("20-agents/aeco/engineering/tekla/manifest.yaml")
}

fn bridge_source_path() -> PathBuf {
    repository_root().join("cli-tekla/Program.cs")
}

/// Blank out `//` and `/* … */` comments, preserving everything else verbatim.
///
/// Without this a commented-out `case "part-list":` — the exact shape a half-reverted
/// implementation leaves behind — would read as a dispatched verb and excuse a missing marker.
/// Replacing comment bytes with spaces rather than deleting them keeps line structure intact, so
/// the slicing below cannot be thrown off by how much was removed.
fn strip_comments(source: &str) -> String {
    let bytes: Vec<char> = source.chars().collect();
    let mut out = String::with_capacity(source.len());
    let mut i = 0;
    while i < bytes.len() {
        let two = (bytes[i], bytes.get(i + 1).copied().unwrap_or('\0'));
        match two {
            ('/', '/') => {
                while i < bytes.len() && bytes[i] != '\n' {
                    out.push(' ');
                    i += 1;
                }
            }
            ('/', '*') => {
                // Consume through the terminator; an unterminated block comment swallows the
                // remainder, which makes the gate fail closed rather than read half a file.
                while i < bytes.len() {
                    if bytes[i] == '*' && bytes.get(i + 1) == Some(&'/') {
                        out.push(' ');
                        out.push(' ');
                        i += 2;
                        break;
                    }
                    out.push(if bytes[i] == '\n' { '\n' } else { ' ' });
                    i += 1;
                }
            }
            _ => {
                out.push(bytes[i]);
                i += 1;
            }
        }
    }
    out
}

/// The verbs `aware-tekla` dispatches, read out of its own `switch (verb)`.
///
/// Scoped to the region between that switch and its `default:` label, because the bridge has other
/// string switches whose cases are not verbs. The `default:` is also the evidence that the region
/// found is the dispatch table and not something that merely reads like one, so its
/// unknown-verb diagnostic is required: if that arm is ever restructured this returns an error and
/// the gate fails, rather than quietly reading a region that no longer means what it meant.
fn dispatched_verbs(source: &str) -> Result<BTreeSet<String>, String> {
    let code = strip_comments(source);
    let switch_at = code
        .find("switch (verb)")
        .ok_or("no `switch (verb)` in the bridge source")?;
    let rest = &code[switch_at..];
    let default_at = rest
        .find("default:")
        .ok_or("`switch (verb)` has no `default:` arm")?;
    let (table, tail) = (&rest[..default_at], &rest[default_at..]);

    // A nested switch inside the dispatch table would put foreign cases in range. Nothing nests
    // one today (every arm is a single `return Verb(parsed);`), so refuse rather than guess.
    if table[("switch (verb)".len())..].contains("switch ") {
        return Err(
            "a nested `switch` inside the dispatch table — this parser cannot scope it".into(),
        );
    }
    // The unknown-verb diagnostic, inside the `default:` arm only (to the end of the switch).
    let default_arm = match tail.find('}') {
        Some(end) => &tail[..end],
        None => tail,
    };
    if !default_arm.contains("unknown verb") {
        return Err("the `default:` arm no longer reports an unknown verb".into());
    }

    let mut verbs = BTreeSet::new();
    let mut cursor = table;
    while let Some(at) = cursor.find("case ") {
        cursor = &cursor[at + "case ".len()..];
        let Some(stripped) = cursor.strip_prefix('"') else {
            // `case SomeConstant:` — not a literal verb. Nothing in the table uses that form, so
            // flag it rather than skip it: a verb dispatched through a constant would otherwise
            // read as undispatched and be marked planned while it works.
            return Err(format!(
                "non-literal `case` in the dispatch table: {:?}",
                cursor.chars().take(40).collect::<String>()
            ));
        };
        let Some(close) = stripped.find('"') else {
            return Err("unterminated string in a `case` label".into());
        };
        let verb = &stripped[..close];
        if !stripped[close + 1..].trim_start().starts_with(':') {
            return Err(format!(
                "`case \"{verb}\"` is not followed by a label colon"
            ));
        }
        if !verbs.insert(verb.to_string()) {
            return Err(format!("`case \"{verb}\"` appears twice"));
        }
        cursor = &stripped[close..];
    }
    if verbs.is_empty() {
        return Err("the dispatch table contains no `case \"…\":` labels".into());
    }
    Ok(verbs)
}

/// Every declared command, mapped to whether it carries `status: planned`.
///
/// Read as YAML rather than through the crate's `Agent` type so the assertion is about the file as
/// written. `status` defaults to `available` in the deserializer, and a gate about a missing marker
/// must not be reading a default that fills the marker in.
fn declared_commands(manifest: &str) -> BTreeMap<String, bool> {
    let manifest: Value = serde_yaml::from_str(manifest).expect("parse the tekla manifest");
    let commands = manifest
        .get("commands")
        .and_then(Value::as_mapping)
        .expect("the tekla manifest declares a `commands:` mapping");
    commands
        .iter()
        .map(|(name, body)| {
            let name = name.as_str().expect("command keys are strings").to_string();
            let planned = body.get("status").and_then(Value::as_str) == Some("planned");
            (name, planned)
        })
        .collect()
}

#[test]
fn dispatch_anchors() {
    let source = fs::read_to_string(bridge_source_path()).expect("read the bridge source");
    let dispatched = dispatched_verbs(&source).expect("read the bridge's dispatch table");
    for anchor in DISPATCH_ANCHORS {
        assert!(
            dispatched.contains(anchor),
            "the parser did not find `{anchor}` in the bridge's dispatch table — it is reading the \
             wrong region, and a parser that finds too few verbs makes the planned-marker \
             assertion below pass vacuously. Found: {dispatched:?}"
        );
    }
}

#[test]
fn every_declared_tekla_command_the_bridge_cannot_dispatch_is_marked_planned() {
    let source = fs::read_to_string(bridge_source_path()).expect("read the bridge source");
    let dispatched = dispatched_verbs(&source).expect("read the bridge's dispatch table");
    let declared = declared_commands(
        &fs::read_to_string(tekla_manifest_path()).expect("read the tekla manifest"),
    );

    let unmarked: Vec<&str> = declared
        .iter()
        .filter(|(name, planned)| !dispatched.contains(name.as_str()) && !**planned)
        .map(|(name, _)| name.as_str())
        .collect();
    assert!(
        unmarked.is_empty(),
        "tekla declares {unmarked:?} as runnable, but `aware-tekla` dispatches none of them — an \
         app using one compiles and then fails at run with `unknown verb`. Add `status: planned` \
         to each (#199, #520), or wire the verb into `cli-tekla/Program.cs`."
    );
}

#[test]
fn every_verb_the_bridge_dispatches_is_declared_and_runnable() {
    let source = fs::read_to_string(bridge_source_path()).expect("read the bridge source");
    let dispatched = dispatched_verbs(&source).expect("read the bridge's dispatch table");
    let declared = declared_commands(
        &fs::read_to_string(tekla_manifest_path()).expect("read the tekla manifest"),
    );

    let undeclared: Vec<&String> = dispatched
        .iter()
        .filter(|verb| !declared.contains_key(verb.as_str()))
        .collect();
    assert!(
        undeclared.is_empty(),
        "`aware-tekla` dispatches {undeclared:?}, which the tekla manifest declares nowhere — \
         working functionality no app author and no `aware agent describe` can see (#520)."
    );

    let wrongly_planned: Vec<&str> = declared
        .iter()
        .filter(|(name, planned)| **planned && dispatched.contains(name.as_str()))
        .map(|(name, _)| name.as_str())
        .collect();
    assert!(
        wrongly_planned.is_empty(),
        "tekla marks {wrongly_planned:?} `status: planned` while `aware-tekla` dispatches them — \
         `E_APP_COMMAND_UNAVAILABLE` will refuse a node that would have run. Delete the marker in \
         the PR that wires a verb."
    );
}

/// Install the real tekla agent into a scratch `AWARE_HOME` and write `app.flo` there.
fn tekla_app(body: &str) -> (tempfile::TempDir, tempfile::TempDir) {
    let home = tempfile::tempdir().expect("temporary AWARE_HOME");
    let installed = home.path().join("agents/tekla");
    fs::create_dir_all(&installed).expect("installed agent directory");
    fs::copy(tekla_manifest_path(), installed.join("manifest.yaml")).expect("install the manifest");

    let app = tempfile::tempdir().expect("temporary app directory");
    fs::write(app.path().join("probe.flo"), body).expect("write the app");
    (home, app)
}

/// What the marker is FOR. The two tests above prove it is present; this proves it bites — and
/// that it bites selectively, which is the half a missing marker and a blanket refusal have in
/// common. `part-list` stands in for the 19: it is read-mode, so validate reaches the availability
/// gate instead of stopping at the safety-contract rule that `insert` trips first.
#[test]
fn validate_refuses_a_planned_tekla_command_and_accepts_a_dispatched_one() {
    let (home, app) = tekla_app(
        "app: tekla-planned-probe\nversion: 0.1.0\ndescription: probe a planned verb\n\
         requires: []\nnodes:\n  - id: parts\n    agent: tekla\n    command: part-list\n",
    );
    Command::cargo_bin("aware")
        .expect("aware test binary")
        .env("AWARE_HOME", home.path())
        .args(["app", "validate"])
        .arg(app.path())
        .assert()
        .failure()
        .stdout(predicate::str::contains("E_APP_COMMAND_UNAVAILABLE"));

    let (home, app) = tekla_app(
        "app: tekla-dispatched-probe\nversion: 0.1.0\ndescription: probe a dispatched verb\n\
         requires: []\nnodes:\n  - id: ping\n    agent: tekla\n    command: send-status\n",
    );
    Command::cargo_bin("aware")
        .expect("aware test binary")
        .env("AWARE_HOME", home.path())
        .args(["app", "validate"])
        .arg(app.path())
        .assert()
        .success()
        .stdout(predicate::str::contains("E_APP_COMMAND_UNAVAILABLE").not());
}

// ── the parser's own negative controls ───────────────────────────────────────────────────────────
//
// Each of these is a way `dispatched_verbs` could report a wrong set while the three tests above
// stayed green, and two of them are mistakes an earlier draft actually made.

mod parser {
    use super::*;

    /// A dispatch table plus the decoys `Program.cs` really contains around it.
    const SOURCE: &str = r#"
        static int Run(string[] args)
        {
            switch ((filter ?? "all").Trim())
            {
                case "all": return 1;
                case "insert": return 2;
            }
            switch (verb)
            {
                case "send-status":
                    return SendStatus(parsed);
                // case "part-list":
                //     return PartList(parsed);
                case "exec":
                    return Exec(parsed);
                default:
                    Console.Error.WriteLine($"aware-tekla: unknown verb '{verb}'. Try --help.");
                    return 2;
            }
        }
    "#;

    #[test]
    fn reads_only_the_verb_switch() {
        let verbs = dispatched_verbs(SOURCE).expect("parse the sample");
        assert_eq!(
            verbs,
            ["exec", "send-status"]
                .into_iter()
                .map(String::from)
                .collect::<BTreeSet<_>>(),
            "a case from another switch, or a commented-out one, was counted as a dispatched verb"
        );
    }

    #[test]
    fn a_commented_out_case_is_not_dispatched() {
        // The decoy above is load-bearing: `// case "part-list":` sits INSIDE the verb switch, so
        // scoping alone does not exclude it. Only comment-stripping does, and without it the
        // planned marker on `part-list` would read as a bug.
        assert!(!dispatched_verbs(SOURCE).unwrap().contains("part-list"));
        let block = SOURCE.replace(
            "// case \"part-list\":",
            "/* case \"part-list\": return PartList(parsed); */",
        );
        assert!(!dispatched_verbs(&block).unwrap().contains("part-list"));
    }

    #[test]
    fn a_case_from_a_foreign_switch_is_not_dispatched() {
        // `insert` is both a decoy case above and a real planned command, so this is the exact
        // collision that would excuse a missing marker.
        assert!(!dispatched_verbs(SOURCE).unwrap().contains("insert"));
        assert!(!dispatched_verbs(SOURCE).unwrap().contains("all"));
    }

    #[test]
    fn missing_structure_is_an_error_not_an_empty_set() {
        // Every one of these would otherwise report "nothing is dispatched", which satisfies the
        // planned-marker test for all 26 commands at once.
        for (label, source) in [
            (
                "no verb switch",
                SOURCE.replace("switch (verb)", "switch (other)"),
            ),
            ("no default arm", SOURCE.replace("default:", "case \"x\":")),
            (
                "no unknown-verb diagnostic",
                SOURCE.replace("unknown verb", "something else"),
            ),
            (
                "an empty table",
                SOURCE
                    .replace("case \"send-status\":", "")
                    .replace("case \"exec\":", ""),
            ),
            (
                "a nested switch",
                SOURCE.replace("case \"exec\":", "switch (inner) { case \"exec\":"),
            ),
            (
                "a non-literal case",
                SOURCE.replace("case \"exec\":", "case ExecVerb:"),
            ),
        ] {
            assert!(
                dispatched_verbs(&source).is_err(),
                "{label}: expected an error, got {:?}",
                dispatched_verbs(&source)
            );
        }
    }

    #[test]
    fn a_planned_marker_is_read_from_the_file_not_a_default() {
        let yaml = "commands:\n  wired:\n    lifecycle: single\n    description: x\n  \
                    unwired:\n    lifecycle: single\n    status: planned\n    description: y\n";
        let declared = declared_commands(yaml);
        assert_eq!(declared.get("wired"), Some(&false));
        assert_eq!(declared.get("unwired"), Some(&true));
        // `status: available` spelled out is not `planned` either — and neither is some third
        // value arriving later, which must not be silently read as "marked".
        let explicit = declared_commands(
            "commands:\n  wired:\n    lifecycle: single\n    status: available\n    description: x\n",
        );
        assert_eq!(explicit.get("wired"), Some(&false));
    }
}
