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
//! ## Two things this gate must not get wrong, both found by review (#433)
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
//! **What counts as being run.** This took two passes, because the first fix
//! was not the whole hole.
//!
//! The original version searched the whole text of `ci.yml` for the project
//! path. `ci.yml` is roughly half prose — every gate in it carries a paragraph
//! explaining why it exists — so a `dotnet test` step could be deleted while a
//! comment above it still named the path, and the gate would report the suite
//! covered with no runner executing it.
//!
//! Narrowing to `jobs.*.steps[].run` scalars closed the YAML
//! comments, and only those. A `run: |` block is one opaque string to the
//! parser, so a *shell* comment inside it survives — `# dotnet test x.csproj`
//! reaches the scalar intact — and so does any path the script merely mentions,
//! such as an `echo` naming it in a message. Both would still have counted.
//!
//! So the accepted shape is intentionally narrow: [`tested_paths`] counts only
//! an unconditional, blocking `run:` step whose entire scalar is one literal
//! `dotnet test <repo-relative.csproj> -c Release --
//! RunConfiguration.TreatNoTestsAsError=true` command on the project's supported
//! runner, after only trusted setup/test steps. It does not emulate
//! Bash and PowerShell. Chaining, control flow, variables, quotes, comments,
//! disabled jobs/steps, `continue-on-error`, and environment-level VSTest
//! listing/filtering controls all make a step non-evidence.
//! The real workflow uses this canonical cross-shell form, so an unmodelled
//! shell construct can only make the gate fail closed.
//!
//! Parsing costs nothing here: `serde_yaml` is already a dependency of this
//! crate.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Repository root — `cli/`'s parent.
fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap_or_else(|| panic!("{} has no parent", env!("CARGO_MANIFEST_DIR")))
        .to_path_buf()
}

/// Directories never worth descending into when the synthetic-fixture fallback
/// has no Git index available.
///
/// `obj/` in particular holds generated `*.props` / `*.targets` next to restored
/// package graphs, and `target/` and `node_modules/` are large enough that
/// walking them turns a millisecond check into a slow one.
const SKIP_DIRS: [&str; 5] = [".git", "bin", "obj", "target", "node_modules"];

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
    let normalized: String = source
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .flat_map(char::to_lowercase)
        .collect();
    normalized.contains("microsoft.net.test.sdk")
        || normalized.contains("<istestproject>true</istestproject>")
        || normalized.contains("mstest.sdk/")
}

/// Static test controls that can turn a discovered suite into a no-op or a
/// partial run. Finding one fails the standing gate instead of attempting to
/// reproduce MSBuild's conditional/import evaluation in Rust.
fn has_disabled_or_filtered_test_metadata(source: &str) -> bool {
    let normalized: String = source
        .chars()
        .filter(|c| !c.is_ascii_whitespace())
        .flat_map(char::to_lowercase)
        .collect();
    [
        "<istestproject>false</istestproject>",
        "<vstestlisttests",
        "<vstesttestcasefilter",
        "<testcasefilter",
        "<treatnotestsaserror>false",
    ]
    .iter()
    .any(|needle| normalized.contains(needle))
}

/// Tracked paths matching `patterns`, if `root` belongs to a Git checkout.
fn tracked_files(root: &Path, patterns: &[&str]) -> Option<Vec<String>> {
    let mut command = Command::new("git");
    command.arg("-C").arg(root).args(["ls-files", "-z", "--"]);
    command.args(patterns);
    let output = command.output().ok()?;
    if !output.status.success() {
        return None;
    }
    let mut paths: Vec<_> = output
        .stdout
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| String::from_utf8_lossy(path).replace('\\', "/"))
        .collect();
    paths.sort();
    Some(paths)
}

/// Every tracked project under `root`, as repo-relative `/`-separated paths.
///
/// The Git index is the authority in the real gate: it includes tracked hidden
/// source trees while excluding local build/test scratch state. A filesystem
/// walk remains only for the isolated synthetic-tree regression tests.
fn project_files(root: &Path) -> Vec<String> {
    if let Some(paths) = tracked_files(root, &["*.csproj"]) {
        return paths;
    }
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
            && let Ok(rel) = path.strip_prefix(root)
        {
            // Forward slashes regardless of host: the workflow spells these
            // paths that way, and this test also runs on Windows.
            out.push(rel.to_string_lossy().replace('\\', "/"));
        }
    }
}

