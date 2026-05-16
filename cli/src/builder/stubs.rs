//! Tier-C builders.
//!
//! `build_from_dlls` and `build_with_decompile` now route to the C# sidecar.
//! `build_from_com` and `build_from_headers` stay stubbed (deferred to v0.5.2+).

#![allow(dead_code)]

use crate::builder::GeneratedAgent;
use crate::error::AwareError;

pub fn build_from_dlls(path: &str, agent_id: Option<&str>) -> Result<GeneratedAgent, AwareError> {
    let args = crate::sidecar::ReflectDllsArgs {
        globs: vec![path.to_string()],
        agent_id: agent_id.map(String::from),
    };
    crate::sidecar::reflect_dlls(args)
}

pub fn build_from_com(
    _progid: &str,
    _agent_id: Option<&str>,
) -> Result<GeneratedAgent, AwareError> {
    Err(AwareError::NotYetImplemented(
        "--from-com is deferred to v0.5.2 (separate phase: COM TypeLib interop in the sidecar)",
    ))
}

pub fn build_from_headers(
    _path: &str,
    _agent_id: Option<&str>,
) -> Result<GeneratedAgent, AwareError> {
    Err(AwareError::NotYetImplemented(
        "--from-headers is deferred to v0.5.3 (separate phase: libclang via P/Invoke or clang.exe shell)",
    ))
}

pub fn build_with_decompile(pkg: &str) -> Result<GeneratedAgent, AwareError> {
    // Parse the pkg spec: <name>@<version>
    let (name, version) = match pkg.split_once('@') {
        Some((n, v)) => (n.to_string(), v.to_string()),
        None => {
            return Err(AwareError::Validation(
                "--decompile requires <pkg>@<version>".into(),
            ));
        }
    };
    // For v0.5.1: --decompile needs an already-downloaded .nupkg. The v0.5 nuget builder
    // does this download already (see cli::builder::nuget). For now, this is the simplest
    // viable form: caller must point at a local .nupkg file via the package_path arg.
    let args = crate::sidecar::DecompileArgs {
        package_path: name,
        version,
        agent_id: None,
        accept_license: false,
    };
    crate::sidecar::decompile(args)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dlls_no_sidecar_returns_not_found() {
        // When AWARE_SIDECAR points to a non-existent file discovery fails with NotFound.
        // SAFETY: single-threaded test; env var is restored immediately after.
        unsafe {
            std::env::set_var("AWARE_SIDECAR", "C:/aware-sidecar-does-not-exist-test.exe");
        }
        let err = build_from_dlls("C:/some.dll", None).unwrap_err();
        unsafe {
            std::env::remove_var("AWARE_SIDECAR");
        }
        // Discovery fails before reaching the sidecar, so the error is NotFound.
        assert!(matches!(err, AwareError::NotFound(_)));
    }

    #[test]
    fn com_returns_not_yet_implemented() {
        let err = build_from_com("progid", None).unwrap_err();
        assert!(matches!(err, AwareError::NotYetImplemented(_)));
        assert!(err.to_string().contains("not yet implemented"));
    }

    #[test]
    fn headers_returns_not_yet_implemented() {
        let err = build_from_headers("path", None).unwrap_err();
        assert!(matches!(err, AwareError::NotYetImplemented(_)));
    }

    #[test]
    fn decompile_no_sidecar_returns_not_found() {
        // SAFETY: single-threaded test; env var is restored immediately after.
        unsafe {
            std::env::set_var("AWARE_SIDECAR", "C:/aware-sidecar-does-not-exist-test.exe");
        }
        let err = build_with_decompile("Pkg@1.0").unwrap_err();
        unsafe {
            std::env::remove_var("AWARE_SIDECAR");
        }
        assert!(matches!(err, AwareError::NotFound(_)));
    }

    #[test]
    fn decompile_missing_version_is_validation_error() {
        let err = build_with_decompile("PkgWithoutVersion").unwrap_err();
        assert!(matches!(err, AwareError::Validation(_)));
        assert!(err.to_string().contains("@"));
    }
}
