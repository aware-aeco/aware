//! Tier-C stubs for sources requiring the v0.5.1 C# NativeAOT sidecar.

#![allow(dead_code)]

use crate::builder::GeneratedAgent;
use crate::error::AwareError;

const MSG: &str = "this source requires the v0.5.1 C# NativeAOT sidecar (tracked separately); v0.5 ships --from-openapi, --from-cli, --from-nuget, --from-python only";

pub fn build_from_dlls(_path: &str, _agent_id: Option<&str>) -> Result<GeneratedAgent, AwareError> {
    Err(AwareError::NotYetImplemented(MSG))
}

pub fn build_from_com(
    _progid: &str,
    _agent_id: Option<&str>,
) -> Result<GeneratedAgent, AwareError> {
    Err(AwareError::NotYetImplemented(MSG))
}

pub fn build_from_headers(
    _path: &str,
    _agent_id: Option<&str>,
) -> Result<GeneratedAgent, AwareError> {
    Err(AwareError::NotYetImplemented(MSG))
}

pub fn build_with_decompile(_pkg: &str) -> Result<GeneratedAgent, AwareError> {
    Err(AwareError::NotYetImplemented(MSG))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dlls_returns_not_yet_implemented() {
        let err = build_from_dlls("path", None).unwrap_err();
        assert!(matches!(err, AwareError::NotYetImplemented(_)));
        assert!(err.to_string().contains("v0.5.1"));
    }

    #[test]
    fn com_returns_not_yet_implemented() {
        let err = build_from_com("progid", None).unwrap_err();
        assert!(matches!(err, AwareError::NotYetImplemented(_)));
    }

    #[test]
    fn headers_returns_not_yet_implemented() {
        let err = build_from_headers("path", None).unwrap_err();
        assert!(matches!(err, AwareError::NotYetImplemented(_)));
    }

    #[test]
    fn decompile_returns_not_yet_implemented() {
        let err = build_with_decompile("pkg").unwrap_err();
        assert!(matches!(err, AwareError::NotYetImplemented(_)));
    }
}
