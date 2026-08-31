//! The JSON body an OAuth token endpoint answers with (RFC 6749 §5.1 / §5.2).
//!
//! Three flows post to a token endpoint and read the reply: `pkce` (the
//! authorization-code exchange), `device` (the RFC 8628 poll) and `refresh`
//! (the refresh-token grant). Each had written out the same five `get(…)`
//! lookups with the same defaults — one hour when `expires_in` is absent,
//! `Bearer` when `token_type` is, `None` when there is no `refresh_token` — and
//! `pkce` and `device` had gone further and grown byte-identical copies of the
//! whole `StoredToken` construction, down to the wording of the missing-field
//! error.
//!
//! That is one wire format, so it is described once here. The defaults in
//! particular are the reason: `expires_in` is optional in RFC 6749, and a
//! provider that omits it against a copy that had drifted to a different
//! fallback would mint a credential the refresh loop reads as expired at a
//! different moment depending on which flow issued it.
//!
//! Deliberately a view over `serde_json::Value` rather than a `Deserialize`
//! struct. Every field is read as "present, and of the expected JSON type, or
//! fall back", which is what the three copies did and what tolerates a provider
//! that answers `"expires_in": "3600"` as a string. A typed model would turn
//! that into a hard parse failure — a behaviour change dressed up as a cleanup.
//!
//! Parsing the body is left to the caller so each flow keeps naming *its own*
//! request in the parse error ("token response", "refresh response"), which is
//! the one thing about these three that genuinely differs.

use serde_json::Value;

use crate::auth::keychain::{StoredToken, TokenSource};
use crate::error::AwareError;

/// A token endpoint's reply, already parsed as JSON.
pub(crate) struct TokenResponse {
    json: Value,
}

/// Lifetime assumed when the endpoint omits `expires_in`. RFC 6749 §5.1 makes
/// the field OPTIONAL, so this is reachable, and all three flows already agreed
/// on an hour.
const DEFAULT_EXPIRES_IN_SECS: i64 = 3600;

impl TokenResponse {
    pub(crate) fn new(json: Value) -> Self {
        Self { json }
    }

    fn str_field(&self, key: &str) -> Option<&str> {
        self.json.get(key).and_then(Value::as_str)
    }

    /// The `error` code an endpoint returns instead of a token (RFC 6749 §5.2,
    /// and the `authorization_pending` / `slow_down` codes of RFC 8628 §3.5).
    ///
    /// Only the device-code poll reads this: it is the one flow that inspects a
    /// non-2xx body, because `pkce` and `refresh` let `ureq` turn a 4xx into a
    /// transport error before any body is parsed.
    pub(crate) fn error_code(&self) -> Option<&str> {
        self.str_field("error")
    }

    /// The access token, when the reply carries one.
    ///
    /// Returned as an `Option` rather than a `Result` so each caller keeps its
    /// own wording for the absence — `refresh` names the refresh grant in its
    /// error, and a user staring at a failed `aware connect --refresh` is better
    /// served by that than by a message about a generic "token response".
    pub(crate) fn access_token(&self) -> Option<String> {
        self.str_field("access_token").map(String::from)
    }

    pub(crate) fn refresh_token(&self) -> Option<String> {
        self.str_field("refresh_token").map(String::from)
    }

    pub(crate) fn scope(&self) -> Option<&str> {
        self.str_field("scope")
    }

    /// Absolute expiry stamped against `now`, per the shared one-hour fallback.
    pub(crate) fn expires_at(&self, now: i64) -> i64 {
        let expires_in = self
            .json
            .get("expires_in")
            .and_then(Value::as_i64)
            .unwrap_or(DEFAULT_EXPIRES_IN_SECS);
        now + expires_in
    }

    /// Build the credential a *fresh* grant mints — the authorization-code
    /// exchange and the device-code poll, which start from nothing and so take
    /// every field from the reply.
    ///
    /// The refresh grant deliberately does not go through here: it starts from a
    /// credential that already exists and carries forward the fields the reply
    /// may legitimately omit. Folding the two together would mean a flag saying
    /// which one this is, and the two constructions really are different.
    pub(crate) fn into_new_credential(
        self,
        integration: &str,
        now: i64,
    ) -> Result<StoredToken, AwareError> {
        let expires_at = self.expires_at(now);
        let access_token = self
            .access_token()
            .ok_or_else(|| AwareError::Validation("token response missing access_token".into()))?;
        Ok(StoredToken {
            access_token,
            refresh_token: self.refresh_token(),
            expires_at,
            scope: self.scope().unwrap_or("").to_string(),
            token_type: self.str_field("token_type").unwrap_or("Bearer").to_string(),
            integration: integration.to_string(),
            obtained_at: now,
            source: TokenSource::Oauth,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(body: &str) -> TokenResponse {
        TokenResponse::new(serde_json::from_str(body).unwrap())
    }

    #[test]
    fn a_full_reply_becomes_a_complete_credential() {
        let token = parse(
            r#"{"access_token":"tk","refresh_token":"rt","expires_in":120,
                "scope":"a b","token_type":"MAC"}"#,
        )
        .into_new_credential("trimble-connect", 1_000)
        .unwrap();

        assert_eq!(token.access_token, "tk");
        assert_eq!(token.refresh_token.as_deref(), Some("rt"));
        assert_eq!(token.expires_at, 1_120);
        assert_eq!(token.scope, "a b");
        assert_eq!(token.token_type, "MAC");
        assert_eq!(token.integration, "trimble-connect");
        assert_eq!(token.obtained_at, 1_000);
        assert_eq!(token.source, TokenSource::Oauth);
    }

    /// The defaults are the reason this type exists — every optional field is
    /// pinned here so a flow cannot quietly disagree with the other two about
    /// what an omitted field means.
    #[test]
    fn the_optional_fields_fall_back_the_way_all_three_flows_did() {
        let token = parse(r#"{"access_token":"tk"}"#)
            .into_new_credential("microsoft-365", 0)
            .unwrap();

        assert_eq!(token.expires_at, DEFAULT_EXPIRES_IN_SECS);
        assert_eq!(token.token_type, "Bearer");
        assert_eq!(token.scope, "");
        assert!(token.refresh_token.is_none());
    }

    #[test]
    fn a_reply_without_an_access_token_is_a_validation_error() {
        let err = parse(r#"{"token_type":"Bearer"}"#)
            .into_new_credential("google-workspace", 0)
            .unwrap_err();
        assert!(
            matches!(&err, AwareError::Validation(m) if m.contains("missing access_token")),
            "got {err:?}"
        );
    }

    /// A provider that spells `expires_in` as a string still gets the shared
    /// fallback rather than a hard failure. This is the leniency the three
    /// hand-written copies had, and the reason this is not a `Deserialize`
    /// struct.
    #[test]
    fn a_non_numeric_expires_in_falls_back_instead_of_failing() {
        let response = parse(r#"{"access_token":"tk","expires_in":"3600"}"#);
        assert_eq!(response.expires_at(10), 10 + DEFAULT_EXPIRES_IN_SECS);
    }

    #[test]
    fn the_error_code_is_visible_to_the_device_poll() {
        assert_eq!(
            parse(r#"{"error":"authorization_pending"}"#).error_code(),
            Some("authorization_pending")
        );
        assert_eq!(parse(r#"{"access_token":"tk"}"#).error_code(), None);
    }
}
