//! Which aliased credential slots an AWARE home knows about (#665).
//!
//! The OS keychain cannot be enumerated, so `aware connect --list` used to show
//! only each integration's default slot: a slot written with `--as <alias>` was
//! invisible unless the person already knew its name. Every successful
//! `aware connect … --as <alias>` now records the alias in
//! `<home>/credentials/slots.json`, and `aware disconnect … --as <alias>`
//! removes it. Listing reads that index plus two sources that need no index:
//! a file-fallback credential `credentials/<integration>.<alias>.json` and a BYO
//! profile `oauth/<integration>.<alias>.yaml`.
//!
//! The index is a list of names, never a credential. It is per AWARE home while
//! the keychain is per user, so a slot another home connected is listed only
//! once this home has seen it by one of the three sources.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::error::AwareError;

fn index_path(aware_home: &Path) -> PathBuf {
    aware_home.join("credentials").join("slots.json")
}

/// The index as stored; an absent file is empty. A damaged one is an error,
/// so a write never silently discards the aliases it held.
fn read_index(aware_home: &Path) -> Result<BTreeMap<String, BTreeSet<String>>, AwareError> {
    match std::fs::read(index_path(aware_home)) {
        Ok(bytes) => serde_json::from_slice(&bytes).map_err(|e| {
            AwareError::Validation(format!(
                "credential slot index {}: {e}",
                index_path(aware_home).display()
            ))
        }),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(BTreeMap::new()),
        Err(e) => Err(e.into()),
    }
}

fn write_index(
    aware_home: &Path,
    index: &BTreeMap<String, BTreeSet<String>>,
) -> Result<(), AwareError> {
    let path = index_path(aware_home);
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension(format!("json.{}.tmp", std::process::id()));
    std::fs::write(&tmp, serde_json::to_vec_pretty(index)?)?;
    std::fs::rename(&tmp, &path).inspect_err(|_| {
        let _ = std::fs::remove_file(&tmp);
    })?;
    Ok(())
}

/// Record that `<integration>.<alias>` holds a credential written by this home.
pub fn remember(aware_home: &Path, integration: &str, alias: &str) -> Result<(), AwareError> {
    let mut index = read_index(aware_home)?;
    if index
        .entry(integration.to_string())
        .or_default()
        .insert(alias.to_string())
    {
        write_index(aware_home, &index)?;
    }
    Ok(())
}

/// Drop `<integration>.<alias>` from the index (a disconnect).
pub fn forget(aware_home: &Path, integration: &str, alias: &str) -> Result<(), AwareError> {
    let mut index = read_index(aware_home)?;
    let Some(aliases) = index.get_mut(integration) else {
        return Ok(());
    };
    if aliases.remove(alias) {
        if aliases.is_empty() {
            index.remove(integration);
        }
        write_index(aware_home, &index)?;
    }
    Ok(())
}

/// Every alias of `integration` this home knows, sorted and without duplicates:
/// the index, file-fallback credentials, and alias-specific BYO profiles.
///
/// Best-effort by design: listing is diagnostic, so an unreadable index or
/// directory contributes nothing rather than hiding the slots the other sources
/// found; `warnings` says what could not be read.
pub fn known_aliases(aware_home: &Path, integration: &str) -> (Vec<String>, Vec<String>) {
    let mut aliases = BTreeSet::new();
    let mut warnings = Vec::new();
    match read_index(aware_home) {
        Ok(mut index) => aliases.extend(index.remove(integration).unwrap_or_default()),
        Err(e) => warnings.push(e.to_string()),
    }
    let prefix = format!("{integration}.");
    for (dir, suffix) in [("credentials", ".json"), ("oauth", ".yaml")] {
        let entries = match std::fs::read_dir(aware_home.join(dir)) {
            Ok(entries) => entries,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => {
                warnings.push(format!("{}: {e}", aware_home.join(dir).display()));
                continue;
            }
        };
        for entry in entries.flatten() {
            let name = entry.file_name();
            let Some(name) = name.to_str() else { continue };
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
        remember(home.path(), "google-workspace", "team").unwrap();
        remember(home.path(), "google-workspace", "team").unwrap();
        remember(home.path(), "microsoft-365", "work").unwrap();
        assert_eq!(
            known_aliases(home.path(), "google-workspace").0,
            ["team", "uat618"]
        );
        assert_eq!(known_aliases(home.path(), "microsoft-365").0, ["work"]);
        forget(home.path(), "google-workspace", "team").unwrap();
        forget(home.path(), "google-workspace", "never").unwrap();
        assert_eq!(known_aliases(home.path(), "google-workspace").0, ["uat618"]);
        forget(home.path(), "microsoft-365", "work").unwrap();
        assert!(known_aliases(home.path(), "microsoft-365").0.is_empty());
    }

    #[test]
    fn credential_files_and_profiles_name_aliases_without_the_index() {
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
        let (aliases, warnings) = known_aliases(home.path(), "google-workspace");
        assert_eq!(aliases, ["byo", "filed"]);
        assert!(warnings.is_empty());
        assert_eq!(known_aliases(home.path(), "trimble-connect").0, ["site"]);
    }

    #[test]
    fn a_damaged_index_is_reported_and_never_overwritten() {
        let home = tempfile::tempdir().unwrap();
        let path = index_path(home.path());
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, "not json").unwrap();
        std::fs::write(
            home.path()
                .join("credentials")
                .join("google-workspace.filed.json"),
            "{}",
        )
        .unwrap();
        let (aliases, warnings) = known_aliases(home.path(), "google-workspace");
        assert_eq!(aliases, ["filed"]);
        assert_eq!(warnings.len(), 1);
        assert!(remember(home.path(), "google-workspace", "x").is_err());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "not json");
    }
}
