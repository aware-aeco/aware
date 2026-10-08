//! `aware app list` with one damaged installed app (#659).
//!
//! One app whose manifest does not parse used to make the whole listing fail:
//! exit 3, nothing on stdout, and the validation error on stderr outside the
//! `--json` envelope. A host UI built on `app list` then showed no apps at all.
//! The readable apps must still be listed, and the damaged one reported by its
//! directory name — inside the envelope — with exit 0.

use std::path::Path;

use assert_cmd::Command;
use serde_json::Value;

fn write_app(home: &Path, dir: &str, body: &str) {
    let app_dir = home.join("apps").join(dir);
    std::fs::create_dir_all(&app_dir).unwrap();
    std::fs::write(app_dir.join(format!("{dir}.flo")), body).unwrap();
}

fn valid(id: &str) -> String {
    format!(
        "app: {id}\nversion: 1.0.0\ndescription: fixture {id}\n\
         nodes: []\nconnections: []\nrequires: []\n"
    )
}

/// Two readable apps and one whose `app:` field is an unclosed YAML sequence —
/// the exact damage from the issue's repro.
fn home_with_one_damaged_app() -> tempfile::TempDir {
    let tmp = tempfile::tempdir().unwrap();
    write_app(tmp.path(), "alpha", &valid("alpha"));
    write_app(tmp.path(), "beta", &valid("beta"));
    write_app(tmp.path(), "broken", "app: [unclosed\n");
    tmp
}

fn aware(home: &Path) -> Command {
    let mut cmd = Command::cargo_bin("aware").unwrap();
    cmd.env("AWARE_HOME", home);
    cmd
}

fn json_stdout(out: &std::process::Output) -> Value {
    let stdout = String::from_utf8_lossy(&out.stdout);
    serde_json::from_str(stdout.trim()).unwrap_or_else(|e| {
        panic!(
            "stdout is not one JSON envelope ({e}): {stdout:?}\nstderr: {}",
            String::from_utf8_lossy(&out.stderr)
        )
    })
}

#[test]
fn json_lists_the_readable_apps_and_reports_the_damaged_one_in_the_envelope() {
    let home = home_with_one_damaged_app();
    let out = aware(home.path())
        .args(["app", "list", "--json"])
        .output()
        .unwrap();
    assert_eq!(
        out.status.code(),
        Some(0),
        "one damaged app must not fail the listing; stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let env = json_stdout(&out);
    assert_eq!(env["ok"], true);
    assert_eq!(env["meta"]["command"], "app list");

    let ids: Vec<&str> = env["data"]["apps"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["id"].as_str().unwrap())
        .collect();
    assert_eq!(ids, ["alpha", "beta"]);

    let invalid = env["data"]["invalid"].as_array().unwrap();
    assert_eq!(invalid.len(), 1, "exactly the damaged app: {invalid:?}");
    let bad = &invalid[0];
    assert_eq!(bad["id"], "broken");
    assert_eq!(bad["code"], "E_APP_MANIFEST_INVALID");
    assert!(
        bad["path"].as_str().unwrap().ends_with("broken.flo"),
        "path names the failing manifest: {bad}"
    );
    let message = bad["message"].as_str().unwrap();
    assert!(
        message.contains("broken.flo") && message.contains("invalid type"),
        "message carries the parse error: {message}"
    );
}

#[test]
fn json_carries_an_empty_invalid_array_when_every_app_loads() {
    let tmp = tempfile::tempdir().unwrap();
    write_app(tmp.path(), "alpha", &valid("alpha"));
    let out = aware(tmp.path())
        .args(["app", "list", "--json"])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(0));
    let env = json_stdout(&out);
    assert_eq!(env["data"]["apps"].as_array().unwrap().len(), 1);
    assert_eq!(env["data"]["invalid"], serde_json::json!([]));
}

#[test]
fn human_output_lists_the_readable_apps_and_names_the_damaged_one() {
    let home = home_with_one_damaged_app();
    let out = aware(home.path()).args(["app", "list"]).output().unwrap();
    assert_eq!(
        out.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("alpha"), "{stdout}");
    assert!(stdout.contains("beta"), "{stdout}");
    let line = stdout
        .lines()
        .find(|l| l.starts_with("unreadable: broken"))
        .unwrap_or_else(|| panic!("no line naming the damaged app: {stdout}"));
    assert!(line.contains("invalid type"), "{line}");
}

/// `doctor` read apps through the same all-or-nothing walk and swallowed the
/// error, so one damaged app made it report "0 installed" for every app.
#[test]
fn doctor_still_counts_the_readable_apps_and_names_the_damaged_one() {
    let home = home_with_one_damaged_app();
    let out = aware(home.path())
        .args(["doctor", "--json"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let report: Value = serde_json::from_slice(&out.stdout).unwrap();
    let ids: Vec<&str> = report["apps"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| a["id"].as_str().unwrap())
        .collect();
    assert_eq!(ids, ["alpha", "beta"]);
    let invalid = report["invalid_apps"].as_array().unwrap();
    assert_eq!(invalid.len(), 1);
    assert_eq!(invalid[0]["id"], "broken");
}

/// By-id resolution falls back to scanning every app's `app:` field when the
/// directory was renamed; that scan must pass over a damaged app rather than
/// fail, or one broken app makes every renamed app unaddressable.
#[test]
fn a_renamed_app_still_resolves_beside_a_damaged_one() {
    let home = home_with_one_damaged_app();
    write_app(home.path(), "gamma-moved", &valid("gamma"));
    let out = aware(home.path())
        .args(["app", "show", "gamma"])
        .output()
        .unwrap();
    assert_eq!(
        out.status.code(),
        Some(0),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(String::from_utf8_lossy(&out.stdout).contains("app:           gamma"));
}

/// The other side of that scan: a renamed app whose manifest is damaged but
/// whose `app:` field is still readable IS installed. Asking for it by id must
/// surface its real load error (exit 3, naming the file), not "not found"
/// (exit 7), which would send the user looking for an app sitting there broken.
#[test]
fn a_renamed_damaged_app_reports_its_load_error_not_not_found() {
    let home = home_with_one_damaged_app();
    write_app(
        home.path(),
        "delta-moved",
        // Well-formed YAML with a readable `app:` field; only `nodes` is the
        // wrong type, so the full load fails while the id is still knowable.
        "app: delta\nversion: 1.0.0\ndescription: fixture\n\
         nodes: 42\nconnections: []\nrequires: []\n",
    );
    let out = aware(home.path())
        .args(["app", "show", "delta"])
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert_eq!(out.status.code(), Some(3), "stderr: {stderr}");
    assert!(stderr.contains("delta-moved.flo"), "{stderr}");
}
