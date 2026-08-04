//! Boundary-safe text truncation.
//!
//! Every "cap this for display" site in the CLI wants the same thing: the first
//! N *characters* of a string. Written by hand it is easy to reach for
//! `&s[..n]`, which indexes by *byte* and panics — not returns an error, panics
//! — when byte `n` lands inside a multi-byte UTF-8 character.
//!
//! That is not hypothetical. `aware search` shipped `&desc_one[..97]` and four
//! command descriptions already in `20-agents/` crash it: the curly quote in
//! sketchup-2026's `color-to-s` ("… in the form of “Color(255, 255, 255, 255)”.")
//! occupies bytes 95..98, so cutting at 97 aborts the process mid-render with
//! `end byte index 97 is not a char boundary`.
//!
//! `char_indices().nth(n)` yields a byte index that is a character boundary by
//! construction, so the slice below can never panic. CLAUDE.md §Code style —
//! "Errors as data, not exceptions" — is what this module exists to keep true
//! for the display paths.

use std::borrow::Cow;

/// The first `max_chars` characters of `s`, or `None` when `s` is already that
/// short and nothing needs cutting.
///
/// The returned prefix always ends on a character boundary. Callers that need
/// the size of what was dropped can take it as `s.len() - prefix.len()`.
pub fn cut_after_chars(s: &str, max_chars: usize) -> Option<&str> {
    // `nth(max_chars)` is the byte offset of the character *after* the ones we
    // keep — `None` when there is no such character, i.e. nothing to truncate.
    let (cut, _) = s.char_indices().nth(max_chars)?;
    Some(&s[..cut])
}

/// `s` capped at `max_chars` characters, with a single `…` appended when it was
/// actually cut. Borrows when no truncation is needed, so the common case
/// allocates nothing.
pub fn ellipsize(s: &str, max_chars: usize) -> Cow<'_, str> {
    match cut_after_chars(s, max_chars) {
        None => Cow::Borrowed(s),
        Some(prefix) => Cow::Owned(format!("{prefix}…")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The exact string that panicked `aware search` before this module existed:
    /// `20-agents/aeco/architecture/sketchup-2026` → `commands.color-to-s`.
    const SHIPPED_CRASHER: &str = "The #to_s method returns a string representation of the Sketchup::Color object, in the form of “Color(255, 255, 255, 255)”.";

    #[test]
    fn cut_returns_none_when_nothing_to_truncate() {
        assert_eq!(cut_after_chars("abc", 3), None);
        assert_eq!(cut_after_chars("abc", 4), None);
        assert_eq!(cut_after_chars("", 0), None);
        // Characters, not bytes: four chars of three bytes each is 12 bytes, and
        // a byte-length test would wrongly decide this needs cutting.
        assert_eq!(cut_after_chars("日本語だ", 4), None);
    }

    #[test]
    fn cut_stops_on_a_character_boundary() {
        assert_eq!(cut_after_chars("abcdef", 3), Some("abc"));
        assert_eq!(cut_after_chars("日本語だよ", 3), Some("日本語"));
    }

    /// The regression this module was written for. `&s[..97]` aborts here; the
    /// boundary-safe cut lands on the character before the quote.
    #[test]
    fn cut_survives_a_multibyte_char_straddling_the_limit() {
        let byte_97 = SHIPPED_CRASHER.as_bytes()[97];
        assert_eq!(
            byte_97 & 0xC0,
            0x80,
            "fixture no longer straddles byte 97, so it no longer reproduces the bug"
        );

        let cut = cut_after_chars(SHIPPED_CRASHER, 97).expect("long enough to cut");
        assert_eq!(cut.chars().count(), 97);
        assert!(SHIPPED_CRASHER.starts_with(cut));
        // Every byte offset must be a legal boundary, for every limit — this is
        // the property, not just the one offset that happened to bite.
        for limit in 0..SHIPPED_CRASHER.chars().count() {
            let prefix = cut_after_chars(SHIPPED_CRASHER, limit).expect("shorter than the whole");
            assert_eq!(prefix.chars().count(), limit);
        }
    }

    #[test]
    fn ellipsize_appends_only_when_it_cut() {
        assert_eq!(ellipsize("abc", 5), "abc");
        assert_eq!(ellipsize("abcdef", 3), "abc…");
        assert_eq!(ellipsize(SHIPPED_CRASHER, 97).chars().count(), 98);
    }

    /// A limit that falls between the two halves of a surrogate-pair-sized
    /// character. Emoji are four bytes, the widest case the cut has to handle.
    #[test]
    fn ellipsize_handles_wide_characters() {
        let s = "🏗🏗🏗🏗";
        assert_eq!(s.len(), 16, "each emoji is four bytes");
        assert_eq!(ellipsize(s, 2), "🏗🏗…");
        assert_eq!(ellipsize(s, 4), "🏗🏗🏗🏗");
    }
}
