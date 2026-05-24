//! RFC 8628 Device Authorization Grant.
//!
//! For headless / IT-managed / restricted-browser environments. Instead of
//! launching the user's browser via PKCE (which assumes the workstation can
//! open `localhost:<port>`), the device-code flow prints a URL + short user
//! code; the user opens the URL on any device, enters the code, and the CLI
//! polls the token endpoint until authorization completes.
//!
//! Per `10-core/cli-spec.md` and the v0.13 phase entry in `cli-roadmap.md`.

#![allow(dead_code)]

use std::io::Read;
use std::thread::sleep;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::auth::config::IntegrationConfig;
use crate::auth::keychain::{StoredToken, TokenSource};
use crate::error::AwareError;

/// Per-integration device-authorization endpoint. Returned alongside the
/// auth/token endpoints from `for_integration_device_endpoints` so the
/// flow stays decoupled from the PKCE config.
struct DeviceEndpoints {
    device_authorization_url: String,
    token_url: String,
}

fn device_endpoints_for(cfg: &IntegrationConfig, tenant: Option<&str>) -> DeviceEndpoints {
    match cfg.id.as_str() {
        "microsoft-365" => {
            // CLI `--tenant` wins; otherwise honor a BYO profile tenant; else `common`.
            let t = tenant.or_else(|| cfg.tenant()).unwrap_or("common");
            // An explicit profile endpoint override (e.g. sovereign cloud / proxy)
            // wins over the tenant-built public-cloud URL.
            DeviceEndpoints {
                device_authorization_url: cfg
                    .device_authorization_url_override()
                    .map(String::from)
                    .unwrap_or_else(|| {
                        format!("https://login.microsoftonline.com/{t}/oauth2/v2.0/devicecode")
                    }),
                token_url: cfg
                    .token_url_override()
                    .map(String::from)
                    .unwrap_or_else(|| {
                        format!("https://login.microsoftonline.com/{t}/oauth2/v2.0/token")
                    }),
            }
        }
        // For the remaining providers there is no tenant substitution, so
        // `token_url()` already yields the resolved value (profile override ▸
        // bundled). A profile `device_authorization_url` override is honored too.
        "google-workspace" => DeviceEndpoints {
            device_authorization_url: cfg
                .device_authorization_url_override()
                .unwrap_or("https://oauth2.googleapis.com/device/code")
                .to_string(),
            token_url: cfg.token_url().to_string(),
        },
        "trimble-connect" => DeviceEndpoints {
            // Trimble Identity supports OAuth device-code per the docs, but the
            // endpoint URL isn't published as a stable identifier — the standard
            // PKCE path remains the supported one. An empty device URL makes the
            // caller error with a clear "not supported" message; a BYO profile
            // that sets `device_authorization_url` opts back into device-code.
            device_authorization_url: cfg
                .device_authorization_url_override()
                .unwrap_or_default()
                .to_string(),
            token_url: cfg.token_url().to_string(),
        },
        _ => DeviceEndpoints {
            device_authorization_url: cfg
                .device_authorization_url_override()
                .unwrap_or_default()
                .to_string(),
            token_url: cfg.token_url().to_string(),
        },
    }
}

