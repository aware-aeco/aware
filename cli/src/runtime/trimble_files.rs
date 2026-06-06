//! Trimble Connect file upload / download (#200).
//!
//! TC files are multi-step, binary, cross-domain flows the single-call REST transport
//! cannot express (see `20-agents/aeco/construction/trimble-connect/skills/files.md`):
//!
//! - **upload** (3-step package upload): `POST files/fs/upload` (auth) to initiate →
//!   `PUT` the bytes to a pre-signed S3 URL (a *different* domain, **no** auth header) →
//!   `GET files/fs/upload?uploadId=…&wait=true` (auth) to complete.
//! - **download** (2-step): `GET files/fs/{id}/downloadurl` (auth) → `GET` the bytes from
//!   the pre-signed URL (**no** auth header).
//!
//! So `trimble-connect`'s `upload`/`download` commands are handled here, dispatched from
//! [`super::invoker`]'s REST path. Auth (with refresh, #198) and the base URL resolve
//! exactly as the generic REST path does. Binary travels as base64 over JSON.

use std::io::Read;
use std::path::{Path, PathBuf};

use base64::Engine;
use serde_json::{Value, json};

use crate::error::AwareError;
use crate::runtime::invoker::{percent_encode_path, resolve_rest_credential, rest_base_url};

const TC_AGENT: &str = "trimble-connect";

/// `trimble-connect.upload` — 3-step package upload. Blocking HTTP runs off the reactor.
pub async fn upload(agents_dir: PathBuf, args: Value) -> Result<Value, AwareError> {
    tokio::task::spawn_blocking(move || upload_blocking(&agents_dir, &args))
        .await
        .map_err(|e| AwareError::Internal(format!("trimble upload task join: {e}")))?
}

/// `trimble-connect.download` — 2-step pre-signed download. Blocking HTTP off the reactor.
pub async fn download(agents_dir: PathBuf, args: Value) -> Result<Value, AwareError> {
    tokio::task::spawn_blocking(move || download_blocking(&agents_dir, &args))
        .await
        .map_err(|e| AwareError::Internal(format!("trimble download task join: {e}")))?
}

/// Resolve the (refreshed, #198) bearer token + trimmed base URL for trimble-connect.
fn auth_and_base(agents_dir: &Path) -> Result<(String, String), AwareError> {
    let manifest =
        crate::manifest::loader::load_agent(&agents_dir.join(TC_AGENT).join("manifest.yaml"))?;
    let auth = manifest.auth.ok_or_else(|| {
        AwareError::Validation("trimble-connect: manifest has no `auth:` block".into())
    })?;
    let token = resolve_rest_credential(agents_dir, &auth).ok_or_else(|| {
        AwareError::Validation(
            "trimble-connect declares `auth` but the credential is missing — \
             run `aware connect trimble-connect`"
                .into(),
        )
    })?;
    let base = rest_base_url(agents_dir, TC_AGENT).ok_or_else(|| {
        AwareError::Validation("trimble-connect: no `transport.rest.base`".into())
    })?;
    Ok((token, base.trim_end_matches('/').to_string()))
}

fn upload_blocking(agents_dir: &Path, args: &Value) -> Result<Value, AwareError> {
    let (token, base) = auth_and_base(agents_dir)?;
    let folder_id = str_arg(args, "folder-id")?;
    let filename = str_arg(args, "filename")?;
    let bytes = extract_bytes(args.get("bytes"))?;

    // Step 1 — initiate. `parentId`/`parentType` are QUERY params; the body is just
    // name + a single empty content slot (per files.md; multipart returns 415).
    let initiate_url = format!(
        "{base}/files/fs/upload?parentId={}&parentType=FOLDER",
        percent_encode_path(folder_id)
    );
    let init = post_json(
        &initiate_url,
        &token,
        &json!({ "name": filename, "contents": [ {} ] }),
        "upload initiate",
    )?;
    // A content-identical file already present: TC returns DUPLICATE + the fileId.
    if init.get("status").and_then(|v| v.as_str()) == Some("DUPLICATE") {
        let file_id = init
            .get("fileId")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AwareError::Network("upload DUPLICATE: no fileId".into()))?
            .to_string();
        // The Files API only guarantees fileId on DUPLICATE, but the result contract
        // promises a usable version-id — fetch it from the file metadata when the
        // initiate response doesn't carry it (#200 Codex).
        let version_id = match init.get("versionId").and_then(|v| v.as_str()) {
            Some(v) => v.to_string(),
            None => {
                let meta = get_json(
                    &format!("{base}/files/{}", percent_encode_path(&file_id)),
                    Some(&token),
                    "duplicate file metadata",
                )?;
                meta.get("versionId")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| {
                        AwareError::Network("duplicate file metadata: no versionId".into())
                    })?
                    .to_string()
            }
        };
        return Ok(json!({
            "file-id": file_id,
            "version-id": version_id,
            "replaced": true,
            "status": "DUPLICATE",
        }));
    }
    let upload_id = init
        .get("uploadId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AwareError::Network("upload initiate: no uploadId".into()))?;
    let presigned = init
        .get("contents")
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("url"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| AwareError::Network("upload initiate: no pre-signed url".into()))?;

    // Step 2 — PUT bytes to the pre-signed S3 URL (different domain — NO auth header).
    check_ok(
        ureq::put(presigned)
            .set("Content-Type", "application/octet-stream")
            .send_bytes(&bytes),
        "upload PUT",
    )?;

    // Step 3 — complete.
    let complete_url = format!(
        "{base}/files/fs/upload?uploadId={}&wait=true",
        percent_encode_path(upload_id)
    );
    let complete = get_json(&complete_url, Some(&token), "upload complete")?;
    // Fail fast on an incomplete 2xx (e.g. a status-only response) rather than
    // reporting success with empty identifiers downstream nodes would persist (#200 Codex).
    let file_id = complete
        .get("fileId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AwareError::Network("upload complete: no fileId".into()))?;
    let version_id = complete
        .get("versionId")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AwareError::Network("upload complete: no versionId".into()))?;
    Ok(json!({
        "file-id": file_id,
        "version-id": version_id,
        "replaced": false,
    }))
}

