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

pub fn build_from_com(progid: &str, agent_id: Option<&str>) -> Result<GeneratedAgent, AwareError> {
    if !cfg!(windows) {
        return Err(AwareError::NotYetImplemented("--from-com is Windows-only"));
    }
    let args = crate::sidecar::FromComArgs {
        progid: progid.to_string(),
        agent_id: agent_id.map(String::from),
    };
    crate::sidecar::from_com(args)
}

pub fn build_from_headers(
    path: &str,
    agent_id: Option<&str>,
) -> Result<GeneratedAgent, AwareError> {
    let args = crate::sidecar::FromHeadersArgs {
        files: vec![path.to_string()],
        agent_id: agent_id.map(String::from),
    };
    crate::sidecar::from_headers(args)
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
    fn com_no_sidecar_returns_not_found_or_not_implemented() {
        // On Windows, build_from_com routes to the sidecar; without a sidecar binary it returns
        // NotFound. On non-Windows it returns NotYetImplemented (Windows-only guard).
        // SAFETY: single-threaded test; env var is restored immediately after.
        unsafe {
            std::env::set_var("AWARE_SIDECAR", "C:/aware-sidecar-does-not-exist-test.exe");
        }
        let err = build_from_com("WScript.Shell", None).unwrap_err();
        unsafe {
            std::env::remove_var("AWARE_SIDECAR");
        }
        let ok = matches!(
            err,
            AwareError::NotFound(_) | AwareError::NotYetImplemented(_)
        );
        assert!(ok, "expected NotFound or NotYetImplemented, got: {err}");
    }

    #[test]
    fn headers_no_sidecar_returns_not_found() {
        // build_from_headers now routes to the sidecar; without a sidecar binary it returns NotFound.
        // SAFETY: single-threaded test; env var is restored immediately after.
        unsafe {
            std::env::set_var("AWARE_SIDECAR", "C:/aware-sidecar-does-not-exist-test.exe");
        }
        let err = build_from_headers("path/to/header.h", None).unwrap_err();
        unsafe {
            std::env::remove_var("AWARE_SIDECAR");
        }
        assert!(matches!(err, AwareError::NotFound(_)));
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
