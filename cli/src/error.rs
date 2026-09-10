//! AWARE CLI error type and exit-code mapping.
//!
//! Exit codes match the table in `10-core/cli-spec.md`.

use thiserror::Error;

/// Heap-owned correlation fields keep the global error enum compact without
/// changing the structured JSON representation.
#[derive(Debug, Clone)]
pub(crate) struct AgentErrorDetails(pub std::collections::BTreeMap<String, String>);

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructuredAgentError {
    pub code: String,
    pub phase: String,
    pub retryable: bool,
    pub message: String,
    pub diagnostic_id: String,
    /// Optional bounded, non-secret correlation data for a failed operation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<std::collections::BTreeMap<String, String>>,
}

#[derive(Debug, Error)]
pub enum AwareError {
    #[error("not yet implemented: {0} (see cli-roadmap.md for phasing)")]
    NotYetImplemented(&'static str),

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("network error: {0}")]
    Network(String),

    #[error(
        "agent error {code} ({phase}, retryable={retryable}, diagnostic-id={diagnostic_id}): {message}"
    )]
    AgentStructured {
        code: String,
        phase: String,
        retryable: bool,
        message: String,
        diagnostic_id: String,
        details: Option<Box<AgentErrorDetails>>,
    },

    #[error("permission denied: {0}")]
    PermissionDenied(String),

    #[error("auth expired — run: aware connect {0} --refresh")]
    AuthExpired(String),

    #[error("agent or app not found: {0}")]
    NotFound(String),

    #[error("conflict: {0}")]
    Conflict(String),

    #[error("io: {0}")]
    Io(#[from] std::io::Error),

    #[error("yaml: {0}")]
    Yaml(#[from] serde_yaml::Error),

    #[error("json: {0}")]
    Json(#[from] serde_json::Error),

    #[error("internal: {0}")]
    Internal(String),
}

impl AwareError {
    /// Return the bounded machine-readable agent failure carried across a bridge boundary.
    pub fn structured_agent_error(&self) -> Option<StructuredAgentError> {
        match self {
            Self::AgentStructured {
                code,
                phase,
                retryable,
                message,
                diagnostic_id,
                details,
            } => Some(StructuredAgentError {
                code: code.clone(),
                phase: phase.clone(),
                retryable: *retryable,
                message: message.clone(),
                diagnostic_id: diagnostic_id.clone(),
                details: details.as_deref().map(|value| value.0.clone()),
            }),
            _ => None,
        }
    }

    /// Exit code per `cli-spec.md` § Exit codes.
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::NotYetImplemented(_) => 1,
            Self::Validation(_) => 3,
            Self::Network(_) | Self::AgentStructured { .. } => 4,
            Self::PermissionDenied(_) => 5,
            Self::AuthExpired(_) => 6,
            Self::NotFound(_) => 7,
            Self::Conflict(_) => 8,
            Self::Io(_) | Self::Yaml(_) | Self::Json(_) | Self::Internal(_) => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structured_error_details_are_optional_and_backward_compatible() {
        let legacy =
            r#"{"code":"x","phase":"dispatch","retryable":false,"message":"m","diagnosticId":"d"}"#;
        let parsed: StructuredAgentError = serde_json::from_str(legacy).unwrap();
        assert!(parsed.details.is_none());

        let mut details = std::collections::BTreeMap::new();
        details.insert("attemptId".into(), "rfi-001-mail-v1".into());
        details.insert("rfcMessageId".into(), "<rfi-001@example.invalid>".into());
        let value = serde_json::to_value(StructuredAgentError {
            code: "gmail.send.outcome-unknown".into(),
            phase: "dispatch".into(),
            retryable: false,
            message: "Send outcome is unknown; reconcile before retrying.".into(),
            diagnostic_id: "rfi-001-mail-v1".into(),
            details: Some(details),
        })
        .unwrap();
        assert_eq!(value["details"]["attemptId"], "rfi-001-mail-v1");
        assert_eq!(
            value["details"]["rfcMessageId"],
            "<rfi-001@example.invalid>"
        );
    }
}
