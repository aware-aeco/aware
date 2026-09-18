pub mod blender;
pub mod file;
pub mod geom;
pub mod html_report;
pub mod ifc;
pub mod scene_roll;
pub mod table;
pub mod topology;
pub mod ui;
pub mod viewer_3d;

/// Resolve a (possibly relative) path to an absolute string for a render
/// primitive's output contract: joins the current dir without resolving
/// symlinks or requiring the file to exist. Falls back to the input on the
/// rare `absolute()` error so a downstream node always gets a usable path.
///
/// `render::file` and `render::blender` each previously carried a byte-
/// identical copy (`blender`'s doc comment even flagged the duplication).
/// Extracted here so a future "resolve symlinks" or "reject relative"
/// decision lands in one place.
pub(super) fn abs_path(path: &str) -> String {
    std::path::absolute(path)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| path.to_string())
}

/// Resolve the optional `output-path` argument: `Some(destination)` when an
/// artifact was asked for, `None` when one was not, and a refusal when the value
/// is not a string at all.
///
/// Absent, `null` and a blank string all mean "no artifact", and each for its
/// own reason. `null` is JSON's spelling of nothing — a YAML `output-path:` with
/// no value parses to exactly that. A blank string is what an *unresolved*
/// optional whole-value `{{ }}` ref renders to, deliberately and not as null
/// (#205, pinned by `render_config_unresolved_whole_value_falls_back_to_empty_string`).
/// Refusing either would turn "the author left it out" into a failed run.
///
/// A number, boolean, array or object is none of those things. Every manifest
/// that reaches here declares `output-path: {type: string}` —
/// `20-agents/_core/{html-report,ui,viewer-3d,ifc}/manifest.yaml` — so a
/// non-string is a typed mistake, and reading it as an opt-out loses the
/// artifact in silence: the node returns no `path` key, no file is written, and
/// the run exits 0. Refusing it enforces the published contract rather than
/// changing it, and closes a direct asymmetry with `render::file`, whose
/// `req_path` already refuses a non-string `path` (#549 item 5).
fn output_path_arg<'a>(
    args: &'a serde_json::Value,
    label: &str,
) -> Result<Option<&'a str>, crate::error::AwareError> {
    use crate::error::AwareError;
    use serde_json::Value;

    match args.get("output-path") {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(s)) => Ok(Some(s.trim()).filter(|s| !s.is_empty())),
        Some(other) => Err(AwareError::Validation(format!(
            "{label}: `output-path` must be a string (got {})",
            crate::json::type_name(other)
        ))),
    }
}