/// Every runnable test project under `root`.
fn test_projects(root: &Path) -> Vec<String> {
    project_files(root)
        .into_iter()
        .filter(|rel| {
            std::fs::read_to_string(root.join(rel)).is_ok_and(|source| is_test_project(&source))
        })
        .collect()
}

/// Every project proved to run by a trusted, unconditional job prefix.
///
/// This intentionally validates effective execution context rather than
/// returning bare `run:` text: no dependencies/matrix/defaults/custom shell,
/// the project's supported runner, only trusted setup actions before it, and no
/// execution-changing environment. Any unmodelled context fails closed.
fn tested_paths(workflow: &str) -> Vec<String> {
    let doc: serde_yaml::Value =
        serde_yaml::from_str(workflow).unwrap_or_else(|e| panic!("parse workflow yaml: {e}"));
    let mut out = Vec::new();
    if doc.get("defaults").is_some() || has_execution_changing_test_env(&doc) {
        return out;
    }
    let Some(jobs) = doc.get("jobs").and_then(|j| j.as_mapping()) else {
        return out;
    };
    for (_, job) in jobs {
        if job.get("if").is_some()
            || continues_on_error(job)
            || has_execution_changing_test_env(job)
            || ["needs", "strategy", "defaults", "container", "services"]
                .iter()
                .any(|key| job.get(*key).is_some())
        {
            continue;
        }
        let Some(runner) = job.get("runs-on").and_then(serde_yaml::Value::as_str) else {
            continue;
        };
        if !matches!(runner, "ubuntu-latest" | "windows-latest") {
            continue;
        }
        let Some(steps) = job.get("steps").and_then(|s| s.as_sequence()) else {
            continue;
        };
        for step in steps {
            if step.get("if").is_some()
                || continues_on_error(step)
                || has_execution_changing_test_env(step)
                || ["shell", "working-directory"]
                    .iter()
                    .any(|key| step.get(*key).is_some())
            {
                break;
            }
            if let Some(run) = step.get("run").and_then(|r| r.as_str()) {
                let Some(project) = canonical_test_project(run) else {
                    break;
                };
                if required_runner(&project) != runner {
                    break;
                }
                out.push(project);
            } else if let Some(action) = step.get("uses").and_then(|u| u.as_str()) {
                if !matches!(action, "actions/checkout@v6" | "actions/setup-dotnet@v4") {
                    break;
                }
            } else {
                break;
            }
        }
    }
    out
}

/// Environment variables become MSBuild properties for `dotnet test`. These
/// entries can make a syntactically canonical command skip, list, or filter
/// tests, so such a scope is not evidence that the complete suite gates CI.
fn has_execution_changing_test_env(scope: &serde_yaml::Value) -> bool {
    scope
        .get("env")
        .and_then(serde_yaml::Value::as_mapping)
        .is_some_and(|env| {
            env.keys().filter_map(serde_yaml::Value::as_str).any(|key| {
                [
                    "IsTestProject",
                    "RunSettingsFilePath",
                    "TestCaseFilter",
                    "TreatNoTestsAsError",
                    "VSTestListTests",
                    "VSTestSetting",
                    "VSTestTestCaseFilter",
                ]
                .iter()
                .any(|forbidden| key.eq_ignore_ascii_case(forbidden))
            })
        })
}

fn continues_on_error(step: &serde_yaml::Value) -> bool {
    match step.get("continue-on-error") {
        None | Some(serde_yaml::Value::Bool(false)) => false,
        Some(serde_yaml::Value::String(value)) if value.eq_ignore_ascii_case("false") => false,
        Some(_) => true,
    }
}

/// Whether `path` is a literal repo-relative project path in the canonical
/// forward-slash spelling used by the workflow.
fn is_literal_project_path(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    !path.starts_with('/')
        && !path.contains(['\\', ':', '$', '"', '\'', '`'])
        && lower.ends_with(".csproj")
        && path
            .split('/')
            .all(|part| !part.is_empty() && part != "." && part != "..")
}

/// The one option sequence accepted by the standing gate. `Release` is pinned
/// so a configuration that removes tests cannot be substituted, and the inline
/// RunSettings value makes zero discovered/selected tests fail the CI step.
fn preserves_full_test_execution(options: &[&str]) -> bool {
    options
        == [
            "-c",
            "Release",
            "--",
            "RunConfiguration.TreatNoTestsAsError=true",
        ]
}

