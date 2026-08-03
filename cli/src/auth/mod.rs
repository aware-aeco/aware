//! OAuth + keychain + refresh for AWARE integrations.
//!
//! Submodules added across Tasks 2-5.

use crate::error::AwareError;

pub mod config;
pub mod device; // v0.13 — RFC 8628 device-code flow
pub mod keychain;
pub mod paste;
pub mod pkce; // Task 4
pub mod profile; // Tier 2 BYO OAuth app profiles (#146)
pub mod refresh; // Task 5

/// Seconds since the Unix epoch.
///
/// Every token lifetime in this module is computed against this value, so a
/// clock behind the epoch must be reported rather than absorbed. Defaulting to
/// `0` would be actively harmful in opposite directions on either side of the
/// same subsystem: `refresh::ensure_fresh` would read every stored token as
/// fresh forever and hand back an expired one (surfacing as a bare 401 with no
/// hint), while the three flows that stamp `expires_at = now + expires_in`
/// would mint tokens that are already expired.
pub(crate) fn unix_now_secs() -> Result<i64, AwareError> {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|elapsed| elapsed.as_secs() as i64)
        .map_err(|e| {
            AwareError::Validation(format!(
                "system clock is before the Unix epoch ({e}); token lifetimes cannot be computed"
            ))
        })
}

/// Build an HTML response for the loopback pages served during an OAuth flow.
///
/// The `Content-Type` is a compile-time constant, so the parse cannot fail in
/// practice — but panicking mid-round-trip would strand the user half-way
/// through a browser login with no token and no error. Degrade instead: the
/// response keeps the `text/plain` content type `Response::from_string` seeds
/// (`add_header` appends rather than replaces), so the worst case is the
/// success page rendering as raw HTML source — not a crashed flow. The
/// explicit `charset=utf-8` is what keeps the ✓ glyph from becoming mojibake
/// (#157).
pub(crate) fn html_response(
    body: impl Into<String>,
) -> tiny_http::Response<std::io::Cursor<Vec<u8>>> {
    let mut response = tiny_http::Response::from_string(body.into());
    if let Ok(header) = "Content-Type: text/html; charset=utf-8".parse::<tiny_http::Header>() {
        response.add_header(header);
    }
    response
}
