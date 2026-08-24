//! The registry-authoring half of `aware agent`: `publish` and `reindex`.
//!
//! Both commands WRITE files a contributor then commits — `registry-index.json`
//! and `registry-catalog.json` — and both locate the file to write by walking up
//! from a path (the agent folder for `publish`, the cwd for `reindex`). Neither
//! had a single test: `merge_publish_entry` and `catalog_content_eq` are unit-
//! tested as string→string functions, but nothing exercised the commands that
//! call them, so the parts that decide WHICH file is rewritten, whether it is
//! rewritten at all, and what stops a bad agent reaching the index were covered
//! only by reading them.
//!
//! Every test here is written against a throwaway mini-checkout, never the real
//! repo — `publish` rewrites the index it finds, and the one this repo ships is
//! not a fixture.

use assert_cmd::Command;
use predicates::prelude::*;
use std::path::{Path, PathBuf};

/// The tarball every substrate-hosted entry points at. Duplicated from
/// `commands::agent::SUBSTRATE_TARBALL` on purpose: a test that read the
/// constant would agree with the code whatever the code said.
const SUBSTRATE_TARBALL: &str =
    "https://github.com/aware-aeco/aware/archive/refs/heads/main.tar.gz";

fn aware() -> Command {
    Command::cargo_bin("aware").expect("built binary")
}

/// A minimal agent that passes `validate_agent_on_disk` with no issues.
fn write_agent(dir: &Path, id: &str, version: &str, description: &str) {
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(
        dir.join("manifest.yaml"),
        format!(
            "agent: {id}\n\
             version: {version}\n\
             description: {description}\n\
             stateful: false\n\
             license: MIT\n\
             transport:\n  cli:\n    binary: {id}\n\
             commands:\n  ping:\n    lifecycle: single\n    description: Ping {id}.\n"
        ),
    )
    .unwrap();
}

/// An agent whose manifest loads but fails validation: it declares a skill file
/// that is not on disk (`E_SKILL_MISSING`).
fn write_agent_with_missing_skill(dir: &Path, id: &str) {
    std::fs::create_dir_all(dir).unwrap();
    std::fs::write(
        dir.join("manifest.yaml"),
        format!(
            "agent: {id}\n\
             version: 1.0.0\n\
             description: Declares a skill it does not ship.\n\
             stateful: false\n\
             license: MIT\n\
             transport:\n  cli:\n    binary: {id}\n\
             commands:\n  ping:\n    lifecycle: single\n    description: Ping.\n\
             skills:\n  - absent.md\n"
        ),
    )
    .unwrap();
}

/// A `registry-index.json` where ONE agent carries several version entries, each
/// `(version, tarball, subdir)`. Distinct from [`write_index`], which gives every
/// entry its own agent id — this is the multi-version-of-one-agent shape #454 is
/// about. The tarball is explicit per version so a test can vary it independently of
/// the subdir: `reindex` refuses a shared subdir either way (it reads the checkout,
/// not the archives), but the two cases get different diagnostics because their
/// remedies differ.
fn write_index_multiversion(root: &Path, id: &str, versions: &[(&str, &str, &str)]) {
    let vmap: serde_json::Map<String, serde_json::Value> = versions
        .iter()
        .map(|(ver, tarball, subdir)| {
            (
                (*ver).to_string(),
                serde_json::json!({ "tarball": *tarball, "subdir": *subdir }),
            )
        })
        .collect();
    let doc = serde_json::json!({
        "version": "1.0",
        "updated-at": "2026-01-01T00:00:00Z",
        "agents": { id: { "versions": vmap } },
        "bundles": {},
    });
    std::fs::write(
        root.join("registry-index.json"),
        serde_json::to_string_pretty(&doc).unwrap(),
    )
    .unwrap();
}