/// Run the RFC 8628 device authorization grant for a given integration.
///
/// `tenant` overrides the Microsoft tenant id (M365 only). For Google
/// Workspace the `hd=` domain hint is passed as part of scopes only.
///
/// When `json` is `true` the function emits NDJSON on stdout instead of
/// human-readable text, so a thin UI can read the device code and URL
/// without scraping (closes #143):
/// - Line 1 (prompt): `{"phase":"prompt","user_code":"...","verification_uri":"...","expires_in":N,"interval":N}`
/// - The caller emits Line 2 (result) after `store_token` succeeds.
pub fn run_device_code_flow(
    cfg: &IntegrationConfig,
    tenant: Option<&str>,
    extra_scopes: &[String],
    json: bool,
) -> Result<StoredToken, AwareError> {
    let endpoints = device_endpoints_for(cfg, tenant);
    if endpoints.device_authorization_url.is_empty() {
        return Err(AwareError::Validation(format!(
            "integration {} does not support device-code flow (use --oauth instead)",
            cfg.id
        )));
    }

    let scopes: Vec<String> = cfg
        .scopes()
        .into_iter()
        .chain(extra_scopes.iter().cloned())
        .collect();
    let scope_str = scopes.join(" ");

    // Step 1: request the user / device codes.
    let req_body = format!(
        "client_id={}&scope={}",
        urlencode(&cfg.client_id()),
        urlencode(&scope_str)
    );
    let resp = ureq::post(&endpoints.device_authorization_url)
        .set("Content-Type", "application/x-www-form-urlencoded")
        .send_string(&req_body)
        .map_err(|e| AwareError::Network(format!("device authorization request: {e}")))?;
    let body = read_body(resp)?;
    let parsed: serde_json::Value = serde_json::from_str(&body)
        .map_err(|e| AwareError::Validation(format!("device authorization response: {e}")))?;

    let device_code = parsed
        .get("device_code")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AwareError::Validation("response missing device_code".into()))?
        .to_string();
    let user_code = parsed
        .get("user_code")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AwareError::Validation("response missing user_code".into()))?;
    let verification_uri = parsed
        .get("verification_uri")
        // Google uses `verification_url`, MS uses `verification_uri`. Try both.
        .or_else(|| parsed.get("verification_url"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| AwareError::Validation("response missing verification_uri/url".into()))?;
    let interval = parsed
        .get("interval")
        .and_then(|v| v.as_u64())
        .unwrap_or(5);
    let expires_in = parsed
        .get("expires_in")
        .and_then(|v| v.as_u64())
        .unwrap_or(900);

    // Display instructions to the user (or emit NDJSON prompt line for UI consumers).
    if json {
        use std::io::Write as _;
        println!(
            "{}",
            serde_json::json!({
                "phase": "prompt",
                "user_code": user_code,
                "verification_uri": verification_uri,
                "expires_in": expires_in,
                "interval": interval,
            })
        );
        // Flush so a piping UI receives the line immediately before the poll loop.
        std::io::stdout().flush().ok();
    } else {
        println!();
        println!("  Open this URL in any browser:");
        println!("    {verification_uri}");
        println!();
        println!("  And enter this code:");
        println!("    {user_code}");
        println!();
        println!("  (Waiting for authorization. This page expires in {expires_in} seconds.)");
        println!();
    }

    // Step 2: poll the token endpoint.
    let started = SystemTime::now();
    loop {
        sleep(Duration::from_secs(interval));
        if started.elapsed().unwrap_or_default().as_secs() > expires_in {
            return Err(AwareError::Validation(
                "device-code authorization expired; re-run `aware connect`".into(),
            ));
        }

        let mut poll_body = format!(
            "grant_type=urn:ietf:params:oauth:grant-type:device_code&device_code={}&client_id={}",
            urlencode(&device_code),
            urlencode(&cfg.client_id())
        );
        // Public clients (M365) carry no secret; only appended when configured.
        if let Some(secret) = cfg.client_secret() {
            poll_body.push_str(&format!("&client_secret={}", urlencode(&secret)));
        }
        let result = ureq::post(&endpoints.token_url)
            .set("Content-Type", "application/x-www-form-urlencoded")
            .send_string(&poll_body);
        let resp = match result {
            Ok(r) => r,
            Err(ureq::Error::Status(_code, r)) => r,
            Err(e) => {
                return Err(AwareError::Network(format!(
                    "device-code token poll: {e}"
                )));
            }
        };
        let body = read_body(resp)?;
        let parsed: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| AwareError::Validation(format!("token response: {e}")))?;

        if let Some(err) = parsed.get("error").and_then(|v| v.as_str()) {
            match err {
                "authorization_pending" => continue,
                "slow_down" => {
                    // Per RFC 8628 §3.5: increase interval by 5s.
                    sleep(Duration::from_secs(5));
                    continue;
                }
                "expired_token" | "access_denied" | "code_expired" => {
                    return Err(AwareError::Validation(format!(
                        "device-code authorization {err} — re-run `aware connect`"
                    )));
                }
                other => {
                    return Err(AwareError::Network(format!(
                        "device-code unexpected error: {other}"
                    )));
                }
            }
        }

        // No `error` field → success.
        let access_token = parsed
            .get("access_token")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AwareError::Validation("token response missing access_token".into()))?
            .to_string();
        let refresh_token = parsed
            .get("refresh_token")
            .and_then(|v| v.as_str())
            .map(String::from);
        let expires_in = parsed
            .get("expires_in")
            .and_then(|v| v.as_i64())
            .unwrap_or(3600);
        let scope = parsed
            .get("scope")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();
        let token_type = parsed
            .get("token_type")
            .and_then(|v| v.as_str())
            .unwrap_or("Bearer")
            .to_string();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        return Ok(StoredToken {
            access_token,
            refresh_token,
            expires_at: now + expires_in,
            scope,
            token_type,
            integration: cfg.id.to_string(),
            obtained_at: now,
            source: TokenSource::Oauth,
        });
    }
}

