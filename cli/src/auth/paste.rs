//! Browser-based token paste flow.
//!
//! `aware connect <integration>` (default — no flags) drops the user into a
//! browser tab with a password-style form. They paste the token; it gets
//! encrypted to the OS keychain. The token NEVER flows through stdout or any
//! AI chat — it only exists in the user's browser tab and the local keychain.

#![allow(dead_code)]

use std::time::{SystemTime, UNIX_EPOCH};

use crate::auth::keychain::{StoredToken, TokenSource};
use crate::error::AwareError;

pub fn run_paste_flow(integration: &str) -> Result<StoredToken, AwareError> {
    let (server, port) = bind_paste_server()?;
    let paste_url = format!("http://localhost:{port}/");

    println!("Opening token entry form in your browser...");
    println!("If it doesn't open automatically, visit:\n  {paste_url}");
    let _ = webbrowser::open(&paste_url);
    println!("\u{2713} Listening on {paste_url}");
    println!("Paste your {integration} token into the browser form to continue.");
    println!(
        "The form is the ONLY place to enter your token \u{2014} never paste it into this chat."
    );

    loop {
        let mut request = server
            .recv()
            .map_err(|e| AwareError::Network(format!("paste recv: {e}")))?;

        match request.method().clone() {
            tiny_http::Method::Get => {
                let html = render_paste_form(integration);
                let response = tiny_http::Response::from_string(html).with_header(
                    "Content-Type: text/html; charset=utf-8"
                        .parse::<tiny_http::Header>()
                        .unwrap(),
                );
                let _ = request.respond(response);
            }
            tiny_http::Method::Post => {
                #[allow(unused_imports)]
                use std::io::Read as _;
                let mut body = String::new();
                request
                    .as_reader()
                    .read_to_string(&mut body)
                    .map_err(|e| AwareError::Network(format!("paste body: {e}")))?;
                let token_value = extract_token_from_form(&body)?;

                // Respond with success page BEFORE returning so the user sees it
                let html = render_success_page();
                let response = tiny_http::Response::from_string(html).with_header(
                    "Content-Type: text/html; charset=utf-8"
                        .parse::<tiny_http::Header>()
                        .unwrap(),
                );
                let _ = request.respond(response);

                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as i64;
                return Ok(StoredToken {
                    access_token: token_value,
                    refresh_token: None,
                    expires_at: 0,
                    scope: String::new(),
                    token_type: "Bearer".into(),
                    integration: integration.to_string(),
                    obtained_at: now,
                    source: TokenSource::Paste,
                });
            }
            _ => {
                let _ = request.respond(tiny_http::Response::empty(405));
            }
        }
    }
}

fn render_paste_form(integration: &str) -> String {
    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>AWARE — Paste token for {integration}</title>
<style>
body {{ font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; max-width: 520px; margin: 60px auto; padding: 0 20px; color: #333; }}
h1 {{ font-size: 1.3em; margin-bottom: 24px; }}
label {{ display: block; font-weight: 600; margin-bottom: 6px; }}
input[type=password] {{ width: 100%; padding: 10px; font-family: monospace; font-size: 14px; border: 1px solid #ccc; border-radius: 4px; box-sizing: border-box; }}
button {{ margin-top: 16px; padding: 10px 20px; font-size: 14px; background: #2563eb; color: white; border: none; border-radius: 4px; cursor: pointer; }}
button:hover {{ background: #1d4ed8; }}
.hint {{ color: #666; font-size: 13px; margin-top: 6px; line-height: 1.5; }}
.warn {{ background: #fef3c7; color: #92400e; padding: 8px 12px; border-radius: 4px; font-size: 13px; margin-bottom: 20px; }}
</style>
</head>
<body>
<h1>Paste your {integration} token</h1>
<div class="warn">This form is the only safe way to provide a token. The value never leaves this machine.</div>
<form method="POST" action="/">
<label for="token">Token</label>
<input type="password" id="token" name="token" placeholder="Paste here…" autofocus required autocomplete="off">
<div class="hint">The token will be encrypted to your OS keychain under service <code>aware-aeco</code>, account <code>{integration}</code>.</div>
<button type="submit">Save encrypted</button>
</form>
</body>
</html>"##
    )
}

fn render_success_page() -> String {
    String::from(
        r##"<!DOCTYPE html>
<html lang="en">
<head><meta charset="utf-8"><title>Stored</title>
<style>body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; max-width: 480px; margin: 80px auto; text-align: center; } h1 { color: #16a34a; }</style>
</head>
<body>
<h1>&#x2713; Stored</h1>
<p>The token is encrypted in your OS keychain. You can close this tab.</p>
<script>setTimeout(()=>window.close(),800)</script>
</body>
</html>"##,
    )
}

fn extract_token_from_form(body: &str) -> Result<String, AwareError> {
    for (k, v) in url::form_urlencoded::parse(body.as_bytes()) {
        if k == "token" {
            let token = v.to_string();
            if token.trim().is_empty() {
                return Err(AwareError::Validation("token field was empty".into()));
            }
            return Ok(token.trim().to_string());
        }
    }
    Err(AwareError::Validation(
        "no token field in form body".into(),
    ))
}

fn bind_paste_server() -> Result<(tiny_http::Server, u16), AwareError> {
    for port in 7421u16..=7430u16 {
        let addr = format!("127.0.0.1:{port}");
        if let Ok(server) = tiny_http::Server::http(&addr) {
            return Ok((server, port));
        }
    }
    Err(AwareError::Network(
        "could not bind paste port in 7421-7430".into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_token_from_form_body() {
        let body = "token=tk_abc123";
        assert_eq!(extract_token_from_form(body).unwrap(), "tk_abc123");
    }

    #[test]
    fn extracts_url_encoded_token() {
        // tk+abc/123 url-encoded
        let body = "token=tk%2Babc%2F123";
        assert_eq!(extract_token_from_form(body).unwrap(), "tk+abc/123");
    }

    #[test]
    fn empty_token_is_validation_error() {
        let body = "token=";
        let err = extract_token_from_form(body).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }

    #[test]
    fn whitespace_only_token_is_validation_error() {
        let body = "token=%20%20%20";
        let err = extract_token_from_form(body).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }

    #[test]
    fn missing_token_field_is_validation_error() {
        let body = "other=value";
        let err = extract_token_from_form(body).unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
    }

    #[test]
    fn paste_form_contains_integration_name() {
        let html = render_paste_form("trimble-connect");
        assert!(html.contains("trimble-connect"));
        assert!(html.contains("password")); // input type
        assert!(html.contains("never leaves this machine"));
    }

    #[test]
    fn success_page_has_close_script() {
        let html = render_success_page();
        assert!(html.contains("Stored"));
        assert!(html.contains("close"));
    }

    #[test]
    fn bind_paste_server_returns_a_port_in_range() {
        let (_server, port) = bind_paste_server().unwrap();
        assert!((7421..=7430).contains(&port));
    }
}
