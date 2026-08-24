//! Wall-clock timestamp helpers.
//!
//! One helper today, but a single home so provenance stamps, receipts,
//! app-compile timestamps and generator provenance all serialize the same
//! way — the format change ever needs to land in one place.

/// Current UTC time as an RFC 3339 string.
///
/// Byte-identical copies previously lived in `builder::now_iso` and
/// `runtime::provenance::now_iso`; both are now re-exports of this. Everywhere
/// else in the crate that stamps a timestamp calls `chrono::Utc::now()`
/// inline (`app_lock`, `lockfile`, `runtime::context`) — those are one-line
/// call sites whose formats aren't uniform, so this helper doesn't try to
/// swallow them.
pub fn now_iso() -> String {
    chrono::Utc::now().to_rfc3339()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn now_iso_parses_as_rfc3339() {
        let s = now_iso();
        // A quick shape check that any RFC 3339 timestamp must satisfy: 4-digit
        // year, `T` between date and time, and a timezone suffix.
        assert!(s.len() >= 20, "unexpectedly short: {s}");
        assert!(
            s.chars().nth(4) == Some('-'),
            "expected `-` at year-month: {s}"
        );
        assert!(s.contains('T'), "missing date/time separator: {s}");
        // `chrono` proper is the real parser.
        chrono::DateTime::parse_from_rfc3339(&s).expect(
            "now_iso() output must round-trip through chrono::DateTime::parse_from_rfc3339",
        );
    }
}
