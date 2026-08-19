//! Gate: every .NET test project in the repo is actually run by `ci.yml`.
//!
//! The repo carries seven xunit suites. Exactly one of them — `cli-tekla/Tests`
//! — was wired into a workflow, and its presence made the .NET side look
//! covered. The other six (`cli-revit`, `cli-rhino`, `cli-roslyn`,
//! `cli-sidecar`, `cli-sidecar/Ingest/Generator` and `cli-sketchup` — 529 tests
//! between them) were run by nothing at all: `ci.yml` did not name them,
//! `release.yml` only `dotnet publish`es the bridges it ships, and
//! `build-sidecar.yml` builds one binary. Their green-ness rested entirely on
//! whoever last remembered to run them locally — the same hole
//! `cli-connection-reader` (#343) and `steel-detailer-lookup` had, each of which
//! had already shipped a real defect through it.
//!
//! It was not hypothetical here either: the v0.125.0 scene-contract change
//! (5255a3bb) edited `cli-rhino/Tests/CrossSectionProfileTests.cs` and shipped
//! with nothing having executed that file.
//!
//! CLAUDE.md §Engineering rules — "Verify before answering", "No corner-cutting"
//! — is what a suite nobody runs defeats: a green PR check is the claim that the
//! tests passed, and for six of seven projects that claim was never evaluated.
//!
//! Wiring the jobs fixes today. This fixes tomorrow: an *eighth* suite added
//! later would arrive unrun, and nothing would say so. So rather than asserting
//! the seven names, this discovers the test projects in the tree and asserts the
//! workflow runs each one.
//!
//! ## Two things this gate must not get wrong, both found by review
//!
//! **What counts as a test project.** The first version matched the filename
//! suffix `*.Tests.csproj`. That is a naming convention, not a fact about the
//! project, and it already missed one: `cli-sidecar/Ingest/Generator/Tests/
//! Tests.csproj` is a 46-test xunit suite whose filename is plain `Tests.csproj`.
//! It was unrun, and a suffix-matching gate reported the repo clean anyway —
//! the exact failure the gate exists to prevent, reintroduced by the gate.
//! [`is_test_project`] therefore reads the project's own metadata: a
//! `Microsoft.NET.Test.Sdk` package reference, or `<IsTestProject>true`. That
//! discriminates exactly, and for a reason rather than by luck — `Test.Sdk` is
//! what makes `dotnet test` able to run a project at all, so a project without
//! it has nothing to run and a project with it does. It also still excludes
//! `cli-sidecar/Tests/FixtureAssembly` and its `FixtureDataAssembly` sibling,
//! which sit inside a `Tests/` directory but are fixture inputs compiled *by* a
//! suite.
//!
//! **What counts as being run.** The first version searched the whole text of
//! `ci.yml` for the project path. `ci.yml` is roughly half prose — every gate in
//! it carries a paragraph explaining why it exists — so a `dotnet test` step
//! could be deleted while a comment above it still named the path, and the gate
//! would report the suite covered with no runner executing it. [`run_commands`]
//! parses the YAML and collects only `jobs.*.steps[].run` scalars, so a mention
//! in a comment, a `name:`, or a `with:` block does not count. Parsing also
//! costs nothing here: `serde_yaml` is already a dependency of this crate.

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

/// Whether a `.csproj`'s own text marks it as something `dotnet test` can run.
///
/// Either signal alone is sufficient and both are load-bearing:
///   * `Microsoft.NET.Test.Sdk` is the package that supplies the test host. A
///     project without it cannot be run by `dotnet test`; every one of the
///     repo's seven suites carries it and none of its eleven non-test projects
///     does.
///   * `<IsTestProject>true</IsTestProject>` is the MSBuild property that says
///     so explicitly. Redundant against the above today, kept because a project
///     can set it while getting the test host transitively from a shared
///     `Directory.Build.props` — at which point the package reference is not in
///     the file and only this signal is.
fn is_test_project(source: &str) -> bool {
    source.contains("Microsoft.NET.Test.Sdk") || source.contains("<IsTestProject>true")
}

/// Every runnable test project under `root`, as repo-relative `/`-separated
/// paths.
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
        } else if name.ends_with(".csproj")
            && std::fs::read_to_string(&path).is_ok_and(|s| is_test_project(&s))
            && let Ok(rel) = path.strip_prefix(root)
        {
            // Forward slashes regardless of host: the workflow spells these
            // paths that way, and this test also runs on Windows.
            out.push(rel.to_string_lossy().replace('\\', "/"));
        }
    }
}

