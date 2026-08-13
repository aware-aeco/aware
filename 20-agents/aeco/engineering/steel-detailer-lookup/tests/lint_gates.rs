//! Negative control for the lint gate in `src/main.rs`.
//!
//! A `deny` attribute is only worth anything if it still fires. These tests
//! compile a throwaway crate carrying the *same* attribute this crate carries
//! and assert that clippy rejects a violation and accepts its absence — so a
//! future edit that removes or narrows the gate fails the suite instead of
//! passing quietly.
//!
//! This mirrors `cli/tests/lint_gates.rs`, which guards the identical attribute
//! on the `aware` binary. Deliberately a copy rather than a shared helper: the
//! two live in separate cargo workspaces (this crate declares its own
//! `[workspace]`), so there is no crate they could both depend on without
//! inventing one, and a gate's negative control that can be broken from another
//! workspace is not much of a control.
//!
//! What is covered, precisely:
//!   * the attribute denies `unwrap()` / `expect()` in non-test code, and it is
//!     the *denied lint* that rejects — not an unrelated probe failure;
//!   * clean code is still accepted, so a rejection above means something;
//!   * the `cfg(test)` carve-out still holds, witnessed by a companion probe so
//!     the assertion cannot pass by never compiling a test target at all;
//!   * `src/main.rs` still carries the gate verbatim;
//!   * no production source under `src/` — every `.rs` file, not just the crate
//!     root, since a module can `#![allow]` its way out from under one — has
//!     re-opened it with an `#[allow]` / `#[expect]` / `#[warn]` — a `warn`
//!     overrides an outer `deny` and leaves only CI's `-D warnings` enforcing
//!     it — or a `clippy::restriction` group override, including one rustfmt
//!     has wrapped across lines or nested in a `cfg_attr` predicate, nor from
//!     `[lints.clippy]` in the manifest, where a group entry that outranks a
//!     specific `deny` switches it off.
//!
//! The last two scan artefacts that are correct today — the real sources under
//! `src/`, the real `Cargo.toml` — so they would report clean both when they
//! work and when they have stopped matching anything at all. Each therefore has a
//! negative control driving its classifier over synthetic input:
//! `gate_reopener_classifier_matches_its_contract` and
//! `manifest_lint_reader_matches_its_contract`.
//!
//! The clippy-backed probes shell out to `cargo clippy` on a scratch crate with
//! no dependencies (hence `--offline`); the rest are pure file and string
//! checks. If clippy is missing the probes skip — except under `CI`, where the
//! workflow installs the component explicitly and a silent skip would be a hole
//! rather than a courtesy.

use std::path::Path;
use std::process::Command;

/// The gate under test, kept byte-identical to the attribute in `src/main.rs`.
const GATE: &str = "#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]";

/// `true` when the probe can run. Panics instead of skipping under `CI`.
fn clippy_available() -> bool {
    let available = Command::new("cargo")
        .args(["clippy", "--version"])
        .output()
        .is_ok_and(|out| out.status.success());
    assert!(
        available || std::env::var_os("CI").is_none(),
        "cargo clippy is unavailable under CI, where the workflow installs it — \
         skipping here would silently drop the gate's only negative control"
    );
    available
}

/// Run `cargo clippy` over a scratch crate whose `main.rs` is `body`, prefixed
/// with the gate attribute. Returns `(clippy succeeded, combined diagnostics)`.
///
/// The diagnostics matter: asserting only on the exit status would let these
/// tests pass for the wrong reason — a probe that fails to build at all also
/// "fails", which would make a dead gate look enforced.
fn run_gate(body: &str) -> (bool, String) {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    std::fs::create_dir_all(root.join("src")).expect("src dir");
    std::fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"gate_probe\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[workspace]\n",
    )
    .expect("write manifest");
    std::fs::write(root.join("src/main.rs"), format!("{GATE}\n\n{body}")).expect("write main");

    let out = Command::new("cargo")
        // `--all-targets` matches how CI invokes clippy, and is what makes the
        // `cfg(test)` carve-out testable at all: without it the probe never
        // compiles a `#[cfg(test)]` module, so that assertion would pass
        // vacuously.
        .args(["clippy", "--all-targets", "--offline", "--quiet"])
        .current_dir(root)
        // Keep the probe's build products inside the tempdir so it neither
        // pollutes nor contends on this crate's target directory.
        .env("CARGO_TARGET_DIR", root.join("target"))
        .output()
        .expect("run cargo clippy");
    let diagnostics = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    (out.status.success(), diagnostics)
}

/// Assert clippy rejected `body`, and that it rejected it *because of the gate*
/// rather than because the probe crate failed to build.
fn assert_gate_rejects(body: &str, expected_lint: &str, what: &str) {
    let (accepted, diagnostics) = run_gate(body);
    assert!(
        !accepted,
        "clippy accepted {what} in non-test code — the gate is not enforcing"
    );
    assert!(
        diagnostics.contains(expected_lint),
        "clippy rejected {what}, but not for `{expected_lint}` — the probe may be \
         failing for an unrelated reason. Diagnostics:\n{diagnostics}"
    );
}

