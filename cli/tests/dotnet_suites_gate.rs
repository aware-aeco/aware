//! Gate: every .NET test project in the repo is actually run by `ci.yml`.
//!
//! The repo carries six xunit suites. Exactly one of them — `cli-tekla/Tests` —
//! was wired into a workflow, and its presence made the .NET side look covered.
//! The other five (`cli-revit`, `cli-rhino`, `cli-roslyn`, `cli-sidecar`,
//! `cli-sketchup` — 483 tests between them) were run by nothing at all:
//! `ci.yml` did not name them, `release.yml` only `dotnet publish`es the
//! bridges it ships, and `build-sidecar.yml` builds one binary. Their
//! green-ness rested entirely on whoever last remembered to run them locally —
//! the same hole `cli-connection-reader` (#343) and `steel-detailer-lookup` had,
//! each of which had already shipped a real defect through it.
//!
//! It was not hypothetical here either: the v0.125.0 scene-contract change
//! (5255a3bb) edited `cli-rhino/Tests/CrossSectionProfileTests.cs` and shipped
//! with nothing having executed that file.
//!
//! CLAUDE.md §Engineering rules — "Verify before answering", "No corner-cutting"
//! — is what a suite nobody runs defeats: a green PR check is the claim that the
//! tests passed, and for five of six projects that claim was never evaluated.
//!
//! Wiring the jobs fixes today. This fixes tomorrow: a *seventh* suite added
//! later would arrive unrun, and nothing would say so. So rather than asserting
//! the five names, this discovers every `*.Tests.csproj` in the tree and asserts
//! the workflow names each one.
//!
//! Two failure modes are deliberately guarded against:
//!   * a vacuous pass — if the discovery walk stops matching (renamed convention,
//!     a bad skip-list entry) it would find zero projects and report clean.
//!     [`the_walk_still_finds_every_known_suite`] pins the six that exist.
//!   * a classifier that has stopped classifying — the check below runs over a
//!     tree that is correct today, so it reports clean both when it works and
//!     when it is broken. [`unrun_classifier_matches_its_contract`] is its
//!     negative control, driving the same functions over synthetic input where
//!     a suite IS missing from the workflow, and asserting they say so.

use std::path::{Path, PathBuf};

/// Repository root — `cli/`'s parent.
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap_or_else(|| panic!("{} has no parent", env!("CARGO_MANIFEST_DIR")))
        .to_path_buf()
}

/// Directories never worth descending into: build output and vendored deps.
///
/// `obj/` in particular holds generated `*.props` / `*.targets` next to restored
/// package graphs, and `target/` and `node_modules/` are large enough that
/// walking them turns a millisecond check into a slow one.
const SKIP_DIRS: [&str; 5] = ["bin", "obj", "target", "node_modules", ".git"];

/// Every `*.Tests.csproj` under `root`, as repo-relative `/`-separated paths.
///
/// The `*.Tests.csproj` suffix is the repo's own convention and is what makes
/// this discriminate: `cli-sidecar/Tests/FixtureAssembly/FixtureAssembly.csproj`
/// and its `FixtureDataAssembly` sibling live *inside* a `Tests/` directory but
/// are fixture inputs compiled by the suite, not suites themselves. Matching on
/// the directory name would demand a CI step for each and there is nothing to
/// run. Matching the file suffix names exactly the six projects that carry
/// `Microsoft.NET.Test.Sdk`.
fn test_projects(root: &Path) -> Vec<String> {
    let mut found = Vec::new();
    collect(root, root, &mut found);
    found.sort();
    found
}

fn collect(root: &Path, dir: &Path, out: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if path.is_dir() {
            if !SKIP_DIRS.contains(&name.as_ref()) {
                collect(root, &path, out);
            }
        } else if name.ends_with(".Tests.csproj")
            && let Ok(rel) = path.strip_prefix(root)
        {
            // Forward slashes regardless of host: the workflow spells these
            // paths that way, and this test also runs on Windows.
            out.push(rel.to_string_lossy().replace('\\', "/"));
        }
    }
}

/// Those of `projects` that `workflow` does not mention.
///
/// A substring match on the path, which is what a `dotnet test <path>` step
/// contains verbatim. Deliberately not a YAML parse: the point is "some step in
/// this file hands this project to a runner", and every way of spelling that —
/// a `run:` one-liner, a multi-line `|` block, a matrix entry — puts the path in
/// the file's text. A parse would have to model all three and would grow a hole
/// at the first one it missed.
fn unrun<'a>(projects: &'a [String], workflow: &str) -> Vec<&'a str> {
    projects
        .iter()
        .map(String::as_str)
        .filter(|project| !workflow.contains(*project))
        .collect()
}

