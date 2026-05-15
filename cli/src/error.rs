//! AWARE CLI error type and exit-code mapping.
//!
//! Exit codes match the table in `10-core/cli-spec.md`.

use thiserror::Error;

// Variants are forward-declared; callers will be added in later CLI phases.
#[allow(dead_code)]
#[derive(Debug, Error)]
pub enum AwareError {
    #[error("not yet implemented: {0} (see cli-roadmap.md for phasing)")]
    NotYetImplemented(&'static str),

    #[error("validation failed: {0}")]
    Validation(String),

    #[error("network error: {0}")]
    Network(String),

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
    /// Exit code per `cli-spec.md` § Exit codes.
    pub fn exit_code(&self) -> i32 {
        match self {
            Self::NotYetImplemented(_) => 1,
            Self::Validation(_) => 3,
            Self::Network(_) => 4,
            Self::PermissionDenied(_) => 5,
            Self::AuthExpired(_) => 6,
            Self::NotFound(_) => 7,
            Self::Conflict(_) => 8,
            Self::Io(_) | Self::Yaml(_) | Self::Json(_) | Self::Internal(_) => 1,
        }
    }
}
