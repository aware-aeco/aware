//! `file.read` / `file.write` / `file.write-csv` — the generic filesystem IO verbs of the
//! `_core/file` agent (builtin transport, #268). The portable bridge between a composition and the
//! outside filesystem: land a node's output on disk (a report, an export, a binary artifact),
//! read an exported file back into a node, or emit a node's tabular output as an RFC-4180 CSV.
//!
//! Host-free, OS-native — no vendor product, just `std::fs`. Mirrors the `ifc.write` /
//! `viewer-3d.render` output contract: the disk write is gated to a REAL run (skipped under
//! `--dry-run` / `--simulate`), the producer owns the path via `path`, and the writer stays
//! generic (no opinion on the bytes' meaning). The streaming `file.watch` event source (#240) is a
//! separate, per-command `status: planned` until its watcher lands — this module is the read/write
//! half.
//!
//! Determinism: a `write` of identical `bytes` to identical `path` yields identical on-disk bytes;
//! `write-csv` is a pure function of `columns` + `rows`. No clock, no randomness.

use crate::error::AwareError;
use crate::json::type_name as json_type;
use base64::Engine;
use serde_json::Value;

/// The required destination `path`, trimmed; missing/empty/non-string is rejected. (Only `path` —
/// the `folder` key is the `watch` verb's, which this module doesn't handle.)
fn req_path(args: &Value, verb: &str) -> Result<String, AwareError> {
    match args.get("path") {
        Some(Value::String(s)) if !s.trim().is_empty() => Ok(s.trim().to_string()),
        _ => Err(AwareError::Validation(format!(
            "file {verb}: `path` is required (a non-empty destination path)"
        ))),
    }
}

// `abs_path` moved to `render::mod`; imported so this file's call sites (and
// their existing bare `abs_path(...)` calls) still resolve unchanged.
use super::abs_path;

/// Create the parent directory of `path` when `create-dirs` is on (the default).
fn ensure_parent(path: &str, create_dirs: bool) -> Result<(), AwareError> {
    if !create_dirs {
        return Ok(());
    }
    if let Some(parent) = std::path::Path::new(path).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)
            .map_err(|e| AwareError::Internal(format!("file: create {}: {e}", parent.display())))?;
    }
    Ok(())
}

/// Render one JSON value as a CSV cell's raw (pre-escape) text: a string verbatim, a number/bool by
/// its display, null/absent as empty, an array/object as compact JSON (so nothing is silently lost).
fn cell_text(v: Option<&Value>) -> String {
    match v {
        None | Some(Value::Null) => String::new(),
        Some(Value::String(s)) => s.clone(),
        Some(Value::Bool(b)) => b.to_string(),
        Some(Value::Number(n)) => n.to_string(),
        Some(other) => other.to_string(),
    }
}