fn ci_workflow() -> String {
    let path = repo_root().join(".github/workflows/ci.yml");
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// The suites that exist today. If this list is wrong, the gate is measuring
/// nothing — so it is asserted rather than assumed.
const KNOWN_SUITES: [&str; 6] = [
    "cli-revit/Tests/cli-revit.Tests.csproj",
    "cli-rhino/Tests/cli-rhino.Tests.csproj",
    "cli-roslyn/Tests/aware-roslyn.Tests.csproj",
    "cli-sidecar/Tests/cli-sidecar.Tests.csproj",
    "cli-sketchup/Tests/cli-sketchup.Tests.csproj",
    "cli-tekla/Tests/AwareTekla.Tests.csproj",
];

#[test]
fn the_walk_still_finds_every_known_suite() {
    // Without this, a discovery walk that silently stopped matching would make
    // `every_dotnet_suite_is_run_by_ci` pass over an empty list.
    let found = test_projects(&repo_root());
    for suite in KNOWN_SUITES {
        assert!(
            found.iter().any(|f| f == suite),
            "the discovery walk no longer finds `{suite}`, so the gate below is \
             checking less than it looks like. Found: {found:#?}"
        );
    }
}

#[test]
fn every_dotnet_suite_is_run_by_ci() {
    let projects = test_projects(&repo_root());
    let missing = unrun(&projects, &ci_workflow());
    assert!(
        missing.is_empty(),
        "these .NET test projects exist but no job in `.github/workflows/ci.yml` \
         runs them, so their tests are green only by assumption:\n  {}\n\n\
         Add a `dotnet test <path>` step to the `dotnet-bridges` job (plain \
         `net*` target frameworks) or `dotnet-bridges-windows` (`net*-windows` \
         ones). Do not satisfy this by deleting the suite or renaming it out of \
         the `*.Tests.csproj` convention — CLAUDE.md §Engineering rules forbids \
         satisfying a gate by disabling it.",
        missing.join("\n  ")
    );
}

#[test]
fn unrun_classifier_matches_its_contract() {
    // The two tests above read a tree and a workflow that are correct today, so
    // both report clean whether the classifier works or has stopped matching
    // anything. This drives the same two functions over synthetic input in which
    // a suite IS unrun, and asserts they say so.
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    // Three `.csproj` files: two real suites, one fixture assembly that happens
    // to sit under a `Tests/` directory. Plus one inside `obj/`, which the walk
    // must not descend into.
    for rel in [
        "wired/Tests/wired.Tests.csproj",
        "orphan/Tests/orphan.Tests.csproj",
        "wired/Tests/FixtureAssembly/FixtureAssembly.csproj",
        "wired/obj/stale.Tests.csproj",
    ] {
        let path = root.join(rel);
        std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
        std::fs::write(&path, "<Project />").expect("write csproj");
    }

    let found = test_projects(root);
    assert_eq!(
        found,
        vec![
            "orphan/Tests/orphan.Tests.csproj".to_string(),
            "wired/Tests/wired.Tests.csproj".to_string(),
        ],
        "the walk must find `*.Tests.csproj` only, and must not descend into \
         `obj/`: a fixture assembly under `Tests/` is not a suite, and a stale \
         copy in build output is not a source file"
    );

    // One of the two is named by the workflow; the other is not.
    let workflow = "jobs:\n  x:\n    steps:\n      - run: dotnet test wired/Tests/wired.Tests.csproj -c Release\n";
    assert_eq!(
        unrun(&found, workflow),
        vec!["orphan/Tests/orphan.Tests.csproj"],
        "the classifier must flag a suite the workflow does not name — this is \
         the exact condition `every_dotnet_suite_is_run_by_ci` exists to catch, \
         and if it does not fire here that test cannot fire either"
    );

    // Positive control: with both named, nothing is reported. Without this, a
    // classifier that flagged *everything* would satisfy the assertion above.
    let complete =
        format!("{workflow}      - run: dotnet test orphan/Tests/orphan.Tests.csproj -c Release\n");
    assert!(
        unrun(&found, &complete).is_empty(),
        "the classifier reported a suite the workflow does name, so it is \
         flagging indiscriminately rather than classifying"
    );
}