#[test]
fn unwrap_gate_rejects_unwrap_and_expect_outside_tests() {
    if !clippy_available() {
        eprintln!("skipping: cargo clippy unavailable");
        return;
    }

    // Positive control: the same crate, same attribute, no violation. If this
    // fails, the probe itself is broken and the rejections below prove nothing.
    let clean = "fn main() {\n    let v: Option<u8> = std::env::args().count().try_into().ok();\n    println!(\"{v:?}\");\n}\n";
    let (accepted, diagnostics) = run_gate(clean);
    assert!(
        accepted,
        "gate probe rejected clean code — the probe is broken, not the gate:\n{diagnostics}"
    );

    // `Option<u8>` from a runtime value, not a `Some(1)` literal: a literal also
    // trips `unnecessary_literal_unwrap`, which would muddy which lint fired.
    assert_gate_rejects(
        "fn main() {\n    let v: Option<u8> = std::env::args().count().try_into().ok();\n    println!(\"{}\", v.unwrap());\n}\n",
        "unwrap_used",
        "`.unwrap()`",
    );

    assert_gate_rejects(
        "fn main() {\n    let v: Option<u8> = std::env::args().count().try_into().ok();\n    println!(\"{}\", v.expect(\"set\"));\n}\n",
        "expect_used",
        "`.expect()`",
    );
}

#[test]
fn unwrap_gate_still_permits_unwrap_in_tests() {
    if !clippy_available() {
        eprintln!("skipping: cargo clippy unavailable");
        return;
    }

    // Witness first. This probe puts a hard type error inside `#[cfg(test)]`;
    // clippy must reject it. If it does not, the probe never compiled a test
    // target — and the carve-out assertion below would then pass for that
    // reason alone rather than because the carve-out held.
    let (witness_accepted, witness_diagnostics) = run_gate(
        "fn main() {}\n\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn t() {\n        let _x: u8 = \"not a u8\";\n    }\n}\n",
    );
    assert!(
        !witness_accepted,
        "a type error inside `#[cfg(test)]` was not caught, so the probe is not \
         compiling test targets — the carve-out assertion below would be vacuous.\n\
         Check that `run_gate` still passes `--all-targets`.\n{witness_diagnostics}"
    );

    // CLAUDE.md permits `unwrap()` in tests. `cfg_attr(not(test), …)` is what
    // buys that carve-out; this asserts the carve-out survives, so nobody
    // "fixes" the gate into one that would force `unwrap`-free test code.
    let (accepted, diagnostics) = run_gate(
        "fn main() {}\n\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn t() {\n        let v: Option<u8> = Some(1);\n        assert_eq!(v.unwrap(), 1);\n    }\n}\n",
    );
    assert!(
        accepted,
        "the gate fired inside `#[cfg(test)]`, where CLAUDE.md permits `unwrap()`:\n{diagnostics}"
    );
}

#[test]
fn crate_root_actually_carries_the_gate() {
    // The probes above prove the attribute denies. This proves the shipped
    // crate is the thing carrying it — otherwise the gate could be deleted from
    // `src/main.rs` and every behavioural assertion above would still pass.
    let source = std::fs::read_to_string(manifest_dir().join("src/main.rs")).expect("read main.rs");
    assert!(
        source.contains(GATE),
        "`src/main.rs` no longer carries the gate verbatim:\n  {GATE}\n\
         If it was deliberately reworded, update `GATE` here in the same change \
         so the probes keep testing what the crate actually carries."
    );
}

/// Every production `.rs` file, not just `src/main.rs`.
///
/// The crate is a single file today, and scanning only that file was the gate's
/// blind spot: a crate-root `deny` is overridden by an `#![allow(…)]` at the top
/// of any *other* module, so the first `src/helper.rs` this crate grows could
/// re-open the gate with nothing reading it. Walking `src/` means the guard
/// survives the crate being split into modules, which is precisely when a
/// hand-listed path stops covering it.
#[test]
fn nobody_reopened_the_gate_from_source() {
    let root = manifest_dir();
    let mut files = Vec::new();
    collect_rs_files(&root.join("src"), &mut files);
    files.sort();

    // A floor, not an is-empty check: a walk that silently recovered nothing
    // would report a clean scan. `src/main.rs` is the file the crate cannot
    // exist without, so its presence is what proves the walk actually ran.
    assert!(
        files.contains(&root.join("src/main.rs")),
        "the walk of {} did not reach `src/main.rs`, so a clean result means \
         nothing:\n{files:#?}",
        root.join("src").display()
    );

    let mut offenders = Vec::new();
    for file in &files {
        // Not `else { continue }`. A file this gate cannot read is a file it
        // cannot clear, and turning that into silence is the exact failure mode
        // CLAUDE.md §Engineering rules forbids — in the file that enforces it.
        let source = std::fs::read_to_string(file)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", file.display()));
        let relative = file
            .strip_prefix(root)
            .unwrap_or(file)
            .display()
            .to_string();
        for hit in gate_reopeners(&source) {
            offenders.push(format!("{relative}:{}: {}", hit.line, hit.text));
        }
    }

    assert!(
        offenders.is_empty(),
        "these attributes re-open the gate the crate is supposed to be under. \
         CLAUDE.md forbids silencing a gate rather than fixing the violation \
         under it:\n  {}",
        offenders.join("\n  ")
    );
}

