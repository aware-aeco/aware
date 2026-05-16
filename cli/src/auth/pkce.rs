//! PKCE OAuth flow with localhost callback.

#![allow(dead_code)]

use std::time::{SystemTime, UNIX_EPOCH};

use base64::Engine;
use rand::RngCore;
use sha2::Digest;

use crate::auth::config::IntegrationConfig;
use crate::auth::keychain::StoredToken;
use crate::error::AwareError;

pub fn run_pkce_flow(
    config: &IntegrationConfig,
    extra_scopes: &[String],
) -> Result<StoredToken, AwareError> {
    // 1. PKCE pair
    let (verifier, challenge) = make_pkce_pair();

    // 2. Localhost callback server
    let (server, port) = bind_callback_server()?;
    let redirect_uri = format!("http://localhost:{port}/callback");

    // 3. CSRF state
    let state = random_token(16);

    // 4. Build auth URL
    let mut all_scopes: Vec<String> = config
        .default_scopes
        .iter()
        .map(|s| s.to_string())
        .collect();
    for s in extra_scopes {
        all_scopes.push(s.clone());
    }
    let scope_param = all_scopes.join(" ");
    let auth_url = format!(
        "{base}?response_type=code&client_id={cid}&redirect_uri={ru}&scope={sc}&code_challenge={ch}&code_challenge_method=S256&state={st}",
        base = config.auth_url,
        cid = urlencode(&config.client_id()),
        ru = urlencode(&redirect_uri),
        sc = urlencode(&scope_param),
        ch = challenge,
        st = state,
    );

    // 5. Open browser
    println!("Opening {} OAuth in your default browser...", config.id);
    println!("If it doesn't open automatically, visit:\n  {auth_url}");
    let _ = webbrowser::open(&auth_url);

    // 6. Wait for callback
    println!("\u{2713} Listening on {redirect_uri}");
    let request = server
        .recv()
        .map_err(|e| AwareError::Network(format!("callback recv: {e}")))?;

    // Parse query string
    let url_str = format!("http://localhost{}", request.url());
    let parsed = url::Url::parse(&url_str)
        .map_err(|e| AwareError::Network(format!("callback URL parse: {e}")))?;
    let mut code: Option<String> = None;
    let mut returned_state: Option<String> = None;
    for (k, v) in parsed.query_pairs() {
        if k == "code" {
            code = Some(v.to_string());
        }
        if k == "state" {
            returned_state = Some(v.to_string());
        }
    }
    let code = code.ok_or_else(|| AwareError::Network("callback missing 'code'".into()))?;
    let returned_state =
        returned_state.ok_or_else(|| AwareError::Network("callback missing 'state'".into()))?;
    if returned_state != state {
        return Err(AwareError::PermissionDenied(
            "OAuth state mismatch (possible CSRF)".into(),
        ));
    }

    // 7. Respond to browser
    let html = "<html><body><h1>\u{2713} Authenticated</h1><p>You can close this tab.</p>\
                <script>setTimeout(()=>window.close(),500)</script></body></html>";
    let response = tiny_http::Response::from_string(html).with_header(
        "Content-Type: text/html"
            .parse::<tiny_http::Header>()
            .unwrap(),
    );
    let _ = request.respond(response);

    // 8. Exchange code for token
    println!("\u{2713} Received auth code");
    println!("\u{2713} Exchanging for tokens...");

    let body_params = [
        ("grant_type", "authorization_code".to_string()),
        ("code", code),
        ("redirect_uri", redirect_uri.clone()),
        ("client_id", config.client_id()),
        ("code_verifier", verifier),
    ];
    let body = body_params
        .iter()
        .map(|(k, v)| format!("{}={}", urlencode(k), urlencode(v)))
        .collect::<Vec<_>>()
        .join("&");
    let resp = ureq::post(config.token_url)
        .set("Content-Type", "application/x-www-form-urlencoded")
        .send_string(&body)
        .map_err(|e| AwareError::Network(format!("token exchange: {e}")))?;
    let resp_body = {
        use std::io::Read;
        let mut buf = String::new();
        resp.into_reader()
            .read_to_string(&mut buf)
            .map_err(|e| AwareError::Network(format!("reading token response: {e}")))?;
        buf
    };
    let token_json: serde_json::Value = serde_json::from_str(&resp_body)
        .map_err(|e| AwareError::Validation(format!("token response: {e}")))?;

    // 9. Build StoredToken
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let expires_in = token_json
        .get("expires_in")
        .and_then(|v| v.as_i64())
        .unwrap_or(3600);
    let access_token = token_json
        .get("access_token")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AwareError::Validation("token response missing access_token".into()))?
        .to_string();
    let refresh_token = token_json
        .get("refresh_token")
        .and_then(|v| v.as_str())
        .map(String::from);
    let scope = token_json
        .get("scope")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let token_type = token_json
        .get("token_type")
        .and_then(|v| v.as_str())
        .unwrap_or("Bearer")
        .to_string();

    let token = StoredToken {
        access_token,
        refresh_token,
        expires_at: now + expires_in,
        scope,
        token_type,
        integration: config.id.to_string(),
        obtained_at: now,
    };
    Ok(token)
}

fn make_pkce_pair() -> (String, String) {
    let mut bytes = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut bytes);
    let verifier = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(bytes);
    let mut hasher = sha2::Sha256::new();
    hasher.update(verifier.as_bytes());
    let challenge = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(hasher.finalize());
    (verifier, challenge)
}

fn random_token(n: usize) -> String {
    use rand::Rng;
    let alphabet: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    (0..n)
        .map(|_| alphabet[rng.gen_range(0..alphabet.len())] as char)
        .collect()
}

fn urlencode(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

fn bind_callback_server() -> Result<(tiny_http::Server, u16), AwareError> {
    for port in 7421u16..=7430u16 {
        let addr = format!("127.0.0.1:{port}");
        if let Ok(server) = tiny_http::Server::http(&addr) {
            return Ok((server, port));
        }
    }
    Err(AwareError::Network(
        "could not bind callback port in 7421-7430".into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pkce_pair_has_valid_shape() {
        let (verifier, challenge) = make_pkce_pair();
        // URL-safe base64-no-pad of 32 bytes is 43 chars
        assert_eq!(verifier.len(), 43);
        assert_eq!(challenge.len(), 43);
        assert!(
            verifier
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        );
        assert!(
            challenge
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        );
        // Two different calls produce different verifiers (random)
        let (v2, _) = make_pkce_pair();
        assert_ne!(verifier, v2);
    }

    #[test]
    fn random_token_uses_alphanumerics_only() {
        let t = random_token(16);
        assert_eq!(t.len(), 16);
        assert!(t.chars().all(|c| c.is_alphanumeric()));
    }

    #[test]
    fn urlencode_handles_special_chars() {
        assert_eq!(urlencode("hello world"), "hello+world");
        assert_eq!(urlencode("a=b&c"), "a%3Db%26c");
    }

    #[test]
    fn bind_callback_server_returns_a_port_in_range() {
        let (_server, port) = bind_callback_server().unwrap();
        assert!((7421..=7430).contains(&port));
    }
}
