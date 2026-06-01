//! Fetch the registry index from HTTPS or a `file://` URL, with TTL-based caching.

use std::path::Path;
use std::time::{Duration, SystemTime};

use crate::error::AwareError;
use crate::registry::Index;
use crate::registry::catalog::Catalog;

/// Default location of the AWARE registry index. Override via `AWARE_REGISTRY`.
pub const DEFAULT_REGISTRY_URL: &str =
    "https://raw.githubusercontent.com/aware-aeco/aware/main/registry-index.json";

/// Default location of the AWARE agent catalog — the searchable sidecar to the
/// index (browse / search / capability-check). Override via `AWARE_CATALOG`.
pub const DEFAULT_CATALOG_URL: &str =
    "https://raw.githubusercontent.com/aware-aeco/aware/main/registry-catalog.json";

/// Cache TTL — 1 hour. Re-fetch happens after this expires.
const CACHE_TTL: Duration = Duration::from_secs(60 * 60);

pub fn registry_source() -> String {
    std::env::var("AWARE_REGISTRY").unwrap_or_else(|_| DEFAULT_REGISTRY_URL.to_string())
}

pub fn catalog_source() -> String {
    std::env::var("AWARE_CATALOG").unwrap_or_else(|_| DEFAULT_CATALOG_URL.to_string())
}

/// Fetch the agent catalog. Honors `AWARE_CATALOG`. Caches at
/// `<cache_dir>/registry-catalog.json` for `CACHE_TTL`, falls back to a stale
/// cache on network error. Returns `Ok(None)` when the source is **absent**
/// (HTTP 404 / missing file) — i.e. this registry has no catalog yet — so
/// callers can print clean guidance instead of treating it as a hard error.
pub fn fetch_catalog(cache_dir: &Path) -> Result<Option<Catalog>, AwareError> {
    std::fs::create_dir_all(cache_dir)?;
    let cache_path = cache_dir.join("registry-catalog.json");
    let source = catalog_source();

    if let Ok(meta) = std::fs::metadata(&cache_path)
        && let Ok(modified) = meta.modified()
        && let Ok(age) = SystemTime::now().duration_since(modified)
        && age < CACHE_TTL
    {
        return Ok(Some(Catalog::parse(std::fs::File::open(&cache_path)?)?));
    }

    match fetch_catalog_body(&source) {
        Ok(Some(body)) => {
            let catalog = Catalog::parse(body.as_bytes())?;
            std::fs::write(&cache_path, &body)?;
            Ok(Some(catalog))
        }
        Ok(None) => Ok(None), // 404 / missing — no catalog published for this registry
        Err(net_err) => {
            if cache_path.is_file() {
                eprintln!("warning: catalog fetch failed, using stale cache: {net_err}");
                Ok(Some(Catalog::parse(std::fs::File::open(&cache_path)?)?))
            } else {
                Err(net_err)
            }
        }
    }
}

/// `Ok(Some(body))` on 200, `Ok(None)` on a 404 / missing file (absent catalog),
/// `Err` on any other transport failure.
fn fetch_catalog_body(source: &str) -> Result<Option<String>, AwareError> {
    if let Some(path) = source.strip_prefix("file://") {
        return match std::fs::read_to_string(path) {
            Ok(s) => Ok(Some(s)),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(AwareError::Io(e)),
        };
    }
    match ureq::get(source).timeout(Duration::from_secs(20)).call() {
        Ok(resp) => {
            Ok(Some(resp.into_string().map_err(|e| {
                AwareError::Network(format!("read body: {e}"))
            })?))
        }
        Err(ureq::Error::Status(404, _)) => Ok(None),
        Err(e) => Err(AwareError::Network(format!("GET {source}: {e}"))),
    }
}

