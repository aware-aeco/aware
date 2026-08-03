//! OAuth + keychain + refresh for AWARE integrations.
//!
//! Submodules added across Tasks 2-5.

#![allow(dead_code)]

pub mod config;
pub mod device; // v0.13 — RFC 8628 device-code flow
pub mod keychain;
pub mod paste;
pub mod pkce; // Task 4
pub mod profile; // Tier 2 BYO OAuth app profiles (#146)
pub mod refresh; // Task 5

/// Percent-encode one OAuth parameter value.
///
/// Every OAuth request this module builds is `application/x-www-form-urlencoded`
/// — RFC 6749 mandates it for the token, refresh and device-code bodies (§4.1.3,
/// §6, RFC 8628 §3.1) and specifies the same encoding for the authorization
/// request's query string (§3.1, Appendix B). So there is exactly one encoding
/// here, and this is it.
///
/// `pkce`, `refresh` and `device` each had their own `urlencode`. The first two
/// delegated to `url::form_urlencoded`; the third was a hand-rolled RFC 3986
/// percent-encoder that had drifted into encoding a space as `%20` where its two
/// siblings emit `+`, and leaving `~` bare where they escape nothing else the
/// same way. Both spellings decode to the same value at any conformant provider,
/// but only one of them was the encoding the requests claim to use.
pub(crate) fn urlencode(s: &str) -> String {
    url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_form_urlencoded_reserved_characters() {
        assert_eq!(urlencode("hello world"), "hello+world");
        assert_eq!(urlencode("a=b&c"), "a%3Db%26c");
        assert_eq!(urlencode("a b/c"), "a+b%2Fc");
    }

    #[test]
    fn leaves_unreserved_characters_alone() {
        assert_eq!(urlencode("hello.world-test_X"), "hello.world-test_X");
    }
}