/// `registry-index.json` naming `entries` as `(id, version, subdir)`.
fn write_index(root: &Path, entries: &[(&str, &str, &str)]) {
    let agents: serde_json::Map<String, serde_json::Value> = entries
        .iter()
        .map(|(id, ver, subdir)| {
            (
                (*id).to_string(),
                serde_json::json!({
                    "versions": { *ver: { "tarball": SUBSTRATE_TARBALL, "subdir": *subdir } }
                }),
            )
        })
        .collect();
    let doc = serde_json::json!({
        "version": "1.0",
        "updated-at": "2026-01-01T00:00:00Z",
        "agents": agents,
        "bundles": {},
    });
    std::fs::write(
        root.join("registry-index.json"),
        serde_json::to_string_pretty(&doc).unwrap(),
    )
    .unwrap();
}

fn index_of(root: &Path) -> serde_json::Value {
    serde_json::from_str(&std::fs::read_to_string(root.join("registry-index.json")).unwrap())
        .unwrap()
}

/// A per-test AWARE_HOME so nothing reaches the developer's real one.
fn home_in(tmp: &Path) -> PathBuf {
    let home = tmp.join("aware-home");
    std::fs::create_dir_all(&home).unwrap();
    home
}

/// Guard the two "there is no checkout here" tests before they spawn a command
/// that WRITES what it finds by walking up.
///
/// Those tests assert on the absence of a `registry-index.json` above the
/// fixture, and a `TempDir` only guarantees a fresh *leaf* — its ancestors are
/// whatever `TMPDIR` points at. Point `TMPDIR` inside an aware checkout (a
/// configuration a developer or a CI image may well have) and the fixture is
/// born UNDER the real `registry-index.json`: `publish` would then walk up into
/// the tracked index and rewrite it, and `reindex` would rebuild the tracked
/// `registry-catalog.json` from the whole repo — both before the assertion that
/// was supposed to catch it. The assertion would fail, but only after the damage.
///
/// So refuse to run at all in that case, and say which ancestor is the problem.
/// This runs before the binary is spawned, which is what makes it a guard rather
/// than a second detector. (Codex review, PR #449.)
fn assert_no_index_above(dir: &Path) {
    let start = dir.canonicalize().expect("fixture dir exists");
    let mut cursor = Some(start.as_path());
    while let Some(d) = cursor {
        assert!(
            !d.join("registry-index.json").is_file(),
            "refusing to run: {} is inside a registry checkout ({} holds a registry-index.json), \
             so this test would rewrite tracked files instead of exercising the \
             no-checkout path. Point TMPDIR outside any aware checkout.",
            start.display(),
            d.display(),
        );
        cursor = d.parent();
    }
}

// ---------------------------------------------------------------- publish ---

#[test]
fn publish_stages_the_agent_under_a_repo_relative_subdir() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let agent_dir = root.join("20-agents/aeco/demo");
    write_agent(&agent_dir, "demo", "1.0.0", "A demo agent.");
    write_index(
        root,
        &[("keeper", "0.1.0", "aware-main/20-agents/aeco/keeper")],
    );

    aware()
        .env("AWARE_HOME", home_in(root))
        .args(["agent", "publish"])
        .arg(&agent_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("staged demo@1.0.0"));

    let doc = index_of(root);
    let entry = &doc["agents"]["demo"]["versions"]["1.0.0"];
    // The subdir is the agent's path RELATIVE to the index's directory, prefixed
    // with the archive's top-level folder — that prefix is what makes the entry
    // resolvable inside `main.tar.gz`, and an absolute or bare-relative path
    // would produce an index that installs nothing.
    assert_eq!(entry["subdir"], "aware-main/20-agents/aeco/demo");
    assert_eq!(entry["tarball"], SUBSTRATE_TARBALL);
    // Publishing is a merge, not a rewrite: the agent already in the index survives.
    assert_eq!(
        doc["agents"]["keeper"]["versions"]["0.1.0"]["subdir"],
        "aware-main/20-agents/aeco/keeper"
    );
}

