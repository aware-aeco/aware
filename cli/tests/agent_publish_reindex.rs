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
    // No registry-index.json anywhere above the agent.

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
fn reindex_outside_a_checkout_names_the_remedy_and_writes_nothing() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let bare = root.join("not-a-checkout");
    std::fs::create_dir_all(&bare).unwrap();

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