fn read_body(resp: ureq::Response) -> Result<String, AwareError> {
    let mut buf = String::new();
    resp.into_reader()
        .read_to_string(&mut buf)
        .map_err(|e| AwareError::Network(format!("read response: {e}")))?;
    Ok(buf)
}

fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char);
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::config;

    #[test]
    fn microsoft_endpoints_use_common_by_default() {
        let cfg = config::for_integration("microsoft-365").unwrap();
        let e = device_endpoints_for(&cfg, None);
        assert!(e.device_authorization_url.contains("/common/"));
        assert!(e.token_url.contains("/common/"));
    }

    #[test]
    fn microsoft_endpoints_use_tenant_when_supplied() {
        let cfg = config::for_integration("microsoft-365").unwrap();
        let e = device_endpoints_for(&cfg, Some("acme.onmicrosoft.com"));
        assert!(e.device_authorization_url.contains("/acme.onmicrosoft.com/"));
        assert!(e.token_url.contains("/acme.onmicrosoft.com/"));
    }

    #[test]
    fn microsoft_device_honors_explicit_profile_endpoint_override() {
        // A BYO profile with explicit sovereign-cloud endpoints must be honored by
        // the device-code flow, not rebuilt as a public-cloud tenant URL (#146 review).
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("oauth");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("microsoft-365.yaml"),
            "client_id: x\n\
             token_url: https://login.microsoftonline.us/contoso/oauth2/v2.0/token\n\
             device_authorization_url: https://login.microsoftonline.us/contoso/oauth2/v2.0/devicecode\n",
        )
        .unwrap();
        let cfg = config::for_integration("microsoft-365")
            .unwrap()
            .with_profile(tmp.path(), None)
            .unwrap();
        let e = device_endpoints_for(&cfg, None);
        assert_eq!(
            e.token_url,
            "https://login.microsoftonline.us/contoso/oauth2/v2.0/token"
        );
        assert_eq!(
            e.device_authorization_url,
            "https://login.microsoftonline.us/contoso/oauth2/v2.0/devicecode"
        );
    }

    #[test]
    fn google_has_fixed_endpoints() {
        let cfg = config::for_integration("google-workspace").unwrap();
        let e = device_endpoints_for(&cfg, None);
        assert_eq!(e.device_authorization_url, "https://oauth2.googleapis.com/device/code");
    }

    #[test]
    fn google_device_honors_explicit_profile_endpoint_override() {
        // A Google BYO profile that points device-code at a proxy/alternate
        // endpoint must be honored, not posted to Google's public URLs (#146 review).
        let tmp = tempfile::tempdir().unwrap();
        let dir = tmp.path().join("oauth");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("google-workspace.yaml"),
            "client_id: x\n\
             token_url: https://proxy.example.com/token\n\
             device_authorization_url: https://proxy.example.com/device/code\n",
        )
        .unwrap();
        let cfg = config::for_integration("google-workspace")
            .unwrap()
            .with_profile(tmp.path(), None)
            .unwrap();
        let e = device_endpoints_for(&cfg, None);
        assert_eq!(
            e.device_authorization_url,
            "https://proxy.example.com/device/code"
        );
        assert_eq!(e.token_url, "https://proxy.example.com/token");
    }

    #[test]
    fn trimble_returns_empty_device_url_so_caller_can_error() {
        let cfg = config::for_integration("trimble-connect").unwrap();
        let e = device_endpoints_for(&cfg, None);
        assert!(e.device_authorization_url.is_empty());
    }

    #[test]
    fn urlencode_preserves_unreserved_chars() {
        assert_eq!(urlencode("hello.world-test_X~"), "hello.world-test_X~");
    }

    #[test]
    fn urlencode_percent_encodes_reserved() {
        assert_eq!(urlencode("a b/c"), "a%20b%2Fc");
    }
}