fn canonical_test_project(script: &str) -> Option<String> {
    if script.lines().count() != 1
        || script.contains([';', '&', '|', '#', '"', '\'', '`', '$', '<', '>'])
    {
        return None;
    }
    let mut words = script.split_whitespace();
    if words.next() != Some("dotnet") || words.next() != Some("test") {
        return None;
    }
    let project = words.next()?;
    let options: Vec<_> = words.collect();
    (is_literal_project_path(project) && preserves_full_test_execution(&options))
        .then(|| project.to_string())
}

fn required_runner(project: &str) -> &'static str {
    let windows_by_path = ["cli-revit/", "cli-rhino/", "cli-sketchup/", "cli-tekla/"]
        .iter()
        .any(|prefix| project.starts_with(prefix));
    let windows_by_metadata = std::fs::read_to_string(repo_root().join(project))
        .map(|source| {
            let lower = source.to_ascii_lowercase();
            lower.contains("-windows</targetframework")
                || lower.contains("<targetframework>net48</targetframework>")
        })
        .unwrap_or(false);
    if windows_by_path || windows_by_metadata {
        "windows-latest"
    } else {
        "ubuntu-latest"
    }
}

/// Those of `projects` that no `dotnet test` invocation in `workflow` is given.
///
fn unrun<'a>(projects: &'a [String], workflow: &str) -> Vec<&'a str> {
    let tested = tested_paths(workflow);
    projects
        .iter()
        .map(String::as_str)
        .filter(|project| !tested.iter().any(|arg| arg == *project))
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

/// Projects that intentionally have no tests of their own. Pinning this side of
/// the partition makes discovery fail closed: a future `.csproj` whose test
/// status is inherited from props/imports or expressed through an unfamiliar
/// SDK cannot disappear merely because [`is_test_project`] does not understand
/// that form yet. It arrives unclassified and fails CI until wired or explicitly
/// reviewed as non-test.
const KNOWN_NON_TEST_PROJECTS: [&str; 11] = [
    "cli-reader/AwareReader.csproj",
    "cli-revit/AwareRevit/AwareRevit.csproj",
    "cli-revit/Shared/Shared.csproj",
    "cli-revit/Sidecar/cli-revit.csproj",
    "cli-rhino/cli-rhino.csproj",
    "cli-roslyn/cli-roslyn.csproj",
    "cli-sidecar/cli-sidecar.csproj",
    "cli-sidecar/Tests/FixtureAssembly/FixtureAssembly.csproj",
    "cli-sidecar/Tests/FixtureDataAssembly/FixtureDataAssembly.csproj",
    "cli-sketchup/cli-sketchup.csproj",
    "cli-tekla/cli-tekla.csproj",
];

fn unclassified<'a>(projects: &'a [String], tests: &[String], non_tests: &[&str]) -> Vec<&'a str> {
    projects
        .iter()
        .map(String::as_str)
        .filter(|project| !tests.iter().any(|test| test == project) && !non_tests.contains(project))
        .collect()
}

#[test]
fn tracked_msbuild_metadata_cannot_disable_or_filter_test_execution() {
    let root = repo_root();
    let files = tracked_files(
        &root,
        &["*.csproj", "*.props", "*.targets", "*.runsettings"],
    )
    .expect("repository root must have a readable Git index");
    let offenders: Vec<_> = files
        .iter()
        .filter(|path| {
            std::fs::read_to_string(root.join(path))
                .is_ok_and(|source| has_disabled_or_filtered_test_metadata(&source))
        })
        .collect();
    assert!(
        offenders.is_empty(),
        "tracked MSBuild/runsettings files disable, list, or filter test execution: {offenders:#?}"
    );
}