/// Every `run:` script in `workflow`, concatenated.
///
/// Only `jobs.<job>.steps[].run` — the shell a runner actually executes.
/// Comments are gone by construction (the parser drops them), and `name:`,
/// `uses:` and `with:` values are never collected, so naming a project path in
/// prose or in a step's title does not make it run.
fn run_commands(workflow: &str) -> String {
    let doc: serde_yaml::Value =
        serde_yaml::from_str(workflow).unwrap_or_else(|e| panic!("parse workflow yaml: {e}"));
    let mut out = String::new();
    let Some(jobs) = doc.get("jobs").and_then(|j| j.as_mapping()) else {
        return out;
    };
    for (_, job) in jobs {
        let Some(steps) = job.get("steps").and_then(|s| s.as_sequence()) else {
            continue;
        };
        for step in steps {
            if let Some(run) = step.get("run").and_then(|r| r.as_str()) {
                out.push_str(run);
                out.push('\n');
            }
        }
    }
    out
}

/// Those of `projects` that no `run:` script in `commands` mentions.
///
/// A substring match, which is what a `dotnet test <path>` invocation contains
/// verbatim. Now that the haystack is only executable shell, a match means a
/// runner is handed the path — which is the property being asserted. Matching
/// the path rather than modelling `dotnet test`'s argument grammar keeps every
/// spelling working: a one-line `run:`, a `|` block, a loop over paths.
fn unrun<'a>(projects: &'a [String], commands: &str) -> Vec<&'a str> {
    projects
        .iter()
        .map(String::as_str)
        .filter(|project| !commands.contains(*project))
        .collect()
}