fn download_blocking(agents_dir: &Path, args: &Value) -> Result<Value, AwareError> {
    let (token, base) = auth_and_base(agents_dir)?;
    let file_id = str_arg(args, "file-id")?;

    // Step 1 — pre-signed download URL (optionally for a specific version).
    let mut url_endpoint = format!(
        "{base}/files/fs/{}/downloadurl",
        percent_encode_path(file_id)
    );
    if let Some(ver) = args
        .get("version-id")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
    {
        url_endpoint.push_str(&format!("?versionId={}", percent_encode_path(ver)));
    }
    let url_json = get_json(&url_endpoint, Some(&token), "download url")?;
    let presigned = url_json
        .get("url")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AwareError::Network("download: no pre-signed url".into()))?;

    // Step 2 — GET the bytes from the pre-signed URL (NO auth header).
    let resp = ureq::get(presigned)
        .call()
        .map_err(|e| AwareError::Network(format!("download GET: {e}")))?;
    let mut bytes = Vec::new();
    resp.into_reader()
        .read_to_end(&mut bytes)
        .map_err(|e| AwareError::Network(format!("download read: {e}")))?;
    Ok(json!({
        "bytes": base64::engine::general_purpose::STANDARD.encode(&bytes),
        "encoding": "base64",
        "size-kb": (bytes.len() as f64) / 1024.0,
    }))
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn str_arg<'a>(args: &'a Value, key: &str) -> Result<&'a str, AwareError> {
    args.get(key)
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| {
            AwareError::Validation(format!("trimble-connect: missing required input `{key}`"))
        })
}

/// Decode the `bytes` input. Binary travels over JSON, so a string is treated as base64
/// (the convention) and only used as raw UTF-8 bytes if it isn't valid base64; an array
/// of numbers is taken as raw bytes.
fn extract_bytes(v: Option<&Value>) -> Result<Vec<u8>, AwareError> {
    match v {
        Some(Value::String(s)) => Ok(base64::engine::general_purpose::STANDARD
            .decode(s)
            .unwrap_or_else(|_| s.clone().into_bytes())),
        // A byte array must be integers 0–255 — reject anything else rather than
        // silently dropping/truncating it into corrupted content (#200 Codex).
        Some(Value::Array(items)) => items
            .iter()
            .map(|n| {
                n.as_u64()
                    .filter(|&b| b <= 255)
                    .map(|b| b as u8)
                    .ok_or_else(|| {
                        AwareError::Validation(format!(
                            "trimble-connect upload: byte array element {n} is not an integer 0-255"
                        ))
                    })
            })
            .collect(),
        _ => Err(AwareError::Validation(
            "trimble-connect upload: missing required input `bytes`".into(),
        )),
    }
}

fn post_json(url: &str, token: &str, body: &Value, what: &str) -> Result<Value, AwareError> {
    let resp = ureq::post(url)
        .set("Authorization", &format!("Bearer {token}"))
        .set("Content-Type", "application/json")
        .send_string(&body.to_string());
    json_body(resp, what)
}

fn get_json(url: &str, token: Option<&str>, what: &str) -> Result<Value, AwareError> {
    let mut req = ureq::get(url);
    if let Some(t) = token {
        req = req.set("Authorization", &format!("Bearer {t}"));
    }
    json_body(req.call(), what)
}

