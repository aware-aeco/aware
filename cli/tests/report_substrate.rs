//! `aware report substrate` — the command path around `render_substrate_html`.
//!
//! The renderer's own behaviour is unit-tested in `src/commands/report.rs`.
//! What only shows up out here is the plumbing: does `--output` actually put
//! the document on disk (and *not* on stdout), does the bare form put it on
//! stdout (and not on disk), and does the report reflect the substrate that
//! was discovered rather than an empty one.

mod common;

use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn writes_the_document_to_the_output_path_and_keeps_it_off_stdout() {
    let home = common::aware_home();
    let tmp = tempfile::tempdir().unwrap();
    let out = tmp.path().join("substrate.html");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["report", "substrate", "--output"])
        .arg(&out)
        .assert()
        .success()
        // The confirmation line, not the report itself.
        .stdout(predicate::str::contains("wrote substrate report to"))
        .stdout(predicate::str::contains("<!DOCTYPE html>").not());

    let html = std::fs::read_to_string(&out).expect("report written to --output path");
    assert!(html.starts_with("<!DOCTYPE html>"), "not a document");
    assert!(
        html.trim_end().ends_with("</body></html>"),
        "document truncated"
    );
    assert!(
        html.contains("<style>") && html.contains("<script>"),
        "CSS/JS not inlined — the report is not self-contained"
    );
}

#[test]
fn the_bare_form_prints_the_document_and_writes_no_file() {
    let home = common::aware_home();
    let tmp = tempfile::tempdir().unwrap();

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .current_dir(tmp.path())
        .args(["report", "substrate"])
        .assert()
        .success()
        .stdout(predicate::str::starts_with("<!DOCTYPE html>"))
        .stdout(predicate::str::contains("</body></html>"))
        .stdout(predicate::str::contains("wrote substrate report to").not());

    assert_eq!(
        std::fs::read_dir(tmp.path()).unwrap().count(),
        0,
        "stdout form left a file behind"
    );
}

#[test]
fn the_report_describes_the_agents_that_were_discovered() {
    // An empty `<main>` is the failure mode that still exits 0 and still looks
    // like a valid document, so assert the substrate actually reached the page.
    let home = common::aware_home();
    let installed = std::fs::read_dir(home.join("agents")).unwrap().count();
    assert!(installed > 0, "fixture has no agents; test proves nothing");

    let tmp = tempfile::tempdir().unwrap();
    let out = tmp.path().join("substrate.html");
    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["report", "substrate", "--output"])
        .arg(&out)
        .assert()
        .success();

    let html = std::fs::read_to_string(&out).unwrap();
    let sections = html.matches(r#"<details class="agent""#).count();
    assert_eq!(
        sections, installed,
        "report rendered {sections} agent sections for {installed} installed agents"
    );
    assert!(
        html.contains(&format!("<b>{installed}</b> agents")),
        "header count disagrees with the rendered sections"
    );
    // Derive the names to look for from the fixture rather than pinning one.
    // Naming a repository agent (`tekla`) coupled this command-plumbing test to
    // substrate content: renaming or retiring that agent would fail it even
    // though the report still rendered every discovered agent correctly. The
    // rendered name is the manifest's `agent:` field, which is what
    // `render_agent_section` puts in the `aname` span.
    let mut rendered_ids: Vec<String> = std::fs::read_dir(home.join("agents"))
        .unwrap()
        .filter_map(|entry| {
            let text = std::fs::read_to_string(entry.ok()?.path().join("manifest.yaml")).ok()?;
            let manifest: serde_yaml::Value = serde_yaml::from_str(&text).ok()?;
            Some(manifest.get("agent")?.as_str()?.to_string())
        })
        .collect();
    rendered_ids.sort();
    assert_eq!(
        rendered_ids.len(),
        installed,
        "could not read an `agent:` id for every fixture agent"
    );
    for id in &rendered_ids {
        assert!(
            html.contains(&format!(r#"<span class="aname">{id}</span>"#)),
            "discovered agent `{id}` is missing from the report"
        );
    }
}

#[test]
fn an_output_path_under_a_missing_directory_fails_instead_of_reporting_success() {
    let home = common::aware_home();
    let tmp = tempfile::tempdir().unwrap();
    // A missing parent directory, deliberately, rather than a `chmod 000` one:
    // `fs::write` will not create parents, and ENOENT is not a permission
    // check — so this still fails when the suite runs as root, which a
    // permission-based fixture would not.
    let out = tmp
        .path()
        .join("no")
        .join("such")
        .join("dir")
        .join("r.html");

    Command::cargo_bin("aware")
        .unwrap()
        .env("AWARE_HOME", home)
        .args(["report", "substrate", "--output"])
        .arg(&out)
        .assert()
        .failure()
        .stdout(predicate::str::contains("wrote substrate report to").not());
}