/// Fetch the index. Honors `AWARE_REGISTRY` for the source. Caches at
/// `<cache_dir>/registry-index.json` for `CACHE_TTL`. Falls back to the
/// cached copy on network error.
pub fn fetch_index(cache_dir: &Path) -> Result<Index, AwareError> {
    std::fs::create_dir_all(cache_dir)?;
    let cache_path = cache_dir.join("registry-index.json");
    let source = registry_source();

    // Cache hit within TTL?
    if let Ok(meta) = std::fs::metadata(&cache_path)
        && let Ok(modified) = meta.modified()
        && let Ok(age) = SystemTime::now().duration_since(modified)
        && age < CACHE_TTL
    {
        return Index::parse(std::fs::File::open(&cache_path)?);
    }

    // Fetch fresh.
    let body = match fetch_body(&source) {
        Ok(b) => b,
        Err(net_err) => {
            if cache_path.is_file() {
                eprintln!("warning: registry fetch failed, using stale cache: {net_err}");
                return Index::parse(std::fs::File::open(&cache_path)?);
            }
            return Err(net_err);
        }
    };

    let index = Index::parse(body.as_bytes())?;
    std::fs::write(&cache_path, &body)?;
    Ok(index)
}

fn fetch_body(source: &str) -> Result<String, AwareError> {
    if let Some(path) = source.strip_prefix("file://") {
        return std::fs::read_to_string(path).map_err(AwareError::Io);
    }
    let resp = ureq::get(source)
        .timeout(Duration::from_secs(20))
        .call()
        .map_err(|e| AwareError::Network(format!("GET {source}: {e}")))?;
    resp.into_string()
        .map_err(|e| AwareError::Network(format!("read body: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetches_from_file_url() {
        let tmp = tempfile::tempdir().unwrap();
        let idx = tmp.path().join("idx.json");
        std::fs::write(
            &idx,
            r#"{"version":"1.0","updated-at":"x","agents":{},"bundles":{}}"#,
        )
        .unwrap();

        let url = format!("file://{}", idx.display());
        let body = fetch_body(&url).unwrap();
        assert!(body.contains("\"version\""));
    }

    #[test]
    fn caches_after_first_fetch() {
        let tmp = tempfile::tempdir().unwrap();
        let idx = tmp.path().join("idx.json");
        std::fs::write(
            &idx,
            r#"{"version":"1.0","updated-at":"x","agents":{},"bundles":{}}"#,
        )
        .unwrap();
        let url = format!("file://{}", idx.display());
        // SAFETY: test-only; single-threaded test binary. set_var is process-global,
        // which is a known parallelism hazard, but acceptable here since only this
        // test touches AWARE_REGISTRY.
        unsafe { std::env::set_var("AWARE_REGISTRY", &url) };

        let cache = tmp.path().join("cache");
        let idx1 = fetch_index(&cache).unwrap();
        assert!(cache.join("registry-index.json").is_file());
        assert_eq!(idx1.version, "1.0");

        // Delete source — second call should still succeed from cache.
        std::fs::remove_file(&idx).unwrap();
        let idx2 = fetch_index(&cache).unwrap();
        assert_eq!(idx2.version, "1.0");

        // SAFETY: same test-only, single-threaded context as set_var above.
        unsafe { std::env::remove_var("AWARE_REGISTRY") };
    }

    #[test]
    fn fetch_catalog_some_then_absent() {
        let tmp = tempfile::tempdir().unwrap();
        // Present source → Ok(Some) + cached.
        let cat = tmp.path().join("registry-catalog.json");
        std::fs::write(&cat, r#"{"version":"1.0","updated-at":"x","agents":{}}"#).unwrap();
        // SAFETY: test-only; only catalog tests touch AWARE_CATALOG.
        unsafe { std::env::set_var("AWARE_CATALOG", format!("file://{}", cat.display())) };
        let cache = tmp.path().join("cache");
        let got = fetch_catalog(&cache).unwrap();
        assert!(got.is_some(), "present source → Some");
        assert!(cache.join("registry-catalog.json").is_file(), "cached");

        // Absent source (missing file) → Ok(None), not an error.
        let missing = tmp.path().join("nope.json");
        unsafe { std::env::set_var("AWARE_CATALOG", format!("file://{}", missing.display())) };
        let none = fetch_catalog(&tmp.path().join("cache2")).unwrap();
        assert!(none.is_none(), "absent source → None");

        // SAFETY: same test-only context.
        unsafe { std::env::remove_var("AWARE_CATALOG") };
    }
}