/// Parse a ureq result as JSON, mapping any non-2xx into a clear error (a multi-step
/// write can't continue past a 4xx/5xx).
fn json_body(res: Result<ureq::Response, ureq::Error>, what: &str) -> Result<Value, AwareError> {
    let resp = ok_response(res, what)?;
    let text = resp
        .into_string()
        .map_err(|e| AwareError::Network(format!("{what}: read body: {e}")))?;
    serde_json::from_str(&text).map_err(|e| AwareError::Network(format!("{what}: bad JSON: {e}")))
}

fn check_ok(res: Result<ureq::Response, ureq::Error>, what: &str) -> Result<(), AwareError> {
    ok_response(res, what).map(|_| ())
}

fn ok_response(
    res: Result<ureq::Response, ureq::Error>,
    what: &str,
) -> Result<ureq::Response, AwareError> {
    match res {
        Ok(r) => Ok(r),
        Err(ureq::Error::Status(code, r)) => {
            let body = r.into_string().unwrap_or_default();
            Err(AwareError::Network(format!("{what}: HTTP {code}: {body}")))
        }
        Err(ureq::Error::Transport(t)) => Err(AwareError::Network(format!("{what}: {t}"))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_bytes_decodes_base64_string() {
        let b64 = base64::engine::general_purpose::STANDARD.encode(b"hello");
        let got = extract_bytes(Some(&Value::String(b64))).unwrap();
        assert_eq!(got, b"hello");
    }

    #[test]
    fn extract_bytes_non_base64_string_is_utf8() {
        // "!!!" isn't valid base64 → treated as raw UTF-8 bytes.
        let got = extract_bytes(Some(&Value::String("!!!".into()))).unwrap();
        assert_eq!(got, b"!!!");
    }

    #[test]
    fn extract_bytes_array_is_raw_bytes() {
        let got = extract_bytes(Some(&json!([104, 105]))).unwrap();
        assert_eq!(got, b"hi");
    }

    #[test]
    fn extract_bytes_missing_errors() {
        assert!(extract_bytes(None).is_err());
    }

    // `Read` is already in scope via `use super::*`; only `Write` is new here.
    use std::io::Write as _;
    use std::net::TcpListener;
    use std::sync::mpsc;
    use std::time::Duration;

    /// Multi-request HTTP/1.1 mock. Its routes are built from the bound base URL (so a
    /// response can embed a pre-signed URL that points back at the mock), it serves
    /// `count` connections (ureq sends `Connection: close`, one request each), routes
    /// each by a substring of the request line, and ships the full captured request back.
    fn mock_routed<F>(count: usize, routes_fn: F) -> (String, mpsc::Receiver<String>)
    where
        F: FnOnce(&str) -> Vec<(&'static str, u16, String)> + Send + 'static,
    {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let base = format!("http://127.0.0.1:{}", listener.local_addr().unwrap().port());
        let base_for_thread = base.clone();
        let (tx, rx) = mpsc::channel();
        std::thread::spawn(move || {
            let routes = routes_fn(&base_for_thread);
            for _ in 0..count {
                let Ok((mut stream, _)) = listener.accept() else {
                    break;
                };
                stream
                    .set_read_timeout(Some(Duration::from_millis(250)))
                    .ok();
                let mut data = Vec::new();
                let mut tmp = [0u8; 2048];
                while let Ok(n) = stream.read(&mut tmp) {
                    if n == 0 {
                        break;
                    }
                    data.extend_from_slice(&tmp[..n]);
                }
                let req = String::from_utf8_lossy(&data).to_string();
                let line = req.lines().next().unwrap_or("").to_string();
                let (status, body) = routes
                    .iter()
                    .find(|(pat, _, _)| line.contains(pat))
                    .map(|(_, s, b)| (*s, b.clone()))
                    .unwrap_or((404, "{}".to_string()));
                let resp = format!(
                    "HTTP/1.1 {status} X\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                    body.len()
                );
                stream.write_all(resp.as_bytes()).ok();
                tx.send(req).ok();
            }
        });
        (base, rx)
    }

    /// Install a trimble-connect manifest pointing at `base` with a NON-registered
    /// secret (so credential resolution is a hermetic raw file load — no real keychain
    /// or token-endpoint refresh) plus a stored token `TESTTOKEN`.
    fn mock_agents(base: &str) -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        let agent = tmp.path().join("agents").join("trimble-connect");
        std::fs::create_dir_all(&agent).unwrap();
        std::fs::write(
            agent.join("manifest.yaml"),
            format!(
                "agent: trimble-connect\nversion: 0.1.0\ndescription: x\nstateful: false\n\
                 license: MIT\ntransport:\n  rest:\n    base: {base}/\nauth:\n  scheme: oauth2\n  \
                 secret: mock-tc-tok\ncommands:\n  upload:\n    lifecycle: single\n    description: x\n"
            ),
        )
        .unwrap();
        let creds = tmp.path().join("credentials");
        std::fs::create_dir_all(&creds).unwrap();
        std::fs::write(
            creds.join("mock-tc-tok.json"),
            r#"{"access_token":"TESTTOKEN"}"#,
        )
        .unwrap();
        tmp
    }

    #[tokio::test]
    async fn download_two_step_attaches_auth_then_fetches_bytes() {
        let (base, rx) = mock_routed(2, |b| {
            vec![
                ("downloadurl", 200, format!(r#"{{"url":"{b}/s3-get"}}"#)),
                ("/s3-get", 200, "FILEBYTES".to_string()),
            ]
        });
        let agents = mock_agents(&base);
        let out = download(
            agents.path().join("agents"),
            serde_json::json!({ "file-id": "f1" }),
        )
        .await
        .unwrap();
        assert_eq!(out["encoding"], "base64");
        let decoded = base64::engine::general_purpose::STANDARD
            .decode(out["bytes"].as_str().unwrap())
            .unwrap();
        assert_eq!(decoded, b"FILEBYTES");
        // Step 1 (downloadurl) carries the bearer; step 2 (pre-signed S3) must NOT.
        let r1 = rx.recv().unwrap();
        assert!(
            r1.contains("downloadurl") && r1.contains("Bearer TESTTOKEN"),
            "{r1}"
        );
        let r2 = rx.recv().unwrap();
        assert!(
            r2.contains("/s3-get") && !r2.contains("Authorization"),
            "{r2}"
        );
    }

    #[tokio::test]
    async fn upload_three_step_orchestration() {
        let (base, rx) = mock_routed(3, |b| {
            vec![
                // complete (GET, request line carries `uploadId=`).
                (
                    "uploadId=",
                    200,
                    r#"{"fileId":"FID","versionId":"VID"}"#.to_string(),
                ),
                // initiate (POST).
                (
                    "POST /files/fs/upload",
                    200,
                    format!(
                        r#"{{"uploadId":"up1","status":"UPLOADABLE","contents":[{{"url":"{b}/s3-put"}}]}}"#
                    ),
                ),
                // pre-signed S3 PUT.
                ("/s3-put", 200, "{}".to_string()),
            ]
        });
        let agents = mock_agents(&base);
        let out = upload(
            agents.path().join("agents"),
            // bytes: base64("hello")
            serde_json::json!({ "folder-id": "fold1", "filename": "a.txt", "bytes": "aGVsbG8=" }),
        )
        .await
        .unwrap();
        assert_eq!(out["file-id"], "FID");
        assert_eq!(out["version-id"], "VID");
        let reqs: Vec<String> = (0..3).map(|_| rx.recv().unwrap()).collect();
        // initiate carries the bearer + name; the S3 PUT carries the decoded bytes and NO auth.
        assert!(
            reqs.iter()
                .any(|r| r.contains("POST /files/fs/upload") && r.contains("Bearer TESTTOKEN"))
        );
        let put = reqs.iter().find(|r| r.contains("/s3-put")).unwrap();
        assert!(
            !put.contains("Authorization") && put.contains("hello"),
            "{put}"
        );
    }

    #[test]
    fn extract_bytes_rejects_out_of_range_or_non_int_array() {
        assert!(extract_bytes(Some(&json!([256]))).is_err());
        assert!(extract_bytes(Some(&json!([1, "x"]))).is_err());
        assert!(extract_bytes(Some(&json!([1, -1]))).is_err());
    }

    #[tokio::test]
    async fn upload_duplicate_fetches_version_from_metadata() {
        // DUPLICATE initiate carries only fileId → the handler fetches versionId from
        // the file metadata so the result stays usable.
        let (base, _rx) = mock_routed(2, |_b| {
            vec![
                (
                    "POST /files/fs/upload",
                    200,
                    r#"{"status":"DUPLICATE","fileId":"DUPFID"}"#.to_string(),
                ),
                (
                    "/files/DUPFID",
                    200,
                    r#"{"versionId":"DUPVER"}"#.to_string(),
                ),
            ]
        });
        let agents = mock_agents(&base);
        let out = upload(
            agents.path().join("agents"),
            serde_json::json!({ "folder-id": "f", "filename": "a.txt", "bytes": "aGk=" }),
        )
        .await
        .unwrap();
        assert_eq!(out["file-id"], "DUPFID");
        assert_eq!(out["version-id"], "DUPVER");
        assert_eq!(out["replaced"], true);
    }
}
