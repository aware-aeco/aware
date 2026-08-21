//! OS-keychain backed token storage via the `keyring` crate.
//!
//! Token JSON stored under service="aware-aeco", account="<integration>[.<alias>]".
//!
//! ## Windows file fallback
//! Windows Credential Manager caps credential blobs at 2 560 bytes
//! (`CRED_MAX_CREDENTIAL_BLOB_SIZE`). Microsoft Graph / Google / Trimble tokens
//! are large JWTs — typically >4 KB — so `CredWrite` rejects them. When a
//! keyring write fails for any reason we silently fall back to a JSON file at
//! `~/.aware/credentials/<account>.json` (same path the runtime's
//! `load_secret_value` already reads as a manual-override fallback). On read we
//! check the keyring first, then the file; on delete we clear both.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::AwareError;

const SERVICE_NAME: &str = "aware-aeco";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TokenSource {
    Oauth,
    Paste,
}

fn default_source() -> TokenSource {
    TokenSource::Oauth
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoredToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: i64,
    pub scope: String,
    pub token_type: String,
    pub integration: String,
    pub obtained_at: i64,
    #[serde(default = "default_source")]
    pub source: TokenSource,
}

fn account_name(integration: &str, alias: Option<&str>) -> String {
    match alias {
        Some(a) => format!("{integration}.{a}"),
        None => integration.to_string(),
    }
}

/// Keychain account for a BYO OAuth client secret. Distinct `oauth-app.` prefix
/// keeps it from colliding with the access/refresh token slot (#146).
fn app_account_name(integration: &str, alias: Option<&str>) -> String {
    match alias {
        Some(a) => format!("oauth-app.{integration}.{a}"),
        None => format!("oauth-app.{integration}"),
    }
}

/// Path of the file fallback for a given account.
/// `account` is the value returned by `account_name()`.
fn cred_file_path(aware_home: &Path, account: &str) -> PathBuf {
    aware_home
        .join("credentials")
        .join(format!("{account}.json"))
}

/// Whether to use the OS keychain at all.
///
/// Disabled when `AWARE_DISABLE_KEYRING` is set — the credentials-file fallback
/// under `aware_home` then becomes the sole store (useful on headless / locked-down
/// machines, and for test isolation since the OS keychain is global and not scoped
/// by `AWARE_HOME`). In unit tests we default to file-only for the same isolation
/// reason, unless a test explicitly opts into the real keychain via
/// `AWARE_TEST_KEYRING`.
fn keyring_enabled() -> bool {
    if std::env::var_os("AWARE_DISABLE_KEYRING").is_some() {
        return false;
    }
    if cfg!(test) && std::env::var_os("AWARE_TEST_KEYRING").is_none() {
        return false;
    }
    true
}

// ── Public API ────────────────────────────────────────────────────────────────

/// Persist `token` to the OS keychain.
/// If the keychain write fails (e.g. Windows 2 560-byte blob limit), the full
/// token JSON is written to `<aware_home>/credentials/<account>.json` instead
/// so that large cloud JWTs are always stored regardless of platform limits.
pub fn store_token(
    token: &StoredToken,
    alias: Option<&str>,
    aware_home: &Path,
) -> Result<(), AwareError> {
    let body = serde_json::to_string(token)
        .map_err(|e| AwareError::Internal(format!("serialize token: {e}")))?;
    let account = account_name(&token.integration, alias);

    if !keyring_enabled() {
        return write_cred_file(aware_home, &account, &body);
    }

    let entry = keyring::Entry::new(SERVICE_NAME, &account)
        .map_err(|e| AwareError::Internal(format!("keyring entry: {e}")))?;

    match entry.set_password(&body) {
        Ok(()) => Ok(()),
        Err(e) => {
            // Keyring write failed — most commonly the Windows Credential Manager
            // 2 560-byte limit. Fall back to a credentials file so large OAuth
            // tokens don't silently fail the connect flow.
            eprintln!(
                "aware: keyring write failed ({e}); \
                 falling back to ~/.aware/credentials file"
            );
            // A PRIOR keychain entry for this account must go first. `load_token`
            // reads the keychain before the file, so one left standing shadows the
            // value we are about to write: the store reports success and every
            // later read keeps serving the credential this write was meant to
            // supersede. That is reachable exactly when it hurts most — a small
            // token stored to the keychain, then rotated to one over the Windows
            // blob limit, which is the rotation `aware credential put` exists for
            // (#436).
            //
            // Clearing before the file write also fails in the safe direction: if
            // the file write then fails there is no credential at all, which the
            // next call surfaces, rather than a superseded one it cannot see.
            clear_shadowing_entry(&entry, &account)?;
            write_cred_file(aware_home, &account, &body)
        }
    }
}

