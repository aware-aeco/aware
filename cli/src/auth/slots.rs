//! Which aliased credential slots an AWARE home knows about (#665).
//!
//! The OS keychain cannot be enumerated, so `aware connect --list` used to show
//! only each integration's default slot: a slot written with `--as <alias>` was
//! invisible unless the person already knew its name. Every successful
//! `aware connect … --as <alias>` now leaves an empty marker file
//! `<home>/credentials/slots/<integration>/<hex(alias)>`, and
//! `aware disconnect … --as <alias>` removes it. One file per slot, created and
//! removed whole, so concurrent connects never lose each other's entries (a
//! shared index would need read-modify-write). The alias is hex-encoded because
//! `--as` accepts characters no file name can hold.
//!
//! Listing reads those markers plus two sources that need none: a file-fallback
//! credential `credentials/<integration>.<alias>.json` and a BYO profile
//! `oauth/<integration>.<alias>.yaml`.
//!
//! A marker holds no credential, only the slot's name. Markers are per AWARE
//! home while the keychain is per user, so a slot another home connected is
//! listed only once this home has seen it through one of the three sources.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::error::AwareError;

fn marker_dir(aware_home: &Path, integration: &str) -> PathBuf {
    aware_home
        .join("credentials")
        .join("slots")
        .join(integration)
}

fn encode(alias: &str) -> String {
    alias.bytes().map(|b| format!("{b:02x}")).collect()
}

fn decode(name: &str) -> Option<String> {
    if name.is_empty() || !name.len().is_multiple_of(2) {
        return None;
    }
    let bytes = (0..name.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(name.get(i..i + 2)?, 16).ok())
        .collect::<Option<Vec<u8>>>()?;
    String::from_utf8(bytes)
        .ok()
        .filter(|alias| !alias.is_empty())
}

/// Record that `<integration>.<alias>` holds a credential written by this home.
pub fn remember(aware_home: &Path, integration: &str, alias: &str) -> Result<(), AwareError> {
    let dir = marker_dir(aware_home, integration);
    std::fs::create_dir_all(&dir)?;
    std::fs::write(dir.join(encode(alias)), b"")?;
    Ok(())
}

/// Drop `<integration>.<alias>`'s marker (a disconnect). An absent marker is
/// already forgotten.
pub fn forget(aware_home: &Path, integration: &str, alias: &str) -> Result<(), AwareError> {
    match std::fs::remove_file(marker_dir(aware_home, integration).join(encode(alias))) {
        Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(e.into()),
        _ => Ok(()),
    }
}

/// Every alias of `integration` this home knows, sorted and without duplicates:
/// slot markers, file-fallback credentials, and alias-specific BYO profiles.
///
/// Best-effort by design: listing is diagnostic, so a directory that cannot be
/// read contributes nothing rather than hiding what the other sources found;
/// the second value says what could not be read.
pub fn known_aliases(aware_home: &Path, integration: &str) -> (Vec<String>, Vec<String>) {
    let mut aliases = BTreeSet::new();
    let mut warnings = Vec::new();
    let mut names = |dir: PathBuf| -> Vec<String> {
        match std::fs::read_dir(&dir) {
            Ok(entries) => entries
                .flatten()
                .filter_map(|entry| entry.file_name().to_str().map(str::to_string))
                .collect(),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Vec::new(),
            Err(e) => {
                warnings.push(format!("{}: {e}", dir.display()));
                Vec::new()
            }
        }
    };
    aliases.extend(
        names(marker_dir(aware_home, integration))
            .iter()
            .filter_map(|name| decode(name)),
    );
    let prefix = format!("{integration}.");
    for (dir, suffix) in [("credentials", ".json"), ("oauth", ".yaml")] {
        for name in names(aware_home.join(dir)) {
            if let Some(alias) = name
                .strip_prefix(&prefix)
                .and_then(|rest| rest.strip_suffix(suffix))
                .filter(|alias| !alias.is_empty())
            {
                aliases.insert(alias.to_string());
            }
        }
    }
    (aliases.into_iter().collect(), warnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remembered_aliases_are_listed_and_forgotten() {
        let home = tempfile::tempdir().unwrap();
        assert!(known_aliases(home.path(), "google-workspace").0.is_empty());
        remember(home.path(), "google-workspace", "uat618").unwrap();
        remember(home.path(), "google-workspace", "team one/..\\x").unwrap();
        remember(home.path(), "google-workspace", "uat618").unwrap();
        remember(home.path(), "microsoft-365", "work").unwrap();
        assert_eq!(
            known_aliases(home.path(), "google-workspace").0,
            ["team one/..\\x", "uat618"]
        );
        assert_eq!(known_aliases(home.path(), "microsoft-365").0, ["work"]);
        forget(home.path(), "google-workspace", "team one/..\\x").unwrap();
        forget(home.path(), "google-workspace", "never").unwrap();
        assert_eq!(known_aliases(home.path(), "google-workspace").0, ["uat618"]);
        forget(home.path(), "microsoft-365", "work").unwrap();
        assert!(known_aliases(home.path(), "microsoft-365").0.is_empty());
    }

    /// Markers are independent files: many concurrent connects each keep theirs.
    #[test]
    fn concurrent_connects_never_lose_a_slot() {
        let home = tempfile::tempdir().unwrap();
        let threads: Vec<_> = (0..16)
            .map(|i| {
                let home = home.path().to_path_buf();
                std::thread::spawn(move || {
                    remember(&home, "google-workspace", &format!("a{i:02}")).unwrap()
                })
            })
            .collect();
        for thread in threads {
            thread.join().unwrap();
        }
        assert_eq!(known_aliases(home.path(), "google-workspace").0.len(), 16);
    }

    #[test]
    fn credential_files_and_profiles_name_aliases_without_a_marker() {
        let home = tempfile::tempdir().unwrap();
        let creds = home.path().join("credentials");
        let oauth = home.path().join("oauth");
        std::fs::create_dir_all(&creds).unwrap();
        std::fs::create_dir_all(&oauth).unwrap();
        for name in [
            "google-workspace.json",
            "google-workspace.filed.json",
            "google-workspace..json",
            "google-workspace-other.x.json",
            ".generation-abc.lock",
            "oauth-app.google-workspace.secret.json",
            "trimble-connect.site.json",
        ] {
            std::fs::write(creds.join(name), "{}").unwrap();
        }
        std::fs::write(oauth.join("google-workspace.byo.yaml"), "client_id: x\n").unwrap();
        std::fs::write(oauth.join("google-workspace.yaml"), "client_id: x\n").unwrap();
        // A stray file in the marker directory is not a slot.
        let markers = marker_dir(home.path(), "google-workspace");
        std::fs::create_dir_all(&markers).unwrap();
        std::fs::write(markers.join("not-hex"), "").unwrap();
        let (aliases, warnings) = known_aliases(home.path(), "google-workspace");
        assert_eq!(aliases, ["byo", "filed"]);
        assert!(warnings.is_empty());
        assert_eq!(known_aliases(home.path(), "trimble-connect").0, ["site"]);
    }

    #[test]
    fn alias_names_round_trip_through_the_marker_encoding() {
        for alias in ["a", "uat618", "team one", "é", "../x"] {
            assert_eq!(decode(&encode(alias)).as_deref(), Some(alias));
            assert!(encode(alias).bytes().all(|b| b.is_ascii_hexdigit()));
        }
        for bad in ["", "a", "zz", "c3"] {
            assert_eq!(decode(bad), None, "{bad}");
        }
    }
}