#[test]
fn publish_stages_into_the_nearest_index_not_the_outermost() {
    // Two checkouts, one inside the other. `publish` must write the index that
    // actually contains the agent — writing the outer one would stage a subdir
    // that is wrong relative to it, and silently edit an unrelated registry.
    let tmp = tempfile::tempdir().unwrap();
    let outer = tmp.path();
    let inner = outer.join("vendor/inner-checkout");
    let agent_dir = inner.join("agents/demo");
    write_agent(&agent_dir, "demo", "1.0.0", "A demo agent.");
    std::fs::create_dir_all(&inner).unwrap();
    write_index(outer, &[("outer-only", "0.1.0", "aware-main/outer")]);
    write_index(&inner, &[("inner-only", "0.1.0", "aware-main/inner")]);

    let outer_before = std::fs::read(outer.join("registry-index.json")).unwrap();

    aware()
        .env("AWARE_HOME", home_in(outer))
        .args(["agent", "publish"])
        .arg(&agent_dir)
        .assert()
        .success();

    let inner_doc = index_of(&inner);
    assert_eq!(
        inner_doc["agents"]["demo"]["versions"]["1.0.0"]["subdir"], "aware-main/agents/demo",
        "subdir is relative to the index that was written"
    );
    assert_eq!(
        std::fs::read(outer.join("registry-index.json")).unwrap(),
        outer_before,
        "the outer checkout's index must not be touched"
    );
}

#[test]
fn publish_refuses_a_failing_agent_and_leaves_the_index_byte_identical() {
    // The index is shared state a contributor commits; a publish that validated
    // AFTER writing would leave a broken entry staged in it.
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let agent_dir = root.join("20-agents/aeco/broken");
    write_agent_with_missing_skill(&agent_dir, "broken");
    write_index(
        root,
        &[("keeper", "0.1.0", "aware-main/20-agents/aeco/keeper")],
    );
    let before = std::fs::read(root.join("registry-index.json")).unwrap();

    aware()
        .env("AWARE_HOME", home_in(root))
        .args(["agent", "publish"])
        .arg(&agent_dir)
        .assert()
        .failure()
        .code(3)
        .stdout(predicate::str::contains("absent.md"));

    assert_eq!(
        std::fs::read(root.join("registry-index.json")).unwrap(),
        before,
        "a rejected agent must not reach the index — not even its `updated-at`"
    );
}

#[test]
fn publish_outside_a_checkout_explains_itself_and_creates_no_index() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let agent_dir = root.join("standalone/demo");
    write_agent(&agent_dir, "demo", "1.0.0", "A demo agent.");
    // No registry-index.json anywhere above the agent — enforced, not assumed.
    assert_no_index_above(&agent_dir);

    aware()
        .env("AWARE_HOME", home_in(root))
        .args(["agent", "publish"])
        .arg(&agent_dir)
        .assert()
        // Not an error: the agent is fine, there is just nowhere to stage it.
        .success()
        .stdout(predicate::str::contains("registry-index.json"));

    assert!(
        !root.join("registry-index.json").exists(),
        "publish must not conjure an index outside a checkout"
    );
    assert!(!agent_dir.join("registry-index.json").exists());
}

#[test]
fn publishing_a_second_version_into_one_subdir_is_refused_and_writes_nothing() {
    // The producer half of #454. `publish` always derives the substrate tarball and the
    // agent's repo-relative subdir, so an in-place version bump would stage a second key
    // on one folder — an index `agent reindex` refuses, after publish said "✓ staged".
    //
    // It REFUSES rather than retiring the old key. An index key is a RELEASE key on its
    // own axis (68 of 78 shipped entries differ from their manifest version), so deleting
    // the one that shares the folder breaks external pins on a released version, and
    // whether it is stale is not knowable from the manifest (Codex review, PR #457 r6).
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let agent_dir = root.join("20-agents/aeco/demo");
    let home = home_in(root);
    // A release key deliberately on a different axis from the manifest version, exactly
    // as `tekla@2025.0.1` sits over a `0.1.4` manifest.
    write_index_multiversion(
        root,
        "demo",
        &[(
            "2025.0.1",
            SUBSTRATE_TARBALL,
            "aware-main/20-agents/aeco/demo",
        )],
    );
    let before = std::fs::read(root.join("registry-index.json")).unwrap();

    write_agent(&agent_dir, "demo", "0.2.0", "The current build.");
    aware()
        .env("AWARE_HOME", &home)
        .args(["agent", "publish"])
        .arg(&agent_dir)
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("demo@2025.0.1"))
        .stderr(predicate::str::contains("already publishes subdir"));

    assert_eq!(
        std::fs::read(root.join("registry-index.json")).unwrap(),
        before,
        "a refused publish must leave the index byte-identical — the release key survives"
    );
}