/// RFC-4180: a field containing a comma, double-quote, CR or LF is wrapped in double-quotes with
/// embedded quotes doubled. (A generic writer — formula-injection neutralization is a consumer
/// policy, applied by the caller before handing cells here.)
fn csv_field(s: &str) -> String {
    if s.contains([',', '"', '\r', '\n']) {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn bool_arg(args: &Value, key: &str, default: bool) -> bool {
    args.get(key).and_then(Value::as_bool).unwrap_or(default)
}

/// `file.write` — land `bytes` at `path`. A string is written as UTF-8 (or, with
/// `encoding: base64`, base64-decoded to raw bytes so a pre-generated binary artifact — a `.xlsx`,
/// an image, a zip — can be persisted through the same verb); any non-string JSON value is
/// serialized to compact JSON text. Returns `{ path, bytes-written }`; the write is gated to a real
/// run (a preview reports the would-be size without touching disk).
pub fn file_write(args: &Value, dry_run: bool) -> Result<Value, AwareError> {
    let path = req_path(args, "write")?;
    let create_dirs = bool_arg(args, "create-dirs", true);
    let encoding = args
        .get("encoding")
        .and_then(Value::as_str)
        .unwrap_or("text");

    let bytes: Vec<u8> = match args.get("bytes") {
        None | Some(Value::Null) => {
            return Err(AwareError::Validation(
                "file write: `bytes` is required (the content to write)".into(),
            ));
        }
        Some(Value::String(s)) => match encoding {
            "text" => s.clone().into_bytes(),
            "base64" => base64::engine::general_purpose::STANDARD
                .decode(s.as_bytes())
                .map_err(|e| {
                    AwareError::Validation(format!("file write: `bytes` is not valid base64: {e}"))
                })?,
            other => {
                return Err(AwareError::Validation(format!(
                    "file write: unknown encoding {other:?} (use `text` or `base64`)"
                )));
            }
        },
        // A non-string JSON value is serialized to compact JSON text (documented behavior).
        Some(other) => serde_json::to_vec(other)
            .map_err(|e| AwareError::Internal(format!("file write: serialize `bytes`: {e}")))?,
    };

    let mut out = serde_json::Map::new();
    out.insert("path".into(), Value::from(abs_path(&path)));
    out.insert("bytes-written".into(), Value::from(bytes.len() as u64));

    if !dry_run {
        ensure_parent(&path, create_dirs)?;
        std::fs::write(&path, &bytes)
            .map_err(|e| AwareError::Internal(format!("file write: {path}: {e}")))?;
    }
    Ok(Value::Object(out))
}

/// `file.write-csv` — emit `rows` as an RFC-4180 CSV at `path` with an explicit `columns` order (the
/// header row). Each row is either an object keyed by the column names or a positional array
/// (matching `columns`); a missing key renders as an empty cell. Returns `{ path, row-count }` (data
/// rows, excluding the header); the write is gated to a real run.
pub fn file_write_csv(args: &Value, dry_run: bool) -> Result<Value, AwareError> {
    let path = req_path(args, "write-csv")?;
    let create_dirs = bool_arg(args, "create-dirs", true);

    let columns: Vec<String> = match args.get("columns") {
        Some(Value::Array(a)) => a.iter().map(|v| cell_text(Some(v))).collect(),
        _ => {
            return Err(AwareError::Validation(
                "file write-csv: `columns` is required (an array of column names)".into(),
            ));
        }
    };
    let empty = Vec::new();
    let rows = match args.get("rows") {
        Some(Value::Array(a)) => a.as_slice(),
        None | Some(Value::Null) => empty.as_slice(),
        Some(other) => {
            return Err(AwareError::Validation(format!(
                "file write-csv: `rows` must be an array (got {})",
                json_type(other)
            )));
        }
    };

    let mut csv = String::new();
    csv.push_str(
        &columns
            .iter()
            .map(|c| csv_field(c))
            .collect::<Vec<_>>()
            .join(","),
    );
    csv.push_str("\r\n");
    let mut row_count: u64 = 0;
    for row in rows {
        let cells: Vec<String> = match row {
            Value::Object(map) => columns
                .iter()
                .map(|c| csv_field(&cell_text(map.get(c))))
                .collect(),
            Value::Array(arr) => {
                // A positional row longer than `columns` would silently drop its tail — refuse it
                // rather than lose data (a shorter row is fine; the missing cells render empty).
                if arr.len() > columns.len() {
                    return Err(AwareError::Validation(format!(
                        "file write-csv: a row has {} cells but there are {} column(s) — extra cells would be dropped",
                        arr.len(),
                        columns.len()
                    )));
                }
                (0..columns.len())
                    .map(|i| csv_field(&cell_text(arr.get(i))))
                    .collect()
            }
            other => {
                return Err(AwareError::Validation(format!(
                    "file write-csv: each row must be an object or array (got {})",
                    json_type(other)
                )));
            }
        };
        csv.push_str(&cells.join(","));
        csv.push_str("\r\n");
        row_count += 1;
    }

    let mut out = serde_json::Map::new();
    out.insert("path".into(), Value::from(abs_path(&path)));
    out.insert("row-count".into(), Value::from(row_count));

    if !dry_run {
        ensure_parent(&path, create_dirs)?;
        std::fs::write(&path, csv.as_bytes())
            .map_err(|e| AwareError::Internal(format!("file write-csv: {path}: {e}")))?;
    }
    Ok(Value::Object(out))
}

/// `file.read` — read a file's contents as UTF-8 text (default) or base64 (for binary). Returns
/// `{ content, bytes, path }`. Read is side-effect-free, but a preview (`--dry-run`/`--simulate`)
/// still must not require the file to exist, so it returns an empty stub without touching disk.
pub fn file_read(args: &Value, dry_run: bool) -> Result<Value, AwareError> {
    let path = req_path(args, "read")?;
    let encoding = args
        .get("encoding")
        .and_then(Value::as_str)
        .unwrap_or("text");
    // Validate the encoding up front so a bad value fails the SAME way (Validation, exit 3) whether
    // previewing or running — never a confusing IO error or a stub that masks the typo on dry-run.
    if encoding != "text" && encoding != "base64" {
        return Err(AwareError::Validation(format!(
            "file read: unknown encoding {encoding:?} (use `text` or `base64`)"
        )));
    }

    if dry_run {
        let mut out = serde_json::Map::new();
        out.insert("content".into(), Value::from(String::new()));
        out.insert("bytes".into(), Value::from(0u64));
        out.insert("path".into(), Value::from(abs_path(&path)));
        return Ok(Value::Object(out));
    }

    let raw = std::fs::read(&path)
        .map_err(|e| AwareError::Internal(format!("file read: {path}: {e}")))?;
    let bytes = raw.len() as u64;
    let content = if encoding == "base64" {
        base64::engine::general_purpose::STANDARD.encode(&raw)
    } else {
        String::from_utf8(raw).map_err(|e| {
            AwareError::Validation(format!(
                "file read: {path} is not valid UTF-8 (use `encoding: base64`): {e}"
            ))
        })?
    };

    let mut out = serde_json::Map::new();
    out.insert("content".into(), Value::from(content));
    out.insert("bytes".into(), Value::from(bytes));
    out.insert("path".into(), Value::from(abs_path(&path)));
    Ok(Value::Object(out))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// An EMPTY temp dir for one test (no extra dev-dep — std + pid + a per-call salt).
    ///
    /// Cleared on entry, which is the whole point. The name is keyed on the process
    /// id and the OS reuses those, so without this a directory left by an earlier
    /// run is handed to a later one — and six tests here assert that some path does
    /// NOT exist. That is not a theoretical staleness: the mutation discipline this
    /// module is tested under *creates* those very files. Running the documented
    /// mutation for `an_unknown_write_encoding_is_refused_rather_than_written_as_text`
    /// leaves `…-wenc/enc.bin` behind, so once the mutation is reverted the test
    /// keeps failing against correct code until someone clears the temp dir — a
    /// false signal that reads as "the fix didn't work".
    ///
    /// Each salt is used by exactly one test, so clearing here cannot race another
    /// test running in parallel.
    fn tmp(salt: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("aware-file-test-{}-{salt}", std::process::id()));
        // Not `create_dir_all` alone: that is a no-op over a stale directory.
        //
        // And not a discarded result either. `create_dir_all` succeeds over a
        // directory that is still there, so swallowing a real removal failure —
        // a locked or read-only entry on Windows, an external deleter racing us —
        // hands the test the dirty fixture this helper exists to prevent, and its
        // absence assertions then fail against correct production code. Only
        // `NotFound` means "already clear"; anything else fails setup loudly.
        match std::fs::remove_dir_all(&d) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => panic!("fixture {d:?} could not be cleared: {e}"),
        }
        std::fs::create_dir_all(&d)
            .unwrap_or_else(|e| panic!("fixture {d:?} could not be created: {e}"));
        d
    }

    #[test]
    fn write_text_creates_parent_and_writes_utf8() {
        let p = tmp("wt").join("nested/out.txt");
        let path = p.to_str().unwrap();
        let res = file_write(&json!({ "path": path, "bytes": "héllo" }), false).unwrap();
        assert_eq!(res["bytes-written"], json!(6)); // é is 2 bytes UTF-8
        assert_eq!(std::fs::read_to_string(path).unwrap(), "héllo");
    }

    #[test]
    fn write_base64_lands_raw_binary() {
        // "PK\x03\x04" — the zip/xlsx magic — base64-encoded.
        let b64 = base64::engine::general_purpose::STANDARD.encode([0x50, 0x4b, 0x03, 0x04]);
        let p = tmp("b64").join("a.bin");
        let path = p.to_str().unwrap();
        let res = file_write(
            &json!({ "path": path, "bytes": b64, "encoding": "base64" }),
            false,
        )
        .unwrap();
        assert_eq!(res["bytes-written"], json!(4));
        assert_eq!(std::fs::read(path).unwrap(), vec![0x50, 0x4b, 0x03, 0x04]);
    }

    #[test]
    fn write_json_value_serializes() {
        let p = tmp("wj").join("o.json");
        let path = p.to_str().unwrap();
        file_write(&json!({ "path": path, "bytes": { "a": 1 } }), false).unwrap();
        assert_eq!(std::fs::read_to_string(path).unwrap(), "{\"a\":1}");
    }

    #[test]
    fn write_dry_run_reports_size_without_touching_disk() {
        let p = tmp("dry").join("never.txt");
        let path = p.to_str().unwrap();
        let res = file_write(&json!({ "path": path, "bytes": "x" }), true).unwrap();
        assert_eq!(res["bytes-written"], json!(1));
        assert!(!std::path::Path::new(path).exists(), "dry-run never writes");
    }

    #[test]
    fn write_rejects_bad_base64_and_missing_bytes() {
        let p = tmp("err").join("x");
        let path = p.to_str().unwrap();
        assert!(file_write(&json!({ "path": path }), false).is_err()); // no bytes
        assert!(
            file_write(
                &json!({ "path": path, "bytes": "not base64!!", "encoding": "base64" }),
                false
            )
            .is_err()
        );
    }

    #[test]
    fn write_csv_quotes_per_rfc4180_objects_and_arrays() {
        let p = tmp("csv").join("bom.csv");
        let path = p.to_str().unwrap();
        let res = file_write_csv(
            &json!({
                "path": path,
                "columns": ["Profile", "Qty"],
                "rows": [
                    { "Profile": "W10X33", "Qty": 3 },
                    { "Profile": "PL,1/2", "Qty": 1 },   // comma → quoted
                    ["He said \"hi\"", 2],               // positional + embedded quote → doubled
                ]
            }),
            false,
        )
        .unwrap();
        assert_eq!(res["row-count"], json!(3));
        let csv = std::fs::read_to_string(path).unwrap();
        let lines: Vec<&str> = csv.split("\r\n").collect();
        assert_eq!(lines[0], "Profile,Qty");
        assert_eq!(lines[1], "W10X33,3");
        assert_eq!(lines[2], "\"PL,1/2\",1");
        assert_eq!(lines[3], "\"He said \"\"hi\"\"\",2");
        assert!(csv.ends_with("\r\n"));
    }

    #[test]
    fn write_csv_rejects_a_row_wider_than_columns() {
        let p = tmp("wide").join("w.csv");
        let path = p.to_str().unwrap();
        let res = file_write_csv(
            &json!({ "path": path, "columns": ["A", "B"], "rows": [["x", "y", "z"]] }),
            false,
        );
        assert!(
            res.is_err(),
            "a 3-cell row against 2 columns must be refused, not truncated"
        );
        assert!(
            !std::path::Path::new(path).exists(),
            "nothing is written on the error"
        );
    }

    #[test]
    fn outputs_an_absolute_path_even_for_a_relative_input() {
        // A relative `path` must be reported absolute (the output contract) so a downstream node can use it.
        let res = file_write(&json!({ "path": "rel-out.txt", "bytes": "x" }), true).unwrap();
        let reported = res["path"].as_str().unwrap();
        assert!(
            std::path::Path::new(reported).is_absolute(),
            "reported path should be absolute, got {reported:?}"
        );
    }

    #[test]
    fn read_rejects_unknown_encoding_on_dry_run_and_live() {
        let p = tmp("enc").join("e.txt");
        let path = p.to_str().unwrap();
        std::fs::write(path, "hi").unwrap();
        assert!(file_read(&json!({ "path": path, "encoding": "utf16" }), true).is_err());
        assert!(file_read(&json!({ "path": path, "encoding": "utf16" }), false).is_err());
    }

    #[test]
    fn read_round_trips_text_and_base64() {
        let p = tmp("rd").join("r.bin");
        let path = p.to_str().unwrap();
        std::fs::write(path, [0x50, 0x4b, 0xff]).unwrap();
        // base64 reads raw bytes back; text would fail on invalid UTF-8.
        let res = file_read(&json!({ "path": path, "encoding": "base64" }), false).unwrap();
        assert_eq!(res["bytes"], json!(3));
        assert_eq!(
            res["content"].as_str().unwrap(),
            base64::engine::general_purpose::STANDARD.encode([0x50, 0x4b, 0xff])
        );
        // dry-run returns an empty stub even if the file exists.
        let dry = file_read(&json!({ "path": path }), true).unwrap();
        assert_eq!(dry["bytes"], json!(0));
    }

    // ── the destination ─────────────────────────────────────────────────────
    //
    // `req_path` is the first statement of all three verbs and nothing pinned
    // it, so every property below could have been deleted with the suite green.

    #[test]
    fn a_path_that_names_no_destination_is_refused_by_every_verb() {
        // The whitespace-only spelling is the one that earns this test. It is a
        // *non-empty* string, so without the `trim()` inside the guard it walks
        // straight through and the bytes land in a file whose name is three
        // spaces — created, reported as a success, and effectively unfindable.
        // The other four spellings are here so the guard cannot be narrowed to
        // the one case a fix happened to be written for.
        //
        // Previewed rather than written, for all three verbs. `req_path` runs
        // before the `if !dry_run` gate, so the guard is tested exactly the same
        // either way — but these paths are deliberately malformed and relative, so
        // if the guard ever regresses a real write lands them in the PROCESS CWD,
        // which is the crate root. That is not hypothetical: it leaves a file
        // actually named `   \t ` sitting untracked in `cli/`, which `git add -A`
        // would commit. A test for a destination guard must not become the thing
        // that writes to an unintended destination.
        for bad in [
            json!(null),
            json!(""),
            json!("   \t "),
            json!(7),
            json!({}),
            json!([]),
        ] {
            assert!(
                file_write(&json!({ "path": bad, "bytes": "x" }), true).is_err(),
                "write accepted path {bad}"
            );
            assert!(
                file_write_csv(&json!({ "path": bad, "columns": ["A"] }), true).is_err(),
                "write-csv accepted path {bad}"
            );
            assert!(
                file_read(&json!({ "path": bad }), true).is_err(),
                "read accepted path {bad}"
            );
        }
        // …and the key being absent altogether, which is a different match arm.
        assert!(file_write(&json!({ "bytes": "x" }), true).is_err());
        assert!(file_write_csv(&json!({ "columns": ["A"] }), true).is_err());
        assert!(file_read(&json!({}), true).is_err());
    }

    #[test]
    fn a_padded_path_is_trimmed_before_it_is_reported_or_written() {
        // The other half of the same `trim()`: padding that surrounds a real
        // path must be stripped rather than carried onto disk. Left in, the
        // leading spaces make the path RELATIVE — `"  /tmp/x"` parses as a
        // directory named `"  "` under the cwd — so the bytes land in an
        // entirely different tree from the one the author named.
        let d = tmp("trim");
        let clean = d.join("padded.txt");
        let padded = format!("  {}  ", clean.display());
        // Reported first, under dry-run, so this half touches no disk at all.
        let dry = file_write(&json!({ "path": &padded, "bytes": "x" }), true).unwrap();
        assert_eq!(dry["path"].as_str().unwrap(), clean.to_str().unwrap());
        // Then for real: the bytes are at the unpadded path.
        file_write(&json!({ "path": &padded, "bytes": "x" }), false).unwrap();
        assert_eq!(std::fs::read_to_string(&clean).unwrap(), "x");
    }

    #[test]
    fn create_dirs_off_refuses_to_invent_the_parent_directory() {
        // `create-dirs` defaults ON, so every existing test exercises the same
        // branch and the flag could have been ignored entirely. Turning it off
        // is the only way to observe it — and the failure it must produce is a
        // refusal, not a directory tree the author asked not to have.
        let d = tmp("cdirs");
        let missing = d.join("no-such-dir");
        let target = missing.join("out.txt");
        let p = target.to_str().unwrap();

        // `tmp` keys its directory on the process id, which the OS reuses, and
        // this test ends by creating `missing` for real. A later run that
        // inherits the same id would therefore find the "missing" parent
        // already on disk and fail against a correct implementation. Clear what
        // an earlier run left before asserting the parent is absent — this test
        // is the only one here that asserts the absence of a path it goes on to
        // create, so it is the only one that cannot start from a reused fixture.
        let _ = std::fs::remove_dir_all(&missing);
        assert!(
            !missing.exists(),
            "fixture from an earlier run survived; the refusals below would not \
             be the flag talking"
        );

        assert!(
            file_write(
                &json!({ "path": p, "bytes": "x", "create-dirs": false }),
                false
            )
            .is_err()
        );
        assert!(
            !missing.exists(),
            "write created the parent behind the flag"
        );
        assert!(
            file_write_csv(
                &json!({ "path": p, "columns": ["A"], "create-dirs": false }),
                false
            )
            .is_err()
        );
        assert!(
            !missing.exists(),
            "write-csv created the parent behind the flag"
        );

        // With the default the very same call succeeds, so the two refusals
        // above are the flag talking rather than a broken fixture path.
        file_write(&json!({ "path": p, "bytes": "x" }), false).unwrap();
        assert!(target.is_file());
    }

    // ── RFC-4180 quoting ────────────────────────────────────────────────────

    #[test]
    fn a_cell_carrying_a_line_break_is_quoted_so_the_record_survives() {
        // `csv_field` quotes on four characters; the suite covered two. An
        // unquoted embedded CR or LF ends the record early, so one row becomes
        // two and every column after it shifts — a corruption a reader reports
        // as valid CSV. Asserted as the whole file, because the bug is in what
        // lies BETWEEN the cells.
        let d = tmp("crlf");
        let path = d.join("multiline.csv");
        let p = path.to_str().unwrap();
        file_write_csv(
            &json!({
                "path": p,
                "columns": ["Note", "Qty"],
                "rows": [
                    { "Note": "line one\nline two", "Qty": 1 },
                    { "Note": "carriage\rreturn", "Qty": 2 },
                ]
            }),
            false,
        )
        .unwrap();
        assert_eq!(
            std::fs::read_to_string(p).unwrap(),
            "Note,Qty\r\n\"line one\nline two\",1\r\n\"carriage\rreturn\",2\r\n"
        );
    }

    #[test]
    fn a_column_name_needing_quotes_is_quoted_in_the_header_too() {
        // The header is built by its own `join`, separate from the row loop, so
        // it can lose the escaping the rows keep. A comma in a column name would
        // then give the header more fields than the records beneath it, and a
        // reader would bind every value to the wrong column.
        let d = tmp("hdr");
        let path = d.join("head.csv");
        let p = path.to_str().unwrap();
        file_write_csv(
            &json!({
                "path": p,
                "columns": ["Profile, mm", "Say \"hi\"", "plain"],
                "rows": [{ "plain": "v" }]
            }),
            false,
        )
        .unwrap();
        let csv = std::fs::read_to_string(p).unwrap();
        assert_eq!(
            csv.split("\r\n").next().unwrap(),
            "\"Profile, mm\",\"Say \"\"hi\"\"\",plain"
        );
        // The row is still addressed by the RAW column name, not the quoted one.
        assert_eq!(csv.split("\r\n").nth(1).unwrap(), ",,v");
    }

    // ── what a cell may hold ────────────────────────────────────────────────

    #[test]
    fn every_json_shape_becomes_a_cell_and_an_absent_key_is_blank() {
        // `cell_text`'s entire contract in one record. The two arms that matter
        // most are the ends: null / absent must be EMPTY (a literal "null" in a
        // spreadsheet reads as data), and a nested array or object must survive
        // as compact JSON rather than being dropped on the floor.
        let d = tmp("cells");
        let path = d.join("shapes.csv");
        let p = path.to_str().unwrap();
        file_write_csv(
            &json!({
                "path": p,
                "columns": ["s", "n", "b", "nul", "arr", "obj", "absent"],
                "rows": [{
                    "s": "x", "n": 2.5, "b": true, "nul": null,
                    "arr": [1, 2], "obj": { "k": "v" }
                }]
            }),
            false,
        )
        .unwrap();
        let csv = std::fs::read_to_string(p).unwrap();
        assert_eq!(
            csv.split("\r\n").nth(1).unwrap(),
            "x,2.5,true,,\"[1,2]\",\"{\"\"k\"\":\"\"v\"\"}\","
        );
    }

    #[test]
    fn a_positional_row_shorter_than_its_columns_pads_rather_than_shifting() {
        // The cell loop runs over `columns`, not over the row. Running it over
        // the row instead emits a SHORT record, and a reader then either rejects
        // the file or pulls the next row's leading values up into it. The
        // over-long row is already refused; this is the other side of it.
        let d = tmp("short");
        let path = d.join("short.csv");
        let p = path.to_str().unwrap();
        let res = file_write_csv(
            &json!({ "path": p, "columns": ["A", "B", "C"], "rows": [["x"], []] }),
            false,
        )
        .unwrap();
        assert_eq!(res["row-count"], json!(2));
        assert_eq!(
            std::fs::read_to_string(p).unwrap(),
            "A,B,C\r\nx,,\r\n,,\r\n"
        );
    }

    // ── the shapes `write-csv` refuses ──────────────────────────────────────

    #[test]
    fn rows_that_are_not_records_are_refused_and_no_rows_is_a_header_only_file() {
        let d = tmp("rows");
        // Absent and explicit-null `rows` are legal and mean "header only" — a
        // real result (an empty export) rather than an error.
        for (salt, args) in [
            ("absent", json!({ "columns": ["A", "B"] })),
            ("null", json!({ "columns": ["A", "B"], "rows": null })),
        ] {
            let path = d.join(format!("{salt}.csv"));
            let mut args = args;
            args["path"] = json!(path.to_str().unwrap());
            let res = file_write_csv(&args, false).unwrap();
            assert_eq!(res["row-count"], json!(0), "{salt}");
            assert_eq!(std::fs::read_to_string(&path).unwrap(), "A,B\r\n", "{salt}");
        }

        // A `rows` that is not an array at all, named by type.
        let path = d.join("bad.csv");
        let p = path.to_str().unwrap();
        let err = file_write_csv(
            &json!({ "path": p, "columns": ["A"], "rows": { "A": 1 } }),
            false,
        )
        .unwrap_err();
        assert!(
            format!("{err}").contains("`rows` must be an array (got object)"),
            "{err}"
        );

        // An array whose ELEMENTS are not records. Each must stop the write —
        // rendering a scalar as a one-cell row would quietly produce a file
        // whose shape nobody asked for.
        for bad in [json!(42), json!("x"), json!(null), json!(true)] {
            assert!(
                file_write_csv(
                    &json!({ "path": p, "columns": ["A"], "rows": [bad] }),
                    false
                )
                .is_err(),
                "a row of {bad} was accepted"
            );
        }
        assert!(!path.exists(), "a refused write leaves nothing behind");
    }

    #[test]
    fn columns_is_required_and_must_be_a_list() {
        // Without `columns` there is no header and no cell order, so defaulting
        // it to empty would emit a blank header over blank records and call that
        // a successful export.
        let d = tmp("cols");
        let path = d.join("cols.csv");
        let p = path.to_str().unwrap();
        for bad in [json!(null), json!("A,B"), json!({ "A": 1 }), json!(3)] {
            assert!(
                file_write_csv(&json!({ "path": p, "columns": bad }), false).is_err(),
                "columns {bad} was accepted"
            );
        }
        assert!(file_write_csv(&json!({ "path": p }), false).is_err());
        assert!(!path.exists());
    }

    // ── previews never touch disk ───────────────────────────────────────────

    #[test]
    fn write_csv_dry_run_counts_the_rows_without_touching_disk() {
        // The same gate `file_write` has, on the verb that had no test for it.
        let d = tmp("csvdry");
        let path = d.join("never.csv");
        let p = path.to_str().unwrap();
        let res = file_write_csv(
            &json!({ "path": p, "columns": ["A"], "rows": [{ "A": 1 }, { "A": 2 }] }),
            true,
        )
        .unwrap();
        assert_eq!(res["row-count"], json!(2));
        assert!(!path.exists(), "dry-run never writes");
    }

    // ── encodings ───────────────────────────────────────────────────────────

    #[test]
    fn an_unknown_write_encoding_is_refused_rather_than_written_as_text() {
        // `read` pins this; `write` did not. Falling through to `text` writes
        // the base64 SOURCE — the literal characters `UEsDBA==` — into a file
        // the author expected to hold four bytes of zip header, and reports the
        // wrong length as a success.
        let d = tmp("wenc");
        let path = d.join("enc.bin");
        let p = path.to_str().unwrap();
        let err = file_write(
            &json!({ "path": p, "bytes": "UEsDBA==", "encoding": "utf16" }),
            false,
        )
        .unwrap_err();
        assert!(format!("{err}").contains("unknown encoding"), "{err}");
        assert!(!path.exists(), "nothing is written on the refusal");
    }

    #[test]
    fn a_binary_file_read_as_text_is_refused_and_says_to_use_base64() {
        // `read_round_trips_text_and_base64` reads these bytes as base64 only;
        // its comment claims text "would fail" without ever asking. A lossy
        // decode would instead hand back a string of U+FFFD and a byte count
        // that no longer matches it — a corrupt read wearing a success.
        let d = tmp("renc");
        let path = d.join("bin.dat");
        let p = path.to_str().unwrap();
        std::fs::write(p, [0x50, 0x4b, 0xff]).unwrap();
        let err = file_read(&json!({ "path": p }), false).unwrap_err();
        assert!(
            matches!(err, AwareError::Validation(_)),
            "the caller chose the wrong encoding, which is their error, not an internal one: {err:?}"
        );
        let msg = format!("{err}");
        assert!(msg.contains("not valid UTF-8"), "{msg}");
        assert!(msg.contains("base64"), "{msg}");
    }
}
