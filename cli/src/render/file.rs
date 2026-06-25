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
use base64::Engine;
use serde_json::Value;

fn json_type(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

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

/// Resolve a (possibly relative) path to an absolute string for the output contract — joins the
/// current dir without resolving symlinks or requiring the file to exist; falls back to the input
/// on the rare error so a downstream node always gets a usable path.
fn abs_path(path: &str) -> String {
    std::path::absolute(path)
        .map(|p| p.to_string_lossy().into_owned())
        .unwrap_or_else(|_| path.to_string())
}

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

    /// A unique temp dir for one test (no extra dev-dep — std + pid + a per-call salt).
    fn tmp(salt: &str) -> std::path::PathBuf {
        let d = std::env::temp_dir().join(format!("aware-file-test-{}-{salt}", std::process::id()));
        std::fs::create_dir_all(&d).unwrap();
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
}