#[test]
fn publishing_each_version_to_its_own_subdir_yields_an_index_reindex_accepts() {
    // The shape that works, driven end to end: two versions, each frozen at its own
    // folder, so every key has a manifest of its own for `reindex` to read.
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let home = home_in(root);
    std::fs::write(
        root.join("registry-index.json"),
        r#"{"version":"1.0","updated-at":"2026-01-01T00:00:00Z","agents":{},"bundles":{}}"#,
    )
    .unwrap();

    write_agent(
        &root.join("archive/demo-0.1.0"),
        "demo",
        "0.1.0",
        "The frozen 0.1.0 build.",
    );
    write_agent(
        &root.join("20-agents/aeco/demo"),
        "demo",
        "0.2.0",
        "The current build.",
    );
    for dir in ["archive/demo-0.1.0", "20-agents/aeco/demo"] {
        aware()
            .env("AWARE_HOME", &home)
            .args(["agent", "publish"])
            .arg(root.join(dir))
            .assert()
            .success();
    }

    aware()
        .current_dir(root)
        .env("AWARE_HOME", &home)
        .args(["agent", "reindex"])
        .assert()
        .success();

    let cat: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(root.join("registry-catalog.json")).unwrap())
            .unwrap();
    let versions = &cat["agents"]["demo"]["versions"];
    assert_eq!(
        versions["0.1.0"]["description"], "The frozen 0.1.0 build.",
        "each entry describes its OWN subdir's manifest: {cat:#}"
    );
    assert_eq!(versions["0.2.0"]["description"], "The current build.");
}

// ---------------------------------------------------------------- reindex ---

#[test]
fn reindex_writes_the_catalog_beside_the_index_it_walked_up_to() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let agent_dir = root.join("20-agents/aeco/demo");
    write_agent(&agent_dir, "demo", "1.0.0", "A demo agent.");
    write_index(root, &[("demo", "1.0.0", "aware-main/20-agents/aeco/demo")]);

    // Run from DEEP inside the checkout: the catalog belongs next to the index,
    // not next to wherever the contributor happened to be standing.
    aware()
        .current_dir(&agent_dir)
        .env("AWARE_HOME", home_in(root))
        .args(["agent", "reindex"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1 agents"));

    assert!(
        !agent_dir.join("registry-catalog.json").exists(),
        "the catalog must not land in the cwd"
    );
    let cat: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(root.join("registry-catalog.json")).unwrap())
            .unwrap();
    // The catalog is built from the on-disk manifest, not from the index entry —
    // so the manifest's own command surface has to show up in it.
    let commands = &cat["agents"]["demo"]["versions"]["1.0.0"]["commands"];
    assert_eq!(commands[0]["name"], "ping", "catalog: {cat:#}");
}

#[test]
fn reindex_check_passes_when_current_and_fails_once_a_manifest_drifts() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let agent_dir = root.join("20-agents/aeco/demo");
    write_agent(&agent_dir, "demo", "1.0.0", "The original description.");
    write_index(root, &[("demo", "1.0.0", "aware-main/20-agents/aeco/demo")]);

    aware()
        .current_dir(root)
        .env("AWARE_HOME", home_in(root))
        .args(["agent", "reindex"])
        .assert()
        .success();

    // Freshly written → up to date, despite `updated-at` differing on every run.
    aware()
        .current_dir(root)
        .env("AWARE_HOME", home_in(root))
        .args(["agent", "reindex", "--check"])
        .assert()
        .success()
        .stdout(predicate::str::contains("up to date"));

    let after_check = std::fs::read(root.join("registry-catalog.json")).unwrap();

    // Edit the manifest the catalog was built from: `--check` must now fail.
    write_agent(&agent_dir, "demo", "1.0.0", "A rewritten description.");
    aware()
        .current_dir(root)
        .env("AWARE_HOME", home_in(root))
        .args(["agent", "reindex", "--check"])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("stale"));

    assert_eq!(
        std::fs::read(root.join("registry-catalog.json")).unwrap(),
        after_check,
        "--check is read-only: it must report staleness, never fix it"
    );
}