/// Remove a keychain entry that would otherwise shadow the file fallback.
///
/// `load_token` reads the keychain before the file, so any entry left standing
/// wins over the value about to be written. Three cases:
///
/// * `NoEntry` — nothing to shadow, nothing to do.
/// * `Ok(_)` — a live entry `load_token` would return in preference to the file.
///   It must go, and a delete we cannot complete is an error rather than a
///   silently stale credential.
/// * any other error — **also fatal**, because we cannot tell whether an entry is
///   sitting there.
///
/// That third case was originally allowed through, on the reasoning that an
/// unreadable backend makes `load_token` fail loudly rather than serve a stale
/// value. Codex refuted it on #443 and it is worth recording why, because the
/// argument is not obviously wrong: `runtime::context::load_secret` — the path
/// REST auth actually takes — *swallows* a `load_token` error and falls through
/// to the file. So during a transient outage the runtime reads the new
/// credential and everything looks correct. When the backend comes back,
/// `load_token` starts returning the untouched old entry, and calls silently
/// revert to the superseded credential. The outage is temporary; the entry
/// outlives it.
///
/// So an unverifiable keychain means the rotation cannot be completed, and
/// saying so beats a success that decays. The error names
/// `AWARE_DISABLE_KEYRING`, which is the supported way to keep AWARE out of the
/// keychain entirely on a headless or locked-down machine — with it set,
/// `keyring_enabled` is false, this path is never reached, and the file store is
/// the only one, so nothing can shadow anything.
fn clear_shadowing_entry(entry: &keyring::Entry, account: &str) -> Result<(), AwareError> {
    match entry.get_password() {
        Err(keyring::Error::NoEntry) => return Ok(()),
        Ok(_) => {}
        Err(e) => {
            return Err(AwareError::PermissionDenied(format!(
                "the keychain entry for {account} could not be read ({e}), so whether a superseded \
                 credential is sitting there cannot be established — refusing to write the \
                 credentials-file fallback, which such an entry would shadow on every read once \
                 the keychain is reachable again. Set AWARE_DISABLE_KEYRING=1 to keep AWARE out of \
                 the keychain and use the credentials file as the only store"
            )));
        }
    }
    match entry.delete_credential() {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(AwareError::PermissionDenied(format!(
            "keyring delete of the superseded entry for {account}: {e} — refusing to write the \
             credentials-file fallback, which that entry would shadow on every read"
        ))),
    }
}

/// Load the token from the OS keychain, falling back to the credentials file
/// if the keychain has no entry.
pub fn load_token(
    integration: &str,
    alias: Option<&str>,
    aware_home: &Path,
) -> Result<Option<StoredToken>, AwareError> {
    let account = account_name(integration, alias);

    if !keyring_enabled() {
        return read_cred_file(integration, alias, aware_home);
    }

    let entry = keyring::Entry::new(SERVICE_NAME, &account)
        .map_err(|e| AwareError::Internal(format!("keyring entry: {e}")))?;

    match entry.get_password() {
        Ok(body) => {
            let token: StoredToken = serde_json::from_str(&body)
                .map_err(|e| AwareError::Validation(format!("token JSON: {e}")))?;
            Ok(Some(token))
        }
        Err(keyring::Error::NoEntry) => {
            // Nothing in the keychain — check the file fallback.
            read_cred_file(integration, alias, aware_home)
        }
        Err(e) => Err(AwareError::PermissionDenied(format!("keyring read: {e}"))),
    }
}