/// Collect every `.rs` file under `dir`, recursively.
///
/// Panics rather than returning early on an unreadable directory. Turning a
/// failed walk into a short file list would hand the caller a clean scan of
/// almost nothing — coverage collapse looks exactly like coverage.
fn collect_rs_files(dir: &Path, out: &mut Vec<std::path::PathBuf>) {
    let entries =
        std::fs::read_dir(dir).unwrap_or_else(|e| panic!("cannot walk {}: {e}", dir.display()));
    for entry in entries {
        let path = entry
            .unwrap_or_else(|e| panic!("cannot read an entry in {}: {e}", dir.display()))
            .path();
        if path.is_dir() {
            collect_rs_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
}

#[test]
fn nobody_reopened_the_gate_from_the_manifest() {
    // `[lints.clippy]` reaches every target in the package, so an entry here can
    // re-open a gate as effectively as an `#[allow]` — and from a file the
    // source scan above never reads.
    let manifest = std::fs::read_to_string(manifest_dir().join("Cargo.toml")).expect("read mani");

    assert_eq!(
        manifest_clippy_entry(&manifest, "undocumented_unsafe_blocks")
            .map(|(level, _)| level)
            .as_deref(),
        Some("deny"),
        "`Cargo.toml` no longer actively denies `undocumented_unsafe_blocks` under \
         `[lints.clippy]` — CLAUDE.md §Code style requires every `unsafe` block to \
         carry a justification, and `cargo clippy -D warnings` does not enable \
         that lint on its own"
    );

    let reopened = manifest_reopenings(&manifest);
    assert!(
        reopened.is_empty(),
        "these `[lints.clippy]` entries put a gated lint below `deny`:\n  {}",
        reopened.join("\n  ")
    );
}

/// Every way `manifest` puts a gated lint below `deny`, directly or by group.
///
/// Pure, so `the_manifest_reader_agrees_with_clippy` can drive it over forms
/// whose real effect has been measured.
fn manifest_reopenings(manifest: &str) -> Vec<String> {
    let mut reopened: Vec<String> = GATED_LINTS
        .iter()
        .filter_map(|lint| {
            let (level, _) = manifest_clippy_entry(manifest, lint)?;
            (!level_enforces(&level)).then(|| format!("{lint} = \"{level}\""))
        })
        .collect();

    // Groups need a priority comparison, not a priority threshold.
    // `undocumented_unsafe_blocks` is the one gate that lives only in the
    // manifest, so a group entry that outranks it switches it off with every
    // other check in this file still green. Cargo applies the higher-priority
    // entry last, so the group wins exactly when its priority is strictly
    // greater than the lint's — measured on clippy 1.95:
    //
    //   deny @ 0   vs allow @  1  -> gate OFF     deny @  0 vs allow @  0 -> holds
    //   deny @ -2  vs allow @ -1  -> gate OFF     deny @ -1 vs allow @ -2 -> holds
    //
    // Comparing the group against zero instead would miss the second row, where
    // both priorities are negative, and comparing it against nothing at all
    // would reject the legitimate manifests on the right.
    for group in GATED_GROUPS {
        let Some((group_level, group_priority)) = manifest_clippy_entry(manifest, group) else {
            continue;
        };
        if level_enforces(&group_level) {
            continue;
        }
        for lint in GATED_LINTS {
            let Some((lint_level, lint_priority)) = manifest_clippy_entry(manifest, lint) else {
                continue;
            };
            if level_enforces(&lint_level) && group_priority > lint_priority {
                reopened.push(format!(
                    "{group} = {{ level = \"{group_level}\", priority = {group_priority} }} \
                     outranks {lint} = {{ level = \"{lint_level}\", priority = {lint_priority} }}"
                ));
            }
        }
    }
    reopened
}

/// The `[lints.clippy]` table as this crate ships it.
const SHIPPED_LINTS: &str = "[lints.clippy]\nundocumented_unsafe_blocks = \"deny\"\n";

/// `true` when clippy still rejects an undocumented `unsafe` block in a scratch
/// crate carrying `lints` as its `[lints.clippy]` table.
///
/// This is the oracle. Every other assertion about the manifest reader is a
/// claim about TOML syntax written by hand, and two rounds of review on #408
/// found forms those claims had missed. Asking clippy directly cannot miss one.
fn unsafe_gate_holds(lints: &str) -> bool {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();
    std::fs::create_dir_all(root.join("src")).expect("src dir");
    std::fs::write(
        root.join("Cargo.toml"),
        format!(
            "[package]\nname = \"gate_probe\"\nversion = \"0.0.0\"\nedition = \"2024\"\n\n[workspace]\n\n{lints}"
        ),
    )
    .expect("write manifest");
    // An `unsafe` block with no `// SAFETY:` above it — the exact thing
    // `undocumented_unsafe_blocks` exists to reject. Never executed: only
    // clippy runs here, the binary is not.
    std::fs::write(
        root.join("src/main.rs"),
        "fn main() {\n    unsafe {\n        std::ptr::null::<u8>();\n    }\n}\n",
    )
    .expect("write main");

    let out = Command::new("cargo")
        .args(["clippy", "--offline", "--quiet"])
        .current_dir(root)
        .env("CARGO_TARGET_DIR", root.join("target"))
        .output()
        .expect("run cargo clippy");
    let diagnostics = format!(
        "{}{}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
    // Named, not merely non-zero: a scratch crate that failed to build for some
    // unrelated reason would otherwise read as an enforcing gate.
    let named = diagnostics.contains("undocumented_unsafe_blocks");
    // And rejected, not merely named: at `warn` the lint still fires and still
    // prints its name, but clippy exits 0 and the undocumented `unsafe` ships.
    // A gate that only warns is not a gate, so naming alone must not read as
    // one — `level_enforces` agrees, counting `deny`/`forbid` and nothing else.
    let rejected = !out.status.success();
    // And the converse trap, which this test walked into while being written: a
    // manifest cargo *rejects* names no lint either, so it would read as "gate
    // off" and quietly agree with whatever the reader said. A failure that is
    // not the lint firing means the probe broke, not that the gate is open.
    assert!(
        named || !rejected,
        "the probe crate failed for a reason other than the gate — the manifest \
         under test may not be valid TOML at all:\n{lints}\n{diagnostics}"
    );
    rejected && named
}

/// The reader's verdict must match what clippy actually does, form by form.
///
/// Hand-written fixtures encode what their author believed cargo accepts, and
/// on #408 that belief was wrong four times running — a single-line inline
/// table, a multiline one, a `+`-signed priority, and a lint promoted to its own
/// sub-table. Each was measured to switch the unsafe gate off while the reader
/// called the manifest clean. This test measures instead of believing: it runs
/// clippy over each form and fails when the reader disagrees with the result.
#[test]
fn the_manifest_reader_agrees_with_clippy() {
    if !clippy_available() {
        eprintln!("skipping: cargo clippy unavailable");
        return;
    }
    let deny = "undocumented_unsafe_blocks = \"deny\"";
    let cases = [
        (
            "the table as this crate ships it",
            SHIPPED_LINTS.to_string(),
        ),
        (
            "a single-line inline table outranking the deny",
            format!(
                "[lints.clippy]\n{deny}\nrestriction = {{ level = \"allow\", priority = 1 }}\n"
            ),
        ),
        (
            "a multiline inline table",
            format!(
                "[lints.clippy]\n{deny}\nrestriction = {{\n  level = \"allow\",\n  priority = 1,\n}}\n"
            ),
        ),
        (
            "an explicitly `+`-signed priority",
            format!(
                "[lints.clippy]\n{deny}\nrestriction = {{ level = \"allow\", priority = +1 }}\n"
            ),
        ),
        (
            "the group promoted to its own sub-table",
            format!(
                "[lints.clippy]\n{deny}\n\n[lints.clippy.restriction]\nlevel = \"allow\"\npriority = 1\n"
            ),
        ),
        (
            "a group that loses the priority comparison",
            format!(
                "[lints.clippy]\n{deny}\nrestriction = {{ level = \"allow\", priority = -1 }}\n"
            ),
        ),
        (
            "the lint downgraded outright",
            "[lints.clippy]\nundocumented_unsafe_blocks = \"allow\"\n".to_string(),
        ),
        (
            // Measured, not assumed: at `warn` the lint still fires and still
            // prints its own name, but clippy exits 0 and the undocumented
            // `unsafe` compiles. An oracle keyed on the name alone read that as
            // an enforcing gate. The reader has always counted `deny`/`forbid`
            // and nothing else, so this case is what holds the two to the same
            // meaning of "enforcing".
            "the lint downgraded to a warning",
            "[lints.clippy]\nundocumented_unsafe_blocks = \"warn\"\n".to_string(),
        ),
        (
            // Both dotted under one `[lints]` header. `[lints.clippy]` followed
            // by a second `[lints]` is a duplicate key and not valid TOML — a
            // fixture this test was first written with, and which the hardened
            // `unsafe_gate_holds` now refuses to read as an open gate.
            "the dotted keys cargo accepts under `[lints]`",
            "[lints]\nclippy.undocumented_unsafe_blocks = \"deny\"\nclippy.restriction = { level = \"allow\", priority = 1 }\n".to_string(),
        ),
    ];

    for (what, lints) in cases {
        let holds = unsafe_gate_holds(&lints);
        let reader_says_clean = manifest_reopenings(&lints).is_empty();
        assert_eq!(
            reader_says_clean,
            holds,
            "the manifest reader and clippy disagree about {what}. \
             clippy: gate {}. reader: manifest {}.\n{lints}",
            if holds { "HOLDS" } else { "is OFF" },
            if reader_says_clean {
                "is clean"
            } else {
                "re-opens the gate"
            }
        );
    }
}

/// Negative control for [`gate_reopeners`]. The scan above reads a `src/main.rs`
/// that is clean today, so it reports success both when the classifier works and
/// when it has stopped matching anything; this drives it over known answers.
#[test]
fn gate_reopener_classifier_matches_its_contract() {
    for source in [
        "#[allow(clippy::unwrap_used)]\nfn f() {}\n",
        "    #![allow(clippy::expect_used)]\n",
        "#[expect(clippy::unwrap_used)]\nfn f() {}\n",
        "#![allow(clippy::restriction)]\n",
        "#[cfg_attr(windows, allow(clippy::unwrap_used))]\nfn f() {}\n",
        "#[allow(clippy::undocumented_unsafe_blocks)]\nfn f() {}\n",
        // Anywhere, not just line-initial — clippy honours this form too.
        "fn f() { let w = { #[allow(clippy::unwrap_used)] g() }; }\n",
        // Whitespace between the name and its list is legal and honoured.
        "#[allow (clippy::unwrap_used)]\nfn f() {}\n",
        // The form Codex flagged on this PR (#408): rustfmt wraps a long
        // `reason` across lines, splitting `allow(` from the lint name. A
        // line-at-a-time classifier reads every line as clean while clippy
        // honours the attribute in full.
        "#[allow(\n    clippy::unwrap_used,\n    reason = \"a long justification that pushes rustfmt into wrapping this attribute\"\n)]\nfn f() {}\n",
        "#[cfg_attr(\n    not(test),\n    allow(clippy::expect_used)\n)]\nfn f() {}\n",
        // A module-level `warn` overrides the crate-root `deny`, leaving only
        // CI's `-D warnings` between the `unwrap()` and a green build. Measured
        // against the pinned 1.95.0: with this in a `src/helper.rs`, clippy
        // demotes `unwrap_used` to a warning, and `cargo clippy` on its own
        // exits 0.
        "#![warn(clippy::unwrap_used)]\n\npub fn f(s: &str) -> i32 { s.parse().unwrap() }\n",
        "#[cfg_attr(windows, warn(clippy::expect_used))]\nfn f() {}\n",
        "#![warn(clippy::restriction)]\n",
    ] {
        assert!(
            !gate_reopeners(source).is_empty(),
            "should have been flagged:\n{source}"
        );
    }

    for source in [
        "#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]\n",
        "#[allow(dead_code)]\nfn f() {}\n",
        // `warn` counts only for a gated lint. Raising an ungated one is
        // ordinary code, not a gate being taken apart.
        "#[warn(dead_code)]\nfn f() {}\n",
        // Nor is tightening a gated lint further.
        "#[deny(clippy::unwrap_used)]\nfn f() {}\n",
        "// an #[allow(clippy::unwrap_used)] mentioned in prose, not an attribute\n",
        "/* #[allow(clippy::unwrap_used)] in a block comment */\n",
        "let s = \"#[allow(clippy::unwrap_used)]\";\n",
        // Naming the lint in a `reason` string is not setting its level.
        "#[expect(dead_code, reason = \"unlike clippy::unwrap_used, harmless\")]\nfn f() {}\n",
        "let unwrap_used = 1;\n",
        // Another tool's lint of the same bare name is not clippy's.
        "#[allow(othertool::unwrap_used)]\nfn f() {}\n",
        // An unbalanced bracket in a comment must not swallow what follows.
        "// TODO(#412): first element is items[0\nfn f() {}\n",
    ] {
        assert!(
            gate_reopeners(source).is_empty(),
            "should not have been flagged:\n{source}"
        );
    }

    // The reported line is the attribute's own, so a maintainer can go to it.
    let hits = gate_reopeners("fn a() {}\n\n#[allow(clippy::unwrap_used)]\nfn b() {}\n");
    assert_eq!(hits.len(), 1);
    assert_eq!(hits[0].line, 3);
}

/// Negative control for [`manifest_clippy_entry`]. Same reasoning: the test
/// above reads a `Cargo.toml` that is correct today.
#[test]
fn manifest_lint_reader_matches_its_contract() {
    let cases = [
        (
            "the gate as it actually ships",
            "[lints.clippy]\nundocumented_unsafe_blocks = \"deny\"\n",
            Some(("deny", 0)),
        ),
        (
            "commented out — the gate is off, however the line reads",
            "[lints.clippy]\n# undocumented_unsafe_blocks = \"deny\"\n",
            None,
        ),
        (
            "downgraded",
            "[lints.clippy]\nundocumented_unsafe_blocks = \"allow\"\n",
            Some(("allow", 0)),
        ),
        (
            "single quotes, which are valid TOML",
            "[lints.clippy]\nundocumented_unsafe_blocks = 'allow'\n",
            Some(("allow", 0)),
        ),
        (
            "the inline-table form cargo also accepts",
            "[lints.clippy]\nundocumented_unsafe_blocks = { level = \"warn\", priority = 1 }\n",
            Some(("warn", 1)),
        ),
        (
            // Every row below was measured against the real gate before being
            // written down — see `cargo_accepts_the_forms_this_reader_claims`.
            "the multiline inline table (#408, round 2)",
            "[lints.clippy]\nundocumented_unsafe_blocks = {\n  level = \"allow\",\n  priority = 1,\n}\n",
            Some(("allow", 1)),
        ),
        (
            "an explicitly `+`-signed priority (#408, round 2)",
            "[lints.clippy]\nundocumented_unsafe_blocks = { level = \"allow\", priority = +1 }\n",
            Some(("allow", 1)),
        ),
        (
            "the lint promoted to its own sub-table",
            "[lints.clippy.undocumented_unsafe_blocks]\nlevel = \"allow\"\npriority = 1\n",
            Some(("allow", 1)),
        ),
        (
            "a negative priority, which loses to the specific lint",
            "[lints.clippy]\nundocumented_unsafe_blocks = { level = \"allow\", priority = -1 }\n",
            Some(("allow", -1)),
        ),
        (
            "the dotted key cargo accepts under `[lints]`",
            "[lints]\nclippy.undocumented_unsafe_blocks = \"allow\"\n",
            Some(("allow", 0)),
        ),
        (
            "a different tool's table cannot answer for clippy's",
            "[lints.rust]\nundocumented_unsafe_blocks = \"deny\"\n",
            None,
        ),
        (
            "the entry moved out from under the table header",
            "[lints.clippy]\nother = \"deny\"\n\n[profile.release]\nundocumented_unsafe_blocks = \"deny\"\n",
            None,
        ),
        (
            "a longer lint name must not answer for this one",
            "[lints.clippy]\nundocumented_unsafe_blocks_extra = \"deny\"\n",
            None,
        ),
        ("absent entirely", "[lints.clippy]\n", None),
    ];
    for (what, manifest, expected) in cases {
        let expected = expected.map(|(level, priority)| (level.to_string(), priority));
        assert_eq!(
            manifest_clippy_entry(manifest, "undocumented_unsafe_blocks"),
            expected,
            "misread the manifest with {what}:\n{manifest}"
        );
    }

    // The group form the priority comparison exists for, including the negative
    // pair a threshold-against-zero test reads as harmless.
    for (manifest, expected) in [
        (
            "[lints.clippy]\nrestriction = { level = \"allow\", priority = 1 }\n",
            Some(("allow".to_string(), 1)),
        ),
        (
            "[lints.clippy]\nrestriction = { level = \"allow\", priority = -1 }\n",
            Some(("allow".to_string(), -1)),
        ),
    ] {
        assert_eq!(manifest_clippy_entry(manifest, "restriction"), expected);
    }
}

/// This crate's root directory.
fn manifest_dir() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

/// The lints this crate's gates rest on. All three live in clippy's
/// `restriction` group, which is why `restriction` appears in [`GATED_GROUPS`]
/// and the other groups do not.
///
/// Bare, not `clippy::`-qualified, because both lists are matched two ways —
/// against qualified attribute text in [`gate_reopeners`], and against the
/// unqualified keys a `[lints.clippy]` table uses.
const GATED_LINTS: [&str; 3] = ["unwrap_used", "expect_used", "undocumented_unsafe_blocks"];

/// Lint *groups* whose allow-level suppresses a gated lint. `clippy::all`,
/// `pedantic`, `nursery` and `correctness` do not contain these lints, so
/// listing them would make this reject code that is in fact gated.
const GATED_GROUPS: [&str; 1] = ["restriction"];

/// A lint-level attribute that would re-open one of the gates.
#[derive(Debug, PartialEq, Eq)]
struct Reopener {
    /// 1-indexed line the attribute starts on.
    line: usize,
    /// The attribute, whitespace-collapsed onto one line.
    text: String,
}

/// Report every `allow`/`expect` attribute in `source` that re-opens a gate.
///
/// Pure, so [`gate_reopener_classifier_matches_its_contract`] can drive it with
/// synthetic sources — the scan over the real `src/main.rs` has nothing to say
/// about whether the classifier still matches anything at all.
///
/// Attributes are matched as *complete bracketed spans*, not line by line.
/// Codex flagged the line-at-a-time version on this PR (#408) and was right: an
/// `#[allow(…)]` carrying a long `reason` gets wrapped by rustfmt so that
/// `allow(` and `clippy::unwrap_used` land on different lines, and a predicate
/// requiring both on one line then reports clean while clippy honours the
/// attribute in full. Bracket matching over
/// [`blank_comments_and_strings`] also catches attributes that are not
/// line-initial (`let w = { #[allow(clippy::unwrap_used)] v.unwrap() };`), and
/// [`opens_a_lint`] finds the level by position, so one nested in a `cfg_attr`
/// predicate counts too.
///
/// The rule is deliberately absolute: no allow/expect of a gated lint (or of
/// `clippy::restriction`) anywhere in the file, test module included. Inside
/// `#[cfg(test)]` the `cfg_attr(not(test), …)` gate is not in force, so such an
/// attribute is redundant — delete it. `undocumented_unsafe_blocks` is denied
/// from `Cargo.toml` and so applies to every target; the fix for that one is a
/// `// SAFETY:` comment, never a deletion.
fn gate_reopeners(source: &str) -> Vec<Reopener> {
    let chars: Vec<char> = source.chars().collect();
    let code = blank_comments_and_strings(&chars);
    let mut found = Vec::new();
    let mut i = 0;
    while i < code.len() {
        // An attribute opens with `#[` or `#![` and nothing else.
        let open = match (code.get(i), code.get(i + 1), code.get(i + 2)) {
            (Some('#'), Some('['), _) => i + 1,
            (Some('#'), Some('!'), Some('[')) => i + 2,
            _ => {
                i += 1;
                continue;
            }
        };

        let mut depth = 0usize;
        let mut end = None;
        for (k, c) in code.iter().enumerate().skip(open) {
            match c {
                '[' => depth += 1,
                ']' => {
                    depth -= 1;
                    if depth == 0 {
                        end = Some(k);
                        break;
                    }
                }
                _ => {}
            }
        }
        let Some(end) = end else {
            // Unterminated — not an attribute this can reason about. Step past
            // the `#` only, so nothing after it is swallowed.
            i += 1;
            continue;
        };

        let collapsed = collapse(&code[i..=end]);
        // Qualified, so `#[allow(othertool::unwrap_used)]` is another tool's
        // business rather than a finding here.
        let names_gated = GATED_LINTS
            .iter()
            .chain(GATED_GROUPS.iter())
            .any(|name| collapsed.contains(&format!("clippy::{name}")));
        if opens_a_lint(&collapsed) && names_gated {
            found.push(Reopener {
                line: code[..i].iter().filter(|c| **c == '\n').count() + 1,
                // Reported from the original, so a maintainer sees the text as
                // written rather than with its strings blanked out.
                text: collapse(&chars[i..=end]),
            });
        }
        i = end + 1;
    }
    found
}

/// `true` when `collapsed` sets an `allow`, `expect` or `warn` lint level
/// *anywhere* inside it, including nested in a `cfg_attr` payload.
///
/// Nesting is the point. A prefix test (`starts_with("#[allow(")`) misses
/// `#[cfg_attr(not(test), allow(clippy::unwrap_used))]`, which clippy honours in
/// full — and that form is the exact mirror of this crate's own gate, so it is
/// the first thing someone reaching for a target-conditional override would
/// write.
///
/// `warn` counts alongside `allow` and `expect`. It reads like a lesser thing
/// and is not: a module-level `#![warn(clippy::unwrap_used)]` overrides the
/// crate-root `deny`, after which the only thing still rejecting the `unwrap()`
/// is the `-D warnings` in CI's clippy invocation — measured, and it does still
/// reject today. That is the point. A `deny` in the source is enforced by the
/// source; a `warn` is enforced by a flag in a workflow file, so downgrading one
/// to the other moves the gate somewhere this test cannot see and makes the
/// `deny` it is guarding decorative. [`level_enforces`] already draws the line
/// in the same place for the manifest, counting `deny`/`forbid` and nothing
/// else; this keeps the two readers saying the same thing about the same word.
///
/// A lint level is recognised by its position rather than by enumerating
/// wrappers: with whitespace removed, `allow(`/`expect(`/`warn(` counts only
/// when the character before it opens a list — `[`, `(` or `,`. That admits
/// `#[allow(`, `#![allow(` and any `cfg_attr(<pred>, allow(` depth, while the
/// crate's own `deny(…)` gate is left alone. Whitespace is stripped rather than
/// trusted, so `#[allow (clippy::unwrap_used)]` — which clippy honours — still
/// counts.
fn opens_a_lint(collapsed: &str) -> bool {
    let dense: String = collapsed.chars().filter(|c| !c.is_whitespace()).collect();
    ["allow(", "expect(", "warn("].iter().any(|level| {
        dense.match_indices(level).any(|(at, _)| {
            // A level at index 0 has no opening bracket before it, so it is not
            // an attribute at all.
            matches!(dense[..at].chars().next_back(), Some('[' | '(' | ','))
        })
    })
}

/// Collapse a span onto one whitespace-normalized line.
fn collapse(span: &[char]) -> String {
    span.iter()
        .collect::<String>()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Blank out comment and string-literal *contents*, preserving every character
/// position and newline so offsets still map to lines.
///
/// This is what makes bracket matching trustworthy, and it is not a nicety. A
/// scanner that counts `[` and `]` over raw text is thrown by one unbalanced
/// bracket inside a comment — `// TODO(#412): first element is items[0` — and
/// then runs on until the brackets happen to rebalance, swallowing every
/// attribute in between.
///
/// Blanking string contents earns its keep twice more: it removes the
/// false-positive class where a comment or an `#[expect(…, reason = "…")]`
/// merely *names* a gated lint, and it disarms `#[doc = "…[…"]`.
fn blank_comments_and_strings(chars: &[char]) -> Vec<char> {
    let mut out = chars.to_vec();
    let mut i = 0;
    let at = |k: usize| chars.get(k).copied();
    // Blank `chars[k]` unless it is the newline that keeps line numbers aligned.
    let blank = |out: &mut Vec<char>, k: usize| {
        if chars.get(k).is_some_and(|c| *c != '\n')
            && let Some(slot) = out.get_mut(k)
        {
            *slot = ' ';
        }
    };
    while i < chars.len() {
        match (chars[i], at(i + 1)) {
            ('/', Some('/')) => {
                while i < chars.len() && chars[i] != '\n' {
                    blank(&mut out, i);
                    i += 1;
                }
            }
            ('/', Some('*')) => {
                let mut depth = 0usize;
                while i < chars.len() {
                    if chars[i] == '/' && at(i + 1) == Some('*') {
                        depth += 1;
                        blank(&mut out, i);
                        blank(&mut out, i + 1);
                        i += 2;
                        continue;
                    }
                    if chars[i] == '*' && at(i + 1) == Some('/') {
                        depth -= 1;
                        blank(&mut out, i);
                        blank(&mut out, i + 1);
                        i += 2;
                        if depth == 0 {
                            break;
                        }
                        continue;
                    }
                    blank(&mut out, i);
                    i += 1;
                }
            }
            // Raw string: `r`, then any number of `#`, then `"`. Anything else
            // beginning with `r` is an ordinary identifier.
            ('r', Some('"' | '#')) => {
                let mut hashes = 0;
                let mut j = i + 1;
                while at(j) == Some('#') {
                    hashes += 1;
                    j += 1;
                }
                if at(j) != Some('"') {
                    i += 1;
                    continue;
                }
                j += 1;
                while j < chars.len() {
                    if chars[j] == '"' {
                        let closed = (1..=hashes).all(|n| at(j + n) == Some('#'));
                        if closed {
                            j += hashes + 1;
                            break;
                        }
                    }
                    blank(&mut out, j);
                    j += 1;
                }
                i = j;
            }
            ('"', _) => {
                let mut j = i + 1;
                while j < chars.len() && chars[j] != '"' {
                    // Skip the character an escape protects, so `\"` does not
                    // look like the closing quote.
                    let step = usize::from(chars[j] == '\\') + 1;
                    for k in j..j + step {
                        blank(&mut out, k);
                    }
                    j += step;
                }
                i = j + 1;
            }
            // A char literal, not a lifetime: `'x'`, `'\n'`. A lifetime (`'a`)
            // has no closing quote and must be left alone.
            ('\'', _) => {
                let width = if at(i + 1) == Some('\\') { 3 } else { 2 };
                if at(i + width) == Some('\'') {
                    for k in i + 1..i + width {
                        blank(&mut out, k);
                    }
                    i += width + 1;
                } else {
                    i += 1;
                }
            }
            _ => i += 1,
        }
    }
    out
}

/// The level and priority a manifest `[lints.clippy]` table assigns to `name`,
/// or `None` when it assigns none. Priority defaults to 0, as cargo does.
///
/// Reads *active* entries only. A `contains` over the whole file cannot tell
/// `undocumented_unsafe_blocks = "deny"` from the same line commented out, so
/// the gate could be switched off by prefixing one `#` and the anchor would stay
/// green — the precise failure it exists to prevent. Tracking the table header
/// also keeps a same-named `[lints.rust]` entry from answering for the clippy
/// one.
///
/// Parsed as TOML rather than scanned, and that is the whole point. A
/// line-at-a-time reader was tried first and shipped three separate holes,
/// each of which cargo 1.95.0 accepts and each of which was *measured* to
/// switch the unsafe gate off with this test still green:
///
/// | form | what the scanner did |
/// |---|---|
/// | `restriction = { level = "allow", priority = 1 }` | compared the whole inline table against `"allow"`, matched nothing |
/// | `restriction = {`⏎`  level = "allow",`⏎`  priority = 1,`⏎`}` | read only the first line, recorded an empty level at priority 0 |
/// | `priority = +1` | a digits-and-`-` filter rejected `+`, so the parse failed and defaulted to 0 |
///
/// A fourth, `[lints.clippy.restriction]` as its own sub-table, was found while
/// checking the first three. That is four for four against a hand-rolled
/// reader, so this reads the manifest the way cargo does instead of guessing at
/// its syntax — `+1`, multiline tables, sub-tables and dotted keys all come out
/// right because the parser, not this function, decides what they mean.
///
/// One thing the parse gives for free that the scanner needed a special case
/// for: a commented-out `undocumented_unsafe_blocks = "deny"` is not data, so
/// it cannot answer for the live entry. A `contains` over the file could not
/// tell the two apart, and the gate would have been switchable off by prefixing
/// one `#`.
///
/// Priority is read because it decides whether a *group* entry wins. Measured
/// against the real gate: with `undocumented_unsafe_blocks = "deny"` present,
/// `restriction = { level = "allow", priority = 1 }` turns the unsafe gate off,
/// while the same entry at `priority = -1` leaves it enforcing. Flagging the
/// second would reject a legitimate manifest.
fn manifest_clippy_entry(manifest: &str, name: &str) -> Option<(String, i64)> {
    // `Table`, not `Value`: in toml 1.x `Value`'s `FromStr` parses a single TOML
    // *value*, so a whole document fails with "unexpected content" at the first
    // table header.
    let parsed: toml::Table = manifest.parse().expect("manifest is not valid TOML");
    // `[lints.clippy]`, `[lints] clippy.<name> = …` and `[lints.clippy.<name>]`
    // are three spellings of one path, and the parser has already reconciled
    // them — so navigating the path covers all three with no cases here.
    let entry = parsed.get("lints")?.get("clippy")?.get(name)?;
    match entry {
        // `name = "level"`. Priority defaults to 0, as cargo does.
        toml::Value::String(level) => Some((level.clone(), 0)),
        // `name = { level = "…", priority = N }`, however it is laid out.
        toml::Value::Table(table) => {
            let level = table.get("level")?.as_str()?.to_string();
            let priority = table.get("priority").and_then(toml::Value::as_integer);
            Some((level, priority.unwrap_or(0)))
        }
        _ => None,
    }
}

/// `true` when `level` still enforces.
fn level_enforces(level: &str) -> bool {
    level == "deny" || level == "forbid"
}