#[test]
fn reindex_check_reports_an_absent_catalog_as_stale() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write_agent(
        &root.join("20-agents/aeco/demo"),
        "demo",
        "1.0.0",
        "A demo agent.",
    );
    write_index(root, &[("demo", "1.0.0", "aware-main/20-agents/aeco/demo")]);

    // Nothing has been generated yet — the CI gate must fail rather than treat
    // "no catalog" as "nothing to do".
    aware()
        .current_dir(root)
        .env("AWARE_HOME", home_in(root))
        .args(["agent", "reindex", "--check"])
        .assert()
        .failure()
        .code(3);

    assert!(
        !root.join("registry-catalog.json").exists(),
        "--check must not write the catalog it was only asked to verify"
    );
}

#[test]
fn reindex_refuses_a_partial_catalog_and_leaves_the_previous_one_intact() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write_agent(
        &root.join("20-agents/aeco/demo"),
        "demo",
        "1.0.0",
        "A demo agent.",
    );
    write_index(root, &[("demo", "1.0.0", "aware-main/20-agents/aeco/demo")]);
    aware()
        .current_dir(root)
        .env("AWARE_HOME", home_in(root))
        .args(["agent", "reindex"])
        .assert()
        .success();
    let good_catalog = std::fs::read(root.join("registry-catalog.json")).unwrap();

    // Add an index entry whose manifest is not on disk. Dropping it silently
    // would publish a catalog that is missing an agent the index advertises.
    write_index(
        root,
        &[
            ("demo", "1.0.0", "aware-main/20-agents/aeco/demo"),
            ("ghost", "0.1.0", "aware-main/20-agents/aeco/ghost"),
        ],
    );

    aware()
        .current_dir(root)
        .env("AWARE_HOME", home_in(root))
        .args(["agent", "reindex"])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("ghost@0.1.0"));

    assert_eq!(
        std::fs::read(root.join("registry-catalog.json")).unwrap(),
        good_catalog,
        "a refused reindex must not half-write over the last good catalog"
    );
}

#[test]
fn reindex_refuses_two_versions_sharing_one_subdir_and_writes_nothing() {
    // #454: an agent bumped from 0.1.0 to 0.2.0 whose index gains a 0.2.0 entry at the
    // SAME subdir. Both keys resolve to that path's single (0.2.0) manifest, so the
    // historical 0.1.0 catalog entry would be stamped with the current build's
    // description/commands/manifest-version. reindex must refuse rather than write that
    // fiction — and it must not have half-written a catalog before refusing.
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let agent_dir = root.join("20-agents/aeco/demo");
    write_agent(&agent_dir, "demo", "0.2.0", "The current 0.2.0 build.");
    write_index_multiversion(
        root,
        "demo",
        &[
            ("0.1.0", SUBSTRATE_TARBALL, "aware-main/20-agents/aeco/demo"),
            ("0.2.0", SUBSTRATE_TARBALL, "aware-main/20-agents/aeco/demo"),
        ],
    );

    aware()
        .current_dir(root)
        .env("AWARE_HOME", home_in(root))
        .args(["agent", "reindex"])
        .assert()
        .failure()
        .code(3)
        // The failing key is the newer one, and the message points at the shared subdir.
        .stderr(predicate::str::contains("demo@0.2.0"))
        .stderr(predicate::str::contains("shares subdir"));

    assert!(
        !root.join("registry-catalog.json").exists(),
        "a refused reindex must not write the misleading catalog"
    );
}