/// Remove the credential from the OS keychain and the file fallback (if any).
pub fn delete_token(
    integration: &str,
    alias: Option<&str>,
    aware_home: &Path,
) -> Result<(), AwareError> {
    let account = account_name(integration, alias);

    let keyring_result = if keyring_enabled() {
        let entry = keyring::Entry::new(SERVICE_NAME, &account)
            .map_err(|e| AwareError::Internal(format!("keyring entry: {e}")))?;
        match entry.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()), // idempotent
            Err(e) => Err(AwareError::PermissionDenied(format!("keyring delete: {e}"))),
        }
    } else {
        Ok(())
    };

    // Also clean up any file fallback, regardless of keyring result — and report
    // a removal that did not happen. This used to be `let _ = remove_file(…)`,
    // which made a revoke that left the credential readable exit 0: the caller is
    // told the credential is gone while `load_token` still returns it. Revocation
    // has to fail closed to be worth anything (#436).
    let file_result = match std::fs::remove_file(cred_file_path(aware_home, &account)) {
        // Already absent is success — `delete_token` is idempotent by contract.
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(AwareError::PermissionDenied(format!(
            "credential file delete for {account}: {e}"
        ))),
    };

    // Both stores are attempted before either error surfaces, so a failure in one
    // never leaves the other standing.
    keyring_result.and(file_result)
}

// ── BYO OAuth app-secret API (Tier 2, #146) ──────────────────────────────────

/// Store a BYO OAuth client secret in the OS keychain. Falls back to a
/// credentials file (same path scheme as tokens) when the keychain write fails.
pub fn store_app_secret(
    integration: &str,
    alias: Option<&str>,
    secret: &str,
    aware_home: &Path,
) -> Result<(), AwareError> {
    let account = app_account_name(integration, alias);
    if !keyring_enabled() {
        return write_app_secret_file(aware_home, &account, secret);
    }
    let entry = keyring::Entry::new(SERVICE_NAME, &account)
        .map_err(|e| AwareError::Internal(format!("keyring entry: {e}")))?;
    match entry.set_password(secret) {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!(
                "aware: keyring write failed ({e}); \
                 falling back to ~/.aware/credentials file"
            );
            write_app_secret_file(aware_home, &account, secret)
        }
    }
}

/// Load a BYO client secret: keychain first, then file fallback. `Ok(None)` when unset.
///
/// A BYO app secret is *optional* config. If the OS keychain backend is
/// unavailable (e.g. headless Linux / CI with no libsecret/dbus), we treat that
/// as "not configured" and fall through to the credentials file — never an error.
/// Otherwise `with_profile` would abort every `aware connect` flow (even
/// `--from-file`, which needs no OAuth) on keychain-less machines.
pub fn load_app_secret(
    integration: &str,
    alias: Option<&str>,
    aware_home: &Path,
) -> Result<Option<String>, AwareError> {
    // Exact-account lookup. The caller (`config::with_profile`) decides whether to
    // pass the alias or the default scope, so that the secret slot matches the
    // resolved profile's client_id and a stale secret is never paired with the
    // wrong app.
    let account = app_account_name(integration, alias);
    if !keyring_enabled() {
        return read_app_secret_file(aware_home, &account);
    }
    match keyring::Entry::new(SERVICE_NAME, &account).and_then(|e| e.get_password()) {
        Ok(s) => Ok(Some(s)),
        // NoEntry or any keyring-unavailable error → check the file fallback.
        Err(_) => read_app_secret_file(aware_home, &account),
    }
}