#[test]
fn every_project_is_classified_so_new_test_forms_fail_closed() {
    let root = repo_root();
    let projects = project_files(&root);
    let tests = test_projects(&root);
    let unknown = unclassified(&projects, &tests, &KNOWN_NON_TEST_PROJECTS);
    assert!(
        unknown.is_empty(),
        "these projects are neither detected test suites nor reviewed non-test projects: {unknown:#?}. \
         If one is a suite, teach `is_test_project` its metadata form and add a blocking CI step; \
         otherwise add it to KNOWN_NON_TEST_PROJECTS with deliberate review"
    );
    for known in KNOWN_NON_TEST_PROJECTS {
        assert!(
            projects.iter().any(|project| project == known),
            "stale non-test classification `{known}` must be removed"
        );
        assert!(
            !tests.iter().any(|project| project == known),
            "reviewed non-test project `{known}` now carries test metadata; remove it from \
             KNOWN_NON_TEST_PROJECTS and add a blocking CI test step"
        );
    }
}

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
    let missing = unrun(&projects, &ci_workflow());
    assert!(
        missing.is_empty(),
        "these .NET test projects exist but no `dotnet test` invocation in \
         `.github/workflows/ci.yml` is given them, so their tests are green \
         only by assumption:\n  {}\n\n\
         Add a `dotnet test <path>` step to the `dotnet-bridges` job (plain \
         `net*` target frameworks) or `dotnet-bridges-windows` (`net*-windows` \
         ones). Only an executed invocation counts, and that is deliberate: a \
         path in a YAML comment, in a shell comment inside a `run: |` block, in \
         a step's `name:`, or in an `echo` is not a suite anybody runs — each of \
         those was a hole this gate has been closed against. Do not satisfy this \
         by deleting the suite or stripping its test-sdk reference — CLAUDE.md \
         §Engineering rules forbids satisfying a gate by disabling it.",
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
        (
            "a case-varied test-host package reference",
            r#"<Project><ItemGroup><PackageReference Include="microsoft.net.test.sdk" /></ItemGroup></Project>"#,
        ),
        (
            "a case-varied IsTestProject property",
            "<Project><PropertyGroup><IsTestProject>True</IsTestProject></PropertyGroup></Project>",
        ),
        (
            "an SDK-style MSTest project",
            r#"<Project Sdk="MSTest.Sdk/3.8.3"><PropertyGroup><TargetFramework>net10.0</TargetFramework></PropertyGroup></Project>"#,
        ),
        (
            "a test-host package even when conflicting disabled metadata is present",
            r#"<Project><PropertyGroup><IsTestProject>False</IsTestProject></PropertyGroup>
               <ItemGroup><PackageReference Include="Microsoft.NET.Test.Sdk" /></ItemGroup></Project>"#,
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

    assert!(
        has_disabled_or_filtered_test_metadata(
            r#"<Project><PropertyGroup><IsTestProject>False</IsTestProject></PropertyGroup>
               <ItemGroup><PackageReference Include="Microsoft.NET.Test.Sdk" /></ItemGroup></Project>"#
        ),
        "disabled metadata must fail the separate standing control gate without hiding the suite"
    );
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
      - run: dotnet test oneline/Tests/oneline.Tests.csproj -c Release -- RunConfiguration.TreatNoTestsAsError=true
      - name: Test titled/Tests/titled.Tests.csproj
        uses: some/action@v1
        with:
          project: configured/Tests/configured.Tests.csproj
      - name: block form, and the three ways a path can sit in a run scalar
        run: |
          dotnet test block/Tests/block.Tests.csproj -c Release
          # dotnet test shellcommented/Tests/shellcommented.Tests.csproj -c Release
          echo "skipping echoed/Tests/echoed.Tests.csproj for now"
          dotnet test trailing/Tests/trailing.Tests.csproj # -c Release
      - name: continued across a line break
        run: |
          dotnet test \
            continued/Tests/continued.Tests.csproj -c Release
      - name: chained after another command
        run: cd . && dotnet test chained/Tests/chained.Tests.csproj -c Release
"#;

    let oneline = "oneline/Tests/oneline.Tests.csproj".to_string();
    assert!(
        unrun(std::slice::from_ref(&oneline), workflow).is_empty(),
        "the canonical standalone invocation must count as running the suite"
    );

    // `shellcommented` and `echoed` are the second half of the finding: narrowing
    // to `run:` scalars closes YAML comments only. A `run: |` block is one opaque
    // string to the parser, so a shell comment inside it survives intact, and so
    // does a path an `echo` merely names. Neither is executed by a test runner.
    for spelling in [
        "commented",
        "titled",
        "configured",
        "shellcommented",
        "echoed",
        "block",
        "trailing",
        "continued",
        "chained",
    ] {
        let project = format!("{spelling}/Tests/{spelling}.Tests.csproj");
        assert_eq!(
            unrun(std::slice::from_ref(&project), workflow),
            vec![project.as_str()],
            "a {spelling} form is not an unconditional standalone literal test \
             step, so it must NOT count as CI gating the suite"
        );
    }
}

#[test]
fn only_unconditional_standalone_test_steps_count() {
    let workflow = r#"
jobs:
  disabled-job:
    if: false
    runs-on: ubuntu-latest
    steps:
      - run: dotnet test disabled-job/Tests/disabled-job.Tests.csproj -c Release
  nonblocking-job:
    continue-on-error: true
    runs-on: ubuntu-latest
    steps:
      - run: dotnet test nonblocking-job/Tests/nonblocking-job.Tests.csproj -c Release
  bridges:
    runs-on: ubuntu-latest
    steps:
      - run: dotnet test canonical/Tests/canonical.Tests.csproj -c Release -- RunConfiguration.TreatNoTestsAsError=true
      - if: false
        run: dotnet test disabled-step/Tests/disabled-step.Tests.csproj -c Release
      - continue-on-error: true
        run: dotnet test nonblocking/Tests/nonblocking.Tests.csproj -c Release
      - run: echo 'disabled; dotnet test quoted/Tests/quoted.Tests.csproj -c Release'
      - run: false && dotnet test chained-false/Tests/chained-false.Tests.csproj -c Release
      - run: dotnet test listing/Tests/listing.Tests.csproj --list-tests
      - env:
          VSTestListTests: "true"
        run: dotnet test step-listing/Tests/step-listing.Tests.csproj -c Release -- RunConfiguration.TreatNoTestsAsError=true
  filtered-job:
    runs-on: ubuntu-latest
    env:
      vstesttestcasefilter: Category=Fast
    steps:
      - run: dotnet test job-filtered/Tests/job-filtered.Tests.csproj -c Release -- RunConfiguration.TreatNoTestsAsError=true
"#;

    let canonical = "canonical/Tests/canonical.Tests.csproj".to_string();
    assert!(
        unrun(std::slice::from_ref(&canonical), workflow).is_empty(),
        "an unconditional standalone literal test command must count"
    );

    for spelling in [
        "disabled-job",
        "nonblocking-job",
        "disabled-step",
        "nonblocking",
        "quoted",
        "chained-false",
        "listing",
        "step-listing",
        "job-filtered",
    ] {
        let project = format!("{spelling}/Tests/{spelling}.Tests.csproj");
        assert_eq!(
            unrun(std::slice::from_ref(&project), workflow),
            vec![project.as_str()],
            "a {spelling} command is not an unconditional blocking CI test"
        );
    }
}

#[test]
fn workflow_environment_cannot_turn_a_canonical_command_into_listing() {
    let workflow = r#"
env:
  VSTESTLISTTESTS: "true"
jobs:
  bridges:
    runs-on: ubuntu-latest
    steps:
      - run: dotnet test globally-listed/Tests/globally-listed.Tests.csproj -c Release -- RunConfiguration.TreatNoTestsAsError=true
"#;
    let project = "globally-listed/Tests/globally-listed.Tests.csproj".to_string();
    assert_eq!(
        unrun(std::slice::from_ref(&project), workflow),
        vec![project.as_str()],
        "workflow-level VSTest controls apply to every step and must fail closed"
    );
}

#[test]
fn runner_shell_prefix_and_configuration_must_all_be_canonical() {
    let command = |project: &str| {
        format!("dotnet test {project} -c Release -- RunConfiguration.TreatNoTestsAsError=true")
    };
    let workflow = format!(
        r#"
jobs:
  dependency:
    runs-on: ubuntu-latest
    steps:
      - run: exit 1
  skipped-through-needs:
    needs: dependency
    runs-on: ubuntu-latest
    steps:
      - run: {needs}
  matrix:
    strategy:
      matrix:
        os: [ubuntu-latest]
    runs-on: ${{{{ matrix.os }}}}
    steps:
      - run: {matrix}
  custom-shell:
    runs-on: ubuntu-latest
    steps:
      - shell: echo {{0}}
        run: {shell}
  relocated:
    runs-on: ubuntu-latest
    steps:
      - working-directory: elsewhere
        run: {relocated}
  wrong-os:
    runs-on: ubuntu-latest
    steps:
      - run: {wrong_os}
  poisoned-prefix:
    runs-on: ubuntu-latest
    steps:
      - run: echo VSTestTestCaseFilter=Category=Fast >> "$GITHUB_ENV"
      - run: {poisoned}
  defaults:
    runs-on: ubuntu-latest
    defaults:
      run:
        shell: echo {{0}}
    steps:
      - run: {defaults}
  debug-configuration:
    runs-on: ubuntu-latest
    steps:
      - run: dotnet test debug/Tests/debug.Tests.csproj -c Debug -- RunConfiguration.TreatNoTestsAsError=true
  zero-tests-can-pass:
    runs-on: ubuntu-latest
    steps:
      - run: dotnet test zero/Tests/zero.Tests.csproj -c Release
"#,
        needs = command("needs/Tests/needs.Tests.csproj"),
        matrix = command("matrix/Tests/matrix.Tests.csproj"),
        shell = command("shell/Tests/shell.Tests.csproj"),
        relocated = command("relocated/Tests/relocated.Tests.csproj"),
        wrong_os = command("cli-revit/Tests/wrong-os.Tests.csproj"),
        poisoned = command("poisoned/Tests/poisoned.Tests.csproj"),
        defaults = command("defaults/Tests/defaults.Tests.csproj"),
    );

    for project in [
        "needs/Tests/needs.Tests.csproj",
        "matrix/Tests/matrix.Tests.csproj",
        "shell/Tests/shell.Tests.csproj",
        "relocated/Tests/relocated.Tests.csproj",
        "cli-revit/Tests/wrong-os.Tests.csproj",
        "poisoned/Tests/poisoned.Tests.csproj",
        "defaults/Tests/defaults.Tests.csproj",
        "debug/Tests/debug.Tests.csproj",
        "zero/Tests/zero.Tests.csproj",
    ] {
        let project = project.to_string();
        assert_eq!(
            unrun(std::slice::from_ref(&project), &workflow),
            vec![project.as_str()],
            "non-canonical execution context must not prove `{project}` ran"
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
        (".scratch/wired/Tests/temporary.Tests.csproj", test_sdk),
        ("inherited/Tests/Inherited.csproj", "<Project />"),
    ] {
        let path = root.join(rel);
        std::fs::create_dir_all(path.parent().expect("parent")).expect("mkdir");
        std::fs::write(&path, body).expect("write csproj");
    }

    let found = test_projects(root);
    assert_eq!(
        found,
        vec![
            ".scratch/wired/Tests/temporary.Tests.csproj".to_string(),
            "orphan/Tests/Tests.csproj".to_string(),
            "wired/Tests/wired.Tests.csproj".to_string(),
        ],
        "the walk must find test projects by their metadata regardless of \
         filename — `orphan/Tests/Tests.csproj` carries no `.Tests.csproj` \
         suffix and is exactly the real project the suffix version missed — \
         must skip a fixture assembly and `obj/`, while still inventorying a \
         hidden source directory rather than silently excluding it"
    );

    let projects = project_files(root);
    assert_eq!(
        unclassified(
            &projects,
            &found,
            &["wired/Tests/FixtureAssembly/FixtureAssembly.csproj"]
        ),
        vec!["inherited/Tests/Inherited.csproj"],
        "an unfamiliar or inherited test-project form must fail the inventory gate instead of disappearing"
    );

    let workflow = "jobs:\n  x:\n    runs-on: ubuntu-latest\n    steps:\n      - run: dotnet test wired/Tests/wired.Tests.csproj -c Release -- RunConfiguration.TreatNoTestsAsError=true\n";
    assert_eq!(
        unrun(&found, workflow),
        vec![
            ".scratch/wired/Tests/temporary.Tests.csproj",
            "orphan/Tests/Tests.csproj",
        ],
        "the gate must flag a suite no `run:` step names — this is the exact \
         condition `every_dotnet_suite_is_run_by_ci` exists to catch, and if it \
         does not fire here that test cannot fire either"
    );

    // Positive control: with both named, nothing is reported. Without this, a
    // classifier that flagged *everything* would satisfy the assertion above.
    let complete = format!(
        "{workflow}      - run: dotnet test orphan/Tests/Tests.csproj -c Release -- RunConfiguration.TreatNoTestsAsError=true\n      - run: dotnet test .scratch/wired/Tests/temporary.Tests.csproj -c Release -- RunConfiguration.TreatNoTestsAsError=true\n"
    );
    assert!(
        unrun(&found, &complete).is_empty(),
        "the gate reported a suite a `run:` step does name, so it is flagging \
         indiscriminately rather than classifying"
    );
}