/// The optional `output-path` half of a render primitive's output contract:
/// write the rendered artifact when one was asked for, and stamp the location
/// and size onto the response.
///
/// Every primitive that produces an artifact — `html-report.render`,
/// `ui.render`, `viewer-3d.render`, `ifc.write` — offers the same deal:
/// `output-path` is optional, a real run writes the bytes there (creating
/// parents), and a preview (`--dry-run` / `--simulate`) reports the would-be
/// path and size without touching disk. Four byte-for-byte copies of that
/// block existed, and `viewer_3d`'s module doc already described itself as
/// mirroring the other two.
///
/// They had drifted, which is the reason to collapse them rather than leave
/// them alone: three emitted the documented `path` alias beside `output-path`
/// and `ui.render` emitted only `output-path`, so an app reading
/// `{{ node.path }}` — the spelling an engineering output seal uses — got a
/// value from a `viewer-3d` node and nothing from a `ui` node. Nothing chose
/// that; the fourth copy was simply written without the line. One
/// implementation is what stops the next divergence.
///
/// `label` is the primitive's name for its I/O errors ("ifc: write …"), which
/// is data rather than a mode switch: no caller changes *behaviour* through
/// this function, so none of them needs a flag to get its old output back.
/// `contents` is the artifact's bytes — HTML for the three renderers, the IFC
/// document for `ifc.write` — so the helper stays indifferent to what was
/// rendered, per decalog #4: the substrate writes a file, it does not know the
/// format.
pub(super) fn write_artifact(
    out: &mut serde_json::Map<String, serde_json::Value>,
    args: &serde_json::Value,
    dry_run: bool,
    contents: &[u8],
    label: &str,
) -> Result<(), crate::error::AwareError> {
    use crate::error::AwareError;
    use serde_json::Value;

    // Resolved before `dry_run` is consulted: a bad value fails the same way
    // whether previewing or running, so a `--dry-run` never green-lights a
    // config the real run would refuse.
    let Some(path) = output_path_arg(args, label)? else {
        return Ok(());
    };

    // Real run only: a preview returns the would-be path and size but never
    // touches disk.
    if !dry_run {
        if let Some(parent) = std::path::Path::new(path).parent()
            && !parent.as_os_str().is_empty()
        {
            std::fs::create_dir_all(parent).map_err(|e| {
                AwareError::Internal(format!("{label}: create {}: {e}", parent.display()))
            })?;
        }
        std::fs::write(path, contents)
            .map_err(|e| AwareError::Internal(format!("{label}: write {path}: {e}")))?;
    }

    out.insert("output-path".into(), Value::String(path.to_string()));
    // `path` alias: existing apps reference the artifact under both names —
    // `{{ node.output-path }}` (e.g. an email attachment) and `{{ node.path }}`
    // (e.g. an engineering output seal). Both are declared in the manifest
    // schema of every primitive that reaches here.
    out.insert("path".into(), Value::String(path.to_string()));
    out.insert("bytes".into(), Value::from(contents.len() as u64));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};

    /// Run the helper the way a primitive does and hand back the response map.
    fn call(args: Value, dry_run: bool, contents: &[u8]) -> serde_json::Map<String, Value> {
        let mut out = serde_json::Map::new();
        write_artifact(&mut out, &args, dry_run, contents, "test").unwrap();
        out
    }

    /// The three spellings of "no artifact", each deliberate: the key absent, an
    /// explicit `null` (what a valueless YAML `output-path:` parses to), and a
    /// blank string (what an unresolved optional `{{ }}` ref renders to, #205).
    /// None may become a refusal — that would fail runs the author wrote
    /// correctly.
    #[test]
    fn no_output_path_leaves_the_response_untouched() {
        for args in [
            json!({}),
            json!({ "output-path": null }),
            json!({ "output-path": "" }),
            json!({"output-path": "   "}),
        ] {
            let out = call(args.clone(), false, b"x");
            assert!(
                out.is_empty(),
                "{args} asks for no artifact, so no path/bytes keys: {out:?}"
            );
        }
    }

    /// #549 item 5: a non-string `output-path` was read as "the author asked for
    /// no artifact", so the write was skipped, the response carried no `path`
    /// key, and the run exited 0 — 100% artifact loss reported as success. Every
    /// manifest reaching here declares `output-path: {type: string}`, so these
    /// are typed mistakes, not opt-outs.
    ///
    /// Asserted per shape rather than in one lump: a guard that refuses only
    /// *some* non-strings would pass a test that checked a single fixture.
    #[test]
    fn a_non_string_output_path_is_refused_rather_than_read_as_no_artifact() {
        for (value, want_type) in [
            (json!(123), "number"),
            (json!(1.5), "number"),
            (json!(true), "boolean"),
            (json!(["x"]), "array"),
            (json!({ "p": "x" }), "object"),
        ] {
            // The refusal does not depend on the mode: a preview must not
            // green-light a config the real run would reject.
            for dry_run in [true, false] {
                let mut out = serde_json::Map::new();
                let err = write_artifact(
                    &mut out,
                    &json!({ "output-path": value.clone() }),
                    dry_run,
                    b"REPORT",
                    "html-report",
                )
                .expect_err("{value} is not a path, and skipping the write loses the artifact");

                assert_eq!(
                    err.exit_code(),
                    3,
                    "a caller mistake is Validation (exit 3), not Internal: {err}"
                );
                let msg = err.to_string();
                assert!(
                    msg.contains(&format!("(got {want_type})")),
                    "the message must name the shape that arrived: {msg}"
                );
                assert!(
                    msg.contains("html-report: `output-path`"),
                    "the message must name the primitive and the argument: {msg}"
                );
                assert!(
                    out.is_empty(),
                    "a refused write reports nothing it did not do: {out:?}"
                );
            }
        }
    }

    /// The refusal is about the *type*, not about the key being unwelcome: a
    /// well-formed string at the same key still writes. Without this, a guard
    /// that rejected `output-path` outright would pass the test above.
    #[test]
    fn a_string_output_path_still_writes_after_the_type_guard() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("report.html");
        let ps = path.to_string_lossy().to_string();

        let out = call(json!({ "output-path": ps.clone() }), false, b"REPORT");

        assert_eq!(std::fs::read(&path).unwrap(), b"REPORT");
        assert_eq!(out["output-path"], json!(ps));
    }

    #[test]
    fn a_dry_run_reports_the_path_and_size_without_writing() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("sub").join("a.html");
        let ps = path.to_string_lossy().to_string();

        let out = call(json!({ "output-path": ps.clone() }), true, b"hello");

        assert_eq!(out["output-path"], json!(ps));
        assert_eq!(out["bytes"], json!(5));
        assert!(!path.exists(), "a dry run must not write the file");
        assert!(
            !path.parent().unwrap().exists(),
            "nor create its parent directory"
        );
    }

    #[test]
    fn a_real_run_creates_parents_and_writes_the_bytes() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("sub").join("deeper").join("a.ifc");
        let ps = path.to_string_lossy().to_string();

        let out = call(json!({ "output-path": ps }), false, b"ISO-10303-21;");

        assert_eq!(std::fs::read(&path).unwrap(), b"ISO-10303-21;");
        assert_eq!(out["bytes"], json!(13));
    }

    /// The drift this helper exists to end: `html-report`, `viewer-3d` and `ifc`
    /// emitted `path` beside `output-path`; `ui.render`'s copy of the block was
    /// written without it, so `{{ node.path }}` resolved against three of the
    /// four artifact producers and silently against the fourth. One
    /// implementation means one answer.
    #[test]
    fn path_is_always_an_alias_of_output_path() {
        let tmp = tempfile::tempdir().unwrap();
        let ps = tmp.path().join("a.html").to_string_lossy().to_string();

        for dry_run in [true, false] {
            let out = call(json!({ "output-path": ps.clone() }), dry_run, b"x");
            assert_eq!(out["path"], out["output-path"], "dry_run={dry_run}");
        }
    }

    /// `output-path` is echoed as given, minus surrounding whitespace — the file
    /// is written to the trimmed path, so reporting the untrimmed one would name
    /// a location that does not exist.
    #[test]
    fn a_padded_path_is_trimmed_in_both_the_write_and_the_report() {
        let tmp = tempfile::tempdir().unwrap();
        let path = tmp.path().join("a.html");
        let ps = path.to_string_lossy().to_string();

        let out = call(json!({ "output-path": format!("  {ps}  ") }), false, b"x");

        assert_eq!(out["output-path"], json!(ps));
        assert!(path.exists());
    }

    /// The label is the caller's name for its own I/O errors, and nothing else:
    /// it never changes which keys come back or whether a write happens.
    #[test]
    fn the_label_only_names_the_primitive_in_an_io_error() {
        let tmp = tempfile::tempdir().unwrap();
        // A path whose parent is an existing *file* cannot be created.
        let blocker = tmp.path().join("not-a-dir");
        std::fs::write(&blocker, b"").unwrap();
        let doomed = blocker.join("a.html").to_string_lossy().to_string();

        let mut out = serde_json::Map::new();
        let err = write_artifact(
            &mut out,
            &json!({ "output-path": doomed }),
            false,
            b"x",
            "ifc",
        )
        .unwrap_err();

        assert!(err.to_string().contains("ifc: "), "{err}");
        assert!(
            out.is_empty(),
            "a failed write reports nothing it did not do: {out:?}"
        );
    }
}