/// Remove a stored BYO client secret from the keychain and file fallback (idempotent).
///
/// Not yet wired: `run_disconnect` clears the token but not the BYO app secret,
/// so the secret currently outlives `aware connect disconnect`. Kept because the
/// gap is the missing call, not this function.
#[allow(dead_code)]
pub fn delete_app_secret(
    integration: &str,
    alias: Option<&str>,
    aware_home: &Path,
) -> Result<(), AwareError> {
    let account = app_account_name(integration, alias);
    let keyring_result = if keyring_enabled() {
        let entry = keyring::Entry::new(SERVICE_NAME, &account)
            .map_err(|e| AwareError::Internal(format!("keyring entry: {e}")))?;
        match entry.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(AwareError::PermissionDenied(format!("keyring delete: {e}"))),
        }
    } else {
        Ok(())
    };
    let file = cred_file_path(aware_home, &account);
    if file.is_file() {
        let _ = std::fs::remove_file(&file);
    }
    keyring_result
}

fn write_app_secret_file(aware_home: &Path, account: &str, secret: &str) -> Result<(), AwareError> {
    let path = cred_file_path(aware_home, account);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| AwareError::Internal(format!("create credentials dir: {e}")))?;
    }
    write_atomic_restricted(&path, secret.as_bytes())
        .map_err(|e| AwareError::PermissionDenied(format!("app-secret file write: {e}")))
}

fn read_app_secret_file(aware_home: &Path, account: &str) -> Result<Option<String>, AwareError> {
    let path = cred_file_path(aware_home, account);
    if !path.is_file() {
        return Ok(None);
    }
    let s = std::fs::read_to_string(&path)
        .map_err(|e| AwareError::Internal(format!("app-secret file read: {e}")))?;
    Ok(Some(s.trim().to_string()))
}

// ── File fallback helpers ────────────────────────────────────────────────────

fn write_cred_file(aware_home: &Path, account: &str, body: &str) -> Result<(), AwareError> {
    let path = cred_file_path(aware_home, account);
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| AwareError::Internal(format!("create credentials dir: {e}")))?;
    }
    // On Unix set mode 0o600; on Windows the user-profile directory is
    // already ACL-restricted, so plain write suffices.
    write_atomic_restricted(&path, body.as_bytes())
        .map_err(|e| AwareError::PermissionDenied(format!("credential file write: {e}")))
}

fn read_cred_file(
    integration: &str,
    alias: Option<&str>,
    aware_home: &Path,
) -> Result<Option<StoredToken>, AwareError> {
    let account = account_name(integration, alias);
    let path = cred_file_path(aware_home, &account);
    if !path.is_file() {
        return Ok(None);
    }
    let body = std::fs::read_to_string(&path)
        .map_err(|e| AwareError::Internal(format!("credential file read: {e}")))?;

    // Try the full StoredToken shape first (written by this module's fallback).
    if let Ok(token) = serde_json::from_str::<StoredToken>(&body) {
        return Ok(Some(token));
    }

    // Legacy / manual format: {"access_token": "...", ...} written by users or
    // the runtime's documented manual-override path.
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64;
    let v: serde_json::Value = serde_json::from_str(&body)
        .map_err(|e| AwareError::Validation(format!("credential file JSON: {e}")))?;
    let access_token = v
        .get("access_token")
        .and_then(|x| x.as_str())
        .or_else(|| v.get("token").and_then(|x| x.as_str()))
        .ok_or_else(|| {
            AwareError::Validation(
                "credential file has neither access_token nor token field".into(),
            )
        })?
        .to_string();
    Ok(Some(StoredToken {
        access_token,
        refresh_token: v
            .get("refresh_token")
            .and_then(|x| x.as_str())
            .map(String::from),
        expires_at: v.get("expires_at").and_then(|x| x.as_i64()).unwrap_or(0),
        scope: v
            .get("scope")
            .and_then(|x| x.as_str())
            .unwrap_or("")
            .to_string(),
        token_type: v
            .get("token_type")
            .and_then(|x| x.as_str())
            .unwrap_or("Bearer")
            .to_string(),
        integration: integration.to_string(),
        obtained_at: now,
        source: TokenSource::Paste,
    }))
}

#[cfg(unix)]
fn create_restricted(path: &Path) -> std::io::Result<std::fs::File> {
    use std::os::unix::fs::OpenOptionsExt;
    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        // 0600 is applied by `open` itself, so the file is never briefly
        // world-readable the way a create-then-chmod would leave it.
        .mode(0o600)
        .open(path)
}