fn ci_workflow() -> String {
    let path = repo_root().join(".github/workflows/ci.yml");
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// The suites that exist today. If this list is wrong, the gate is measuring
/// nothing — so it is asserted rather than assumed.
const KNOWN_SUITES: [&str; 7] = [
    "cli-revit/Tests/cli-revit.Tests.csproj",
    "cli-rhino/Tests/cli-rhino.Tests.csproj",
    "cli-roslyn/Tests/aware-roslyn.Tests.csproj",
    "cli-sidecar/Ingest/Generator/Tests/Tests.csproj",
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
fn the_walk_does_not_mistake_fixture_projects_for_suites() {
    // The other half of the discovery contract. `the_walk_still_finds_every_
    // known_suite` would be satisfied by a classifier that returned every
    // `.csproj` in the repo; this pins the eleven it must leave out, using the
    // two that are most easily confused for suites — they live under `Tests/`.
    let found = test_projects(&repo_root());
    for not_a_suite in [
        "cli-sidecar/Tests/FixtureAssembly/FixtureAssembly.csproj",
        "cli-sidecar/Tests/FixtureDataAssembly/FixtureDataAssembly.csproj",
    ] {
        assert!(
            !found.iter().any(|f| f == not_a_suite),
            "`{not_a_suite}` is a fixture input compiled by a suite, not a suite \
             — demanding a CI step for it would mean adding a step that runs \
             nothing. Found: {found:#?}"
        );
    }
    assert_eq!(
        found.len(),
        KNOWN_SUITES.len(),
        "the walk found a different number of suites than the {} known ones. If a \
         suite was added, wire it into `ci.yml` and add it to `KNOWN_SUITES`; if \
         the classifier has started over-matching, fix `is_test_project`. \
         Found: {found:#?}",
        KNOWN_SUITES.len()
    );
}

#[test]
fn every_dotnet_suite_is_run_by_ci() {
    let projects = test_projects(&repo_root());
    let missing = unrun(&projects, &run_commands(&ci_workflow()));
    assert!(
        missing.is_empty(),
        "these .NET test projects exist but no `run:` step in \
         `.github/workflows/ci.yml` runs them, so their tests are green only by \
         assumption:\n  {}\n\n\
         Add a `dotnet test <path>` step to the `dotnet-bridges` job (plain \
         `net*` target frameworks) or `dotnet-bridges-windows` (`net*-windows` \
         ones). Naming the path in a comment does not count and is not meant to \
         — that was the hole this gate was rewritten to close. Do not satisfy \
         this by deleting the suite or stripping its test-sdk reference — \
         CLAUDE.md §Engineering rules forbids satisfying a gate by disabling it.",
        missing.join("\n  ")
    );
}

#[test]
fn test_project_classifier_matches_its_contract() {
    // `is_test_project` is driven over the real tree by the tests above, where
    // it is correct today — so it would report clean whether it works or has
    // stopped matching. These are the cases it must separate, stated directly.
    for (label, source) in [
        (
            "a package reference to the test host",
            r#"<Project Sdk="Microsoft.NET.Sdk"><ItemGroup>
               <PackageReference Include="Microsoft.NET.Test.Sdk" Version="17.11.1" />
               </ItemGroup></Project>"#,
        ),
        (
            "an explicit IsTestProject property",
            "<Project><PropertyGroup><IsTestProject>true</IsTestProject></PropertyGroup></Project>",
        ),
    ] {
        assert!(
            is_test_project(source),
            "a project with {label} is runnable by `dotnet test` and must be \
             classified as a suite"
        );
    }

    for (label, source) in [
        (
            "a plain library",
            r#"<Project Sdk="Microsoft.NET.Sdk"><PropertyGroup>
               <TargetFramework>net10.0</TargetFramework></PropertyGroup></Project>"#,
        ),
        (
            "a fixture assembly whose only job is to be compiled by a suite",
            "<Project><PropertyGroup><IsPackable>false</IsPackable></PropertyGroup></Project>",
        ),
        (
            "IsTestProject explicitly disabled",
            "<Project><PropertyGroup><IsTestProject>false</IsTestProject></PropertyGroup></Project>",
        ),
    ] {
        assert!(
            !is_test_project(source),
            "{label} has nothing for `dotnet test` to run, so demanding a CI \
             step for it would mean adding a step that runs nothing"
        );
    }
}

#[test]
fn only_executable_run_steps_count_as_running_a_suite() {
    // The finding this test exists for: the first version of this gate searched
    // the whole file, so `ci.yml`'s extensive comments could keep a suite
    // looking covered after its step was deleted. Each case below would have
    // passed then and must not now.
    let workflow = r#"
jobs:
  # A comment naming commented/Tests/commented.Tests.csproj, which is exactly
  # the shape of the real file: every gate in ci.yml carries a paragraph.
  bridges:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
      - name: Test titled/Tests/titled.Tests.csproj
        uses: some/action@v1
        with:
          project: configured/Tests/configured.Tests.csproj
      - run: dotnet test oneline/Tests/oneline.Tests.csproj -c Release
      - name: block form
        run: |
          dotnet test block/Tests/block.Tests.csproj -c Release
"#;
    let commands = run_commands(workflow);

    for spelling in ["oneline", "block"] {
        let project = format!("{spelling}/Tests/{spelling}.Tests.csproj");
        assert!(
            unrun(std::slice::from_ref(&project), &commands).is_empty(),
            "a `{spelling}`-form `run:` step hands the project to a runner and \
             must count as running it"
        );
    }

    for spelling in ["commented", "titled", "configured"] {
        let project = format!("{spelling}/Tests/{spelling}.Tests.csproj");
        assert_eq!(
            unrun(std::slice::from_ref(&project), &commands),
            vec![project.as_str()],
            "a path appearing only in a {spelling} position is not executed by \
             any runner, so it must NOT count as running the suite — this is \
             the hole the whole-file search left open"
        );
    }
}

#[test]
fn the_gate_fires_on_a_suite_no_run_step_names() {
    // End-to-end over a synthetic tree: discovery and matching composed the way
    // `every_dotnet_suite_is_run_by_ci` composes them, but with input where a
    // suite IS unrun. Without this, that test reports clean both when it works
    // and when it has stopped classifying.
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    let test_sdk = r#"<Project><ItemGroup>
        <PackageReference Include="Microsoft.NET.Test.Sdk" Version="17.11.1" />
        </ItemGroup></Project>"#;
    for (rel, body) in [
        ("wired/Tests/wired.Tests.csproj", test_sdk),
        ("orphan/Tests/Tests.csproj", test_sdk),
        (
            "wired/Tests/FixtureAssembly/FixtureAssembly.csproj",
            "<Project />",
        ),
        ("wired/obj/stale.Tests.csproj", test_sdk),
    ] {
        let path = root.join(rel);
        std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
        std::fs::write(&path, body).expect("write csproj");
    }

    let found = test_projects(root);
    assert_eq!(
        found,
        vec![
            "orphan/Tests/Tests.csproj".to_string(),
            "wired/Tests/wired.Tests.csproj".to_string(),
        ],
        "the walk must find test projects by their metadata regardless of \
         filename — `orphan/Tests/Tests.csproj` carries no `.Tests.csproj` \
         suffix and is exactly the real project the suffix version missed — \
         must skip a fixture assembly, and must not descend into `obj/`"
    );

    let workflow = "jobs:\n  x:\n    steps:\n      - run: dotnet test wired/Tests/wired.Tests.csproj -c Release\n";
    assert_eq!(
        unrun(&found, &run_commands(workflow)),
        vec!["orphan/Tests/Tests.csproj"],
        "the gate must flag a suite no `run:` step names — this is the exact \
         condition `every_dotnet_suite_is_run_by_ci` exists to catch, and if it \
         does not fire here that test cannot fire either"
    );

    // Positive control: with both named, nothing is reported. Without this, a
    // classifier that flagged *everything* would satisfy the assertion above.
    let complete =
        format!("{workflow}      - run: dotnet test orphan/Tests/Tests.csproj -c Release\n");
    assert!(
        unrun(&found, &run_commands(&complete)).is_empty(),
        "the gate reported a suite a `run:` step does name, so it is flagging \
         indiscriminately rather than classifying"
    );
}