#[test]
fn reindex_refuses_one_subdir_even_when_the_tarballs_differ() {
    // Codex review (PR #457, round 5): reindex resolves a version's subdir against the
    // LOCAL CHECKOUT and never opens its tarball, so two versions sharing a subdir are
    // both described by that path's single manifest however their tarballs differ —
    // reproducing #454 while reporting success. Refusing is honest until reindex can
    // fetch each archive.
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write_agent(
        &root.join("20-agents/aeco/demo"),
        "demo",
        "1.3.0",
        "A demo agent.",
    );
    write_index_multiversion(
        root,
        "demo",
        &[
            (
                "1.2.0",
                "https://example.invalid/demo-1.2.0.tar.gz",
                "aware-main/20-agents/aeco/demo",
            ),
            (
                "1.3.0",
                "https://example.invalid/demo-1.3.0.tar.gz",
                "aware-main/20-agents/aeco/demo",
            ),
        ],
    );

    aware()
        .current_dir(root)
        .env("AWARE_HOME", home_in(root))
        .args(["agent", "reindex"])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("demo@1.3.0"))
        .stderr(predicate::str::contains("tarballs differ"));

    assert!(
        !root.join("registry-catalog.json").exists(),
        "a refused reindex must not write the misleading catalog"
    );
}

#[test]
fn reindex_refuses_a_collision_spelled_with_a_trailing_slash() {
    // Codex review (PR #457, P2): the installer's `extract_subdir` trims trailing
    // slashes, so `foo` and `foo/` are one directory. A raw-string guard let that
    // spelling through into the catalog it exists to prevent.
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write_agent(
        &root.join("20-agents/aeco/demo"),
        "demo",
        "0.2.0",
        "The current 0.2.0 build.",
    );
    write_index_multiversion(
        root,
        "demo",
        &[
            ("0.1.0", SUBSTRATE_TARBALL, "aware-main/20-agents/aeco/demo"),
            (
                "0.2.0",
                SUBSTRATE_TARBALL,
                "aware-main/20-agents/aeco/demo/",
            ),
        ],
    );

    aware()
        .current_dir(root)
        .env("AWARE_HOME", home_in(root))
        .args(["agent", "reindex"])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("demo@0.2.0"));

    assert!(!root.join("registry-catalog.json").exists());
}

#[test]
fn reindex_accepts_two_versions_at_distinct_subdirs() {
    // The correct multi-version shape: 0.1.0 frozen at its own subdir, 0.2.0 at the
    // live path. No collision, so reindex succeeds and each catalog entry reflects the
    // manifest its OWN subdir holds — the 0.1.0 entry is NOT rewritten from 0.2.0.
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    write_agent(
        &root.join("archive/demo-0.1.0"),
        "demo",
        "0.1.0",
        "The frozen 0.1.0 build.",
    );
    write_agent(
        &root.join("20-agents/aeco/demo"),
        "demo",
        "0.2.0",
        "The current 0.2.0 build.",
    );
    write_index_multiversion(
        root,
        "demo",
        &[
            ("0.1.0", SUBSTRATE_TARBALL, "aware-main/archive/demo-0.1.0"),
            ("0.2.0", SUBSTRATE_TARBALL, "aware-main/20-agents/aeco/demo"),
        ],
    );

    aware()
        .current_dir(root)
        .env("AWARE_HOME", home_in(root))
        .args(["agent", "reindex"])
        .assert()
        .success()
        .stdout(predicate::str::contains("1 agents"));

    let cat: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(root.join("registry-catalog.json")).unwrap())
            .unwrap();
    let versions = &cat["agents"]["demo"]["versions"];
    assert_eq!(
        versions["0.1.0"]["description"], "The frozen 0.1.0 build.",
        "the 0.1.0 entry keeps its own subdir's metadata, not the current build's: {cat:#}"
    );
    assert_eq!(versions["0.1.0"]["manifest-version"], "0.1.0");
    assert_eq!(versions["0.2.0"]["description"], "The current 0.2.0 build.");
    assert_eq!(versions["0.2.0"]["manifest-version"], "0.2.0");
}

#[test]
fn reindex_outside_a_checkout_names_the_remedy_and_writes_nothing() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let bare = root.join("not-a-checkout");
    std::fs::create_dir_all(&bare).unwrap();
    assert_no_index_above(&bare);

    aware()
        .current_dir(&bare)
        .env("AWARE_HOME", home_in(root))
        .args(["agent", "reindex"])
        .assert()
        .failure()
        .code(3)
        .stderr(predicate::str::contains("no registry-index.json found"));

    assert!(!bare.join("registry-catalog.json").exists());
}
