//! Workflow migration machinery (#628): carrying an approved app forward to a
//! newer agent version without pretending a person re-approved it.
//!
//! * [`effect`] — what each node of a workflow is *declared* to do (read or
//!   write), where that mode came from, and whether a whole workflow is
//!   *declared read-only* under both the old and the new pins.
//! * [`contract`] — the executable-contract diff between two stored packages of
//!   one agent (`aware.contract-diff/v1`): what a run would hand the executor,
//!   minus what the run never reads.
//! * [`compare`] — the fixed-state comparison verdict built on that diff.
//!   `identical-instructions` (checked by inspection, nothing run) is distinct
//!   from an executed `pass` and is not accepted for the no-click path in v1.
//! * [`plan`] — `aware app migrate plan|prepare`: one evaluation per app (its
//!   state, targets, effect, contract, comparison and plain-English reasons).
//! * [`files`] — the only files a migration writes before promotion: the
//!   candidate + evidence under `<source-dir>/.aware-migration/`, and the HOLD
//!   record under `<source-dir>/.aware-approvals/`. `aware app run` never reads
//!   either.
//!
//! Nothing here replaces `<app>.lock`. Promotion (`migrate promote|revert`)
//! arrives with #628 PR3.

pub mod compare;
pub mod contract;
pub mod effect;
pub mod files;
pub mod plan;

use serde::{Deserialize, Serialize};

use crate::error::AwareError;

/// One reason, as data: a stable machine `code` and a plain-English `text` a
/// front door shows a person as written.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Reason {
    pub code: String,
    pub text: String,
}

impl Reason {
    pub fn new(code: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            code: code.into(),
            text: text.into(),
        }
    }

    /// The `E_…` error code this reason was made from, when it was made from
    /// one by [`reason_from_error`] (`migrate-target-ambiguous` →
    /// `E_MIGRATE_TARGET_AMBIGUOUS`).
    pub fn error_code(&self) -> String {
        format!("E_{}", self.code.to_ascii_uppercase().replace('-', "_"))
    }
}

/// An error turned into a reason: the bracketed `[E_…]` code (when present)
/// becomes the kebab-case `code` without its `E_` prefix, and the rest of the
/// message — minus the error-class prefix — becomes the `text`. An error with
/// no bracketed code becomes `cannot-evaluate`.
pub fn reason_from_error(error: &AwareError) -> Reason {
    let message = error.to_string();
    let body = message
        .split_once(": ")
        .filter(|(class, _)| !class.contains('['))
        .map_or(message.as_str(), |(_, rest)| rest);
    if let Some(start) = body.find("[E_")
        && let Some(len) = body[start..].find(']')
    {
        let code = &body[start + 3..start + len];
        let text = format!("{}{}", &body[..start], body[start + len + 1..].trim_start());
        return Reason::new(code.to_ascii_lowercase().replace('_', "-"), text.trim());
    }
    Reason::new("cannot-evaluate", body.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_error_code_becomes_a_reason_code_and_back() {
        let error = AwareError::Validation(
            "[E_MIGRATE_TARGET_AMBIGUOUS] agent t 1.0.0 is stored twice".into(),
        );
        let reason = reason_from_error(&error);
        assert_eq!(reason.code, "migrate-target-ambiguous");
        assert_eq!(reason.text, "agent t 1.0.0 is stored twice");
        assert_eq!(reason.error_code(), "E_MIGRATE_TARGET_AMBIGUOUS");

        let plain = reason_from_error(&AwareError::Internal("disk on fire".into()));
        assert_eq!(plain.code, "cannot-evaluate");
        assert!(plain.text.contains("disk on fire"), "{plain:?}");
    }
}