#[cfg(not(unix))]
fn create_restricted(path: &Path) -> std::io::Result<std::fs::File> {
    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(path)
}

fn write_restricted(path: &Path, data: &[u8]) -> std::io::Result<()> {
    use std::io::Write;
    let mut file = create_restricted(path)?;
    file.write_all(data)?;
    // Flush to the device before the caller renames this over a live credential:
    // a rename is ordered against the metadata, not against unwritten data, so an
    // unsynced temp file can be published as a zero-length credential.
    file.sync_all()
}

/// Serial for the temp-file name, so two writes racing inside one process cannot
/// pick the same scratch path and truncate each other's bytes mid-write.
static TEMP_SERIAL: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

/// Write `data` to `path` so that a concurrent reader sees either the whole old
/// credential or the whole new one — never a half-written file, and never the
/// empty window a truncating in-place write opens.
///
/// That window is not theoretical here: the runtime re-reads the credential on
/// every REST invocation (`runtime::invoker::resolve_rest_credential`), so an
/// in-place rewrite is a live rotation racing live traffic. #436 asks for a
/// rotation whose next invocation uses the new value; a torn read is that
/// rotation failing.
///
/// `rename` is the atomic publish on both platforms: POSIX defines it as atomic,
/// and Rust's Windows implementation passes `MOVEFILE_REPLACE_EXISTING`, so an
/// existing credential is replaced rather than colliding.
fn write_atomic_restricted(path: &Path, data: &[u8]) -> std::io::Result<()> {
    let dir = path.parent().unwrap_or_else(|| Path::new("."));
    let stem = path.file_name().unwrap_or_default().to_string_lossy();
    let serial = TEMP_SERIAL.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    // Same directory as the target: `rename` is only atomic within one filesystem,
    // and the system temp dir is routinely on another.
    let tmp = dir.join(format!(".{stem}.{}.{serial}.tmp", std::process::id()));

    write_restricted(&tmp, data)?;
    match std::fs::rename(&tmp, path) {
        Ok(()) => Ok(()),
        Err(e) => {
            // The credential never got published, so the scratch file is pure
            // litter — and litter holding a secret. Best-effort is right here:
            // the write's own error is the one worth reporting.
            let _ = std::fs::remove_file(&tmp);
            Err(e)
        }
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn account_name_with_alias() {
        assert_eq!(account_name("trimble-connect", None), "trimble-connect");
        assert_eq!(
            account_name("google-workspace", Some("personal")),
            "google-workspace.personal"
        );
    }

    #[test]
    fn file_fallback_round_trip() {
        let tmp = tempfile::tempdir().unwrap();
        let aware_home = tmp.path();
        let token = StoredToken {
            access_token: "tk_abc".repeat(300), // >2560 bytes when UTF-16 encoded
            refresh_token: Some("rt_xyz".into()),
            expires_at: 1_735_689_600,
            scope: "openid profile".into(),
            token_type: "Bearer".into(),
            integration: "microsoft-365".into(),
            obtained_at: 1_735_686_000,
            source: TokenSource::Paste,
        };

        // Write directly to file (simulates keyring-write-fail path).
        let body = serde_json::to_string(&token).unwrap();
        write_cred_file(aware_home, "microsoft-365", &body).unwrap();

        // load_token should find it via file fallback (keyring has no entry).
        let loaded = read_cred_file("microsoft-365", None, aware_home)
            .unwrap()
            .unwrap();
        assert_eq!(loaded.access_token, token.access_token);
        assert_eq!(loaded.refresh_token.as_deref(), Some("rt_xyz"));
    }

    #[test]
    fn file_fallback_legacy_format() {
        let tmp = tempfile::tempdir().unwrap();
        let aware_home = tmp.path();
        let cred_dir = aware_home.join("credentials");
        std::fs::create_dir_all(&cred_dir).unwrap();
        std::fs::write(
            cred_dir.join("trimble-connect.json"),
            r#"{"access_token":"manual_tk"}"#,
        )
        .unwrap();

        let loaded = read_cred_file("trimble-connect", None, aware_home)
            .unwrap()
            .unwrap();
        assert_eq!(loaded.access_token, "manual_tk");
        assert_eq!(loaded.source, TokenSource::Paste);
    }

    /// Replacing a credential publishes the new bytes whole and leaves no
    /// scratch file behind. A leftover `.tmp` is not cosmetic here — it holds
    /// the secret, outside the name anything ever cleans up.
    #[test]
    fn rewriting_a_credential_replaces_it_and_leaves_no_scratch_file() {
        let tmp = tempfile::tempdir().unwrap();
        write_cred_file(tmp.path(), "my-api", r#"{"access_token":"first"}"#).unwrap();
        write_cred_file(tmp.path(), "my-api", r#"{"access_token":"second"}"#).unwrap();

        let dir = tmp.path().join("credentials");
        let body = std::fs::read_to_string(dir.join("my-api.json")).unwrap();
        assert_eq!(body, r#"{"access_token":"second"}"#);

        let leftovers: Vec<String> = std::fs::read_dir(&dir)
            .unwrap()
            .flatten()
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|name| name != "my-api.json")
            .collect();
        assert!(
            leftovers.is_empty(),
            "atomic write left scratch files behind: {leftovers:?}"
        );
    }

    /// The publish is a rename, and the mode has to survive it. Writing the temp
    /// file with a plain `fs::write` would create it 0644 and rename would carry
    /// that straight onto the credential — a world-readable secret, with every
    /// other assertion in this module still green.
    #[cfg(unix)]
    #[test]
    fn an_atomically_written_credential_keeps_its_restricted_mode() {
        use std::os::unix::fs::PermissionsExt;
        let tmp = tempfile::tempdir().unwrap();
        write_cred_file(tmp.path(), "my-api", r#"{"access_token":"x"}"#).unwrap();
        let mode = std::fs::metadata(tmp.path().join("credentials").join("my-api.json"))
            .unwrap()
            .permissions()
            .mode();
        assert_eq!(mode & 0o777, 0o600, "got {:o}", mode & 0o777);
    }

    /// Deleting a credential that was never stored is success — the contract a
    /// caller tearing down on sign-out relies on, so it need not check first.
    #[test]
    fn delete_token_is_idempotent_when_nothing_is_stored() {
        let tmp = tempfile::tempdir().unwrap();
        delete_token("never-stored", None, tmp.path()).unwrap();
    }

    /// A revoke that could not remove the credential must not report success.
    /// It used to: the file removal was `let _ = remove_file(…)`, so the caller
    /// was told the credential was gone while `load_token` still returned it.
    ///
    /// The failure is forced with a *directory* at the credential's path rather
    /// than by removing write permission, because these tests also run as root,
    /// where a permission-denied unlink is not reachable at all.
    #[test]
    fn delete_token_reports_a_removal_it_could_not_perform() {
        let tmp = tempfile::tempdir().unwrap();
        let blocked = cred_file_path(tmp.path(), "stuck-api");
        std::fs::create_dir_all(&blocked).unwrap();

        let err = delete_token("stuck-api", None, tmp.path()).unwrap_err();
        assert!(
            matches!(&err, AwareError::PermissionDenied(m) if m.contains("stuck-api")),
            "expected the failed removal to be reported, got {err:?}"
        );
    }

    #[test]
    fn app_account_name_with_alias() {
        assert_eq!(
            app_account_name("microsoft-365", None),
            "oauth-app.microsoft-365"
        );
        assert_eq!(
            app_account_name("google-workspace", Some("staging")),
            "oauth-app.google-workspace.staging"
        );
    }

    #[test]
    fn app_secret_file_fallback_round_trip() {
        let tmp = tempfile::tempdir().unwrap();
        let account = app_account_name("google-workspace", None);
        write_app_secret_file(tmp.path(), &account, "s3cr3t").unwrap();
        let got = read_app_secret_file(tmp.path(), &account).unwrap();
        assert_eq!(got.as_deref(), Some("s3cr3t"));
    }

    #[test]
    fn app_secret_absent_is_none() {
        let tmp = tempfile::tempdir().unwrap();
        assert!(
            read_app_secret_file(tmp.path(), "oauth-app.nope")
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn load_app_secret_is_exact_account_no_cross_fallback() {
        // Secret stored only at the default slot must NOT be returned for an
        // alias lookup — the alias→default policy lives in config::with_profile,
        // keyed to the resolved profile scope, not in this exact-account lookup.
        let tmp = tempfile::tempdir().unwrap();
        write_app_secret_file(
            tmp.path(),
            &app_account_name("google-workspace", None),
            "default-secret",
        )
        .unwrap();
        assert_eq!(
            load_app_secret("google-workspace", Some("personal"), tmp.path()).unwrap(),
            None
        );
        assert_eq!(
            load_app_secret("google-workspace", None, tmp.path())
                .unwrap()
                .as_deref(),
            Some("default-secret")
        );
    }

    #[test]
    fn load_app_secret_uses_file_fallback_without_keychain_entry() {
        // Synthetic integration name → no keychain entry exists, so load_app_secret
        // must resolve via the file fallback (and never error if the keychain
        // backend is unavailable). Covers the headless/CI regression Codex flagged.
        let tmp = tempfile::tempdir().unwrap();
        let integration = "test-byo-fallback-xyz";
        let account = app_account_name(integration, None);
        write_app_secret_file(tmp.path(), &account, "file-secret").unwrap();
        let got = load_app_secret(integration, None, tmp.path()).unwrap();
        assert_eq!(got.as_deref(), Some("file-secret"));
    }

    #[test]
    fn cred_file_path_with_alias() {
        let home = Path::new("/home/user/.aware");
        assert_eq!(
            cred_file_path(home, "google-workspace.personal"),
            PathBuf::from("/home/user/.aware/credentials/google-workspace.personal.json")
        );
    }

    /// Only runs when `AWARE_TEST_KEYRING=1` because keyring tests are flaky
    /// on Linux without libsecret/dbus services.
    #[test]
    fn store_load_delete_round_trip_when_keyring_available() {
        if std::env::var("AWARE_TEST_KEYRING").is_err() {
            return;
        }
        let tmp = tempfile::tempdir().unwrap();
        let aware_home = tmp.path();
        let token = StoredToken {
            access_token: "tk_abc".into(),
            refresh_token: Some("rt_xyz".into()),
            expires_at: 1_735_689_600,
            scope: "openid profile".into(),
            token_type: "Bearer".into(),
            integration: "test-integration-keyring".into(),
            obtained_at: 1_735_686_000,
            source: TokenSource::Oauth,
        };
        store_token(&token, None, aware_home).unwrap();
        let loaded = load_token("test-integration-keyring", None, aware_home)
            .unwrap()
            .unwrap();
        assert_eq!(loaded.access_token, "tk_abc");
        delete_token("test-integration-keyring", None, aware_home).unwrap();
        let after = load_token("test-integration-keyring", None, aware_home).unwrap();
        assert!(after.is_none());
    }

    #[test]
    fn token_source_serializes_as_lowercase() {
        let t = StoredToken {
            access_token: "x".into(),
            refresh_token: None,
            expires_at: 0,
            scope: "".into(),
            token_type: "Bearer".into(),
            integration: "test".into(),
            obtained_at: 0,
            source: TokenSource::Paste,
        };
        let s = serde_json::to_string(&t).unwrap();
        assert!(s.contains(r#""source":"paste""#));
    }

    #[test]
    fn legacy_token_defaults_to_oauth_source() {
        let json = r#"{"access_token":"x","refresh_token":null,"expires_at":0,"scope":"","token_type":"Bearer","integration":"test","obtained_at":0}"#;
        let t: StoredToken = serde_json::from_str(json).unwrap();
        assert_eq!(t.source, TokenSource::Oauth);
    }
}
