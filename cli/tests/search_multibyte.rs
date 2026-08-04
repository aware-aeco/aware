//! `aware search` must not abort on a command description containing a
//! multi-byte character.
//!
//! It used to. The renderer trimmed long descriptions with `&desc[..97]`, a
//! *byte* index, and Rust aborts the process when a byte index lands inside a
//! UTF-8 character. Four descriptions already committed under `20-agents/` do
//! exactly that — sketchup-2025/2026 `color-to-s`, acc-issues
//! `delete-attachment`, slack `admin-conversations-ekm-list-…` — so the crash
//! was reachable from a plain `aware search` against the shipped substrate, no
//! unusual input required:
//!
//! ```text
//! thread 'main' panicked at src/commands/search.rs:132:41:
//! end byte index 97 is not a char boundary; it is inside '“' (bytes 95..98)
//! ```
//!
//! CLAUDE.md §Code style — "Errors as data, not exceptions" — forbids that. The
//! fixture below is the real sketchup-2026 description verbatim; if the byte
//! slice ever comes back, this test aborts with exit 101 instead of 0.

use assert_cmd::Command;
use predicates::prelude::*;

/// Verbatim from `20-agents/aeco/architecture/sketchup-2026/manifest.yaml`,
/// `commands.color-to-s.description`. The `“` occupies bytes 95..98, so a cut at
/// byte 97 lands inside it.
const CRASHER: &str = "The #to_s method returns a string representation of the Sketchup::Color object, in the form of “Color(255, 255, 255, 255)”.";

/// A second shape the byte cut also mishandles: the trim point falls inside a
/// *four*-byte character rather than a three-byte one, so a fix that only
/// handled the two- and three-byte cases would still abort here.
const WIDE: &str = "Bake the coordination model and hand the finished baked scene back to every caller for review 🏗🏗🏗🏗 and mark the run complete for the downstream reviewers.";

fn write_agent(home: &std::path::Path) {
    let dir = home.join("agents").join("fixture-agent");
    std::fs::create_dir_all(&dir).unwrap();
    let manifest = format!(
        "agent: fixture-agent\n\
         version: 0.1.0\n\
         description: Fixture for the multibyte-truncation regression.\n\
         stateful: false\n\
         license: Apache-2.0\n\
         transport:\n  \
           cli:\n    \
             binary: aware-fixture\n\
         commands:\n  \
           color-to-s:\n    \
             lifecycle: single\n    \
             description: {CRASHER:?}\n  \
           bake-scene:\n    \
             lifecycle: single\n    \
             description: {WIDE:?}\n"
    );
    std::fs::write(dir.join("manifest.yaml"), manifest).unwrap();
}

/// Guards the fixtures themselves. If an edit made either description ASCII, or
/// short enough not to be trimmed, the assertions below would pass while
/// exercising nothing — the truncation path would never run.
#[test]
fn fixtures_actually_straddle_the_cut() {
    for (label, text) in [("CRASHER", CRASHER), ("WIDE", WIDE)] {
        assert!(
            text.chars().count() > 97,
            "{label} is too short to be truncated at all"
        );
        let byte_97 = text.as_bytes()[97];
        assert_eq!(
            byte_97 & 0xC0,
            0x80,
            "{label} no longer has a multi-byte character straddling byte 97, so it \
             would not reproduce the panic"
        );
    }
}

#[test]
fn search_renders_multibyte_descriptions_without_aborting() {
    let tmp = tempfile::tempdir().unwrap();
    write_agent(tmp.path());

    // `.success()` is the whole assertion's point: the bug was exit 101, not
    // wrong output.
    let out = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["search", "the", "--agent", "fixture-agent"])
        .assert()
        .success()
        .stdout(predicate::str::contains("color-to-s"))
        .stdout(predicate::str::contains("bake-scene"))
        .get_output()
        .stdout
        .clone();

    let stdout = String::from_utf8(out).expect("output must be valid UTF-8");
    // Truncation happened (so the risky path really ran) and it cut cleanly:
    // the rendered line ends at a whole character, never a replacement char or
    // a split sequence.
    for line in stdout.lines().filter(|l| l.contains(" → ")) {
        let rendered = line.split(" → ").nth(1).expect("description column");
        assert!(
            rendered.ends_with('…'),
            "expected a truncated description, got: {rendered}"
        );
        assert!(
            !rendered.contains('\u{FFFD}'),
            "truncation produced a replacement character: {rendered}"
        );
        // 97 kept characters plus the ellipsis.
        assert_eq!(rendered.chars().count(), 98, "unexpected width: {rendered}");
    }
}

/// The JSON surface carries descriptions untruncated, so it must round-trip the
/// multi-byte text intact rather than lose or mangle it.
#[test]
fn search_json_round_trips_multibyte_descriptions() {
    let tmp = tempfile::tempdir().unwrap();
    write_agent(tmp.path());

    let out = Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", tmp.path())
        .args(["search", "the", "--agent", "fixture-agent", "--json"])
        .assert()
        .success()
        .get_output()
        .stdout
        .clone();

    let parsed: serde_json::Value = serde_json::from_slice(&out).unwrap();
    let text = parsed.to_string();
    assert!(
        text.contains("Color(255, 255, 255, 255)"),
        "JSON lost the multibyte description: {text}"
    );
}
